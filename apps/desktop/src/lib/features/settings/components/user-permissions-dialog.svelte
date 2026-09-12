<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { ChevronRight, Search, UserCheck, Shield, Sparkles } from "@lucide/svelte";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";

let {
  open = $bindable(false),
  users = [],
  searchQuery = $bindable(""),
  filteredUsers = [],
  onselect,
}: {
  open: boolean;
  users: any[];
  searchQuery: string;
  filteredUsers: any[];
  onselect: (id: string) => void;
} = $props();
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md text-start bg-card border border-border/80 shadow-2xl p-6">
    <DialogHeader class="space-y-1.5">
      <div class="flex items-center gap-2">
        <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <UserCheck class="w-4 h-4" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            {m.settings_permissions_dialog_user_title()}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground">
            {m.settings_permissions_dialog_user_desc()}
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <!-- Search Input -->
    <div class="relative my-2">
      <Search class="absolute start-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
      <Input
        type="search"
        placeholder={m.settings_permissions_search_user_placeholder()}
        bind:value={searchQuery}
        class="h-9 ps-9 text-xs"
      />
    </div>

    <!-- Users List -->
    <div class="flex flex-col gap-1.5 max-h-72 overflow-y-auto pr-1 py-1">
      {#if filteredUsers.length === 0}
        <div class="text-xs text-muted-foreground text-center py-8 border border-dashed rounded-xl flex flex-col items-center gap-2">
          <Users class="w-6 h-6 text-muted-foreground/50" />
          <span>{m.settings_users_empty()}</span>
        </div>
      {:else}
        {#each filteredUsers as user}
          <button
            type="button"
            onclick={() => onselect(user.id)}
            class="w-full text-start p-3 rounded-xl border border-border/70 bg-background/60 hover:bg-accent hover:border-primary/30 transition-all flex items-center justify-between group cursor-pointer shadow-2xs"
          >
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-xs font-bold text-primary shrink-0 font-mono">
                {(user.full_name || user.email || "U").charAt(0).toUpperCase()}
              </div>
              <div class="flex flex-col gap-0.5 min-w-0">
                <span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors truncate">
                  {user.full_name || user.email}
                </span>
                <span class="text-[10px] text-muted-foreground font-mono truncate">
                  {user.email || user.id}
                </span>
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              <Badge variant="secondary" class="text-[10px] font-semibold">
                {user.primary_role || user.role || "user"}
              </Badge>
              <ChevronRight class="w-4 h-4 text-muted-foreground group-hover:text-foreground transition-transform group-hover:translate-x-0.5" />
            </div>
          </button>
        {/each}
      {/if}
    </div>

    <DialogFooter class="border-t border-border pt-4 mt-2">
      <Button
        variant="ghost"
        size="sm"
        class="h-8 text-xs font-medium"
        onclick={() => (open = false)}
      >
        {m.common_titlebar_close()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
