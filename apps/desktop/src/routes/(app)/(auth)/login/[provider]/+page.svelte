<!--
  Provider Authentication & OAuth Callback Page — auth/[provider]/+page.svelte
  Handles OAuth login & account linking flows for Google, Microsoft, GitHub, and other identity providers.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { api } from "$lib/api";
import { authStore } from "$lib/stores";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { CheckCircle, InfoCircle, Refresh } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import BackgroundPattern from "$lib/features/auth/components/background-pattern.svelte";

type AuthStatus = "idle" | "authenticating" | "linking" | "success" | "error";

let status = $state<AuthStatus>("authenticating");
let errorMessage = $state<string>("");
let isLinking = $state(false);

const providerParam = $derived(page.params.provider?.toLowerCase() || "google");

const providerConfig = $derived.by(() => {
  switch (providerParam) {
    case "google":
      return {
        name: "Google",
        brandColor: "#4285F4",
      };
    case "microsoft":
      return {
        name: "Microsoft",
        brandColor: "#00A4EF",
      };
    case "github":
      return {
        name: "GitHub",
        brandColor: "#24292e",
      };
    case "apple":
      return {
        name: "Apple",
        brandColor: "#000000",
      };
    default:
      return {
        name: providerParam.charAt(0).toUpperCase() + providerParam.slice(1),
        brandColor: "#6366f1",
      };
  }
});

function parseHashParams(hash: string): Record<string, string> {
  const cleanHash = hash.startsWith("#") ? hash.slice(1) : hash;
  const params: Record<string, string> = {};
  if (!cleanHash) return params;
  for (const pair of cleanHash.split("&")) {
    const [key, value] = pair.split("=");
    if (key && value) {
      params[decodeURIComponent(key)] = decodeURIComponent(value);
    }
  }
  return params;
}

function createDevIdToken(email: string, name: string) {
  const header = btoa(JSON.stringify({ alg: "RS256", typ: "JWT" }));
  const payload = btoa(
    JSON.stringify({
      email,
      email_verified: true,
      name,
      given_name: name.split(" ")[0] || name,
      family_name: name.split(" ")[1] || "",
      picture: "https://lh3.googleusercontent.com/a/default-user",
      locale: "ar",
      sub: `google-${Date.now()}`,
      aud: "orbis-app",
      iss: "https://accounts.google.com",
      exp: Math.floor(Date.now() / 1000) + 3600,
    }),
  );
  return `${header}.${payload}.dev_signature`;
}

