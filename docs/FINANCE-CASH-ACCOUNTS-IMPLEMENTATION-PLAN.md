# Finance Cash and Bank Accounts - Implementation Plan

## 1. Executive Decision

Cash boxes and bank accounts will be represented by `accounts` records. A cash or bank account is a normal asset account with a controlled subtype:

```text
accounts
  type: asset
  sub_type: cash | bank
  balances: { currency_code: amount }
```

There will be no separate `treasuries` entity for balances or accounting movements. Journal entries remain the only accounting source of truth. Optional bank metadata may be stored separately, but it must never store a second balance.

The account remains the same account when it contains multiple currencies. Currency is a balance dimension, not a reason to create another ledger account.

## 2. Scope

### Included

- Replace treasury accounting identity with `account_id`.
- Support multiple currency balances on the same cash or bank account.
- Make journal entries authoritative for receipts, payments, deposits, withdrawals, and transfers.
- Migrate existing treasury records and payment references safely.
- Preserve bank identification data where it is operationally required.
- Add validation, reconciliation, and regression tests.

### Deferred

- Full bank statement import and automatic matching.
- Cash-count sessions and till closing workflows.
- Multi-entity intercompany cash management.
- Historical redesign of already posted journal entries.
- External payment gateways, electronic wallets, and provider settlement flows.
- Payment webhooks, refunds, provider fees, and external reconciliation.

## 3. Target Model

### Account classification

Use the existing chart of accounts:

- `type = asset` is mandatory.
- `sub_type = cash` for physical cash.
- `sub_type = bank` for bank accounts.
- Existing parent accounts remain `110101` for cash and `110102` for banks.
- `balances` stores one balance per currency.
- `currency` is the account functional/base currency and must not limit the currency keys in `balances`.

### Payment contract

Use `cash_account_id` as the account receiving or paying money. There is no compatibility contract because the system is pre-production.

This plan covers internal receipts and cash/bank accounting only. Online payment providers and wallet integrations will use a separate future plan and are not part of this migration.

### Bank metadata

If bank details are needed, create an optional `bank_account_details` table linked one-to-one to `accounts`:

```text
account_id, bank_name, branch, account_number, iban, swift_code
```

This table contains identity and reconciliation metadata only. It must not contain `current_balance` or accounting transactions.

## 4. Ordered Tasks

### Phase 0 - Decisions and contracts

- [ ] Confirm the organization base currency and the meaning of `account.currency`.
- [ ] Confirm canonical subtypes: `cash` and `bank`.
- [x] Confirm the new API field name: `cash_account_id`.
- [x] Document that `balances` is the multi-currency balance projection of posted journal entries.
- [ ] Define rounding rules and exchange-rate direction.

**Acceptance:** product, accounting, and engineering agree that one account can contain several currency balances and no account balance is mixed across currencies.

### Phase 1 - Account and currency invariants

- [x] Add validation that cash and bank accounts are `asset` accounts.
- [x] Restrict cash and bank accounts to the correct parent account where applicable.
- [x] Validate ISO currency codes and active organization currencies.
- [ ] Validate positive exchange rates and non-negative transaction amounts where the operation requires it.
- [ ] Replace new financial `f64` amounts with a decimal-safe type or database `NUMERIC` representation.
- [x] Add a single helper for updating `accounts.balances` atomically.

**Acceptance:** invalid account types, inactive currencies, and invalid rates are rejected before journal insertion.

### Phase 2 - Replace treasury creation and queries

- [x] Change `svc-finance` creation logic to create a cash/bank account directly.
- [x] Stop creating a `treasuries` row for new accounts.
- [x] Expose cash and bank accounts through the account query contract, with subtype filters.
- [x] Update API gateway routes and SDK types to use accounts.
- [x] Update desktop forms to choose the account subtype and currency behavior.
- [x] Remove old treasury consumers; no compatibility adapter is needed before production.

**Acceptance:** creating a cash or bank account creates exactly one account and no duplicate balance record.

