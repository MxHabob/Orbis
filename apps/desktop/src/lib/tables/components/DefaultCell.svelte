<script lang="ts">
interface Props {
  row: Record<string, any>;
  column: Record<string, any>;
  api?: any;
  onaction?: (ev: any) => void;
}
let { row, column }: Props = $props();

const value = $derived.by(() => {
  const val = row[column.id];
  return val !== undefined && val !== null ? String(val) : "";
});

const cellProps = $derived.by(() => {
  if (column.cellProperties) {
    return column.cellProperties(row) || {};
  }
  return {};
});

const alignClass = $derived(
  column.align === "center"
    ? "text-center justify-center text-center"
    : column.align === "end"
      ? "text-end justify-end text-end"
      : "text-start justify-start text-start",
);

const customClass = $derived(
  cellProps.class || cellProps.className || column.cellClass || "",
);
</script>

<div class="flex items-center w-full h-full px-2 truncate {alignClass} {customClass}">
  {value}
</div>
