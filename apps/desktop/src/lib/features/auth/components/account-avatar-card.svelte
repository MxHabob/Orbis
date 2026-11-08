<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Checkbox } from "@orbis/ui/checkbox";
import { Label } from "@orbis/ui/label";
import AvatarUpload from "$lib/shared/components/avatar-upload.svelte";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";

interface Props {
  avatarUrl: string | null;
  fullName: string;
  userId: string;
  useGravatar: boolean;
  isSavingAvatar?: boolean;
  onsave: (e: Event) => void;
}

let {
  avatarUrl = $bindable(),
  fullName,
  userId,
  useGravatar = $bindable(),
  isSavingAvatar = false,
  onsave,
}: Props = $props();
</script>

<Card>
  <CardHeader>
    <CardTitle>{m.common_account_avatar_section()}</CardTitle>
  </CardHeader>
  <CardContent class="px-6 py-5 flex flex-col md:flex-row md:items-center justify-between gap-6 text-start select-none">
    <div class="flex-1 flex items-center justify-start gap-4">
      <AvatarUpload
        bind:avatarUrl={avatarUrl}
        size={64}
        fallbackText={fullName}
        entityType="user"
        entityId={userId}
      />
      {#if useGravatar}
        <span class="text-xs font-semibold text-muted-foreground">{m.common_account_avatar_use_gravatar()} ({m.auth_ui_active()})</span>
      {/if}
    </div>

    <div class="w-full md:w-120 max-w-full shrink-0 flex items-center justify-start md:justify-start gap-3">
      <Checkbox id="use_gravatar" bind:checked={useGravatar} />
      <div class="flex flex-col text-start leading-relaxed">
        <Label for="use_gravatar" class="text-xs font-semibold text-foreground cursor-pointer">
          {m.common_account_avatar_use_gravatar()}
        </Label>
        <span class="text-[10px] text-muted-foreground leading-normal">
          {m.common_account_avatar_gravatar_desc()}
        </span>
      </div>
    </div>
  </CardContent>
  <CardFooter>
    <Button
      type="button"
      class="font-semibold text-xs h-8 px-4"
      onclick={onsave}
      disabled={isSavingAvatar}
    >
      {isSavingAvatar ? m.common_state_saving() : m.common_action_save()}
    </Button>
  </CardFooter>
</Card>
