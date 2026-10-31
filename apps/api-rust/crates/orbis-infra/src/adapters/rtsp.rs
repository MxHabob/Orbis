use orbis_domain::errors::DomainError;
use serde_json::Value;
use std::collections::HashMap;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use url::Url;

use super::hikvision::{digest_authorization, parse_digest_challenge};

const RTSP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(12);
const VIDEO_PACKET_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(4);
const MAX_RTSP_HEADERS: usize = 16 * 1024;
const MAX_RTSP_BODY: usize = 64 * 1024;

pub struct RtspAdapter;

impl RtspAdapter {
    pub async fn check_connection(
        settings: &Value,
        secrets: &HashMap<String, String>,
    ) -> Result<(), DomainError> {
        let endpoint = settings
            .get("rtsp_url")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|endpoint| !endpoint.is_empty())
            .ok_or_else(|| {
                DomainError::Validation(
                    "The RTSP integration is missing the required 'rtsp_url' setting".into(),
                )
            })?;
        let url = Url::parse(endpoint)
            .map_err(|_| DomainError::Validation("RTSP stream URL is invalid".into()))?;
        if url.scheme() != "rtsp"
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.fragment().is_some()
        {
            return Err(DomainError::Validation(
                "RTSP stream URL must use rtsp:// and must not contain embedded credentials or a fragment"
                    .into(),
            ));
        }
        let username = settings
            .get("username")
            .and_then(Value::as_str)
            .map(str::trim)
            .unwrap_or_default();
        let password = secrets
            .get("password")
            .map(String::as_str)
            .unwrap_or_default();

        tokio::time::timeout(RTSP_TIMEOUT, verify_video_source(&url, username, password))
            .await
            .map_err(|_| {
                DomainError::Validation("RTSP camera connection or video check timed out".into())
            })?
    }
}

fn digest_challenge(headers: &[String]) -> Result<HashMap<String, String>, DomainError> {
    let challenge = header_value(headers, "www-authenticate")
        .filter(|value| {
            value
                .get(..7)
                .is_some_and(|scheme| scheme.eq_ignore_ascii_case("digest "))
        })
        .ok_or_else(|| {
            DomainError::Validation(
                "RTSP camera must provide a Digest authentication challenge".into(),
            )
        })?;
    parse_digest_challenge(challenge)
}

