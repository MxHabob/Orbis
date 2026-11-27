<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";
import { updateManager } from "$lib/features/system/update-manager.svelte";
import { authStore, tabsStore, themeStore, uiStore } from "$lib/stores";
import { localeState } from "$lib/core/i18n";
import { MULVERSE_URLS } from "@orbis/utils";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import { Logo } from "@orbis/icons";
import { AboutDialog, SupportDialog } from "$lib/features/system";
import type {
  Menu,
  MenuItemOptions,
  MenuOptions,
  SubmenuOptions,
} from "@tauri-apps/api/menu";
import { onDestroy } from "svelte";

type SubItem = MenuItemOptions | { item: "Separator" };

let {
  orgId = "",
  onOpenNewWindow,
  onCloseApp,
}: {
  orgId?: string;
  onOpenNewWindow: () => void;
  onCloseApp?: () => void;
} = $props();

let isSupportOpen = $state(false);
let isAboutOpen = $state(false);
let activeMenu: Menu | null = null;
let cachedMenuKey = "";

onDestroy(() => {
  void activeMenu?.close().catch(() => {});
  activeMenu = null;
  cachedMenuKey = "";
});

const SEP: SubItem = { item: "Separator" };

function go(path: string) {
  void goto(localizeHref(orgId ? `/${orgId}${path}` : "/"));
}

async function checkUpdates() {
  try {
    const res = await updateManager.checkForUpdates();
    if (!res.has_update) {
      toast.info(
        m.shared_ui_latest_version_installed({ version: res.current_version }),
      );
    }
  } catch (err) {
    toast.error(err instanceof Error ? err.message : String(err));
  }
}

function openWebsite() {
  void import("@tauri-apps/plugin-opener")
    .then(({ openUrl }) => openUrl(MULVERSE_URLS.website))
    .catch(() =>
      window.open(MULVERSE_URLS.website, "_blank", "noopener,noreferrer"),
    );
}

function buildMenuItems(): NonNullable<MenuOptions["items"]> {
  const authed = authStore.isAuthenticated;

  const fileItems: SubItem[] = [
    ...(authed
      ? [
          {
            text: m.titlebar_menu_new_tab(),
            accelerator: "CmdOrCtrl+T",
            action: () =>
              tabsStore.openNewTab(orgId, `/${orgId}`, m.common_nav_overview()),
          },
          {
            text: m.titlebar_menu_new_window(),
            accelerator: "CmdOrCtrl+Shift+N",
            action: onOpenNewWindow,
          },
          {
            text: m.titlebar_menu_reopen_tab(),
            accelerator: "CmdOrCtrl+Shift+T",
            action: () => tabsStore.reopenLastClosed(orgId),
          },
          SEP,
          {
            text: m.titlebar_menu_close_tab(),
            accelerator: "CmdOrCtrl+W",
            action: () => {
              if (tabsStore.activeTabId)
                tabsStore.closeTab(orgId, tabsStore.activeTabId);
            },
          },
          SEP,
        ]
      : []),
    {
      text: m.titlebar_menu_print(),
      accelerator: "CmdOrCtrl+P",
      action: () => window.print(),
    },
    {
      text: m.titlebar_menu_reload(),
      accelerator: "CmdOrCtrl+R",
      action: () => window.location.reload(),
    },
    SEP,
    {
      text: m.titlebar_menu_check_updates(),
      action: () => void checkUpdates(),
    },
    ...(onCloseApp
      ? [SEP, { text: m.titlebar_menu_exit(), action: onCloseApp }]
      : []),
  ];

  const viewItems: SubItem[] = [
    ...(authed
      ? [
          {
            text: m.common_header_search(),
            accelerator: "CmdOrCtrl+K",
            action: () => uiStore.setSearch(true),
          },
          {
            text: m.shared_ui_ask_orbis(),
            accelerator: "CmdOrCtrl+/",
            action: () => uiStore.toggleAskOrbis(),
          },
          {
            text: m.common_titlebar_focus_mode(),
            action: () => uiStore.toggleFocusMode(),
          },
          SEP,
        ]
      : []),
    {
      text: m.common_titlebar_theme_toggle(),
      action: () => themeStore.toggle(),
    },
    {
      text: m.titlebar_menu_reload(),
      accelerator: "CmdOrCtrl+R",
      action: () => window.location.reload(),
    },
  ];

  const historyItems: SubItem[] = [
    {
      text: m.titlebar_menu_back(),
      accelerator: "Alt+Left",
      action: () => window.history.back(),
    },
    {
      text: m.titlebar_menu_forward(),
      accelerator: "Alt+Right",
      action: () => window.history.forward(),
    },
    ...(authed
      ? [SEP, { text: m.common_nav_overview(), action: () => go("") }]
      : []),
  ];

  const submenus: SubmenuOptions[] = [
    { text: m.titlebar_menu_file(), items: fileItems },
    { text: m.titlebar_menu_view(), items: viewItems },
    { text: m.titlebar_menu_history(), items: historyItems },
  ];

  if (authed) {
    const goTargets: [string, string][] = [
      [m.common_word_students(), "/students"],
      [m.common_nav_academics(), "/academics"],
      [m.common_nav_finance(), "/finance"],
      [m.common_nav_reports(), "/reports"],
      [m.license_nav_title(), "/billing"],
    ];
    submenus.push({
      text: m.titlebar_menu_go_to(),
      items: [
        ...goTargets.map(([text, path]) => ({ text, action: () => go(path) })),
        SEP,
        { text: m.common_word_settings(), action: () => go("/settings") },
      ],
    });
  }

  return [
    ...submenus,
    { item: "Separator" },
    {
      text: m.titlebar_menu_help(),
      items: [
        {
          text: m.ui_system_support_support_diagnostic_assistance(),
          action: () => (isSupportOpen = true),
        },
        { text: m.mulverse_intro_visit_website(), action: openWebsite },
        SEP,
        { text: "Orbis", action: () => (isAboutOpen = true) },
      ],
    },
  ];
}

async function openNativeMenu(event: MouseEvent) {
  const rect = (
    event.currentTarget as HTMLElement | null
  )?.getBoundingClientRect();
  try {
    const [{ Menu }, { LogicalPosition }] = await Promise.all([
      import("@tauri-apps/api/menu"),
      import("@tauri-apps/api/dpi"),
    ]);

    const menuKey = `${localeState.revision}:${authStore.isAuthenticated}:${orgId}:${Boolean(onCloseApp)}`;
    if (!activeMenu || cachedMenuKey !== menuKey) {
      await activeMenu?.close().catch(() => {});
      activeMenu = await Menu.new({ items: buildMenuItems() });
      cachedMenuKey = menuKey;
    }

    await (rect
      ? activeMenu.popup(new LogicalPosition(rect.left, rect.bottom + 2))
      : activeMenu.popup());
  } catch (err) {
    console.warn("Failed to open native titlebar menu:", err);
  }
}
</script>

<Button
  variant="ghost"
  size="icon-sm"
  class="size-7 rounded-md text-foreground hover:bg-accent shrink-0"
  aria-label="Orbis Menu"
  title="Orbis"
  onclick={openNativeMenu}
>
  <Logo class="w-4.5 h-4.5" />
</Button>

<SupportDialog bind:open={isSupportOpen} />
<AboutDialog bind:open={isAboutOpen} />
