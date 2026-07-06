<script lang="ts">
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbSeparator,
  BreadcrumbPage,
} from "@orbis/ui-svelte/breadcrumb";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { authStore } from "$lib/stores";
import { useQueryClient } from "@tanstack/svelte-query";

const queryClient = useQueryClient();

// Breadcrumb items generator based on active route
const breadcrumbs = $derived.by(() => {
  const pathname = page.url.pathname;
  const currentOrgId = page.params.orgId || authStore.orgId || "";
  const studentId = page.params.studentId;

  const items: { label: string; href?: string }[] = [];

  // Base dashboard link
  items.push({ label: m.breadcrumb_home(), href: `/${currentOrgId}` });

  if (pathname.includes("/reports")) {
    items.push({
      label: m.breadcrumb_reports(),
      href: `/${currentOrgId}/reports`,
    });
  } else if (pathname.includes("/students")) {
    items.push({
      label: m.breadcrumb_students(),
      href: `/${currentOrgId}/students`,
    });

    if (pathname.includes("/students/new")) {
      items.push({ label: m.breadcrumb_new_student() });
    } else if (studentId) {
      const enrollments =
        queryClient.getQueryData<any[]>(["enrollments", currentOrgId]) || [];
      const student = enrollments.find((s) => s.id === studentId);
      const studentName =
        (student?.full_name as string) ||
        (student?.name_ar as string) ||
        (student?.name_en as string) ||
        m.breadcrumb_student_profile();
      items.push({
        label: studentName,
        href: `/${currentOrgId}/students/${studentId}`,
      });

      if (pathname.includes("/edit")) {
        items.push({ label: m.breadcrumb_edit_data() });
      } else if (pathname.includes("/grades")) {
        items.push({ label: m.breadcrumb_grades() });
      } else if (pathname.includes("/fees")) {
        items.push({ label: m.breadcrumb_fees() });
      } else if (pathname.includes("/attendance")) {
        items.push({ label: m.breadcrumb_attendance() });
      } else if (pathname.includes("/behavior")) {
        items.push({ label: m.breadcrumb_behavior() });
      }
    }
  } else if (pathname.includes("/academics")) {
    items.push({
      label: m.breadcrumb_academics(),
      href: `/${currentOrgId}/academics`,
    });

    if (pathname.includes("/classes")) {
      items.push({
        label: m.academics_layout_nav_classes(),
        href: `/${currentOrgId}/academics/classes`,
      });
    } else if (pathname.includes("/schedule")) {
      items.push({
        label: m.academics_layout_nav_schedule(),
        href: `/${currentOrgId}/academics/schedule`,
      });
    } else if (pathname.includes("/exams")) {
      items.push({
        label: m.nav_exams(),
        href: `/${currentOrgId}/academics/exams`,
      });
    } else if (pathname.includes("/control")) {
      items.push({
        label: m.academics_control_label
          ? m.academics_control_label()
          : "الكنترول",
        href: `/${currentOrgId}/academics/control`,
      });
    }
  } else if (pathname.includes("/finance")) {
    items.push({
      label: m.breadcrumb_finance(),
      href: `/${currentOrgId}/finance`,
    });

    if (pathname.includes("/fees")) {
      items.push({
        label: m.finance_layout_nav_fees(),
        href: `/${currentOrgId}/finance/fees`,
      });
    } else if (pathname.includes("/payments")) {
      items.push({
        label: m.finance_layout_nav_payments(),
        href: `/${currentOrgId}/finance/payments`,
      });
    } else if (pathname.includes("/treasuries")) {
      items.push({
        label: m.finance_layout_nav_treasuries(),
        href: `/${currentOrgId}/finance/treasuries`,
      });
    } else if (pathname.includes("/journal-entries")) {
      items.push({
        label: m.finance_layout_nav_journal
          ? m.finance_layout_nav_journal()
          : "القيود اليومية",
        href: `/${currentOrgId}/finance/journal-entries`,
      });
    } else if (
      pathname.includes("/ledger") ||
      pathname.includes("/statement")
    ) {
      items.push({
        label: m.finance_layout_nav_ledger
          ? m.finance_layout_nav_ledger()
          : "كشف الحساب",
        href: `/${currentOrgId}/finance/statement`,
      });
    } else if (pathname.includes("/reports")) {
      items.push({
        label: m.finance_layout_nav_reports(),
        href: `/${currentOrgId}/finance/reports`,
      });
    }
  } else if (pathname.includes("/settings")) {
    items.push({
      label: m.breadcrumb_settings(),
      href: `/${currentOrgId}/settings`,
    });

    if (pathname.includes("/notifications")) {
      items.push({ label: m.settings_nav_notifications() });
    } else if (pathname.includes("/security")) {
      items.push({ label: m.settings_nav_security() });
    } else if (pathname.includes("/users")) {
      items.push({ label: m.settings_nav_users() });
    } else if (pathname.includes("/permissions")) {
      items.push({ label: m.settings_nav_permissions() });
    } else if (pathname.includes("/cycles")) {
      items.push({ label: m.settings_nav_cycles() });
    } else if (pathname.includes("/units")) {
      items.push({ label: m.settings_nav_units() });
    } else if (pathname.includes("/rooms")) {
      items.push({
        label: m.settings_nav_rooms ? m.settings_nav_rooms() : "الغرف والقاعات",
      });
    } else if (pathname.includes("/subjects")) {
      items.push({
        label: m.settings_nav_subjects
          ? m.settings_nav_subjects()
          : "المواد الدراسية",
      });
    } else if (pathname.includes("/audit")) {
      items.push({ label: m.settings_nav_audit() });
    } else if (pathname.includes("/import-export")) {
      items.push({ label: m.settings_nav_import_export() });
    } else if (pathname.includes("/currencies")) {
      items.push({
        label: m.settings_nav_currencies
          ? m.settings_nav_currencies()
          : "العملات وأسعار الصرف",
      });
    } else if (pathname.includes("/ordering")) {
      items.push({
        label: m.settings_nav_ordering
          ? m.settings_nav_ordering()
          : "ترتيب الطلاب",
      });
    } else if (pathname.includes("/fee-templates")) {
      items.push({
        label: m.settings_nav_fee_templates
          ? m.settings_nav_fee_templates()
          : "قوالب الرسوم",
      });
    } else if (pathname.includes("/discount-templates")) {
      items.push({
        label: m.settings_nav_discount_templates
          ? m.settings_nav_discount_templates()
          : "قوالب الخصومات",
      });
    }
  } else if (pathname.includes("/templates")) {
    items.push({
      label: m.settings_nav_templates
        ? m.settings_nav_templates()
        : "قوالب المستندات",
      href: `/${currentOrgId}/templates`,
    });
  } else if (pathname.includes("/account")) {
    items.push({
      label: m.account_settings_title
        ? m.account_settings_title()
        : "إعدادات الحساب",
    });
  }

  return items;
});
</script>

<Breadcrumb class="min-w-0 overflow-hidden" data-tauri-drag-region>
  <BreadcrumbList class="flex items-center gap-1 text-[10px] font-semibold text-muted-foreground select-none flex-nowrap" data-tauri-drag-region>
    {#each breadcrumbs as item, index}
      <BreadcrumbItem class="min-w-0">
        {#if item.href && index < breadcrumbs.length - 1}
          <BreadcrumbLink href={localizeHref(item.href)} class="hover:text-foreground transition-colors duration-150 truncate max-w-[120px] block">
            {item.label}
          </BreadcrumbLink>
        {:else}
          <BreadcrumbPage class="text-foreground font-bold truncate max-w-[120px] block">
            {item.label}
          </BreadcrumbPage>
        {/if}
      </BreadcrumbItem>
      {#if index < breadcrumbs.length - 1}
        <BreadcrumbSeparator class="opacity-40 shrink-0 select-none">
          {getLocale() === "ar" ? "‹" : "›"}
        </BreadcrumbSeparator>
      {/if}
    {/each}
  </BreadcrumbList>
</Breadcrumb>
