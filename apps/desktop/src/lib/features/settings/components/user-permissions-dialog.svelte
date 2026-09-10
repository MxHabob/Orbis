<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { ChevronRight } from "@lucide/svelte";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@orbis/ui/dialog";
import SearchField from "$lib/shared/components/search-field.svelte";
let {
  open = $bindable(false),
  users,
  searchQuery = $bindable(""),
  filteredUsers,
  onselect,
}: {
  open: boolean;
  users: any[];
  searchQuery: string;
  filteredUsers: any[];
  onselect: (id: string) => void;
} = $props();
</script>
<Dialog bind:open><DialogContent class="max-w-md text-start"><DialogHeader><DialogTitle>{m.settings_permissions_dialog_user_title()}</DialogTitle><p class="text-sm text-muted-foreground">{m.settings_permissions_dialog_user_desc()}</p></DialogHeader><div class="my-2"><SearchField placeholder={m.settings_permissions_search_user_placeholder()} bind:value={searchQuery} class="max-w-none w-full" /></div><div class="flex flex-col gap-1.5 max-h-60 overflow-y-auto pr-1">{#if filteredUsers.length === 0}<span class="text-xs text-muted-foreground text-center py-6">{m.settings_users_empty()}</span>{:else}{#each filteredUsers as user}<button onclick={() => onselect(user.id)} class="w-full text-start p-2.5 rounded-lg border border-border bg-background hover:bg-accent transition-all flex items-center justify-between group cursor-pointer"><div class="flex flex-col gap-0.5"><span class="text-xs font-semibold text-foreground group-hover:text-primary transition-colors">{user.full_name}</span><span class="text-[9px] text-muted-foreground font-mono">{user.email}</span></div><div class="flex items-center gap-1.5"><Badge variant="secondary" class="text-[9px] font-semibold">{user.primary_role}</Badge><ChevronRight size={12} class="text-muted-foreground group-hover:text-foreground transition-colors" /></div></button>{/each}{/if}</div><DialogFooter class="border-t border-border pt-4"><Button variant="ghost" size="sm" onclick={() => (open = false)}>{m.common_titlebar_close()}</Button></DialogFooter></DialogContent></Dialog>