async fn verify_video_source(url: &Url, username: &str, password: &str) -> Result<(), DomainError> {
    let mut connection = RtspConnection::connect(url).await?;
    let describe = connection
        .request("DESCRIBE", url, None, &[("Accept", "application/sdp")])
        .await?;
    let mut authorization = if describe.status_code == 401 {
        if username.is_empty() && password.is_empty() {
            return Err(DomainError::Validation(
                "This RTSP stream requires authentication; add the camera username and/or password"
                    .into(),
            ));
        }
        let challenge = digest_challenge(&describe.headers)?;
        let authorization = digest_authorization(
            &challenge,
            "DESCRIBE",
            url.as_str(),
            username,
            password,
            &hex::encode(rand::random::<[u8; 16]>()),
        )?;
        connection = RtspConnection::connect(url).await?;
        let authenticated = connection
            .request(
                "DESCRIBE",
                url,
                Some(&authorization),
                &[("Accept", "application/sdp")],
            )
            .await?;
        if authenticated.status_code == 401 {
            return Err(DomainError::Validation(
                "RTSP camera rejected the supplied credentials".into(),
            ));
        }
        if !(200..300).contains(&authenticated.status_code) {
            return Err(rtsp_status_error("DESCRIBE", authenticated.status_code));
        }
        Some((challenge, username.to_owned(), password.to_owned()))
    } else {
        if !(200..300).contains(&describe.status_code) {
            return Err(rtsp_status_error("DESCRIBE", describe.status_code));
        }
        None
    };

    let sdp = std::str::from_utf8(&connection.last_body).map_err(|_| {
        DomainError::Validation("RTSP camera returned an invalid SDP video description".into())
    })?;
    if !sdp.lines().any(|line| line.starts_with("m=video ")) {
        return Err(DomainError::Validation(
            "RTSP source responded but did not advertise a video track".into(),
        ));
    }

    let base_url = header_value(&connection.last_headers, "content-base")
        .and_then(|value| Url::parse(value.trim()).ok())
        .unwrap_or_else(|| url.clone());
    if !same_rtsp_origin(url, &base_url)
        || !base_url.username().is_empty()
        || base_url.password().is_some()
    {
        return Err(DomainError::Validation(
            "RTSP camera returned an invalid stream control address".into(),
        ));
    }
    let video_control = video_control_attribute(sdp);
    let track_url = video_control
        .as_deref()
        .filter(|control| *control != "*")
        .and_then(|control| base_url.join(control).ok())
        .unwrap_or_else(|| base_url.clone());
    if !same_rtsp_origin(url, &track_url)
        || !track_url.username().is_empty()
        || track_url.password().is_some()
    {
        return Err(DomainError::Validation(
            "RTSP camera returned an invalid video track address".into(),
        ));
    }
    let mut digest = authorization;
    let setup_authorization = digest
        .as_ref()
        .map(|(challenge, username, password)| {
            digest_authorization(
                challenge,
                "SETUP",
                track_url.as_str(),
                username,
                password,
                &hex::encode(rand::random::<[u8; 16]>()),
            )
        })
        .transpose()?;
    let mut setup = connection
        .request(
            "SETUP",
            &track_url,
            setup_authorization.as_deref(),
            &[("Transport", "RTP/AVP/TCP;unicast;interleaved=0-1")],
        )
        .await?;
    if setup.status_code == 401 {
        if username.is_empty() && password.is_empty() {
            return Err(authentication_error(false));
        }
        let challenge = digest_challenge(&setup.headers)?;
        let setup_auth = digest_authorization(
            &challenge,
            "SETUP",
            track_url.as_str(),
            username,
            password,
            &hex::encode(rand::random::<[u8; 16]>()),
        )?;
        setup = connection
            .request(
                "SETUP",
                &track_url,
                Some(&setup_auth),
                &[("Transport", "RTP/AVP/TCP;unicast;interleaved=0-1")],
            )
            .await?;
        digest = Some((challenge, username.to_owned(), password.to_owned()));
    }
    if !(200..300).contains(&setup.status_code) {
        if setup.status_code == 401 {
            return Err(authentication_error(
                !username.is_empty() || !password.is_empty(),
            ));
        }
        return Err(rtsp_status_error("SETUP", setup.status_code));
    }
    let session_id = header_value(&setup.headers, "session")
        .and_then(|value| value.split(';').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            DomainError::Validation("RTSP camera did not create a streaming session".into())
        })?;
    let video_channel = header_value(&setup.headers, "transport")
        .and_then(|value| interleaved_video_channel(value))
        .unwrap_or(0);
    let play_authorization = digest
        .as_ref()
        .map(|(challenge, username, password)| {
            digest_authorization(
                challenge,
                "PLAY",
                base_url.as_str(),
                username,
                password,
                &hex::encode(rand::random::<[u8; 16]>()),
            )
        })
        .transpose()?;
    let mut play = connection
        .request_with_session("PLAY", &base_url, play_authorization.as_deref(), session_id)
        .await?;
    if play.status_code == 401 {
        if username.is_empty() && password.is_empty() {
            return Err(authentication_error(false));
        }
        let challenge = digest_challenge(&play.headers)?;
        let play_auth = digest_authorization(
            &challenge,
            "PLAY",
            base_url.as_str(),
            username,
            password,
            &hex::encode(rand::random::<[u8; 16]>()),
        )?;
        play = connection
            .request_with_session("PLAY", &base_url, Some(&play_auth), session_id)
            .await?;
    }
    if !(200..300).contains(&play.status_code) {
        if play.status_code == 401 {
            return Err(authentication_error(
                !username.is_empty() || !password.is_empty(),
            ));
        }
        return Err(rtsp_status_error("PLAY", play.status_code));
    }
    tokio::time::timeout(
        VIDEO_PACKET_TIMEOUT,
        connection.wait_for_video_packet(video_channel),
    )
    .await
    .map_err(|_| {
        DomainError::Validation(
            "RTSP camera has a video track but produced no video packets".into(),
        )
    })??;
    Ok(())
}

