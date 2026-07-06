<!--
@component ChatHistoryPopover
Displays a popover of recent chats with search capability.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui-svelte/popover";
import { History } from "lucide-svelte";
import SearchField from "$lib/shared/components/search-field.svelte";

interface Message {
  id: string;
  role: "user" | "assistant";
  content: string;
}

interface HistoryItem {
  id: string;
  title: string;
  timeAgo: string;
  messages: Message[];
}

let {
  onSelect,
}: {
  onSelect: (messages: Message[]) => void;
} = $props();

const staticItems: HistoryItem[] = [
  {
    id: "hist-1",
    title: "Hi",
    timeAgo: "7 minutes ago",
    messages: [
      { id: "m1", role: "user", content: "Hi" },
      { id: "m2", role: "assistant", content: "" }
    ]
  },
  {
    id: "hist-2",
    title: "غير الوضع الى الليلي",
    timeAgo: "4 days ago",
    messages: [
      { id: "m3", role: "user", content: "غير الوضع الى الليلي" },
      { id: "m4", role: "assistant", content: "تم تغيير المظهر إلى الوضع الليلي بنجاح!" }
    ]
  },
  {
    id: "hist-3",
    title: "You are generating a Sentry dashboard. Follo...",
    timeAgo: "5 days ago",
    messages: [
      { id: "m5", role: "user", content: "أنت تقوم بإنشاء لوحة تحكم Sentry. اتبع هذه التعليمات." },
      { id: "m6", role: "assistant", content: "أنا مستعد لمساعدتك في بناء لوحة تحكم Sentry بناءً على المقاييس الخاصة بك." }
    ]
  }
];

let searchQuery = $state("");

const filteredItems = $derived(
  staticItems.filter((item) =>
    item.title.toLowerCase().includes(searchQuery.toLowerCase())
  )
);
</script>

<Popover>
  <PopoverTrigger>
    <Button
      variant="ghost"
      size="icon-xs"
      class="w-6 h-6 flex items-center justify-center rounded-md cursor-pointer"
      title="History"
    >
      <History size={13} />
    </Button>
  </PopoverTrigger>
  
  <PopoverContent 
    align={""}
    class="w-58 space-y-2 z-50 select-none"
  >
    <div class="text-[10px] font-semibold text-foreground px-0.5 text-start tracking-wide">
      Chat history
    </div>
    
    <SearchField
      bind:value={searchQuery}
      placeholder="Search chats..."
      class="w-full max-w-full md:max-w-full"
    />
    

    <div class="flex flex-col gap-4.5 max-h-56 overflow-y-auto scrollbar-hide text-start px-0.5">
      {#if filteredItems.length === 0}
        <div class="text-center text-[11.5px] text-muted-foreground py-6">
          No chats found
        </div>
      {:else}
        {#each filteredItems as item}
          <button
            onclick={() => onSelect(item.messages)}
            class="w-full text-start bg-transparent hover:bg-transparent transition-all cursor-pointer group flex flex-col gap-1 focus:outline-none"
          >
            <div class="text-[12.5px] font-normal leading-normal text-foreground group-hover:text-primary transition-colors break-words">
              {item.title}
            </div>
            <div class="text-[10px] text-muted-foreground/80 underline decoration-muted-foreground/40 underline-offset-2 tracking-wide group-hover:text-primary/90 transition-colors">
              {item.timeAgo}
            </div>
          </button>
        {/each}
      {/if}
    </div>
  </PopoverContent>
</Popover>
