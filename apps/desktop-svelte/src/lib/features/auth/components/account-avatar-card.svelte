<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Checkbox } from "@orbis/ui-svelte/checkbox";
import { Label } from "@orbis/ui-svelte/label";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui-svelte/card";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import * as m from "$lib/paraglide/messages";

interface Props {
  avatarUrl: string | null;
  fullName: string;
  useGravatar: boolean;
  isSavingAvatar?: boolean;
  onsave: (e: Event) => void;
}

let {
  avatarUrl = $bindable(),
  fullName,
  useGravatar = $bindable(),
  isSavingAvatar = false,
  onsave,
  ...restProps
}: Props & Record<string, any> = $props();
</script>

<Card class="mt-2" {...restProps}>
  <!-- Card Header -->
  <CardHeader class="bg-muted/50 px-6 py-3 border-b border-border">
    <CardTitle class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase text-start select-none">
      {m.account_avatar_section ? m.account_avatar_section() : 'AVATAR'}
    </CardTitle>
  </CardHeader>

  <!-- Card Body -->
  <CardContent>
    <div class="px-6 py-5 flex flex-col md:flex-row md:items-center justify-between gap-6 text-start select-none">
      <div class="flex-1 flex items-center justify-start gap-4">
        <AvatarUpload
          bind:avatarUrl={avatarUrl}
          size={64}
          fallbackText={fullName}
        />
        {#if useGravatar}
          <span class="text-xs font-semibold text-muted-foreground">{m.account_avatar_use_gravatar ? m.account_avatar_use_gravatar() : 'Use Gravatar'} (Active)</span>
        {/if}
      </div>

      <div class="w-full md:w-[480px] shrink-0 flex items-center justify-start md:justify-start gap-3">
        <Checkbox id="use_gravatar" bind:checked={useGravatar} />
        <div class="flex flex-col text-start leading-relaxed">
          <Label for="use_gravatar" class="text-xs font-semibold text-foreground cursor-pointer">
            {m.account_avatar_use_gravatar ? m.account_avatar_use_gravatar() : 'Use Gravatar'}
          </Label>
          <span class="text-[10px] text-muted-foreground leading-normal">
            {m.account_avatar_gravatar_desc ? m.account_avatar_gravatar_desc() : 'Manage your Gravatar on gravatar.com.'}
          </span>
        </div>
      </div>
    </div>
  </CardContent>

  <!-- Card Footer -->
  <CardFooter class="bg-muted/10 px-6 py-3 border-t border-border flex justify-start">
    <Button
      type="button"
      class="font-semibold text-xs h-8 px-4"
      onclick={onsave}
      disabled={isSavingAvatar}
    >
      {isSavingAvatar ? m.settings_general_saving() : (m.account_avatar_save_btn ? m.account_avatar_save_btn() : 'Save Avatar')}
    </Button>
  </CardFooter>
</Card>
