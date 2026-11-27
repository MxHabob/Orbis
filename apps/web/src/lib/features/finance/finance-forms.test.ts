import { expect, test } from "bun:test";
import {
  buildInstallmentDates,
  hasOrderedInstallmentDates,
} from "./installment-schedule";
import { paymentRequestFingerprint } from "./payment-attempt-key";

test("generates the requested number of installments across month ends", () => {
  const dates = buildInstallmentDates("2026-01-31", 6, []);
  expect(dates).toEqual([
    "2026-02-28",
    "2026-03-31",
    "2026-04-30",
    "2026-05-31",
    "2026-06-30",
    "2026-07-31",
  ]);
  expect(hasOrderedInstallmentDates(dates, 6)).toBe(true);
  expect(hasOrderedInstallmentDates(dates.slice(0, 2), 6)).toBe(false);
  expect(hasOrderedInstallmentDates(["2026-02-30", "2026-03-31"], 2)).toBe(
    false,
  );
  expect(hasOrderedInstallmentDates(["2026-03-01", "2026-03-01"], 2)).toBe(
    false,
  );
  expect(buildInstallmentDates("2026-01-31", 3, ["2026-02-25"])[0]).toBe(
    "2026-02-25",
  );
});

test("retries fingerprint the payment payload without its idempotency ID", async () => {
  const request = {
    account_id: "cash",
    amount: "10.00",
    method: "credit_card",
    source: "test-token",
  };
  const fingerprint = await paymentRequestFingerprint(request);
  expect(
    await paymentRequestFingerprint({
      ...request,
      idempotency_key: crypto.randomUUID(),
    }),
  ).toBe(fingerprint);
  expect(
    await paymentRequestFingerprint({ ...request, amount: "20.00" }),
  ).not.toBe(fingerprint);
  expect(
    await paymentRequestFingerprint({ ...request, source: "another-token" }),
  ).not.toBe(fingerprint);
});
