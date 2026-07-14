<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import {
    AlertDialog,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogCancel,
    AlertDialogAction,
  } from "@orbis/ui-svelte/alert-dialog";
  
  let {
    open = $bindable(false),
    title,
    description,
    confirmText,
    cancelText = m.common_action_cancel(),
    onConfirm,
    variant = "default",
    isLoading = false,
  }: {
    open: boolean;
    title: string;
    description: string;
    confirmText: string;
    cancelText?: string;
    onConfirm: () => void | Promise<void>;
    variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link";
    isLoading?: boolean;
  } = $props();

  async function handleConfirm() {
    await onConfirm();
    open = false;
  }
</script>

<AlertDialog bind:open>
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{title}</AlertDialogTitle>
      <AlertDialogDescription>
        {description}
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel disabled={isLoading}>{cancelText}</AlertDialogCancel>
      <AlertDialogAction
        onclick={handleConfirm}
        class={variant === "destructive" ? "bg-destructive text-destructive-foreground hover:bg-destructive/90" : ""}
      >
        {isLoading ? m.common_loading() : confirmText}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
