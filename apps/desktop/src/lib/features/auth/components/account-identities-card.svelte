<script lang="ts">
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { toast } from "@orbis/ui";
import { api } from "$lib/api";
import { Link2, CheckCircle2, Shield, Globe } from "@lucide/svelte";

interface Props {
  isGoogleLinked?: boolean;
  googleEmail?: string | null;
  onRefresh?: () => void;
}

let { isGoogleLinked = false, googleEmail = null, onRefresh }: Props = $props();

let isLinking = $state(false);

async function handleLinkGoogle() {
  isLinking = true;
  try {
    // In Tauri desktop, open the Google OAuth popup/deep-link or simulate token linking
    toast.info("Connecting to Google Identity Services...");
    // Mock linking for local demo / deep link
    setTimeout(() => {
      isLinking = false;
      toast.success("Google account linked successfully!");
      isGoogleLinked = true;
      googleEmail = "user@gmail.com";
      onRefresh?.();
    }, 1200);
  } catch (err: any) {
    toast.error(err.message || "Failed to link Google account");
    isLinking = false;
  }
}
</script>

<Card class="border border-border/80 shadow-sm rounded-2xl overflow-hidden">
  <CardHeader class="bg-muted/40 px-6 py-4 border-b border-border/60">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-xl bg-primary/10 text-primary">
          <Globe size={20} class="stroke-[2.2]" />
        </div>
        <div>
          <CardTitle class="text-sm font-bold text-foreground">
            Connected Accounts & Social Login
          </CardTitle>
          <p class="text-xs text-muted-foreground mt-0.5">
            Link external identity providers (OIDC / OAuth 2.0) for seamless 1-tap sign in.
          </p>
        </div>
      </div>
    </div>
  </CardHeader>

  <CardContent class="p-6">
    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
      <div class="flex items-start gap-3">
        <!-- Google SVG Icon -->
        <div class="p-2.5 rounded-xl bg-white dark:bg-muted border border-border shadow-sm shrink-0">
          <svg class="w-5 h-5" viewBox="0 0 24 24">
            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
          </svg>
        </div>

        <div class="flex flex-col text-start gap-1">
          <div class="flex items-center gap-2">
            <span class="text-xs font-bold text-foreground">Google Account</span>
            {#if isGoogleLinked}
              <Badge variant="success" class="text-[9px] font-bold uppercase py-0.5 px-2 rounded-full flex items-center gap-1">
                <CheckCircle2 size={10} />
                <span>Connected</span>
              </Badge>
            {/if}
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed">
            {#if isGoogleLinked}
              Linked to <span class="font-semibold text-foreground font-mono">{googleEmail || "Google Account"}</span>. You can use Google Sign-In on web and mobile.
            {:else}
              Link your Google account to enable 1-tap sign-in on web and mobile platforms.
            {/if}
          </p>
        </div>
      </div>

      <div class="shrink-0 w-full sm:w-auto">
        {#if isGoogleLinked}
          <Button
            variant="outline"
            size="sm"
            class="w-full sm:w-auto rounded-xl text-xs font-medium text-destructive hover:bg-destructive/10 hover:border-destructive/30 h-9"
            onclick={() => {
              isGoogleLinked = false;
              googleEmail = null;
              toast.success("Google account disconnected");
              onRefresh?.();
            }}
          >
            <span>Disconnect</span>
          </Button>
        {:else}
          <Button
            variant="outline"
            size="sm"
            disabled={isLinking}
            class="w-full sm:w-auto rounded-xl text-xs font-bold gap-1.5 h-9 border-border hover:bg-muted"
            onclick={handleLinkGoogle}
          >
            <Link2 size={14} />
            <span>{isLinking ? "Connecting..." : "Connect Google"}</span>
          </Button>
        {/if}
      </div>
    </div>
  </CardContent>
</Card>
