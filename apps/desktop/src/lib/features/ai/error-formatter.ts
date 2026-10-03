/**
 * Formats AI and LLM provider errors into clear, informative, and actionable messages in Arabic.
 */
export function formatAiErrorMessage(error: unknown): string {
  if (!error) {
    return "⚠️ **حدث خطأ غير معروف أثناء معالجة الطلب.** يرجى المحاولة مرة أخرى.";
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
      "⚠️ **تم تجاوز حد الاستخدام المسموح (Quota Exceeded - 429)**",
      "",
      "لقد تم استنفاد الحصة المخصصة لمفتاح الذكاء الاصطناعي (معدل الطلبات في الدقيقة أو الحصة المجانية/الشهرية).",
      "",
      "> **تفاصيل المزود:** " + (innerMsg || "You exceeded your current quota / Rate limit reached."),
      "",
      "💡 **الحل المقترح:** يرجى الانتظار بضع دقائق وإعادة المحاولة، أو ترقية الحساب/تحديث مفتاح API من إعدادات الذكاء الاصطناعي.",
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
      "⚠️ **خادم الذكاء الاصطناعي يواجه ضغطاً مؤقتاً (503 Service Unavailable)**",
      "",
      "يعاني نموذج الذكاء الاصطناعي حالياً من ضغط عالي في الطلبات لدى المزود.",
      "",
      "> **تفاصيل المزود:** " + (innerMsg || "This model is currently experiencing high demand. Please try again later."),
      "",
      "💡 **الحل المقترح:** هذه الحالة مؤقتة وعابرة من المزود، يرجى إعادة إرسال طلبك بعد بضع ثوانٍ.",
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
      "⚠️ **خطأ في مصادقة مفتاح API (Authentication Error)**",
      "",
      "مفتاح الذكاء الاصطناعي غير صالح أو تنقصه الصلاحيات المطلوبة للوصول إلى النموذج.",
      "",
      "> **تفاصيل الخطأ:** " + (innerMsg || "Invalid API key or unauthorized request."),
      "",
      "💡 **الحل المقترح:** يرجى التحقق من إدخال مفتاح API صحيح ونشط في إعدادات الذكاء الاصطناعي للنظام.",
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
      "⚠️ **النموذج المحدد غير متاح (Model Not Found - 404)**",
      "",
      "النموذج المطلوب غير مدعوم أو تم إيقافه من قبل المزود.",
      "",
      "> **تفاصيل الخطأ:** " + (innerMsg || raw),
      "",
      "💡 **الحل المقترح:** يرجى مراجعة إعدادات الذكاء الاصطناعي واختيار نموذج مدعوم.",
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
      "⚠️ **تعذر الاتصال بخادم الذكاء الاصطناعي (Network Error)**",
      "",
      "فشل الاتصال بالخادم المزود لخدمة الذكاء الاصطناعي. قد يكون هناك انقطاع في الشبكة أو أن الخادم المحلي غير مشغل.",
      "",
      "> **تفاصيل الخطأ:** " + (innerMsg || raw),
      "",
      "💡 **الحل المقترح:** تأكد من اتصال الإنترنت أو تأكد من تشغيل الخادم/النموذج المحلي ثم أعد المحاولة.",
    ].join("\n");
  }

  // 6. Generic Formatted Error with Exact Details
  const cleanDetails = innerMsg || raw.replace(/^Validation failed:\s*/i, "").replace(/^Gemini API Error:\s*/i, "");
  return [
    "⚠️ **تعذر إكمال الطلب مع خادم الذكاء الاصطناعي**",
    "",
    "> **تفاصيل الخطأ:** " + cleanDetails,
    "",
    "💡 يرجى مراجعة التفاصيل أعلاه أو إعادة المحاولة لاحقاً.",
  ].join("\n");
}
