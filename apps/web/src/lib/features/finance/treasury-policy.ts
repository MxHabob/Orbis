import type { AccountDto } from "$lib/api";

export function treasuryKind(account: AccountDto): "cash" | "bank" | undefined {
  if (!account.is_active || account.account_type.toLowerCase() !== "asset") return;
  const subtype = account.sub_type?.toLowerCase();
  if (subtype === "cash" || subtype === "bank") return subtype;
  if (account.code === "110101") return "cash";
  if (["110102", "110103", "110104"].includes(account.code)) return "bank";
}

export function eligibleTreasuries(
  accounts: AccountDto[],
  method: string,
  cashAssignment: string | null | undefined,
  currency?: string,
) {
  const isCash = method.toLowerCase() === "cash";
  if (isCash && cashAssignment === undefined) return [];
  return accounts.filter((account) =>
    treasuryKind(account) === (isCash ? "cash" : "bank") &&
    (!isCash || !cashAssignment || account.id === cashAssignment) &&
    (!currency || account.currency.toUpperCase() === currency.toUpperCase()),
  );
}

export function resolveTreasury(accounts: AccountDto[], selectedId: string) {
  return accounts.find((account) => account.id === selectedId)?.id ?? accounts[0]?.id ?? "";
}
