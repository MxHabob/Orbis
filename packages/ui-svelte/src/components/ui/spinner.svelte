<script lang="ts">
  let {
    size = 16,
    strokeWidth = 2,
    class: className = "",
  }: {
    size?: number;
    strokeWidth?: number;
    class?: string;
  } = $props();

  const borderWidth = $derived(Math.max(1, strokeWidth));
</script>

<span
  class={`inline-block shrink-0 animate-spin rounded-full border-solid border-current border-e-transparent ${className}`}
  style={`width: ${size}px; height: ${size}px; border-width: ${borderWidth}px;`}
  role="status"
  aria-label="Loading"
></span>
