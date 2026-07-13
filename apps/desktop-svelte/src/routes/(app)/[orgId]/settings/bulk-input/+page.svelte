<!--
@component BulkImportHub
Hub page linking to specific bulk import sub-pages (students, grades).
Premium card-based layout matching the Orbis design system.
-->
<script lang="ts">
import { goto } from "$app/navigation";
import { Button } from "@orbis/ui-svelte/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@orbis/ui-svelte/card";
import { Badge } from "@orbis/ui-svelte/badge";
import { ArrowLeft, Users, GraduationCap, ArrowRight } from "lucide-svelte";
import * as common from "$lib/paraglide/messages/common";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

let { data } = $props();
const isRtl = getLocale() === "ar";
</script>

<div
  class="h-full w-full overflow-y-auto scrollbar-hide"
  dir={isRtl ? "rtl" : "ltr"}
>
  <div class="max-w-7xl mx-auto p-6 flex flex-col gap-6">
    <div class="flex items-center gap-3 pb-4">
      <Button
        variant="secondary"
        size="icon-sm"
        class="cursor-pointer"
        onclick={() => goto(localizeHref(`/${data.orgId}/students`))}
      >
        <ArrowLeft class="h-4 w-4 rtl:rotate-180" />
      </Button>
      <div class="flex flex-col gap-0.5">
        <h1 class="text-xl font-bold tracking-tight text-foreground">
          {common.bulk_hub_title()}
        </h1>
      </div>
    </div>

    <!-- Cards Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <button
        class="text-start w-full"
        onclick={() =>
          goto(localizeHref(`/${data.orgId}/settings/bulk-input/students`))}
      >
        <Card class="h-full cursor-pointer p-4">
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <div class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center">
                <Users size={20} class="text-primary" />
              </div>
              <ArrowRight
                size={16}
                class="text-muted-foreground group-hover:text-primary transition-colors rtl:rotate-180"
              />
            </div>
            <CardTitle class="text-sm font-semibold mt-3">
              {common.bulk_card_students_title()}
            </CardTitle>
            <CardDescription class="text-xs">
              {common.bulk_card_students_desc()}
            </CardDescription>
          </CardHeader>
          <CardContent class="pt-0">
            <div class="flex items-center gap-2 flex-wrap">
              <Badge variant="secondary" class="text-[10px]">
                {common.bulk_card_students_tag_personal()}
              </Badge>
              <Badge variant="secondary" class="text-[10px]">
                {common.bulk_card_students_tag_enrollment()}
              </Badge>
              <Badge variant="secondary" class="text-[10px]">
                {common.bulk_card_students_tag_guardian()}
              </Badge>
            </div>
          </CardContent>
        </Card>
      </button>

      <button
        class="text-start w-full"
        onclick={() =>
          goto(localizeHref(`/${data.orgId}/settings/bulk-input/grades`))}
      >
        <Card class="h-full cursor-pointer p-4">
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <div class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center">
                <GraduationCap size={20} class="text-primary" />
              </div>
              <ArrowRight
                size={16}
                class="text-muted-foreground group-hover:text-primary transition-colors rtl:rotate-180"
              />
            </div>
            <CardTitle class="text-sm font-semibold mt-3">
              {common.bulk_card_grades_title()}
            </CardTitle>
            <CardDescription class="text-xs">
              {common.bulk_card_grades_desc()}
            </CardDescription>
          </CardHeader>
          <CardContent class="pt-0">
            <div class="flex items-center gap-2 flex-wrap">
              <Badge variant="secondary" class="text-[10px]">
                {common.bulk_card_grades_tag_smart()}
              </Badge>
              <Badge variant="secondary" class="text-[10px]">
                {common.bulk_card_grades_tag_period()}
              </Badge>
            </div>
          </CardContent>
        </Card>
      </button>
    </div>
  </div>
</div>