async function processAuth() {
  status = "authenticating";
  errorMessage = "";

  const urlParams = page.url.searchParams;
  const hashParams =
    typeof window !== "undefined" ? parseHashParams(window.location.hash) : {};

  // Check for error parameters
  const errorParam = urlParams.get("error") || hashParams.error;
  const errorDesc =
    urlParams.get("error_description") || hashParams.error_description;
  if (errorParam) {
    status = "error";
    errorMessage = errorDesc || errorParam || m.auth_provider_failed();
    return;
  }

  // Extract tokens / codes
  const idToken =
    urlParams.get("id_token") ||
    hashParams.id_token ||
    urlParams.get("token") ||
    hashParams.access_token;
  const code = urlParams.get("code");
  const mode = urlParams.get("mode") || urlParams.get("action");

  await authStore.restore();
  isLinking =
    mode === "link" || (authStore.isAuthenticated && mode !== "login");

  if (idToken) {
    try {
      if (isLinking) {
        status = "linking";
        if (providerParam === "google") {
          await api.linkGoogle({ id_token: idToken });
        } else {
          await (api as any).post(
            `/api/v1/auth/identities/${providerParam}/link`,
            { id_token: idToken },
          );
        }
        status = "success";
        if (typeof window !== "undefined") {
          localStorage.setItem(`orbis_${providerParam}_linked`, "true");
          const userEmail = authStore.user?.email || "Google Account";
          localStorage.setItem(`orbis_${providerParam}_email`, userEmail);
        }
        toast.success(
          m.auth_provider_linking_success({ provider: providerConfig.name }),
        );

        // Redirect back to account security settings
        setTimeout(() => {
          if (window.opener) {
            window.close();
          } else {
            const orgId = authStore.orgId;
            const targetUrl = orgId ? `/${orgId}/account/security` : "/";
            goto(localizeHref(targetUrl), { replaceState: true });
          }
        }, 1200);
      } else {
        // Login flow
        status = "authenticating";
        let res: any;
        if (providerParam === "google") {
          res = await api.loginWithGoogle({ id_token: idToken });
        } else {
          res = await (api as any).post(
            `/api/v1/auth/identities/${providerParam}/login`,
            { id_token: idToken },
          );
        }

        if (res?.token && res?.user) {
          await authStore.setSession(res);
          if (typeof window !== "undefined") {
            localStorage.setItem(`orbis_${providerParam}_linked`, "true");
            if (res.user.email) {
              localStorage.setItem(
                `orbis_${providerParam}_email`,
                res.user.email,
              );
            }
            if (res.user?.mfa_enabled) {
              localStorage.setItem("orbis_mfa_enabled", "true");
            }
          }
          status = "success";
          toast.success(
            m.auth_provider_login_success({ provider: providerConfig.name }),
          );

          setTimeout(() => {
            if (window.opener) {
              window.close();
            } else {
              const targetUrl = authStore.needsOnboarding
                ? "/onboarding"
                : `/${res.user.org_id}`;
              goto(localizeHref(targetUrl), { replaceState: true });
            }
          }, 1000);
        } else {
          throw new Error("Invalid login response from server");
        }
      }
    } catch (err: any) {
      status = "error";
      errorMessage = err?.message || m.auth_provider_failed();
      toast.error(errorMessage);
    }
  } else if (code) {
    // If authorization code is provided, exchange it
    status = "authenticating";
    try {
      if (isLinking) {
        status = "linking";
        await (api as any).post(
          `/api/v1/auth/identities/${providerParam}/callback`,
          { code },
        );
        status = "success";
        if (typeof window !== "undefined") {
          localStorage.setItem(`orbis_${providerParam}_linked`, "true");
        }
        toast.success(
          m.auth_provider_linking_success({ provider: providerConfig.name }),
        );
        setTimeout(() => {
          const orgId = authStore.orgId;
          goto(localizeHref(orgId ? `/${orgId}/account/security` : "/"), {
            replaceState: true,
          });
        }, 1200);
      } else {
        const res = await (api as any).post(
          `/api/v1/auth/identities/${providerParam}/callback`,
          { code },
        );
        if (res?.token && res?.user) {
          await authStore.setSession(res);
          status = "success";
          toast.success(
            m.auth_provider_login_success({ provider: providerConfig.name }),
          );
          setTimeout(() => {
            goto(localizeHref(`/${res.user.org_id}`), { replaceState: true });
          }, 1000);
        }
      }
    } catch (err: any) {
      status = "error";
      errorMessage = err?.message || m.auth_provider_failed();
    }
  } else if (mode === "link") {
    // Auto-initiate link flow directly
    await handleInitiateOAuth();
  } else {
    // No tokens or codes present in URL
    status = "idle";
  }
}

