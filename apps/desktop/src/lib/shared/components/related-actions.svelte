<!--
@component RelatedActions
Generic, minimal Related Actions / Support component using Accordion.
-->
<script lang="ts">
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
  AccordionContent,
} from "@orbis/ui/accordion";
import { Button } from "@orbis/ui/button";
import { AdjustmentsHorizontal, GlobeAlt, QuestionMarkCircle } from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

export interface SubLink {
  label: string;
  href?: string;
  onclick?: () => void;
}

export interface ActionGroup {
  id: string;
  title?: string;
  icon?: any;
  subLinks: SubLink[];
  defaultExpanded?: boolean;
}

export interface FooterAction {
  label: string;
  icon?: any;
  href?: string;
  onclick?: () => void;
}

let {
  groups = [],
  footerActions = [],
  class: className = "",
}: {
  groups: ActionGroup[];
  footerActions?: FooterAction[];
  class?: string;
} = $props();

const isArabic = $derived(getLocale() === "ar");

const defaultValues = $derived(
  groups.filter((g) => g.defaultExpanded ?? true).map((g) => g.id),
);
</script>

<div class="flex flex-col w-full font-sans {className}">
  <Accordion
    type="multiple"
    value={defaultValues}
    class="w-full space-y-1"
  >
    {#each groups as group (group.id)}
      {@const IconComponent = group.icon || GlobeAlt}

      <AccordionItem 
        value={group.id} 
        class="rounded-lg overflow-hidden"
      >
        <AccordionTrigger class="text-sm font-medium text-foreground hover:no-underline">
          <div class="flex items-center gap-2.5 min-w-0 flex-1 text-start">
            <div class="flex items-center justify-center w-4.5 h-4.5 text-muted-foreground shrink-0">
              <IconComponent size={15} />
            </div>
            {#if group.title}
              <span class="truncate">
                {group.title}
              </span>
            {/if}
          </div>
        </AccordionTrigger>

        <AccordionContent class="px-3.5 sm:px-10.5 text-start">
          <div class="flex flex-col items-start gap-1.5 pt-1">
            {#each group.subLinks as subLink}
              <Button
                variant="ghost"
                size="sm"
                href={subLink.href ? localizeHref(subLink.href) : undefined}
                onclick={subLink.onclick}
                class="h-auto p-0 font-normal text-primary hover:bg-transparent hover:underline"
              >
                {subLink.label}
              </Button>
            {/each}
          </div>
        </AccordionContent>

      </AccordionItem>
    {/each}
  </Accordion>

  {#if footerActions && footerActions.length > 0}
    <div class="flex flex-col items-start gap-0.5 pt-1">
      {#each footerActions as footer}
        {@const FooterIcon = footer.icon || HelpCircle}
        <Button
          variant="ghost"
          size="sm"
          href={footer.href ? localizeHref(footer.href) : undefined}
          onclick={footer.onclick}
          class="h-6 px-1 text-muted-foreground hover:text-foreground font-normal gap-2"
        >
          <FooterIcon size={13} class="opacity-70" />
          <span>{footer.label}</span>
        </Button>
      {/each}
    </div>
  {/if}
</div>
