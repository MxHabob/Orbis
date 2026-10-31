<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Switch } from "@orbis/ui/switch";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { authStore, uiStore } from "$lib/stores";

let {
  open = $bindable(false),
  activeTab = $bindable("schedule"),
  orgId,
  onAutoSchedule,
  onAllocateHalls,
}: {
  open: boolean;
  activeTab: string;
  orgId: string;
  onAutoSchedule: () => void;
  onAllocateHalls: () => void;
} = $props();

const preferenceScope = () => ({
  userId: authStore.userId,
  organizationId: orgId,
});
let preferredPeriod = $state<"both" | "morning" | "afternoon">(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.preferredPeriod",
    "both",
    (value): value is "both" | "morning" | "afternoon" =>
      value === "both" || value === "morning" || value === "afternoon",
  ),
);
let maxExamsPerDay = $state<1 | 2>(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.maxExamsPerDay",
    2,
    (value): value is 1 | 2 => value === 1 || value === 2,
  ),
);
let defaultDurationMinutes = $state(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.defaultDurationMinutes",
    90,
    (value): value is number =>
      typeof value === "number" &&
      Number.isInteger(value) &&
      value >= 30 &&
      value <= 240,
  ),
);
let separateDivisions = $state(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.separateDivisions",
    false,
    (value): value is boolean => typeof value === "boolean",
  ),
);
let separateGenders = $state(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.separateGenders",
    false,
    (value): value is boolean => typeof value === "boolean",
  ),
);
let balanceHalls = $state(
  uiStore.getPreference(
    preferenceScope(),
    "exams.ai.balanceHalls",
    true,
    (value): value is boolean => typeof value === "boolean",
  ),
);

function run(action: () => void) {
  uiStore.setPreference(
    preferenceScope(),
    "exams.ai.preferredPeriod",
    preferredPeriod,
  );
  uiStore.setPreference(
    preferenceScope(),
    "exams.ai.maxExamsPerDay",
    maxExamsPerDay,
  );
  if (
    Number.isInteger(defaultDurationMinutes) &&
    defaultDurationMinutes >= 30 &&
    defaultDurationMinutes <= 240
  ) {
    uiStore.setPreference(
      preferenceScope(),
      "exams.ai.defaultDurationMinutes",
      defaultDurationMinutes,
    );
  }
  uiStore.setPreference(
    preferenceScope(),
    "exams.ai.separateDivisions",
    separateDivisions,
  );
  uiStore.setPreference(
    preferenceScope(),
    "exams.ai.separateGenders",
    separateGenders,
  );
  uiStore.setPreference(
    preferenceScope(),
    "exams.ai.balanceHalls",
    balanceHalls,
  );
  open = false;
  action();
}
</script>

<Dialog bind:open>
  <DialogContent class="max-h-[90vh] max-w-lg overflow-hidden p-0 text-start">
    <DialogHeader class="border-b border-border/60 px-5 py-4">
      <DialogTitle>{m.academics_exams_ai_wizard()}</DialogTitle>
    </DialogHeader>

    <nav class="flex gap-1 border-b border-border/60 p-2" aria-label={m.academics_exams_ai_wizard()}>
      <button
        type="button"
        aria-pressed={activeTab === "schedule"}
        class="flex-1 rounded-md px-2 py-2 text-xs transition-colors {activeTab === 'schedule' ? 'bg-muted text-foreground font-medium' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => activeTab = "schedule"}
      >
        {m.operations_ui_schedule_periods()}
      </button>
      <button
        type="button"
        aria-pressed={activeTab === "halls"}
        class="flex-1 rounded-md px-2 py-2 text-xs transition-colors {activeTab === 'halls' ? 'bg-muted text-foreground font-medium' : 'text-muted-foreground hover:text-foreground'}"
        onclick={() => activeTab = "halls"}
      >
        {m.operations_ui_distribute_committees_students()}
      </button>
    </nav>

    <div class="max-h-[calc(90vh-7rem)] overflow-y-auto p-5">
      {#if activeTab === "schedule"}
        <div class="flex flex-col gap-4">
          <label class="flex flex-col gap-1.5 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_scheduling_preference()}</span>
            <Select
              type="single"
              value={preferredPeriod}
              onValueChange={(value) =>
                (preferredPeriod =
                  value === "morning" || value === "afternoon" ? value : "both")}
            >
              <SelectTrigger class="h-9 w-full text-xs">
                {preferredPeriod === "morning"
                  ? m.academics_exams_morning_session()
                  : preferredPeriod === "afternoon"
                    ? m.academics_exams_afternoon_session()
                    : m.academics_exams_two_sessions()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="both">{m.academics_exams_two_sessions()}</SelectItem>
                <SelectItem value="morning">{m.academics_exams_morning_session()}</SelectItem>
                <SelectItem value="afternoon">{m.academics_exams_afternoon_session()}</SelectItem>
              </SelectContent>
            </Select>
          </label>

          <label class="flex flex-col gap-1.5 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_max_exams_per_day()}</span>
            <Select
              type="single"
              value={String(maxExamsPerDay)}
              onValueChange={(value) => maxExamsPerDay = value === "1" ? 1 : 2}
            >
              <SelectTrigger class="h-9 w-full text-xs">
                {maxExamsPerDay === 1 ? m.academics_exams_one_exam_per_day() : m.academics_exams_two_exams_per_day()}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="1">{m.academics_exams_one_exam_per_day()}</SelectItem>
                <SelectItem value="2">{m.academics_exams_two_exams_per_day()}</SelectItem>
              </SelectContent>
            </Select>
          </label>

          <label class="flex flex-col gap-1.5 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_default_duration()}</span>
            <Input type="number" bind:value={defaultDurationMinutes} min="30" max="240" class="h-9 font-mono text-xs" />
          </label>
        </div>
        <DialogFooter class="mt-5 border-t border-border pt-4">
          <Button variant="outline" size="sm" onclick={() => open = false}>{m.common_action_cancel()}</Button>
          <Button variant="default" size="sm" class="font-semibold" onclick={() => run(onAutoSchedule)}>
            {m.operations_ui_start_smart_scheduling()}
          </Button>
        </DialogFooter>
      {:else if activeTab === "halls"}
        <div class="flex flex-col gap-3">
          <label class="flex items-center justify-between gap-4 rounded-lg border border-border/60 px-3 py-3 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_allocate_different_tracks()}</span>
            <Switch bind:checked={separateDivisions} />
          </label>
          <label class="flex items-center justify-between gap-4 rounded-lg border border-border/60 px-3 py-3 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_allocate_genders_separate()}</span>
            <Switch bind:checked={separateGenders} />
          </label>
          <label class="flex items-center justify-between gap-4 rounded-lg border border-border/60 px-3 py-3 text-xs">
            <span class="text-muted-foreground">{m.academics_exams_balance_allocate_evenly()}</span>
            <Switch bind:checked={balanceHalls} />
          </label>
        </div>
        <DialogFooter class="mt-5 border-t border-border pt-4">
          <Button variant="outline" size="sm" onclick={() => open = false}>{m.common_action_cancel()}</Button>
          <Button variant="default" size="sm" class="font-semibold" onclick={() => run(onAllocateHalls)}>
            {m.operations_ui_start_automatic_distribution()}
          </Button>
        </DialogFooter>
      {/if}
    </div>
  </DialogContent>
</Dialog>