### Phase 3 - Make journal entries authoritative

- [x] Change receipt recording to accept `cash_account_id`.
- [x] Validate that the selected account is an active `cash` or `bank` account.
- [x] Post the debit/credit journal entry using that account.
- [x] Update the same account's `balances[currency]` inside the journal transaction.
- [ ] Remove direct updates to `treasuries.current_balance`.
- [ ] Remove creation of `treasury_transactions` for new operations.
- [ ] Add explicit operations for cash transfer, bank deposit, withdrawal, and account-to-account transfer.
- [ ] Ensure every operation has balanced debit and credit legs.

**Acceptance:** after every successful operation, the account balance equals the sum of its posted journal entries by currency; a failed operation changes neither.

### Phase 4 - Pre-production database reset

- [x] Replace the baseline schema with the account-only model.
- [ ] Drop and recreate development databases from the new baseline.
- [x] Reseed the chart of accounts, including the shared cash/bank parent `1101`, cash account `110101`, and bank accounts `110102`/`110103`.
- [x] Verify that no runtime code expects the removed treasury tables.

**Acceptance:** a fresh database contains no `treasuries` or `treasury_transactions` tables and cash/bank accounts are created through `createAccount`.

### Phase 5 - Remove duplicate models

- [x] Stop writing to `treasuries`, `treasury_transactions`, `opening_balance`, and `current_balance`.
- [x] Remove old entities, repositories, commands, events, and routes.
- [x] Remove `treasury_id` from new contracts; no legacy migration path is required.
- [ ] Add database constraints and indexes for account subtype and currency balance access.
- [x] Update generated RPC and SDK contracts.

**Acceptance:** no production code uses a second cash balance or a second cash movement ledger.

### Phase 6 - Reporting and reconciliation

- [ ] Provide account balance by currency.
- [ ] Provide total reporting in the organization base currency using stored transaction rates.
- [ ] Add a reconciliation report: journal balance versus the balance projection.
- [ ] Show cash and bank accounts using account subtype labels in the desktop UI.
- [ ] Add bank metadata screens only if bank reconciliation is in scope.

**Acceptance:** users can see each currency separately and the base-currency equivalent without adding unlike currencies together.

### Phase 7 - Verification and rollout

- [ ] Add unit tests for account classification and multi-currency balance updates.
- [ ] Add integration tests for receipt, withdrawal, deposit, and transfer flows.
- [ ] Test USD and SAR on the same account and verify independent balances.
- [ ] Test rejection of inactive currencies and non-cash accounts.
- [ ] Test transaction rollback after a journal or balance update failure.
- [ ] Run Rust tests, typecheck, lint, generated-contract checks, and the desktop build.
- [ ] Deploy migration with monitoring and a reversible compatibility period.

**Acceptance:** accounting sign-off confirms balanced entries, correct currency balances, and a successful migration rehearsal on a copy of production data.

## 5. Key Risks and Controls

| Risk | Control |
| --- | --- |
| Mixing USD and local currency in one number | Never use a scalar total for multi-currency accounts; require a currency key. |
| Duplicate balances disagree | Journal entries are the source of truth; balance updates run in the same transaction. |
| Old clients still send `treasury_id` | Temporary adapter and deprecation telemetry. |
| Floating-point rounding differences | Decimal amounts, currency-specific precision, and explicit rounding rules. |
| Historical balances cannot be explained | Preserve old IDs and generate a migration exception report. |
| Bank data gets confused with accounting data | Keep bank metadata separate and prohibit balance fields there. |

## 6. Definition of Done

- One cash or bank account has one `account_id` and can hold multiple currency balances.
- All new cash movements are journal entries; no duplicate treasury movement ledger exists.
- Receipts, transfers, deposits, and withdrawals update the same account balance projection atomically.
- Reports show original currency and base-currency equivalent separately.
- Existing data is migrated and reconciled without rewriting historical entries.
- Old treasury APIs are removed or formally deprecated with a documented migration path.