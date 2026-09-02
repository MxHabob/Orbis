<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import { FormField } from "@orbis/ui-svelte";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui-svelte/card";

interface Props {
  fullName: string;
  userId: string;
  isSavingDetails?: boolean;
}

let {
  fullName = $bindable(),
  userId,
  isSavingDetails = false,
  ...restProps
}: Props & Record<string, any> = $props();
</script>

<Card {...restProps}>
  <!-- Card Header -->
  <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
    <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
      {m.common_account_details_section ? m.common_account_details_section() : 'ACCOUNT DETAILS'}
    </CardTitle>
  </CardHeader>

  <!-- Card Body -->
  <CardContent >
    <div class="flex flex-col">
      <!-- Full Name -->
      <div class="px-6 py-2 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.common_account_details_name_desc ? m.common_account_details_name_desc() : 'Your full name'}
          </span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
          <FormField id="full_name" label={m.common_col_name ? m.common_col_name() : "Name"} required>
            {#snippet children()}
              <Input id="full_name" type="text" bind:value={fullName} required disabled={isSavingDetails} class="h-9 w-full bg-background" />
            {/snippet}
          </FormField>
        </div>
      </div>

      <!-- User ID -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {m.common_account_details_userid_desc ? m.common_account_details_userid_desc() : 'The unique identifier for your account. It cannot be modified.'}
          </span>
        </div>
        <div class="w-full md:w-120 shrink-0 flex items-center justify-end">
          <FormField id="user_id" label={m.common_account_details_userid ? m.common_account_details_userid() : "User ID"}>
            {#snippet children()}
              <Input id="user_id" type="text" value={userId} disabled class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono" />
            {/snippet}
          </FormField>
        </div>
      </div>
    </div>
  </CardContent>
</Card>
