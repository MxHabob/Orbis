import * as m from "$lib/paraglide/messages";

/**
 * Formats AI and LLM provider errors into clear, natural, and conversational responses from the agent.
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
    raw =
      (error as any).message || (error as any).error || JSON.stringify(error);
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
  const rawLower = raw.toLowerCase();

  // 1. Missing / Unconfigured AI Provider (e.g. "No active AI / LLM integration configured")
  if (
    rawLower.includes("no active ai") ||
    rawLower.includes("llm integration") ||
    rawLower.includes("app store") ||
    rawLower.includes("no provider configured") ||
    rawLower.includes("no active provider")
  ) {
    return [
      m.ai_ui_error_no_provider_description(),
      "",
      m.ai_ui_error_no_provider_solution(),
    ].join("\n");
  }

  // 2. Quota & Rate Limits (HTTP 429 / RESOURCE_EXHAUSTED / OpenRouter credit exhaustion)
  if (
    code === 429 ||
    status === "RESOURCE_EXHAUSTED" ||
    raw.includes("RESOURCE_EXHAUSTED") ||
    raw.includes("429") ||
    rawLower.includes("quota exceeded") ||
    rawLower.includes("rate limit") ||
    rawLower.includes("requires more credits") ||
    rawLower.includes("insufficient credits") ||
    rawLower.includes("out of credits") ||
    rawLower.includes("can only afford") ||
    rawLower.includes("only afford") ||
    rawLower.includes("more credits, or fewer max_tokens") ||
    (rawLower.includes("max_tokens") && rawLower.includes("afford"))
  ) {
    return [
      m.ai_ui_error_quota_description(),
      "",
      m.ai_ui_error_quota_solution(),
    ].join("\n");
  }

  // 3. High Demand / Service Unavailable (HTTP 503 / 500 / UNAVAILABLE / Overloaded)
  if (
    code === 503 ||
    status === "UNAVAILABLE" ||
    raw.includes("UNAVAILABLE") ||
    raw.includes("503") ||
    rawLower.includes("high demand") ||
    rawLower.includes("overloaded")
  ) {
    return [
      m.ai_ui_error_overloaded_description(),
      "",
      m.ai_ui_error_overloaded_solution(),
    ].join("\n");
  }

  // 4. Invalid API Key / Authentication (HTTP 401 / 403 / API_KEY_INVALID / PERMISSION_DENIED)
  if (
    code === 401 ||
    code === 403 ||
    status === "PERMISSION_DENIED" ||
    raw.includes("API_KEY_INVALID") ||
    rawLower.includes("api key not valid") ||
    rawLower.includes("unauthorized")
  ) {
    return [
      m.ai_ui_error_auth_description(),
      "",
      m.ai_ui_error_auth_solution(),
    ].join("\n");
  }

  // 5. Model Not Found / Deprecated (HTTP 404 / MODEL_NOT_FOUND)
  if (
    code === 404 ||
    status === "NOT_FOUND" ||
    raw.includes("404") ||
    rawLower.includes("not found") ||
    rawLower.includes("is not supported")
  ) {
    return [
      m.ai_ui_error_model_not_found_description(),
      "",
      m.ai_ui_error_model_not_found_solution(),
    ].join("\n");
  }

  // 6. Missing report definition / invalid report code
  if (
    rawLower.includes("report definition not found") ||
    rawLower.includes("report code is required") ||
    (rawLower.includes("report definition") &&
      rawLower.includes("not found")) ||
    rawLower.includes("invalid report code")
  ) {
    return [
      "The requested report definition could not be found for this organization.",
      "",
      "Please list the available reports and use a valid report code before retrying.",
    ].join("\n");
  }

  // 7. Network / Connection Refused / Timeout
  if (
    rawLower.includes("network error") ||
    rawLower.includes("failed to fetch") ||
    rawLower.includes("connection refused") ||
    rawLower.includes("timeout")
  ) {
    return [
      m.ai_ui_error_network_description(),
      "",
      m.ai_ui_error_network_solution(),
    ].join("\n");
  }

  // 8. Generic Details / Fallback
  const cleanDetails =
    innerMsg ||
    raw
      .replace(/^Validation failed:\s*/i, "")
      .replace(/^Gemini API Error:\s*/i, "")
      .replace(/^DomainError::\w+:\s*/i, "")
      .trim();

  if (
    cleanDetails &&
    cleanDetails !== "Error" &&
    cleanDetails !== "[object Object]"
  ) {
    return [cleanDetails, "", m.ai_ui_error_generic_solution()].join("\n");
  }

  return m.ai_ui_error_unknown();
}
