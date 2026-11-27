import { goto } from "$app/navigation";
import { localizeHref } from "$lib/paraglide/runtime";

export interface OpenTab {
  id: string;
  path: string;
  title: string;
}

const MAX_TABS = 10;
const STORAGE_KEY = "orbis.shell.tabs.v1";

function createId(): string {
  return typeof crypto !== "undefined" && "randomUUID" in crypto
    ? crypto.randomUUID()
    : `tab_${Date.now()}_${Math.random().toString(36).slice(2, 7)}`;
}

class TabsStore {
  tabs = $state<OpenTab[]>([]);
  activeTabId = $state<string>("");
  private closedStack: OpenTab[] = [];
  private openNewOnNextSync = false;
  private pendingNavPath: string | null = null;
  private hydratedOrgId = "";

  private persist(orgId: string) {
    if (typeof sessionStorage === "undefined" || !orgId) return;
    try {
      sessionStorage.setItem(
        `${STORAGE_KEY}:${orgId}`,
        JSON.stringify({ tabs: this.tabs, activeTabId: this.activeTabId }),
      );
    } catch {
      // Ignore storage quota errors
    }
  }

  private hydrate(orgId: string) {
    if (
      typeof sessionStorage === "undefined" ||
      !orgId ||
      this.hydratedOrgId === orgId
    ) {
      return;
    }
    this.hydratedOrgId = orgId;
    try {
      const raw = sessionStorage.getItem(`${STORAGE_KEY}:${orgId}`);
      if (!raw) return;
      const parsed = JSON.parse(raw) as {
        tabs?: OpenTab[];
        activeTabId?: string;
      };
      if (Array.isArray(parsed.tabs) && parsed.tabs.length > 0) {
        this.tabs = parsed.tabs.slice(0, MAX_TABS);
        this.activeTabId =
          parsed.activeTabId &&
          this.tabs.some((t) => t.id === parsed.activeTabId)
            ? parsed.activeTabId
            : this.tabs[0].id;
      }
    } catch {
      // Ignore malformed state
    }
  }

  markNextNavigationAsNewTab() {
    this.openNewOnNextSync = true;
  }

  syncRoute(orgId: string, path: string, title: string) {
    if (!orgId) return;
    this.hydrate(orgId);

    const normalizedPath = path || `/${orgId}`;
    const safeTitle = title || "Orbis";

    if (this.pendingNavPath !== null) {
      if (normalizedPath !== this.pendingNavPath) {
        return;
      }
      this.pendingNavPath = null;
    }

    if (this.tabs.length === 0 || this.openNewOnNextSync) {
      this.openNewOnNextSync = false;
      const existing = this.tabs.find((t) => t.path === normalizedPath);
      if (existing) {
        existing.title = safeTitle;
        this.activeTabId = existing.id;
        this.persist(orgId);
        return;
      }
      const tab: OpenTab = {
        id: createId(),
        path: normalizedPath,
        title: safeTitle,
      };
      const next = [...this.tabs, tab];
      this.tabs =
        next.length > MAX_TABS ? next.slice(next.length - MAX_TABS) : next;
      this.activeTabId = tab.id;
      this.persist(orgId);
      return;
    }

    const activeIndex = this.tabs.findIndex((t) => t.id === this.activeTabId);
    if (activeIndex === -1) {
      const first = this.tabs[0];
      first.path = normalizedPath;
      first.title = safeTitle;
      this.activeTabId = first.id;
      this.persist(orgId);
      return;
    }

    const current = this.tabs[activeIndex];
    if (current.path !== normalizedPath || current.title !== safeTitle) {
      this.tabs[activeIndex] = {
        ...current,
        path: normalizedPath,
        title: safeTitle,
      };
      this.persist(orgId);
    }
  }

  openNewTab(orgId: string, path?: string, title = "Orbis", dedupe = false) {
    if (!orgId) return;
    const targetPath = path || `/${orgId}`;
    if (dedupe) {
      const existing = this.tabs.find((t) => t.path === targetPath);
      if (existing) {
        this.activateTab(orgId, existing.id);
        return;
      }
    }

    const tab: OpenTab = { id: createId(), path: targetPath, title };
    const next = [...this.tabs, tab];
    this.tabs =
      next.length > MAX_TABS ? next.slice(next.length - MAX_TABS) : next;
    this.activeTabId = tab.id;
    this.pendingNavPath = targetPath;
    this.persist(orgId);
    void goto(localizeHref(targetPath));
  }

  activateTab(orgId: string, id: string) {
    const tab = this.tabs.find((t) => t.id === id);
    if (!tab) return;
    this.activeTabId = tab.id;
    this.pendingNavPath = tab.path;
    this.persist(orgId);
    void goto(localizeHref(tab.path));
  }

  closeTab(orgId: string, id: string) {
    const idx = this.tabs.findIndex((t) => t.id === id);
    if (idx === -1) return;

    const removed = this.tabs[idx];
    this.closedStack.push(removed);
    if (this.closedStack.length > MAX_TABS) {
      this.closedStack.shift();
    }

    if (this.tabs.length === 1) {
      const homePath = `/${orgId}`;
      if (removed.path === homePath) return;
      this.tabs = [{ id: createId(), path: homePath, title: removed.title }];
      this.activeTabId = this.tabs[0].id;
      this.pendingNavPath = homePath;
      this.persist(orgId);
      void goto(localizeHref(homePath));
      return;
    }

    const remaining = this.tabs.filter((t) => t.id !== id);
    this.tabs = remaining;

    if (this.activeTabId === id) {
      const fallback = remaining[Math.min(idx, remaining.length - 1)];
      this.activeTabId = fallback.id;
      this.pendingNavPath = fallback.path;
      this.persist(orgId);
      void goto(localizeHref(fallback.path));
    } else {
      this.persist(orgId);
    }
  }

  reopenLastClosed(orgId: string) {
    const last = this.closedStack.pop();
    if (!last || !orgId) return;
    this.openNewTab(orgId, last.path, last.title);
  }
}

export const tabsStore = new TabsStore();
