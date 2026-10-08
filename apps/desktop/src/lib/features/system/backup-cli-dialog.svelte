<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Dialog, DialogContent } from "@orbis/ui/dialog";
import { Input } from "@orbis/ui/input";

type SessionState = "locked" | "connecting" | "ready" | "error";

let {
  open = $bindable(false),
  orgName = "Orbis School",
  onunlock,
}: {
  open: boolean;
  orgName?: string;
  onunlock?: (password: string) => Promise<string | void>;
} = $props();

let password = $state("");
let sessionState = $state<SessionState>("locked");

function createInitialOutput() {
  return [
    "   ____  ____  ____  _________   ________    ____",
    "  / __ \/ __ \/ __ )/  _/ ___/  / ____/ /   /  _/",
    " / / / / /_/ / __  |/ / \__ \  / /   / /    / /",
    "/ /_/ / _, _/ /_/ // / ___/ / / /___/ /____/ /",
    "\\____/_/ |_/_____/___//____/  \\____/_____/___/",
    "",
    `  ${m.shared_ui_enterprise_management_console()}`,
    "  ========================================",
    `  ${m.shared_ui_status_label()}:      ${m.shared_ui_locked()}`,
    `  ${m.shared_ui_environment_label()}: ${m.shared_ui_desktop()}`,
    "  ========================================",
    `  Organization: ${orgName}`,
    "",
  ];
}

let output = $state<string[]>(createInitialOutput());

function resetSession() {
  password = "";
  sessionState = "locked";
  output = createInitialOutput();
}

async function unlock() {
  if (!password.trim() || sessionState === "connecting") return;

  sessionState = "connecting";
  output = [...output, "super@MxHabob:~$ ********", m.shared_ui_authenticating()];

  try {
    if (!onunlock) {
      throw new Error(m.shared_ui_cli_session_not_connected());
    }

    const response = await onunlock(password);
    password = "";
    sessionState = "ready";
    output = [...output, response || m.shared_ui_authenticated(), "", "orbis> _"];
  } catch (error) {
    password = "";
    sessionState = "error";
    output = [
      ...output,
      error instanceof Error ? error.message : m.shared_ui_authentication_failed(),
      m.shared_ui_enter_password_to_retry(),
    ];
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Enter") {
    event.preventDefault();
    unlock();
  }
}
</script>

<Dialog bind:open onOpenChange={(value) => !value && resetSession()}>
  <DialogContent
    showCloseButton={false}
    dir="ltr"
    class="w-[min(640px,calc(100vw-2rem))] max-w-2xl overflow-hidden rounded-none border-0 bg-black p-0 text-left font-mono text-green-400 shadow-2xl"
  >
    <div class="flex min-h-96 flex-col bg-black p-5 font-mono text-xs leading-6">
      <div class="max-h-[70vh] overflow-y-auto scrollbar-hide" aria-live="polite">
        {#each output as line}
          <div class="whitespace-pre-wrap">{line || "\u00a0"}</div>
        {/each}
      </div>

      {#if sessionState === "locked" || sessionState === "error"}
        <form onsubmit={(event) => { event.preventDefault(); unlock(); }} class="mt-4 flex items-center gap-2">
          <span class="shrink-0"><span class="font-semibold text-green-500">super@MxHabob</span><span class="text-white">:~$</span></span>
          <Input
            id="backup-cli-password"
            type="password"
            bind:value={password}
            onkeydown={handleKeydown}
            autocomplete="current-password"
            aria-label={m.shared_ui_super_admin_password()}
            placeholder={m.shared_ui_password()}
            class="h-5 min-h-0 flex-1 rounded-none border-0 bg-transparent p-0 font-mono text-xs leading-5 text-green-300 shadow-none outline-none placeholder:text-green-900 focus-visible:ring-0"
          />
        </form>
      {:else if sessionState === "connecting"}
        <div class="mt-4 text-amber-300">{m.shared_ui_authenticating()}</div>
      {:else}
        <div class="mt-4 flex items-center gap-2">
          <span class="shrink-0"><span class="font-semibold text-green-500">super@MxHabob</span><span class="text-white">:~$</span></span>
          <Input
            type="text"
            aria-label={m.shared_ui_cli_command()}
            placeholder="_"
            class="h-5 min-h-0 flex-1 rounded-none border-0 bg-transparent p-0 font-mono text-xs leading-5 text-green-300 shadow-none outline-none placeholder:text-green-400 focus-visible:ring-0"
          />
        </div>
      {/if}
    </div>
  </DialogContent>
</Dialog>
