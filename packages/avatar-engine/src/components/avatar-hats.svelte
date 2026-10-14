<svelte:options namespace="svg" />
<script lang="ts">
import { getAvatarHatDrawing } from "../core/avatar-svg.js";
import type { AvatarHat } from "../core/types.js";
import AvatarDrawing from "./avatar-drawing.svelte";

const { hat = "none" }: { hat?: AvatarHat } = $props();
const drawing = $derived(getAvatarHatDrawing(hat));
</script>

<g>
  <AvatarDrawing elements={drawing} />
</g>
