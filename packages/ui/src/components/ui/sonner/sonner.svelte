<script lang="ts">
import { CheckCircle2, Info, TriangleAlert, X, XCircle } from "@lucide/svelte";
import { Button } from "../button/index.js";
import { Progress } from "../progress/index.js";
import Spinner from "../spinner.svelte";
import { toastStore } from "./toast-state.svelte.js";

let { ...restProps } = $props();
const visibleToasts = $derived(toastStore.toasts.filter((item) => item.open));
</script>

<div class="toaster" {...restProps}>
  {#each visibleToasts as item (item.id)}
    <div class="toast-item" data-variant={item.variant} role="status" aria-live="polite">
      <div class="toast-main">
        <div class="toast-icon">
          {#if item.variant === "success"}<CheckCircle2 size={16} />
          {:else if item.variant === "error" || item.variant === "destructive"}<XCircle size={16} />
          {:else if item.variant === "warning"}<TriangleAlert size={16} />
          {:else if item.variant === "info" || item.variant === "ai"}<Info size={16} />
          {:else}<Spinner size={16} />{/if}
        </div>
        <div class="toast-content">
          <div class="toast-heading">
            <strong>{typeof item.title === "string" ? item.title : ""}</strong>
            {#if item.variant === "progress"}<span>{Math.round(item.progress ?? 0)}%</span>{/if}
          </div>
          {#if item.description}<p>{item.description}</p>{/if}
        </div>
        <button type="button" class="toast-close" aria-label="Close" onclick={() => toastStore.dismiss(item.id)}><X size={14} /></button>
      </div>
      {#if item.variant === "progress"}<Progress value={item.progress ?? 0} class="toast-progress" />{/if}
      {#if item.action}<Button size="xs" class="toast-action" onclick={item.action.onClick}>{item.action.label}</Button>{/if}
      {#if item.footer}<div class="toast-footer">{item.footer}</div>{/if}
    </div>
  {/each}
</div>

<style>
  .toaster { position: fixed; right: 1rem; bottom: 1rem; z-index: 100; display: flex; width: min(380px, calc(100vw - 2rem)); flex-direction: column; gap: 0.5rem; pointer-events: none; }
  .toast-item { pointer-events: auto; display: flex; flex-direction: column; gap: 0.5rem; padding: 0.75rem 0.875rem; color: var(--color-popover-foreground); background: var(--color-popover); border: 1px solid var(--color-border); border-radius: 0.375rem; box-shadow: 0 2px 8px rgb(0 0 0 / 8%); }
  .toast-main { display: flex; align-items: flex-start; gap: 0.5rem; }
  .toast-icon { display: flex; flex: 0 0 1.25rem; align-items: center; justify-content: center; padding-top: 0.0625rem; color: var(--color-muted-foreground); }
  [data-variant="success"] .toast-icon { color: var(--color-primary); }
  [data-variant="error"] .toast-icon, [data-variant="destructive"] .toast-icon { color: var(--color-destructive); }
  [data-variant="warning"] .toast-icon { color: var(--color-primary); }
  .toast-content { min-width: 0; flex: 1; }
  .toast-heading { display: flex; justify-content: space-between; gap: 0.75rem; font-size: 0.75rem; line-height: 1.4; }
  .toast-heading strong { overflow: hidden; text-overflow: ellipsis; font-weight: 500; }
  .toast-heading span, .toast-content p, .toast-footer { color: var(--color-muted-foreground); font-size: 0.6875rem; }
  .toast-content p { margin: 0.125rem 0 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .toast-close { display: flex; flex: 0 0 auto; padding: 0.125rem; color: var(--color-muted-foreground); background: transparent; border: 0; border-radius: 0.25rem; cursor: pointer; }
  .toast-close:hover { color: var(--color-foreground); background: var(--color-muted); }
  .toast-progress { height: 0.1875rem; border-radius: 0; }
  .toast-action { align-self: flex-end; }
  .toast-footer { padding-top: 0.25rem; }
  @media (max-width: 640px) { .toaster { right: 0.75rem; bottom: 0.75rem; left: 0.75rem; width: auto; } }
</style>
