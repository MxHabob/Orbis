<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
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
} from "@orbis/ui-svelte/dropdown-menu";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import {
  MoreHorizontal,
  HelpCircle,
  Users,
  Scale,
  Megaphone,
  Globe,
  Compass,
  FileText,
  SquareCode,
  Headphones,
  MessageSquare,
  Twitter,
  Github,
  Lock,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

function openUrl(url: string) {
  window.open(url, "_blank");
}

const menuItemClass = "flex items-center gap-2.5 px-2.5 py-2 text-xs cursor-pointer hover:bg-accent rounded-sm";
const subTriggerClass = "flex items-center justify-between text-xs cursor-pointer px-2.5 py-2 hover:bg-accent rounded-sm";
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
            aria-label={m.sidebar_more_actions?.() ?? "More Options"}
          >
            <MoreHorizontal size={16} strokeWidth={1.8} class="text-foreground" />
          </Button>
        </TooltipTrigger>
        <TooltipContent side="right" sideOffset={12}>
          {m.sidebar_more_actions?.() ?? "More Options"}
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
          <span>{m.sidebar_help_resources?.() ?? "Resources"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuPortal>
        <DropdownMenuSubContent sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/welcome')}>
            <Compass size={13} class={iconClass} />
            <span>{m.sidebar_help_welcome_page?.() ?? "Welcome Page"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu')}>
            <FileText size={13} class={iconClass} />
            <span>{m.sidebar_help_documentation?.() ?? "Documentation"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://docs.orbis.edu/api')}>
            <SquareCode size={13} class={iconClass} />
            <span>{m.sidebar_help_api_docs?.() ?? "API Docs"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://help.orbis.edu')}>
            <HelpCircle size={13} class={iconClass} />
            <span>{m.sidebar_help_help_center?.() ?? "Help Center"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/support')}>
            <Headphones size={13} class={iconClass} />
            <span>{m.sidebar_help_contact_support?.() ?? "Contact Support"}</span>
          </DropdownMenuItem>
        </DropdownMenuSubContent>
      </DropdownMenuPortal>
    </DropdownMenuSub>

    <!-- Community Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Users size={14} class={iconClass} />
          <span>{m.sidebar_help_community?.() ?? "Community"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuPortal>
        <DropdownMenuSubContent sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://discord.gg/orbis')}>
            <MessageSquare size={13} class={iconClass} />
            <span>{m.sidebar_help_discord?.() ?? "Discord"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://x.com/orbis')}>
            <Twitter size={13} class={iconClass} />
            <span>{m.sidebar_help_twitter?.() ?? "Twitter / X"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://github.com/orbis')}>
            <Github size={13} class={iconClass} />
            <span>{m.sidebar_help_github?.() ?? "GitHub"}</span>
          </DropdownMenuItem>
        </DropdownMenuSubContent>
      </DropdownMenuPortal>
    </DropdownMenuSub>

    <!-- Legal Submenu -->
    <DropdownMenuSub>
      <DropdownMenuSubTrigger class={subTriggerClass}>
        <div class="flex items-center gap-2.5">
          <Scale size={14} class={iconClass} />
          <span>{m.sidebar_help_legal?.() ?? "Legal"}</span>
        </div>
      </DropdownMenuSubTrigger>
      <DropdownMenuPortal>
        <DropdownMenuSubContent sideOffset={4} class="w-48 text-start">
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/terms')}>
            <FileText size={13} class={iconClass} />
            <span>{m.sidebar_help_terms?.() ?? "Terms of Service"}</span>
          </DropdownMenuItem>
          <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('https://orbis.edu/privacy')}>
            <Lock size={13} class={iconClass} />
            <span>{m.sidebar_help_privacy?.() ?? "Privacy Policy"}</span>
          </DropdownMenuItem>
        </DropdownMenuSubContent>
      </DropdownMenuPortal>
    </DropdownMenuSub>

    <DropdownMenuSeparator />

    <!-- Give Feedback -->
    <DropdownMenuItem class={menuItemClass} onclick={() => openUrl('mailto:feedback@orbis.edu')}>
      <Megaphone size={14} class={iconClass} />
      <span>{m.sidebar_help_feedback?.() ?? "Give feedback"}</span>
    </DropdownMenuItem>

    <!-- Tour Navigation -->
    <DropdownMenuItem class={menuItemClass} onclick={() => alert('Tour is coming soon!')}>
      <Globe size={14} class={iconClass} />
      <span>{m.sidebar_help_tour?.() ?? "Tour the new navigation"}</span>
    </DropdownMenuItem>
  </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>