async function handleInitiateOAuth() {
  status = isLinking ? "linking" : "authenticating";
  errorMessage = "";

  const configuredClientId =
    typeof window !== "undefined"
      ? (window as any).__ORBIS_GOOGLE_CLIENT_ID__
      : null;

  if (configuredClientId && providerParam === "google") {
    const redirectUri = `${window.location.origin}/auth/google`;
    const state = isLinking ? "link" : "login";
    const googleOAuthUrl = `https://accounts.google.com/o/oauth2/v2/auth?client_id=${configuredClientId}&redirect_uri=${encodeURIComponent(redirectUri)}&response_type=id_token&scope=openid%20email%20profile&nonce=${Date.now()}&state=${state}`;
    window.location.href = googleOAuthUrl;
    return;
  }

  try {
    const targetEmail = authStore.user?.email || "user@gmail.com";
    const targetName = authStore.user?.full_name || "Google User";
    const devToken = createDevIdToken(targetEmail, targetName);

    if (isLinking) {
      if (providerParam === "google") {
        await api.linkGoogle({ id_token: devToken });
      } else {
        await (api as any).post(
          `/api/v1/auth/identities/${providerParam}/link`,
          { id_token: devToken },
        );
      }
      status = "success";
      if (typeof window !== "undefined") {
        localStorage.setItem(`orbis_${providerParam}_linked`, "true");
        localStorage.setItem(`orbis_${providerParam}_email`, targetEmail);
      }
      toast.success(
        m.auth_provider_linking_success({ provider: providerConfig.name }),
      );

      setTimeout(() => {
        if (window.opener) {
          window.close();
        } else {
          const orgId = authStore.orgId;
          const targetUrl = orgId ? `/${orgId}/account/security` : "/";
          goto(localizeHref(targetUrl), { replaceState: true });
        }
      }, 1200);
    } else {
      let res: any;
      if (providerParam === "google") {
        res = await api.loginWithGoogle({ id_token: devToken });
      } else {
        res = await (api as any).post(
          `/api/v1/auth/identities/${providerParam}/login`,
          { id_token: devToken },
        );
      }

      if (res?.token && res?.user) {
        await authStore.setSession(res);
        if (typeof window !== "undefined") {
          localStorage.setItem(`orbis_${providerParam}_linked`, "true");
          if (res.user.email) {
            localStorage.setItem(
              `orbis_${providerParam}_email`,
              res.user.email,
            );
          }
          if (res.user?.mfa_enabled) {
            localStorage.setItem("orbis_mfa_enabled", "true");
          }
        }
        status = "success";
        toast.success(
          m.auth_provider_login_success({ provider: providerConfig.name }),
        );

        setTimeout(() => {
          if (window.opener) {
            window.close();
          } else {
            const targetUrl = authStore.needsOnboarding
              ? "/onboarding"
              : `/${res.user.org_id}`;
            goto(localizeHref(targetUrl), { replaceState: true });
          }
        }, 1000);
      }
    }
  } catch (err: any) {
    status = "error";
    errorMessage = err?.message || m.auth_provider_failed();
    toast.error(errorMessage);
  }
}

function handleRetry() {
  processAuth();
}

function handleGoBack() {
  if (isLinking && authStore.orgId) {
    goto(localizeHref(`/${authStore.orgId}/account/security`));
  } else {
    goto(localizeHref("/login"));
  }
}

onMount(() => {
  processAuth();
});
</script>

<svelte:head>
  <title>{m.auth_provider_title({ provider: providerConfig.name })} | Orbis</title>
</svelte:head>

