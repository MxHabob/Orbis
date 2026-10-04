<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { api, extractErrorMessage } from "$lib/api";
import { toast } from "@orbis/ui";

type AuthenticationSettings = {
  max_failed_logins: number;
  lockout_minutes: number;
  session_days: number;
  refresh_token_days: number;
};

let { org, isSaving = false }: { org: any; isSaving?: boolean } = $props();

const defaults = (): AuthenticationSettings => ({
  max_failed_logins: 5,
  lockout_minutes: 15,
  session_days: 30,
  refresh_token_days: 30,
});

function readSettings(): AuthenticationSettings {
  const settings =
    typeof org?.settings === "string"
      ? (() => {
          try {
            return JSON.parse(org.settings);
          } catch {
            return {};
          }
        })()
      : org?.settings || {};
  return { ...defaults(), ...(settings.authentication || {}) };
}

let values = $state<AuthenticationSettings>(defaults());
let savedValues = $state<AuthenticationSettings | null>(null);

$effect(() => {
  if (org && !savedValues) {
    values = readSettings();
    savedValues = { ...values };
  }
});

const isDirty = $derived(
  savedValues !== null &&
    JSON.stringify(values) !== JSON.stringify(savedValues),
);

function setNumber(
  key: keyof AuthenticationSettings,
  raw: string,
  minimum: number,
) {
  values[key] = Math.max(minimum, Number.parseInt(raw, 10) || minimum);
}

function validate(): string | null {
  if (values.max_failed_logins < 1 || values.max_failed_logins > 20)
    return m.settings_ui_auth_max_failed_logins_error();
  if (values.lockout_minutes < 1 || values.lockout_minutes > 1440)
    return m.settings_ui_auth_lockout_duration_error();
  if (values.session_days < 1 || values.session_days > 365)
    return m.settings_ui_auth_session_duration_error();
  if (
    values.refresh_token_days < values.session_days ||
    values.refresh_token_days > 730
  )
    return m.settings_ui_auth_refresh_duration_error();
  return null;
}

async function save() {
  const validationError = validate();
  if (validationError) {
    toast.error(validationError);
    return;
  }
  try {
    await api.updateSettings({ authentication: values });
    savedValues = { ...values };
    toast.success(m.settings_ui_auth_settings_saved());
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}

function reset() {
  if (!savedValues) return;
  values = { ...savedValues };
}
</script>

<Card>
  <CardHeader>
    <div class="flex flex-col">
      <CardTitle>{m.settings_ui_auth_title()}</CardTitle>
      <CardDescription>{m.settings_ui_auth_description()}</CardDescription>
    </div>
  </CardHeader>

  <CardContent class="p-6 grid grid-cols-1 md:grid-cols-2 gap-5 text-start">
    <div class="flex flex-col gap-2">
      <Label for="auth-max-failed-logins" class="text-xs font-semibold">{m.settings_ui_auth_max_failed_logins()}</Label>
      <Input id="auth-max-failed-logins" type="number" min="1" max="20" value={values.max_failed_logins} oninput={(event: Event) => setNumber("max_failed_logins", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-lockout-minutes" class="text-xs font-semibold">{m.settings_ui_auth_lockout_minutes()}</Label>
      <Input id="auth-lockout-minutes" type="number" min="1" max="1440" value={values.lockout_minutes} oninput={(event: Event) => setNumber("lockout_minutes", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-session-days" class="text-xs font-semibold">{m.settings_ui_auth_session_days()}</Label>
      <Input id="auth-session-days" type="number" min="1" max="365" value={values.session_days} oninput={(event: Event) => setNumber("session_days", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-refresh-days" class="text-xs font-semibold">{m.settings_ui_auth_refresh_token_days()}</Label>
      <Input id="auth-refresh-days" type="number" min="1" max="730" value={values.refresh_token_days} oninput={(event: Event) => setNumber("refresh_token_days", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>
  </CardContent>

  {#if isDirty}
    <CardFooter class="justify-end gap-2">
      <Button type="button" variant="outline" size="sm" onclick={reset}>{m.common_action_cancel()}</Button>
      <Button type="button" size="sm" onclick={save} disabled={isSaving}>{m.settings_ui_auth_save_settings()}</Button>
    </CardFooter>
  {/if}
</Card>
