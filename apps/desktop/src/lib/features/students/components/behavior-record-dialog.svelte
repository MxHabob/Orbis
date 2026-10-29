<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
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
import { Textarea } from "@orbis/ui/textarea";

let {
  open = $bindable(false),
  recordType = $bindable("positive"),
  points = $bindable(1),
  description = $bindable(""),
  isSaving = false,
  onsubmit,
}: {
  open?: boolean;
  recordType?: string;
  points?: number;
  description?: string;
  isSaving?: boolean;
  onsubmit: () => void;
} = $props();
</script>

<Dialog bind:open>
  <DialogContent class="max-w-md">
    <DialogHeader>
      <DialogTitle class="text-sm">{m.academics_behavior_add_record()}</DialogTitle>
    </DialogHeader>

    <div class="flex flex-col gap-4 py-2">
      <div class="flex flex-col gap-1.5">
        <Label for="new-record-type-select" class="text-xs">
          {m.academics_behavior_col_type()}
        </Label>
        <Select type="single" bind:value={recordType}>
          <SelectTrigger id="new-record-type-select" class="h-9 text-xs">
            {recordType === "positive"
              ? m.academics_behavior_type_positive()
              : m.academics_behavior_type_negative()}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="positive">{m.academics_behavior_type_positive()}</SelectItem>
            <SelectItem value="negative">{m.academics_behavior_type_negative()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="new-points-input" class="text-xs">
          {m.academics_behavior_points()}
        </Label>
        <Input
          id="new-points-input"
          type="number"
          min="0"
          bind:value={points}
          disabled={isSaving}
          class="text-xs"
        />
      </div>

      <div class="flex flex-col gap-1.5">
        <Label for="new-desc-input" class="text-xs">
          {m.academics_behavior_col_details()}
        </Label>
        <Textarea
          id="new-desc-input"
          dir="auto"
          class="text-xs"
          rows={4}
          placeholder={m.academics_behavior_description_placeholder()}
          bind:value={description}
          disabled={isSaving}
        />
      </div>
    </div>

    <DialogFooter>
      <Button
        variant="outline"
        class="text-xs"
        onclick={() => (open = false)}
        disabled={isSaving}
      >
        {m.common_action_cancel()}
      </Button>
      <Button
        class="text-xs"
        onclick={onsubmit}
        disabled={isSaving || !description.trim()}
      >
        {isSaving ? m.common_promo_running() : m.common_action_save()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
