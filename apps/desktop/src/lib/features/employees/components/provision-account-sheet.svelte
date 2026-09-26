<!--
  @component ProvisionAccountSheet
  Standalone slide-over Sheet component for provisioning employee user accounts,
  following canonical Orbis Sheet styling.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@orbis/ui/sheet";
import { toast } from "@orbis/ui";
import { ArrowPath, ShieldCheck } from "@orbis/icons";
import { extractErrorMessage } from "$lib/api";
import type {
  AccountProvisioningResultDto,
  ProvisionEmployeeAccountInput,
} from "$lib/api/types";
import { createProvisionEmployeeAccountMutation } from "../queries";
import { GroupSelector } from "$lib/features/groups";

interface Props {
  open?: boolean;
  orgId: string;
  employeeId: string;
  employeeName?: string;
  onprovisioned?: (result: AccountProvisioningResultDto) => void;
}

let {
  open = $bindable(false),
  orgId,
  employeeId,
  employeeName = "",
  onprovisioned,
}: Props = $props();

const provisionMutation = createProvisionEmployeeAccountMutation();

let preferredUsername = $state("");
let selectedGroupIds = $state<string[]>([]);

$effect(() => {
  if (open) {
    preferredUsername = "";
    selectedGroupIds = [];
  }
});

async function handleSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!employeeId) return;

  try {
    const payload: ProvisionEmployeeAccountInput = {
      preferred_username: preferredUsername.trim() || undefined,
      delivery_channel: "auto",
      group_ids: selectedGroupIds,
    };

    const result = await provisionMutation.mutateAsync({
      id: employeeId,
      req: payload,
    });

    toast.success(m.employees_account_provisioned?.() || "تم إنشاء حساب المستخدم بنجاح");
    open = false;
    onprovisioned?.(result);
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}
</script>

<Sheet bind:open>
  <SheetContent class="flex flex-col h-full sm:max-w-md md:max-w-lg overflow-hidden p-0 gap-0">
    <form onsubmit={handleSubmit} class="flex flex-col h-full overflow-hidden gap-0">
      <!-- Sheet Header -->
      <SheetHeader class="text-start select-none px-6 py-4 border-b border-border/40 shrink-0">
        <div class="flex items-center gap-2">
          <ShieldCheck size={20} class="text-primary" />
          <SheetTitle class="text-base font-bold text-foreground">
            {m.employees_provision_account?.() || "تهيئة حساب مستخدم جديد"}
          </SheetTitle>
        </div>
        <SheetDescription class="text-xs text-muted-foreground pt-1">
          {employeeName
            ? l(`إنشاء بيانات اعتماد وربط الحساب بالموظف (${employeeName})`, `Provision user credentials for ${employeeName}`)
            : l("إنشاء بيانات اعتماد آمنة وربط الحساب بالموظف.", "Provision secure user credentials.")}
        </SheetDescription>
      </SheetHeader>

      <!-- Sheet Body -->
      <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide text-start">
        <div class="flex flex-col gap-1.5">
          <Label class="text-xs font-medium text-foreground">
            {l("اسم المستخدم المفضل (اختياري)", "Preferred Username (Optional)")}
          </Label>
          <Input
            type="text"
            placeholder="e.g. employee.username"
            class="text-xs font-mono"
            bind:value={preferredUsername}
          />
          <span class="text-[10px] text-muted-foreground leading-relaxed">
            {l("في حال تركه فارغاً، يتم توليد اسم مستخدم فريد تلقائياً من البريد أو الاسم والرقم الوظيفي.", "Auto-generated if left empty.")}
          </span>
        </div>

        <div class="flex flex-col gap-1.5 p-3.5 rounded-xl border border-border/60 bg-muted/20 text-xs text-muted-foreground">
          <span class="font-bold text-foreground text-xs">{l("التسليم والتكاملات", "Delivery & Integrations")}</span>
          <span class="text-[11px] leading-relaxed">
            {l("يتم تسليم بيانات الدخول تلقائياً عبر قنوات التكامل المفعلة في المؤسسة (البريد/الرسائل/واتساب) أو عرضها فوراً لمرة واحدة.", "Credentials are automatically sent via configured integrations or displayed on-screen.")}
          </span>
        </div>

        <div class="flex flex-col gap-2 pt-2 border-t border-border/50">
          <Label class="text-xs font-bold text-foreground">
            {m.employees_user_groups?.() || "مجموعات المستخدمين المبدئية"}
          </Label>
          <GroupSelector
            {orgId}
            bind:selectedGroupIds
            maxHeight="max-h-56"
          />
        </div>
      </div>

      <!-- Sheet Footer -->
      <div class="px-6 py-3 border-t border-border/40 flex gap-2 justify-end shrink-0 bg-background/50">
        <Button
          type="button"
          variant="ghost"
          onclick={() => (open = false)}
          disabled={provisionMutation.isPending}
          class="text-xs h-8 cursor-pointer"
        >
          {m.common_action_cancel?.() || "إلغاء"}
        </Button>
        <Button
          type="submit"
          variant="lime"
          class="font-semibold px-6 text-xs h-8 cursor-pointer"
          disabled={provisionMutation.isPending}
        >
          {#if provisionMutation.isPending}
            <ArrowPath size={13} class="animate-spin me-1.5" />
          {/if}
          {l("إنشاء الحساب وتوليد كلمة المرور", "Create Account")}
        </Button>
      </div>
    </form>
  </SheetContent>
</Sheet>
