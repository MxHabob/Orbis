import type { ProcessPaymentRequest } from "$lib/api/types";

export async function paymentRequestFingerprint(request: ProcessPaymentRequest) {
  const payload = JSON.stringify({ ...request, idempotency_key: undefined });
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(payload));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}
