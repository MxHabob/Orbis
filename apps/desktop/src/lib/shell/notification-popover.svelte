<script lang="ts">
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";
import { notificationStore } from "$lib/stores";
import { toast, LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { Popover, PopoverTrigger, PopoverContent } from "@orbis/ui/popover";
import {
  AcademicCap,
  ArchiveBox,
  AppNotification,
  Bell,
  BookOpen,
  Calendar,
  CheckCircle,
  Cog6,
  CreditCard,
  Settings,
  SoundHigh,
  UserPlus,
} from "@orbis/icons";
import type { NotificationDto } from "$lib/api/types";
import EmptyState from "$lib/shared/components/empty-state.svelte";

let { orgId = "" }: { orgId?: string } = $props();

const isArabic = $derived(getLocale().startsWith("ar"));

let open = $state(false);
type NotificationView = "inbox" | "archive" | "comments";
let activeView = $state<NotificationView>("inbox");

onMount(() => {
  void notificationStore.init();
});

function handleOpenChange(isOpen: boolean) {
  open = isOpen;
  if (isOpen) {
    loadActiveView();
    void notificationStore.loadUnreadCount();
  }
}

function loadActiveView() {
  if (activeView === "archive") {
    void notificationStore.loadNotifications({ is_dismissed: true });
  } else if (activeView === "comments") {
    void notificationStore.loadNotifications({ category: "comment" });
  } else {
    void notificationStore.loadNotifications({ is_dismissed: false });
  }
}

function formatTime(iso: string): string {
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return "";

  const seconds = Math.round((date.getTime() - Date.now()) / 1000);
  const absoluteSeconds = Math.abs(seconds);
  const relativeTime = new Intl.RelativeTimeFormat(getLocale(), {
    numeric: "auto",
    style: "narrow",
  });

  if (absoluteSeconds < 60) return relativeTime.format(seconds, "second");
  if (absoluteSeconds < 60 * 60)
    return relativeTime.format(Math.round(seconds / 60), "minute");
  if (absoluteSeconds < 24 * 60 * 60)
    return relativeTime.format(Math.round(seconds / (60 * 60)), "hour");
  if (absoluteSeconds < 30 * 24 * 60 * 60)
    return relativeTime.format(Math.round(seconds / (24 * 60 * 60)), "day");

  return new Intl.DateTimeFormat(getLocale(), { dateStyle: "medium" }).format(
    date,
  );
}

function getCategoryIcon(category: string) {
  switch (category) {
    case "enrollment":
      return UserPlus;
    case "grades":
      return AcademicCap;
    case "attendance":
      return Calendar;
    case "finance":
      return CreditCard;
    case "schedule":
      return BookOpen;
    case "approval":
      return CheckCircle;
    case "announcement":
      return SoundHigh;
    case "system":
      return Settings;
    default:
      return Bell;
  }
}

function getNotificationHref(item: NotificationDto): string | null {
  if (!orgId) return null;

  if (item.action_route) {
    const route = item.action_route.trim();
    if (!route.startsWith("/") || route.startsWith("//")) return null;

    try {
      const parsed = new URL(route, "https://orbis.invalid");
      if (parsed.origin !== "https://orbis.invalid") return null;

      const appPath = parsed.pathname.replace(/^\/\(app\)(?=\/)/, "");
      const legacyStudentRoute = appPath.match(/^\/student\/([^/]+)\/?$/);
      if (legacyStudentRoute) {
        return localizeHref(
          `/${encodeURIComponent(orgId)}/students/${encodeURIComponent(legacyStudentRoute[1])}${parsed.search}${parsed.hash}`,
        );
      }

      const orgPath = `/${encodeURIComponent(orgId)}`;
      if (appPath === orgPath || appPath.startsWith(`${orgPath}/`)) {
        return localizeHref(`${appPath}${parsed.search}${parsed.hash}`);
      }
    } catch (error) {
      console.warn("Unable to resolve notification action route:", error);
    }
  }

  if (item.entity_type === "student" && item.entity_id) {
    return localizeHref(
      `/${encodeURIComponent(orgId)}/students/${encodeURIComponent(item.entity_id)}`,
    );
  }

  return null;
}

async function navigateTo(href: string) {
  open = false;
  try {
    await goto(href);
  } catch (error) {
    console.error("Failed to navigate from a notification:", error);
    toast.error(m.notifications_popover_navigation_failed());
  }
}

async function handleItemClick(item: NotificationDto, href: string | null) {
  if (!item.is_read) void notificationStore.markAsRead(item.id);

  if (href) await navigateTo(href);
}

function handleViewSelect(view: NotificationView) {
  activeView = view;
  loadActiveView();
}

function handleRetry() {
  loadActiveView();
  void notificationStore.loadUnreadCount();
}

function openNotificationSettings() {
  if (!orgId) return;
  void navigateTo(
    localizeHref(`/${encodeURIComponent(orgId)}/settings/notifications`),
  );
}
</script>

<Popover {open} onOpenChange={handleOpenChange}>
  <PopoverTrigger>
    {#snippet child({ props }: { props: any })}
      <Button
        {...props}
        variant="outline"
        size="icon-sm"
        class="relative cursor-pointer"
        title={m.common_nav_notifications()}
        aria-label={m.common_nav_notifications()}
      >
        <AppNotification size={16} class="text-foreground" />
        {#if notificationStore.hasUnread}
          <span
            class="absolute -top-1 -inset-e-1 flex min-w-4 items-center justify-center rounded-full border border-background bg-destructive px-1 text-[10px] font-bold text-destructive-foreground shadow-xs"
          >
            {notificationStore.unreadCount > 99
              ? "99+"
              : notificationStore.unreadCount}
          </span>
        {/if}
      </Button>
    {/snippet}
  </PopoverTrigger>

  <PopoverContent
    side="bottom"
    align="end"
    sideOffset={6}
    class="flex max-h-[min(40rem,calc(100vh-4rem))] w-[min(440px,calc(100vw-1rem))] flex-col gap-0 overflow-hidden rounded-md border border-border bg-card p-0 text-start font-sans shadow-md"
  >
    <nav
      aria-label={m.common_nav_notifications()}
      class="flex h-11 shrink-0 items-stretch border-b border-border px-2"
    >
      <button
        type="button"
        aria-pressed={activeView === "inbox"}
        class="flex items-center gap-1.5 border-b-2 px-3 text-xs transition-colors {activeView ===
        'inbox'
          ? 'border-foreground text-foreground'
          : 'border-transparent text-muted-foreground hover:text-foreground'}"
        onclick={() => handleViewSelect("inbox")}
      >
        {m.notifications_popover_tab_inbox()}
        {#if notificationStore.unreadCount > 0}
          <span
            class="rounded-full bg-muted px-1.5 py-0.5 text-[10px] leading-none text-muted-foreground"
          >
            {notificationStore.unreadCount > 99
              ? "99+"
              : notificationStore.unreadCount}
          </span>
        {/if}
      </button>
      <button
        type="button"
        aria-pressed={activeView === "archive"}
        class="border-b-2 px-3 text-xs transition-colors {activeView === "archive"
          ? 'border-foreground text-foreground'
          : 'border-transparent text-muted-foreground hover:text-foreground'}"
        onclick={() => handleViewSelect("archive")}
      >
        {m.settings_nav_archive()}
      </button>
      <button
        type="button"
        aria-pressed={activeView === "comments"}
        class="flex items-center gap-1.5 border-b-2 px-3 text-xs transition-colors {activeView ===
        'comments'
          ? 'border-foreground text-foreground'
          : 'border-transparent text-muted-foreground hover:text-foreground'}"
        onclick={() => handleViewSelect("comments")}
      >
        {m.notifications_popover_tab_comments()}
      </button>
      <Button
        variant="ghost"
        size="icon-sm"
        class="ms-auto my-auto size-8 shrink-0 text-muted-foreground"
        aria-label={m.notifications_popover_settings()}
        title={m.notifications_popover_settings()}
        disabled={!orgId}
        onclick={openNotificationSettings}
      >
        <Cog6 size={16} />
      </Button>
    </nav>

    <div class="min-h-0 flex-1 overflow-y-auto">
      {#if notificationStore.isLoading}
        <div
          class="flex min-h-40 flex-col items-center justify-center gap-2.5 text-muted-foreground"
          role="status"
          aria-live="polite"
        >
          <LoadingIndicator size={18} />
          <span class="text-xs">{m.common_loading()}</span>
        </div>
      {:else if notificationStore.loadError}
        <EmptyState
          icon={Bell}
          title={m.notifications_popover_load_error_title()}
          description={m.notifications_popover_load_error_description()}
          actionLabel={m.common_retry()}
          onaction={handleRetry}
          class="py-6"
        />
      {:else if notificationStore.items.length === 0}
        <EmptyState
          icon={Bell}
          title={m.notifications_popover_empty_title()}
          description={m.notifications_popover_empty_description()}
          class="py-6"
        />
      {:else}
        <ul class="divide-y divide-border/60">
          {#each notificationStore.items as item (item.id)}
            {@const Icon = getCategoryIcon(item.category)}
            {@const title =
              isArabic && item.title_ar ? item.title_ar : item.title}
            {@const body = isArabic && item.body_ar ? item.body_ar : item.body}
            {@const actionHref = getNotificationHref(item)}
            <li
              class="group flex items-start gap-3 px-4 py-3 transition-colors hover:bg-muted/40"
            >
              <span
                class="mt-0.5 flex size-7 shrink-0 items-center justify-center rounded-full border {item.priority ===
                'high' ||
                item.priority ===
                  'critical'
                  ? 'border-destructive/30 bg-destructive/10 text-destructive'
                  : 'border-primary/30 bg-primary/10 text-primary'}"
              >
                <Icon size={15} />
              </span>

              <button
                type="button"
                class="min-w-0 flex-1 text-start"
                onclick={() => void handleItemClick(item, actionHref)}
              >
                <span class="block text-xs leading-relaxed text-foreground {item.is_read
                  ? 'font-medium'
                  : 'font-semibold'}">
                  {title}
                  {#if body}
                    <span class="font-normal text-muted-foreground">{body}</span>
                  {/if}
                </span>
              </button>

              {#if !item.is_read}
                <span
                  class="mt-1.5 size-2 shrink-0 rounded-full bg-primary"
                  aria-label={m.notifications_popover_unread()}
                ></span>
              {/if}
              <time
                datetime={item.created_at}
                class="shrink-0 pt-0.5 text-[11px] text-muted-foreground"
              >
                {formatTime(item.created_at)}
              </time>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    {#if activeView === "inbox" && notificationStore.items.length > 0}
      <footer
        class="flex shrink-0 justify-center border-t border-border px-3 py-2"
      >
        <Button
          variant="ghost"
          size="sm"
          class="gap-2 text-xs text-muted-foreground hover:text-foreground"
          onclick={() => void notificationStore.archiveAll()}
        >
          <ArchiveBox size={14} />
          <span>{m.notifications_popover_archive_all()}</span>
        </Button>
      </footer>
    {/if}
  </PopoverContent>
</Popover>
