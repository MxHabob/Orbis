<script lang="ts">
/** @deprecated Use LoadingIndicator instead. */
import { LoadingIndicator } from "./loading-indicator/index.js";

let props: {
  size?: number;
  strokeWidth?: number;
  class?: string;
} = $props();
</script>

<LoadingIndicator size={props.size ?? 16} class={props.class ?? ""} />