struct RtspResponse {
    status_code: u16,
    headers: Vec<String>,
}

struct RtspConnection {
    reader: BufReader<TcpStream>,
    cseq: u32,
    last_headers: Vec<String>,
    last_body: Vec<u8>,
}

impl RtspConnection {
    async fn connect(url: &Url) -> Result<Self, DomainError> {
        let host = url
            .host_str()
            .ok_or_else(|| DomainError::Validation("RTSP stream URL has no host".into()))?;
        let port = url.port().unwrap_or(554);
        let stream = TcpStream::connect((host, port))
            .await
            .map_err(|_| DomainError::Validation("Could not reach the RTSP camera".into()))?;
        Ok(Self {
            reader: BufReader::new(stream),
            cseq: 0,
            last_headers: Vec::new(),
            last_body: Vec::new(),
        })
    }

    async fn request(
        &mut self,
        method: &str,
        url: &Url,
        authorization: Option<&str>,
        headers: &[(&str, &str)],
    ) -> Result<RtspResponse, DomainError> {
        self.cseq += 1;
        let mut request = format!(
            "{method} {url} RTSP/1.0\r\nCSeq: {}\r\nUser-Agent: Orbis/1.0\r\n",
            self.cseq
        );
        if let Some(authorization) = authorization {
            request.push_str(&format!("Authorization: {authorization}\r\n"));
        }
        for (name, value) in headers {
            request.push_str(&format!("{name}: {value}\r\n"));
        }
        request.push_str("\r\n");
        self.reader
            .get_mut()
            .write_all(request.as_bytes())
            .await
            .map_err(|_| DomainError::Validation("Could not query the RTSP camera".into()))?;
        self.read_response().await
    }

    async fn request_with_session(
        &mut self,
        method: &str,
        url: &Url,
        authorization: Option<&str>,
        session_id: &str,
    ) -> Result<RtspResponse, DomainError> {
        self.request(method, url, authorization, &[("Session", session_id)])
            .await
    }

    async fn read_response(&mut self) -> Result<RtspResponse, DomainError> {
        let mut lines = Vec::new();
        let mut total_bytes = 0;
        loop {
            let mut line = Vec::new();
            let bytes = self
                .reader
                .read_until(b'\n', &mut line)
                .await
                .map_err(|_| DomainError::Validation("Could not read the RTSP response".into()))?;
            if bytes == 0 {
                return Err(DomainError::Validation(
                    "RTSP camera closed the connection unexpectedly".into(),
                ));
            }
            total_bytes += bytes;
            if total_bytes > MAX_RTSP_HEADERS {
                return Err(DomainError::Validation(
                    "RTSP camera returned oversized response headers".into(),
                ));
            }
            let line = String::from_utf8_lossy(&line).trim().to_owned();
            if line.is_empty() {
                break;
            }
            lines.push(line);
        }
        let status_code = lines
            .first()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|status| status.parse::<u16>().ok())
            .ok_or_else(|| {
                DomainError::Validation("RTSP camera returned an invalid response".into())
            })?;
        let content_length = header_value(&lines, "content-length")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0);
        if content_length > MAX_RTSP_BODY {
            return Err(DomainError::Validation(
                "RTSP camera returned an oversized stream description".into(),
            ));
        }
        self.last_body.resize(content_length, 0);
        if content_length > 0 {
            self.reader
                .read_exact(&mut self.last_body)
                .await
                .map_err(|_| {
                    DomainError::Validation("Could not read the RTSP stream description".into())
                })?;
        }
        self.last_headers = lines.clone();
        Ok(RtspResponse {
            status_code,
            headers: lines,
        })
    }

    async fn wait_for_video_packet(&mut self, channel: u8) -> Result<(), DomainError> {
        loop {
            let mut marker = [0; 1];
            self.reader.read_exact(&mut marker).await.map_err(|_| {
                DomainError::Validation("RTSP camera stopped sending video packets".into())
            })?;
            if marker[0] != b'$' {
                continue;
            }
            let mut interleaved_header = [0; 3];
            self.reader
                .read_exact(&mut interleaved_header)
                .await
                .map_err(|_| {
                    DomainError::Validation("RTSP camera returned a truncated video packet".into())
                })?;
            let packet_channel = interleaved_header[0];
            let packet_size =
                u16::from_be_bytes([interleaved_header[1], interleaved_header[2]]) as usize;
            if packet_size > MAX_RTSP_BODY {
                return Err(DomainError::Validation(
                    "RTSP camera returned an oversized RTP packet".into(),
                ));
            }
            let mut packet = vec![0; packet_size];
            self.reader.read_exact(&mut packet).await.map_err(|_| {
                DomainError::Validation("RTSP camera returned a truncated RTP packet".into())
            })?;
            if packet_channel == channel
                && packet.len() > 12
                && packet.first().is_some_and(|byte| byte >> 6 == 2)
            {
                return Ok(());
            }
        }
    }
}

