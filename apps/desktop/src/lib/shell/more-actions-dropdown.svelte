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
import { isDesktopApp } from "$lib/core/ipc";
import BackupCliDialog from "./backup-cli-dialog.svelte";
import {
  MoreHorizontal,
  HelpCircle,
  Users,
  Scale,
  Megaphone,
  Globe,
  Compass,
  FileText,
  Headphones,
  MessageSquare,
  Lock,
  SquareCode,
  DatabaseBackup,
} from "@lucide/svelte";

let isBackupCliOpen = $state(false);

let {
  orgName = "Orbis School",
}: {
  orgName?: string;
} = $props();

async function openUrl(url: string) {
  if (isDesktopApp()) {
    try {
      const { openUrl: openExternalUrl } = await import("@tauri-apps/plugin-opener");
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
            aria-label={m.common_sidebar_more_actions?.() ?? "More Options"}
          >
            <MoreHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={12}>
          {m.common_sidebar_more_actions?.() ?? "More Options"}
        </TooltipContent>
      </Tooltip>
    {/snippet}
  </DropdownMenuTrigger>
    <DropdownMenuPortal>
    <DropdownMenuContent align="end" side="right" sideOffset={12} class="w-64 p-1 z-50 bg-popover border border-border shadow-md rounded-md select-none overflow-visible">
    <!-- Resources Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <HelpCircle size={14} class={iconClass} />
          <span>{m.common_sidebar_help_resources?.() ?? "Resources"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="left" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://www.mulverse.com')}>
            <Compass size={13} class={iconClass} />
            <span>{m.common_sidebar_help_welcome_page?.() ?? "Welcome Page"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu')}>
            <FileText size={13} class={iconClass} />
            <span>{m.common_sidebar_help_documentation?.() ?? "Documentation"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu/api')}>
            <SquareCode size={13} class={iconClass} />
            <span>{m.common_sidebar_help_api_docs?.() ?? "API Docs"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://help.orbis.edu')}>
            <HelpCircle size={13} class={iconClass} />
            <span>{m.common_sidebar_help_help_center?.() ?? "Help Center"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/support')}>
            <Headphones size={13} class={iconClass} />
            <span>{m.common_sidebar_help_contact_support?.() ?? "Contact Support"}</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <!-- Community Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Users size={14} class={iconClass} />
          <span>{m.common_sidebar_help_community?.() ?? "Community"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="left" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://discord.gg/e64gRQZhP')}>
            <MessageSquare size={13} class={iconClass} />
            <span>{m.common_sidebar_help_discord?.() ?? "Discord"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://x.com/mulverseco')}>
            <svg class="h-3.5 w-3.5 {iconClass}" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
            </svg>
            <span>{m.common_sidebar_help_twitter?.() ?? "Twitter / X"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://github.com/mulverseco')}>
            <svg class="h-3.5 w-3.5 {iconClass}" viewBox="0 0 24 24" fill="currentColor">
              <path fill-rule="evenodd" clip-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/>
            </svg>
            <span>{m.common_sidebar_help_github?.() ?? "GitHub"}</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <!-- Legal Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Scale size={14} class={iconClass} />
          <span>{m.common_sidebar_help_legal?.() ?? "Legal"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuSubContent side="left" align="end" sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/terms')}>
            <FileText size={13} class={iconClass} />
            <span>{m.common_sidebar_help_terms?.() ?? "Terms of Service"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/privacy')}>
            <Lock size={13} class={iconClass} />
            <span>{m.common_login_privacy_policy?.() ?? "Privacy Policy"}</span>
          </DropdownMenuItem>
      </DropdownMenuSubContent>
    </DropdownMenuSub>

    <DropdownMenuSeparator />

    <!-- Give Feedback -->
    <DropdownMenuItem class={menuItemClass} onclick={() => (isBackupCliOpen = true)}>
      <Megaphone size={14} class={iconClass} />
      <span>{m.common_action_back?.() ?? "Give feedback"}</span>
    </DropdownMenuItem>

    <!-- Tour Navigation -->
    <DropdownMenuItem class={menuItemClass} onclick={() => alert('Tour is coming soon!')}>
      <Globe size={14} class={iconClass} />
      <span>{m.common_sidebar_help_tour?.() ?? "Tour the new navigation"}</span>
    </DropdownMenuItem>
  </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>

<BackupCliDialog bind:open={isBackupCliOpen} {orgName} />