<script lang="ts">
let {
  message = "",
  id,
}: {
  message?: string;
  id?: string;
} = $props();
</script>

{#if message}
  <p id={id} class="text-xs leading-5 text-destructive" role="alert">
    {message}
  </p>
{/if}
