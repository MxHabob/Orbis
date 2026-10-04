import * as m from "$lib/paraglide/messages";

/**
 * Formats AI and LLM provider errors into clear, informative, and actionable messages in Arabic.
 */
export function formatAiErrorMessage(error: unknown): string {
  if (!error) {
    return m.ai_ui_error_unknown();
  }

  let raw = "";
  if (error instanceof Error) {
    raw = error.message;
  } else if (typeof error === "string") {
    raw = error;
  } else if (typeof error === "object") {
    raw = (error as any).message || (error as any).error || JSON.stringify(error);
  } else {
    raw = String(error);
  }

  // Attempt to extract inner JSON payload (e.g. Gemini / OpenAI / Anthropic error JSON)
  let parsedError: any = null;
  const jsonMatch = raw.match(/\{[\s\S]*\}/);
  if (jsonMatch) {
    try {
      parsedError = JSON.parse(jsonMatch[0]);
    } catch {
      // Not strictly JSON, proceed with regex matching
    }
  }

  const errObj = parsedError?.error || parsedError || {};
  const code = Number(errObj.code || errObj.status_code) || 0;
  const status = String(errObj.status || "").toUpperCase();
  const innerMsg = String(errObj.message || "").trim();

  // 1. Quota & Rate Limits (HTTP 429 / RESOURCE_EXHAUSTED)
  if (
    code === 429 ||
    status === "RESOURCE_EXHAUSTED" ||
    raw.includes("RESOURCE_EXHAUSTED") ||
    raw.includes("429") ||
    raw.toLowerCase().includes("quota exceeded") ||
    raw.toLowerCase().includes("rate limit")
  ) {
    return [
      m.ai_ui_error_quota_title(),
      "",
      m.ai_ui_error_quota_description(),
      "",
      m.ai_ui_error_provider_detail({ details: innerMsg || m.ai_ui_error_quota_provider_fallback() }),
      "",
      m.ai_ui_error_quota_solution(),
    ].join("\n");
  }

  // 2. High Demand / Service Unavailable (HTTP 503 / 500 / UNAVAILABLE / Overloaded)
  if (
    code === 503 ||
    status === "UNAVAILABLE" ||
    raw.includes("UNAVAILABLE") ||
    raw.includes("503") ||
    raw.toLowerCase().includes("high demand") ||
    raw.toLowerCase().includes("overloaded")
  ) {
    return [
      m.ai_ui_error_overloaded_title(),
      "",
      m.ai_ui_error_overloaded_description(),
      "",
      m.ai_ui_error_provider_detail({ details: innerMsg || m.ai_ui_error_overloaded_provider_fallback() }),
      "",
      m.ai_ui_error_overloaded_solution(),
    ].join("\n");
  }

  // 3. Invalid API Key / Authentication (HTTP 401 / 403 / API_KEY_INVALID / PERMISSION_DENIED)
  if (
    code === 401 ||
    code === 403 ||
    status === "PERMISSION_DENIED" ||
    raw.includes("API_KEY_INVALID") ||
    raw.toLowerCase().includes("api key not valid") ||
    raw.toLowerCase().includes("unauthorized")
  ) {
    return [
      m.ai_ui_error_auth_title(),
      "",
      m.ai_ui_error_auth_description(),
      "",
      m.ai_ui_error_detail({ details: innerMsg || m.ai_ui_error_auth_provider_fallback() }),
      "",
      m.ai_ui_error_auth_solution(),
    ].join("\n");
  }

  // 4. Model Not Found / Deprecated (HTTP 404 / MODEL_NOT_FOUND)
  if (
    code === 404 ||
    status === "NOT_FOUND" ||
    raw.includes("404") ||
    raw.toLowerCase().includes("not found") ||
    raw.toLowerCase().includes("is not supported")
  ) {
    return [
      m.ai_ui_error_model_not_found_title(),
      "",
      m.ai_ui_error_model_not_found_description(),
      "",
      m.ai_ui_error_detail({ details: innerMsg || raw }),
      "",
      m.ai_ui_error_model_not_found_solution(),
    ].join("\n");
  }

  // 5. Network / Connection Refused / Timeout
  if (
    raw.toLowerCase().includes("network error") ||
    raw.toLowerCase().includes("failed to fetch") ||
    raw.toLowerCase().includes("connection refused") ||
    raw.toLowerCase().includes("timeout")
  ) {
    return [
      m.ai_ui_error_network_title(),
      "",
      m.ai_ui_error_network_description(),
      "",
      m.ai_ui_error_detail({ details: innerMsg || raw }),
      "",
      m.ai_ui_error_network_solution(),
    ].join("\n");
  }

  // 6. Generic Formatted Error with Exact Details
  const cleanDetails = innerMsg || raw.replace(/^Validation failed:\s*/i, "").replace(/^Gemini API Error:\s*/i, "");
  return [
    m.ai_ui_error_generic_title(),
    "",
    m.ai_ui_error_detail({ details: cleanDetails }),
    "",
    m.ai_ui_error_generic_solution(),
  ].join("\n");
}
