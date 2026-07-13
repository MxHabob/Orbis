<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Input } from "@orbis/ui-svelte/input";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
} from "@orbis/ui-svelte/card";
import * as common from "$lib/paraglide/messages/common";

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
      {common.account_details_section ? common.account_details_section() : 'ACCOUNT DETAILS'}
    </CardTitle>
  </CardHeader>

  <!-- Card Body -->
  <CardContent >
    <div class="flex flex-col">
      <!-- Full Name -->
      <div class="px-6 py-2 flex flex-col md:flex-row md:items-center justify-between border-b border-border gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="full_name" class="text-xs font-semibold text-foreground flex items-center gap-1">
            {common.account_details_name ? common.account_details_name() : 'Name'}
            <span class="text-destructive">*</span>
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {common.account_details_name_desc ? common.account_details_name_desc() : 'Your full name'}
          </span>
        </div>
        <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
          <Input
            id="full_name"
            type="text"
            bind:value={fullName}
            required
            disabled={isSavingDetails}
            class="h-9 w-full bg-background"
          />
        </div>
      </div>

      <!-- User ID -->
      <div class="px-6 py-4 flex flex-col md:flex-row md:items-center justify-between last:border-b-0 gap-4">
        <div class="flex flex-col gap-1 text-start">
          <Label for="user_id" class="text-xs font-semibold text-foreground">
            {common.account_details_userid ? common.account_details_userid() : 'User ID'}
          </Label>
          <span class="text-[11px] text-muted-foreground leading-relaxed">
            {common.account_details_userid_desc ? common.account_details_userid_desc() : 'The unique identifier for your account. It cannot be modified.'}
          </span>
        </div>
        <div class="w-full md:w-[480px] shrink-0 flex items-center justify-end">
          <Input
            id="user_id"
            type="text"
            value={userId}
            disabled
            class="h-9 w-full bg-muted/40 text-muted-foreground border-border/80 font-mono"
          />
        </div>
      </div>
    </div>
  </CardContent>
</Card>