fn header_value<'a>(headers: &'a [String], wanted: &str) -> Option<&'a str> {
    headers.iter().find_map(|header| {
        let (name, value) = header.split_once(':')?;
        name.eq_ignore_ascii_case(wanted).then_some(value.trim())
    })
}

fn same_rtsp_origin(left: &Url, right: &Url) -> bool {
    left.scheme() == right.scheme()
        && left
            .host_str()
            .zip(right.host_str())
            .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
        && left.port_or_known_default() == right.port_or_known_default()
}

fn authentication_error(credentials_supplied: bool) -> DomainError {
    DomainError::Validation(if credentials_supplied {
        "RTSP camera rejected the supplied credentials".into()
    } else {
        "This RTSP stream requires authentication; add the camera username and/or password".into()
    })
}

fn video_control_attribute(sdp: &str) -> Option<String> {
    let mut in_video_media = false;
    for line in sdp.lines() {
        if let Some(media) = line.strip_prefix("m=") {
            in_video_media = media.starts_with("video ");
        } else if in_video_media && let Some(control) = line.strip_prefix("a=control:") {
            return Some(control.trim().to_owned());
        }
    }
    None
}

fn interleaved_video_channel(transport: &str) -> Option<u8> {
    let channels = transport
        .split(';')
        .find_map(|parameter| parameter.trim().strip_prefix("interleaved="))?;
    channels
        .split('-')
        .next()
        .and_then(|channel| channel.parse().ok())
}

fn rtsp_status_error(method: &str, status_code: u16) -> DomainError {
    DomainError::Validation(format!(
        "RTSP camera rejected the {method} video check (RTSP {status_code})"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn rejects_rtsp_urls_with_embedded_credentials() {
        let settings = serde_json::json!({
            "rtsp_url": "rtsp://user:secret@camera.example.com/stream",
            "username": "user"
        });
        assert!(
            RtspAdapter::check_connection(
                &settings,
                &HashMap::from([("password".to_owned(), "secret".to_owned())])
            )
            .await
            .is_err()
        );
    }

    #[test]
    fn parses_only_video_track_control_attributes() {
        let sdp = "v=0\r\nm=audio 0 RTP/AVP 0\r\na=control:audio\r\nm=video 0 RTP/AVP 96\r\na=control:trackID=2\r\n";
        assert_eq!(video_control_attribute(sdp).as_deref(), Some("trackID=2"));
    }

    #[test]
    fn parses_interleaved_video_channel() {
        assert_eq!(
            interleaved_video_channel("RTP/AVP/TCP;unicast;interleaved=2-3"),
            Some(2)
        );
    }
}
