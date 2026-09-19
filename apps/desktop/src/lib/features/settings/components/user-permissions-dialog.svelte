<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Input } from "@orbis/ui/input";
import { ChevronRight, Search, UserCheck, Shield, Users } from "@lucide/svelte";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { l } from "$lib/core/i18n";
import type { UserDto } from "$lib/api/types";

let {
  open = $bindable(false),
  users = [],
  onselect,
}: {
  open: boolean;
  users: UserDto[];
  onselect: (userId: string) => void;
} = $props();

let searchQuery = $state("");

const filteredUsers = $derived.by(() => {
  const q = searchQuery.toLowerCase().trim();
  if (!q) return users;
  return users.filter((u) => {
    const name = (u.full_name || "").toLowerCase();
    const nameAr = (u.full_name_ar || "").toLowerCase();
    const email = (u.email || "").toLowerCase();
    return name.includes(q) || nameAr.includes(q) || email.includes(q);
  });
});
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md text-start bg-card border border-border/80 shadow-2xl p-6">
    <DialogHeader class="space-y-1.5">
      <div class="flex items-center gap-3">
        <div class="w-9 h-9 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary">
          <UserCheck class="w-4 h-4" />
        </div>
        <div>
          <DialogTitle class="text-base font-bold text-foreground">
            {l("فحص وتعديل أدوار مستخدم", "Inspect User Permissions")}
          </DialogTitle>
          <DialogDescription class="text-xs text-muted-foreground mt-0.5">
            {l("اختر مستخدماً لإدارة وتعديل أدواره وصلاحياته في المنظومة", "Select a user to manage their assigned roles and permissions")}
          </DialogDescription>
        </div>
      </div>
    </DialogHeader>

    <!-- Search Bar -->
    <div class="relative my-2">
      <Search class="absolute start-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
      <Input
        type="search"
        placeholder={l("البحث باسم المستخدم أو بريده الإلكتروني...", "Search by user name or email...")}
        bind:value={searchQuery}
        class="h-9 ps-9 text-xs"
      />
    </div>

    <!-- Users Scrollable List -->
    <div class="flex flex-col gap-1.5 max-h-72 overflow-y-auto pr-1 py-1">
      {#if filteredUsers.length === 0}
        <div class="text-xs text-muted-foreground text-center py-8 border border-dashed rounded-xl flex flex-col items-center gap-2">
          <Users class="w-6 h-6 text-muted-foreground/50" />
          <span>{l("لا يوجد مستخدمين مطابقين للبحث", "No matching users found")}</span>
        </div>
      {:else}
        {#each filteredUsers as user}
          {@const displayName = user.full_name_ar || user.full_name || user.email || "User"}
          {@const initial = displayName.charAt(0).toUpperCase()}

          <button
            type="button"
            onclick={() => {
              onselect(user.id);
              open = false;
            }}
            class="w-full text-start p-3 rounded-xl border border-border/70 bg-background/60 hover:bg-accent/60 hover:border-primary/40 transition-all flex items-center justify-between group cursor-pointer shadow-2xs"
          >
            <div class="flex items-center gap-3 min-w-0">
              <div class="w-8 h-8 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-xs font-bold text-primary shrink-0 font-mono">
                {initial}
              </div>
              <div class="flex flex-col gap-0.5 min-w-0">
                <span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors truncate">
                  {displayName}
                </span>
                <span class="text-[10px] text-muted-foreground font-mono truncate">
                  {user.email || user.id}
                </span>
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              <Badge variant="secondary" class="text-[10px] font-semibold">
                {(user as any).primary_role || (user as any).role || "user"}
              </Badge>
              <ChevronRight class="w-4 h-4 text-muted-foreground group-hover:text-foreground transition-transform group-hover:translate-x-0.5 rtl:group-hover:-translate-x-0.5" />
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
        {m.common_titlebar_close ? m.common_titlebar_close() : l("إغلاق", "Close")}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
