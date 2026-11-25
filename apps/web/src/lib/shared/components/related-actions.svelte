<!--
@component RelatedActions
Windows 11 Fluent-style Related Actions / Support component using Accordion.
Renders clean collapsible cards with primary hyperlink sub-links and bottom footer actions.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "@orbis/ui/accordion";
import { GlobeAlt, QuestionMarkCircle } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

export interface SubLink {
  id?: string;
  label: string;
  href?: string;
  onclick?: (e: MouseEvent) => void;
  disabled?: boolean;
}

export interface ActionGroup {
  id: string;
  title?: string;
  icon?: any;
  subLinks: SubLink[];
  defaultExpanded?: boolean;
}

export interface FooterAction {
  id?: string;
  label: string;
  icon?: any;
  href?: string;
  onclick?: (e: MouseEvent) => void;
  disabled?: boolean;
  variant?: "default" | "destructive" | "muted";
}

let {
  title = m.shared_ui_related_actions(),
  groups = [],
  footerActions = [],
  class: className = "",
}: {
  title?: string;
  groups: ActionGroup[];
  footerActions?: FooterAction[];
  class?: string;
} = $props();

const defaultValues = $derived(
  groups.filter((g) => g.defaultExpanded ?? true).map((g) => g.id),
);
</script>

{#if groups.length > 0 || (footerActions && footerActions.length > 0)}
  <div
    class="w-full mt-10 py-8 text-start font-sans select-none space-y-3 {className}"
  >
    <!-- Section Title -->
    {#if title}
      <h3 class="text-xs font-semibold text-foreground px-1">
        {title}
      </h3>
    {/if}

    <!-- Accordion Groups -->
    {#if groups.length > 0}
      <Accordion
        type="multiple"
        value={defaultValues}
        class="w-full space-y-2"
      >
        {#each groups as group (group.id)}
          {@const GroupIcon = group.icon || GlobeAlt}
          <AccordionItem
            value={group.id}
            class="rounded-lg border border-border/40 bg-card/40 transition-colors overflow-hidden"
          >
            <AccordionTrigger class="py-3 px-4 text-xs font-medium text-foreground hover:no-underline">
              <div class="flex items-center gap-3 min-w-0 flex-1 text-start">
                <div class="flex items-center justify-center w-4 h-4 text-muted-foreground shrink-0">
                  <GroupIcon size={15} />
                </div>
                {#if group.title}
                  <span class="truncate text-xs font-medium text-foreground">
                    {group.title}
                  </span>
                {/if}
              </div>
            </AccordionTrigger>

            <AccordionContent class="px-4 pb-4 pt-0 text-start">
              <div class="flex flex-col items-start gap-1.5 ps-7 pt-1">
                {#each group.subLinks as subLink (subLink.id || subLink.label)}
                  {#if subLink.href}
                    <a
                      href={localizeHref(subLink.href)}
                      class="text-xs font-normal text-primary hover:underline transition-colors text-start {subLink.disabled ? 'pointer-events-none opacity-40' : ''}"
                    >
                      {subLink.label}
                    </a>
                  {:else}
                    <button
                      type="button"
                      onclick={subLink.onclick}
                      disabled={subLink.disabled}
                      class="text-xs font-normal text-primary hover:underline transition-colors text-start cursor-pointer disabled:pointer-events-none disabled:opacity-40"
                    >
                      {subLink.label}
                    </button>
                  {/if}
                {/each}
              </div>
            </AccordionContent>
          </AccordionItem>
        {/each}
      </Accordion>
    {/if}

    <!-- Footer Action Items (Bottom Links with subtle icons) -->
    {#if footerActions && footerActions.length > 0}
      <div class="flex flex-col items-start gap-1.5 pt-2 px-1">
        {#each footerActions as footer (footer.id || footer.label)}
          {@const FooterIcon = footer.icon || QuestionMarkCircle}
          {#if footer.href}
            <a
              href={localizeHref(footer.href)}
              class="inline-flex items-center gap-2 text-xs text-muted-foreground hover:text-foreground transition-colors {footer.variant === 'destructive' ? 'text-destructive hover:text-destructive' : ''} {footer.disabled ? 'pointer-events-none opacity-40' : ''}"
            >
              <FooterIcon size={14} class="opacity-80 shrink-0" />
              <span>{footer.label}</span>
            </a>
          {:else}
            <button
              type="button"
              onclick={footer.onclick}
              disabled={footer.disabled}
              class="inline-flex items-center gap-2 text-xs text-muted-foreground hover:text-foreground transition-colors cursor-pointer {footer.variant === 'destructive' ? 'text-destructive hover:text-destructive' : ''} disabled:pointer-events-none disabled:opacity-40"
            >
              <FooterIcon size={14} class="opacity-80 shrink-0" />
              <span>{footer.label}</span>
            </button>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
{/if}
