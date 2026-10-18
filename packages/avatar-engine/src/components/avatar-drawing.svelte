<svelte:options namespace="svg" />
<script lang="ts">
import type { AvatarSvgElement } from "../core/avatar-svg.js";

const { elements = [] }: { elements?: AvatarSvgElement[] } = $props();
</script>

{#each elements as element (element.key)}
  <svelte:element this={element.tag} {...element.attributes} />
{/each}
