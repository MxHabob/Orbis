<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import { Label } from "@orbis/ui-svelte/label";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui-svelte/card";

interface Props {
  currentPassword: string;
  newPassword: string;
  confirmPassword: string;
  isUpdating?: boolean;
  onsubmit: (e: Event) => void;
}

let {
  currentPassword = $bindable(),
  newPassword = $bindable(),
  confirmPassword = $bindable(),
  isUpdating = false,
  onsubmit,
  ...restProps
}: Props & Record<string, any> = $props();
</script>

<Card {...restProps}>
  <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
    <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
      {m.settings_security_change_password()}
    </CardTitle>
  </CardHeader>
  <CardContent>
    <form onsubmit={onsubmit} class="flex flex-col">
      <!-- Current Password -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="current_pass" class="text-xs font-semibold text-foreground">
            {m.settings_security_current_password()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.settings_security_current_password_desc()}
          </span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
          <FormField id="current_pass" label={m.settings_security_current_password()} required>
            {#snippet children()}
              <Input id="current_pass" type="password" bind:value={currentPassword} placeholder="••••••••" required disabled={isUpdating} class="h-9 w-full bg-background" />
            {/snippet}
          </FormField>
        </div>
      </div>

      <!-- New Password -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="new_pass" class="text-xs font-semibold text-foreground">
            {m.settings_security_new_password()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.settings_security_new_password_desc()}
          </span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
          <FormField id="new_pass" label={m.settings_security_new_password()} required>
            {#snippet children()}
              <Input id="new_pass" type="password" bind:value={newPassword} placeholder="••••••••" required minlength={6} disabled={isUpdating} class="h-9 w-full bg-background" />
            {/snippet}
          </FormField>
        </div>
      </div>

      <!-- Confirm Password -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="confirm_pass" class="text-xs font-semibold text-foreground">
            {m.settings_security_confirm_password()}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.settings_security_confirm_password_desc()}
          </span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
          <FormField id="confirm_pass" label={m.settings_security_confirm_password()} required>
            {#snippet children()}
              <Input id="confirm_pass" type="password" bind:value={confirmPassword} placeholder="••••••••" required disabled={isUpdating} class="h-9 w-full bg-background" />
            {/snippet}
          </FormField>
        </div>
      </div>

      <CardFooter class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start">
        <Button
          type="submit"
          class="px-6 font-semibold text-xs h-9"
          disabled={isUpdating}
        >
          {isUpdating ? m.settings_security_updating() : m.settings_security_update_password()}
        </Button>
      </CardFooter>
    </form>
  </CardContent>
</Card>
