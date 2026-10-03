<!--
  @component EnrollmentSettingsCard
  Configures institutional enrollment rules and defaults.
-->
<script lang="ts">
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@orbis/ui/card";
import { Label } from "@orbis/ui/label";
import { Switch } from "@orbis/ui/switch";
import { Input } from "@orbis/ui/input";
import { UserBadgeCheck } from "@orbis/icons";
import { l } from "$lib/core/i18n";

let {
  registrationEnabled = $bindable(true),
  requireAdminApproval = $bindable(true),
  allowWaitlist = $bindable(true),
  siblingPriorityEnabled = $bindable(true),
  maxClassSizeDefault = $bindable(25),
  allowLateEnrollment = $bindable(false),
  isSaving = false,
}: {
  registrationEnabled?: boolean;
  requireAdminApproval?: boolean;
  allowWaitlist?: boolean;
  siblingPriorityEnabled?: boolean;
  maxClassSizeDefault?: number;
  allowLateEnrollment?: boolean;
  isSaving?: boolean;
} = $props();
</script>

<Card class="border-border/80 shadow-xs">
  <CardHeader>
    <div class="flex items-center gap-3 text-start">
      <div class="w-8 h-8 rounded-lg bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shrink-0">
        <UserBadgeCheck class="w-4 h-4" />
      </div>
      <div class="flex flex-col">
        <CardTitle>{l("إعدادات التسجيل والقيد", "Enrollment & Admission Defaults")}</CardTitle>
        <CardDescription>{l("التحكم في بوابات التسجيل وقواعد الموافقة والطاقة الاستيعابية", "Manage registration portals, approval workflows, and class capacity defaults")}</CardDescription>
      </div>
    </div>
  </CardHeader>
  <CardContent class="flex flex-col gap-5">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("تفعيل بوابة التسجيل", "Enable Registration")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("السماح بقبول طلبات التسجيل للعام النشط", "Allow incoming student admission requests")}</span>
        </div>
        <Switch bind:checked={registrationEnabled} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("اشتراط موافقة الإدارة", "Require Admin Approval")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("التحقق اليدوي قبل اعتماد القيد النهائي", "Require administrative review before final enrollment")}</span>
        </div>
        <Switch bind:checked={requireAdminApproval} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("تفعيل قوائم الانتظار", "Allow Waitlist")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("وضع الطلاب في قائمة انتظار عند اكتمال الشعب", "Place students on waitlist when classes reach capacity")}</span>
        </div>
        <Switch bind:checked={allowWaitlist} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("أولوية الإخوة والأقارب", "Sibling Priority")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("منح أولوية القبول لإخوة الطلاب المسجلين", "Prioritize siblings of already enrolled students")}</span>
        </div>
        <Switch bind:checked={siblingPriorityEnabled} disabled={isSaving} />
      </div>

      <div class="flex items-center justify-between p-3.5 rounded-lg border border-border/50 bg-muted/20">
        <div class="flex flex-col text-start">
          <Label class="text-xs font-semibold">{l("السماح بالتسجيل المتأخر", "Allow Late Enrollment")}</Label>
          <span class="text-[11px] text-muted-foreground">{l("قبول طلبات القيد بعد بدء الفصل الدراسي", "Accept enrollment applications after period start")}</span>
        </div>
        <Switch bind:checked={allowLateEnrollment} disabled={isSaving} />
      </div>

      <div class="flex flex-col gap-2 p-3.5 rounded-lg border border-border/50 bg-muted/20 text-start">
        <Label class="text-xs font-semibold">{l("الحد الافتراضي لسعة الفصل", "Default Class Capacity")}</Label>
        <Input
          type="number"
          min="1"
          max="200"
          bind:value={maxClassSizeDefault}
          disabled={isSaving}
          class="h-8 text-xs w-32"
        />
      </div>
    </div>
  </CardContent>
</Card>
