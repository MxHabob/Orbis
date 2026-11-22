use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::gateways::PaymentGatewayPort;
use orbis_domain::errors::DomainError;
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use uuid::Uuid;

const PAYMENTS_URL: &str = "https://api.moyasar.com/v1/payments";

pub struct MoyasarAdapter {
    pub secret_key: String,
    pub client: Client,
}

impl MoyasarAdapter {
    pub fn new(secret_key: String) -> Self {
        Self {
            secret_key,
            client: Client::new(),
        }
    }

    async fn find_payment(
        &self,
        idempotency_key: Uuid,
        amount_minor: i64,
        currency: &str,
    ) -> Result<Option<String>, DomainError> {
        let response = self
            .client
            .get(format!("{PAYMENTS_URL}/{idempotency_key}"))
            .basic_auth(&self.secret_key, Some(""))
            .send()
            .await
            .map_err(|error| {
                DomainError::Validation(format!(
                    "Moyasar payment status could not be verified; retry with the same idempotency key: {error}"
                ))
            })?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !response.status().is_success() {
            return Err(DomainError::Validation(format!(
                "Moyasar payment status lookup failed with HTTP {}",
                response.status()
            )));
        }

        let payment: Value = response.json().await.map_err(|error| {
            DomainError::Validation(format!(
                "Moyasar returned an invalid payment status response: {error}"
            ))
        })?;
        validate_payment(&payment, idempotency_key, amount_minor, currency).map(Some)
    }
}

#[async_trait]
impl PaymentGatewayPort for MoyasarAdapter {
    async fn process_payment(
        &self,
        _ctx: &RequestContext,
        amount: f64,
        currency: &str,
        source: &str,
        idempotency_key: Uuid,
    ) -> Result<String, DomainError> {
        if !amount.is_finite() || amount <= 0.0 {
            return Err(DomainError::Validation(
                "Payment amount must be a finite positive value".into(),
            ));
        }
        let amount_minor_value = amount * 100.0;
        let amount_minor = amount_minor_value.round() as i64;
        if amount_minor <= 0 {
            return Err(DomainError::Validation(
                "Payment amount is below the gateway's minimum unit".into(),
            ));
        }
        if (amount_minor_value - amount_minor as f64).abs() > 0.000_001 {
            return Err(DomainError::Validation(
                "Moyasar payments support at most two fractional currency digits".into(),
            ));
        }

        let source_value: Value = serde_json::from_str(source)
            .unwrap_or_else(|_| json!({"type": "token", "token": source}));
        let payload = json!({
            "given_id": idempotency_key,
            "amount": amount_minor,
            "currency": currency,
            "source": source_value,
            "description": "Orbis payment"
        });

        let response = match self
            .client
            .post(PAYMENTS_URL)
            .basic_auth(&self.secret_key, Some(""))
            .json(&payload)
            .send()
            .await
        {
            Ok(response) => response,
            Err(error) => {
                if let Some(payment_id) = self
                    .find_payment(idempotency_key, amount_minor, currency)
                    .await?
                {
                    return Ok(payment_id);
                }
                return Err(DomainError::Validation(format!(
                    "Moyasar request failed; retry with the same idempotency key: {error}"
                )));
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            if let Some(payment_id) = self
                .find_payment(idempotency_key, amount_minor, currency)
                .await?
            {
                return Ok(payment_id);
            }
            return Err(DomainError::Validation(format!(
                "Moyasar payment request failed with HTTP {status}"
            )));
        }

        let payment: Value = response.json().await.map_err(|error| {
            DomainError::Validation(format!(
                "Moyasar response could not be verified; retry with the same idempotency key: {error}"
            ))
        })?;
        validate_payment(&payment, idempotency_key, amount_minor, currency)
    }
}

fn validate_payment(
    payment: &Value,
    idempotency_key: Uuid,
    amount_minor: i64,
    currency: &str,
) -> Result<String, DomainError> {
    let payment_id = payment["id"].as_str().ok_or_else(|| {
        DomainError::Validation("Moyasar response is missing the payment ID".into())
    })?;
    if payment_id != idempotency_key.to_string() {
        return Err(DomainError::Validation(
            "Moyasar returned a payment ID that does not match the idempotency key".into(),
        ));
    }
    if payment["amount"].as_i64() != Some(amount_minor)
        || !payment["currency"]
            .as_str()
            .is_some_and(|value| value.eq_ignore_ascii_case(currency))
    {
        return Err(DomainError::Validation(
            "Moyasar payment does not match the requested amount and currency".into(),
        ));
    }
    if payment["status"].as_str() != Some("paid") {
        return Err(DomainError::Validation(
            "Moyasar payment is not in the paid state".into(),
        ));
    }

    Ok(payment_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::validate_payment;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn accepts_only_matching_paid_payment() {
        let id = Uuid::new_v4();
        let payment = json!({
            "id": id.to_string(),
            "amount": 1250,
            "currency": "SAR",
            "status": "paid"
        });

        assert_eq!(
            validate_payment(&payment, id, 1250, "sar").unwrap(),
            id.to_string()
        );
    }

    #[test]
    fn rejects_non_paid_payment() {
        let id = Uuid::new_v4();
        let payment = json!({
            "id": id.to_string(),
            "amount": 1250,
            "currency": "SAR",
            "status": "initiated"
        });

        assert!(validate_payment(&payment, id, 1250, "SAR").is_err());
    }

    #[test]
    fn rejects_payment_with_different_amount_or_currency() {
        let id = Uuid::new_v4();
        let payment = json!({
            "id": id.to_string(),
            "amount": 1250,
            "currency": "SAR",
            "status": "paid"
        });

        assert!(validate_payment(&payment, id, 1251, "SAR").is_err());
        assert!(validate_payment(&payment, id, 1250, "YER").is_err());
    }
}
