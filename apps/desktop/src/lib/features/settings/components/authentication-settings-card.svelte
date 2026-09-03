<script lang="ts">
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Label } from "@orbis/ui/label";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
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
    const settings = typeof org?.settings === "string"
      ? (() => { try { return JSON.parse(org.settings); } catch { return {}; } })()
      : (org?.settings || {});
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
    savedValues !== null && JSON.stringify(values) !== JSON.stringify(savedValues),
  );

  function setNumber(key: keyof AuthenticationSettings, raw: string, minimum: number) {
    values[key] = Math.max(minimum, Number.parseInt(raw, 10) || minimum);
  }

  function validate(): string | null {
    if (values.max_failed_logins < 1 || values.max_failed_logins > 20) return "عدد محاولات الدخول يجب أن يكون بين 1 و20";
    if (values.lockout_minutes < 1 || values.lockout_minutes > 1440) return "مدة القفل يجب أن تكون بين دقيقة و1440 دقيقة";
    if (values.session_days < 1 || values.session_days > 365) return "مدة الجلسة يجب أن تكون بين يوم و365 يوماً";
    if (values.refresh_token_days < values.session_days || values.refresh_token_days > 730) return "مدة التجديد يجب أن تكون أكبر أو مساوية لمدة الجلسة ولا تتجاوز 730 يوماً";
    return null;
  }

  async function save() {
    const validationError = validate();
    if (validationError) {
      toast.error(validationError);
      return;
    }
    try {
      await commands.organization.updateSettings({ authentication: values });
      savedValues = { ...values };
      toast.success("تم حفظ إعدادات المصادقة");
    } catch (error) {
      toast.error(extractErrorMessage(error));
    }
  }

  function reset() {
    if (!savedValues) return;
    values = { ...savedValues };
  }
</script>

<section class="border border-border rounded-md overflow-hidden flex flex-col">
  <div class="bg-muted/50 px-6 py-3 border-b border-border text-start">
    <h2 class="text-[11px] font-bold text-muted-foreground tracking-wider uppercase">أمان الدخول والجلسات</h2>
    <p class="text-xs text-muted-foreground mt-1">سياسة محاولات الدخول ومدد الجلسات الخاصة بالمنظمة</p>
  </div>

  <div class="p-6 grid grid-cols-1 md:grid-cols-2 gap-5 text-start">
    <div class="flex flex-col gap-2">
      <Label for="auth-max-failed-logins" class="text-xs font-semibold">الحد الأقصى للمحاولات الفاشلة</Label>
      <Input id="auth-max-failed-logins" type="number" min="1" max="20" value={values.max_failed_logins} oninput={(event: Event) => setNumber("max_failed_logins", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-lockout-minutes" class="text-xs font-semibold">مدة القفل بالدقائق</Label>
      <Input id="auth-lockout-minutes" type="number" min="1" max="1440" value={values.lockout_minutes} oninput={(event: Event) => setNumber("lockout_minutes", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-session-days" class="text-xs font-semibold">مدة الجلسة بالأيام</Label>
      <Input id="auth-session-days" type="number" min="1" max="365" value={values.session_days} oninput={(event: Event) => setNumber("session_days", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>

    <div class="flex flex-col gap-2">
      <Label for="auth-refresh-days" class="text-xs font-semibold">مدة رمز التجديد بالأيام</Label>
      <Input id="auth-refresh-days" type="number" min="1" max="730" value={values.refresh_token_days} oninput={(event: Event) => setNumber("refresh_token_days", (event.currentTarget as HTMLInputElement).value, 1)} disabled={isSaving} class="h-9 text-xs" />
    </div>
  </div>

  {#if isDirty}
    <div class="px-6 py-3 border-t border-border flex justify-end gap-2">
      <Button type="button" variant="outline" size="sm" onclick={reset}>إلغاء</Button>
      <Button type="button" size="sm" onclick={save} disabled={isSaving}>حفظ إعدادات المصادقة</Button>
    </div>
  {/if}
</section>