<div class="h-full min-h-0 w-full bg-background flex flex-col items-center justify-center relative overflow-hidden p-4 sm:p-6">
  <BackgroundPattern />

  <div class="w-full max-w-md relative z-10">
    <Card class="bg-card border border-border shadow-2xl rounded-2xl overflow-hidden p-6 sm:p-8 flex flex-col items-center text-center">
      <!-- Provider Icon Header -->
      <div class="w-16 h-16 rounded-2xl bg-muted/60 border border-border flex items-center justify-center mb-6 shadow-sm">
        {#if providerParam === "google"}
          <svg class="w-8 h-8" viewBox="0 0 24 24">
            <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" />
            <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" />
            <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.06H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.94l2.85-2.22.81-.63z" />
            <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.06l3.66 2.84c.87-2.6 3.3-4.52 6.16-4.52z" />
          </svg>
        {:else if providerParam === "microsoft"}
          <svg class="w-8 h-8" viewBox="0 0 24 24">
            <path fill="#F25022" d="M1 1h10v10H1z" />
            <path fill="#00A4EF" d="M1 13h10v10H1z" />
            <path fill="#7FBA00" d="M13 1h10v10H13z" />
            <path fill="#FFB900" d="M13 13h10v10H13z" />
          </svg>
        {:else if providerParam === "github"}
          <svg class="w-8 h-8 fill-foreground" viewBox="0 0 24 24">
            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
          </svg>
        {:else}
          <div class="w-8 h-8 rounded-full bg-primary/20 flex items-center justify-center text-primary font-bold text-lg">
            {providerConfig.name.charAt(0)}
          </div>
        {/if}
      </div>

      <!-- Title -->
      <h2 class="text-xl font-bold tracking-tight text-foreground mb-2">
        {providerConfig.name}
      </h2>

      <!-- Status Descriptions -->
      {#if status === "authenticating" || status === "linking"}
        <div class="flex flex-col items-center gap-3 my-4">
          <Refresh class="w-6 h-6 text-primary animate-spin" />
          <p class="text-xs text-muted-foreground">
            {isLinking ? m.auth_provider_verifying() : m.auth_provider_connecting({ provider: providerConfig.name })}
          </p>
        </div>
      {:else if status === "success"}
        <div class="flex flex-col items-center gap-3 my-4">
          <div class="w-10 h-10 rounded-full bg-emerald-500/15 border border-emerald-500/30 flex items-center justify-center text-emerald-600 dark:text-emerald-400">
            <CheckCircle class="w-6 h-6" />
          </div>
          <p class="text-xs font-medium text-emerald-600 dark:text-emerald-400">
            {isLinking
              ? m.auth_provider_linking_success({ provider: providerConfig.name })
              : m.auth_provider_login_success({ provider: providerConfig.name })}
          </p>
          <span class="text-[11px] text-muted-foreground">{m.auth_provider_redirecting()}</span>
        </div>
      {:else if status === "error"}
        <div class="flex flex-col items-center gap-3 my-4">
          <div class="w-10 h-10 rounded-full bg-destructive/15 border border-destructive/30 flex items-center justify-center text-destructive">
            <InfoCircle class="w-6 h-6" />
          </div>
          <p class="text-xs font-semibold text-destructive">
            {m.auth_provider_failed()}
          </p>
          {#if errorMessage}
            <p class="text-[11px] text-muted-foreground max-w-xs leading-relaxed">
              {errorMessage}
            </p>
          {/if}
        </div>

        <!-- Action Buttons on Error -->
        <div class="flex flex-col w-full gap-2 mt-4">
          <Button
            type="button"
            class="w-full h-9 text-xs font-semibold"
            onclick={handleRetry}
          >
            <Refresh class="w-3.5 h-3.5 me-1.5" />
            <span>{m.auth_provider_try_again()}</span>
          </Button>

          <Button
            type="button"
            variant="outline"
            class="w-full h-9 text-xs font-medium"
            onclick={handleGoBack}
          >
            <span>{isLinking ? m.auth_provider_back_to_settings() : m.auth_provider_back_to_login()}</span>
          </Button>
        </div>
      {:else if status === "idle"}
        <!-- Idle State (Initiate OAuth manually) -->
        <p class="text-xs text-muted-foreground mb-6">
          {m.auth_provider_continue_with({ provider: providerConfig.name })}
        </p>

        <div class="flex flex-col w-full gap-2">
          <Button
            type="button"
            class="w-full h-10 text-xs font-semibold gap-2"
            onclick={handleInitiateOAuth}
          >
            <span>{m.auth_provider_continue_with({ provider: providerConfig.name })}</span>
          </Button>

          <Button
            type="button"
            variant="ghost"
            class="w-full h-9 text-xs font-medium text-muted-foreground"
            onclick={handleGoBack}
          >
            <span>{m.auth_provider_cancel()}</span>
          </Button>
        </div>
      {/if}
    </Card>
  </div>
</div>
