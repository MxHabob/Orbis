export interface SpreadsheetColumnMetadata {
  id: string;
  headerAr: string;
  headerEn: string;
  headerAliases?: string[];
}

export function normalizeSpreadsheetHeader(value: string): string {
  return value
    .normalize("NFKC")
    .replace(/[\u0640\u064B-\u065F\u0670]/g, "")
    .toLocaleLowerCase()
    .replace(/[^\p{L}\p{N}]/gu, "");
}

export function findSpreadsheetColumn(
  header: string,
  columns: SpreadsheetColumnMetadata[],
  getLocalizedHeader: (column: SpreadsheetColumnMetadata) => string,
): SpreadsheetColumnMetadata | undefined {
  const normalizedHeader = normalizeSpreadsheetHeader(header);
  if (!normalizedHeader) return undefined;

  return columns.find((column) =>
    [
      column.id,
      column.headerAr,
      column.headerEn,
      ...(column.headerAliases ?? []),
      getLocalizedHeader(column),
    ].some(
      (candidate) => normalizeSpreadsheetHeader(candidate) === normalizedHeader,
    ),
  );
}
