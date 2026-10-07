<script lang="ts">
import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui";
import { useDocumentDirection } from "../../../utils/direction.svelte.js";

let {
  open = $bindable(false),
  dir,
  ...restProps
}: DropdownMenuPrimitive.RootProps = $props();

const getDocumentDirection = useDocumentDirection();
const resolvedDir = $derived(dir ?? getDocumentDirection());
</script>

<DropdownMenuPrimitive.Root bind:open dir={resolvedDir} {...restProps} />
