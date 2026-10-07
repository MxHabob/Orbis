<script lang="ts">
import * as m from "$lib/paraglide/messages";

import { Button } from "@orbis/ui/button";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuSubContent,
  DropdownMenuPortal,
} from "@orbis/ui/dropdown-menu";
import { Tooltip, TooltipContent, TooltipTrigger } from "@orbis/ui";
import { isDesktopApp } from "$lib/api";
import BackupCliDialog from "../features/system/backup-cli-dialog.svelte";
import {
  AppNotification,
  ChatBubbles,
  CodeBracket,
  DocumentPage,
  EllipsisHorizontal,
  Github,
  GlobeAlt,
  Headphone,
  Lock,
  QuestionMarkCircle,
  Scale,
  Users,
} from "@orbis/icons";

let isBackupCliOpen = $state(false);

let {
  orgName = "Orbis School",
}: {
  orgName?: string;
} = $props();

async function openUrl(url: string) {
  if (isDesktopApp) {
    try {
      const { openUrl: openExternalUrl } = await import(
        "@tauri-apps/plugin-opener"
      );
      await openExternalUrl(url);
      return;
    } catch {
      // Fall back to the browser opener when the Tauri plugin is unavailable.
    }
  }

  window.open(url, "_blank", "noopener,noreferrer");
}

const menuItemClass =
  "flex items-center gap-2.5 px-2.5 py-2 text-xs cursor-pointer hover:bg-accent rounded-sm";
const subTriggerClass =
  "flex items-center justify-between text-xs cursor-pointer px-2.5 py-2 hover:bg-accent rounded-sm";
const iconClass = "text-muted-foreground shrink-0";
</script>

<DropdownMenu>
  <DropdownMenuTrigger>
    {#snippet child({ props }: { props: any })}
      <Tooltip>
        <TooltipTrigger>
          <Button 
            {...props}
            variant="outline" 
            size="icon-sm" 
            class="relative cursor-pointer rounded-[10px]"
            aria-label={m.ui_copy_more_options_2179e961()}
          >
            <EllipsisHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="inline-end" sideOffset={12}>
          {m.ui_copy_more_options_2179e961()}
        </TooltipContent>
      </Tooltip>
    {/snippet}
  </DropdownMenuTrigger>
    <DropdownMenuPortal>
    <DropdownMenuContent align="end" side="inline-end" sideOffset={12} class="w-64 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
    <!-- Resources Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <QuestionMarkCircle size={14} class={iconClass} />
          <span>{m.ui_copy_resources_8bacd643()}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="inline-start" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://www.mulverse.com')}>
            <GlobeAlt size={13} class={iconClass} />
            <span>{m.ui_copy_welcome_page_e980d673()}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu')}>
            <DocumentPage size={13} class={iconClass} />
            <span>{m.ui_copy_documentation_9102cd83()}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu/api')}>
            <CodeBracket size={13} class={iconClass} />
            <span>{m.ui_copy_api_docs_d987cdbd()}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://help.orbis.edu')}>
            <QuestionMarkCircle size={13} class={iconClass} />
            <span>{m.ui_copy_help_center_5494ae10()}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://www.mulverse.com/p/orbis/support')}>
            <Headphone size={13} class={iconClass} />
            <span>{m.ui_copy_contact_support_834f3ed7()}</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <!-- Community Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Users size={14} class={iconClass} />
          <span>{m.common_sidebar_help_community()}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="inline-start" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://discord.gg/e64gRQZhP')}>
            <ChatBubbles size={13} class={iconClass} />
            <span>Discord</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://x.com/mulverseco')}>
            <svg class="h-3.5 w-3.5 {iconClass}" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
            </svg>
            <span>Twitter / X</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://github.com/mulverseco')}>
            <Github size={13} class={iconClass} />
            <span>GitHub</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <!-- Legal Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Scale size={14} class={iconClass} />
          <span>{m.ui_copy_legal_4ddb3d47()}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="inline-start" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://www.mulverse.com/p/orbis/terms')}>
            <DocumentPage size={13} class={iconClass} />
            <span>{m.common_sidebar_help_terms()}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://www.mulverse.com/p/orbis/privacy')}>
            <Lock size={13} class={iconClass} />
            <span>{m.common_login_privacy_policy()}</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <DropdownMenuSeparator />

    <!-- Give Feedback -->
    <DropdownMenuItem class={menuItemClass} onclick={() => (isBackupCliOpen = true)}>
      <AppNotification size={14} class={iconClass} />
      <span>{m.ui_copy_give_feedback_bca18e8d()}</span>
    </DropdownMenuItem>

    <!-- Tour Navigation -->
    <DropdownMenuItem class={menuItemClass} onclick={() => alert('Tour is coming soon!')}>
      <GlobeAlt size={14} class={iconClass} />
      <span>{m.ui_copy_tour_the_new_navigation_a187027e()}</span>
    </DropdownMenuItem>
  </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>

<BackupCliDialog bind:open={isBackupCliOpen} {orgName} />