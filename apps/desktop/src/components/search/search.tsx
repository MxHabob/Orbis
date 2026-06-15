"use client";

import { emit, invoke, listen, Window } from "@orbis/desktop-client/core";
import { isDesktopApp } from "@orbis/desktop-client/platform";
import { Badge } from "@orbis/ui/badge";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@orbis/ui/command";
import { Icons } from "@orbis/ui/icons";
import { Spinner } from "@orbis/ui/spinner";
import { formatDate } from "@orbis/utils/format";
import { useQuery } from "@tanstack/react-query";
import { formatISO } from "date-fns";
import { useNavigate } from "react-router-dom";
import type { ReactNode } from "react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useCopyToClipboard, useDebounceValue } from "usehooks-ts";
import { FormatAmount } from "@/components/format-amount";
import { useClassParams } from "@/hooks/use-class-params";
import { useDocumentParams } from "@/hooks/use-document-params";
import { useFileUrl } from "@/hooks/use-file-url";
import { useReportExportParams } from "@/hooks/use-report-export-params";
import { useStudentParams } from "@/hooks/use-student-params";
import { useSubjectParams } from "@/hooks/use-subject-params";
import { useTrackerParams } from "@/hooks/use-tracker-params";
import { useUserQuery } from "@/hooks/use-user";
import { downloadFile } from "@/lib/download";
import { useI18n } from "@/locales/client";
import { useSearchStore } from "@/store/search";
import { useTRPC } from "@/ipc/query-adapter";
import { FilePreviewIcon } from "../file-preview-icon";
import { TrackerTimer } from "../tracker-timer";

interface SearchItem {
  id: string;
  type:
    | "student"
    | "class"
    | "staff"
    | "subject"
    | "vault"
    | "tracker_project"
    | "inbox"
    | "shortcut"
    | string;
  title: string;
  data?: {
    name?: string;
    email?: string;
    student_number?: string;
    status?: string;
    date?: string;
    display_name?: string;
    file_name?: string;
    file_path?: string[];
    path_tokens?: string[];
    title?: string;
    metadata?: {
      mimetype?: string;
    };
    url?: string;
  };
  action?: () => void;
}

function CopyButton({ path }: { path: string }) {
  const [isCopied, setIsCopied] = useState(false);
  const [_, copy] = useCopyToClipboard();

  const handleCopy = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();

    copy(`${window.location.origin}${path}`);
    setIsCopied(true);
    setTimeout(() => {
      setIsCopied(false);
    }, 1000);
  };

  return (
    <button type="button" onClick={handleCopy}>
      {isCopied ? (
        <Icons.Check className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
      ) : (
        <Icons.Copy className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
      )}
    </button>
  );
}

function DownloadButton({
  href,
  filename,
}: {
  href: string;
  filename?: string;
}) {
  const [isDownloading, setIsDownloading] = useState(false);

  const isFileDownload = href.includes("/files/download/file");
  const isInvoiceDownload = href.includes("/files/download/invoice");
  const needsAuth = isFileDownload || isInvoiceDownload;

  // Extract invoice ID if it's an invoice download
  let invoiceId: string | null = null;
  if (isInvoiceDownload) {
    try {
      invoiceId = new URL(href).searchParams.get("id");
    } catch {
      // Invalid URL, will fall back to url type
    }
  }

  const { url: authenticatedUrl, isLoading } = useFileUrl(
    needsAuth ? { type: "url", url: href } : null,
  );

  const handleDownload = async (e: React.MouseEvent) => {
    e.stopPropagation();
    const url = authenticatedUrl || href;
    if (!url) return;

    try {
      setIsDownloading(true);
      await downloadFile(url, filename || "download");
      setTimeout(() => setIsDownloading(false), 1000);
    } catch (error) {
      console.error("Download failed:", error);
      setIsDownloading(false);
    }
  };

  return (
    <button
      type="button"
      onClick={handleDownload}
      disabled={
        isDownloading || (needsAuth && (!authenticatedUrl || isLoading))
      }
    >
      {isDownloading || isLoading ? (
        <Spinner size={16} />
      ) : (
        <Icons.ArrowCoolDown className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
      )}
    </button>
  );
}

// Helper function to format group names
const formatGroupName = (name: string, t: any): string | null => {
  switch (name) {
    case "shortcut":
      return t("common.actions");
    case "student":
      return t("roles.student");
    case "class":
      return t("settings.classes");
    case "staff":
      return t("settings.staff");
    case "subject":
      return t("settings.subjects");
    case "vault":
      return t("menu.vault");
    case "report":
      return t("menu.reports");
    case "certificate":
      return t("menu.certificates");

    default:
      return null;
  }
};

// Add desktop navigation function
const handleDesktopNavigation = async (
  path: string,
  params?: Record<string, any>,
) => {
  if (!isDesktopApp()) return false;

  try {
    // Step 1: Get the main window first to check its current path
    const mainWindow = await Window.getByLabel("main");

    if (!mainWindow) {
      console.error("Main window not found for navigation");
      return false;
    }

    // Step 2: Close search window
    await emit("search-window-close-requested");

    // Step 3: Show and focus main window
    await invoke("show_window");

    // Step 4: Navigate in main window context
    // If we have params, we need to use the main window's current path
    // If it's a full path (like /inbox?inboxId=...), use it directly
    if (params && Object.keys(params).length > 0) {
      // For param navigation, we want to stay on the current main window page
      // but we can't easily get the main window's current path from here
      // So let's send a special signal to navigate with params on current page
      await mainWindow.emit("desktop-navigate-with-params", { params });
    } else {
      // For full path navigation, use the path directly
      await mainWindow.emit("desktop-navigate", { path, params });
    }

    return true;
  } catch (error) {
    console.error("Failed to handle desktop navigation:", error);
    return false;
  }
};

const useSearchNavigation = () => {
  const navigate = useNavigate();
  const { setOpen } = useSearchStore();
  const { setParams: setStudentParams } = useStudentParams();
  const { setParams: setClassParams } = useClassParams();
  const { setParams: setSubjectParams } = useSubjectParams();
  const { setParams: setTrackerParams } = useTrackerParams();
  const { setParams: setDocumentParams } = useDocumentParams();
  const { setParams: setReportExportParams } = useReportExportParams();

  const shouldUseWebNavigation = async () => {
    if (!isDesktopApp()) {
      return true;
    }

    try {
      const currentWindow = await Window.getCurrent();
      const currentLabel = currentWindow.label;
      return currentLabel === "main";
    } catch (error) {
      console.error("Failed to get current window label:", error);
      return false;
    }
  };

  const navigateWithParams = async (
    params: Record<string, any>,
    paramSetter: (params: any) => void,
  ) => {
    const useWebNav = await shouldUseWebNavigation();

    if (useWebNav) {
      // Web mode - use traditional navigation
      setOpen();
      paramSetter(params);
      return;
    }

    // Desktop mode - use params-only navigation to stay on current main window page
    await handleDesktopNavigation("", params);
  };

  const navigateToPath = async (path: string) => {
    const useWebNav = await shouldUseWebNavigation();

    if (useWebNav) {
      // Web mode - use traditional navigation
      setOpen();
      navigate(path);
      return;
    }

    // Desktop mode - use full path navigation
    await handleDesktopNavigation(path);
  };

  return {
    navigateToDocument: (params: { documentId: string }) => {
      return navigateWithParams(params, setDocumentParams);
    },
    navigateToStudent: (params: { studentId: string; details?: boolean }) => {
      return navigateWithParams({ ...params, details: true }, setStudentParams);
    },
    navigateToClass: (params: { classId: string }) => {
      return navigateWithParams(params, setClassParams);
    },
    navigateToSubject: (params: { subjectId: string }) => {
      return navigateWithParams(params, setSubjectParams);
    },
    navigateToTracker: (params: {
      projectId?: string;
      update?: boolean;
      create?: boolean;
      selectedDate?: string;
    }) => {
      return navigateWithParams(params, setTrackerParams);
    },
    navigateToPath: (path: string) => {
      return navigateToPath(path);
    },
    openReportsSheet: () => {
      return navigateWithParams({ exportReport: true }, setReportExportParams);
    },
    // Action helpers
    createStudent: () => {
      return navigateWithParams({ createStudent: true }, setStudentParams);
    },
    createClass: () => {
      return navigateWithParams({ createClass: true }, setClassParams);
    },
    createSubject: () => {
      return navigateWithParams({ createSubject: true }, setSubjectParams);
    },
    createProject: () => {
      return navigateWithParams({ create: true }, setTrackerParams);
    },
    trackTime: () => {
      return navigateWithParams(
        { selectedDate: formatISO(new Date(), { representation: "date" }) },
        setTrackerParams,
      );
    },
  };
};

// Sub-component to render each search item
const SearchResultItemDisplay = ({
  item,
  dateFormat,
}: {
  item: SearchItem;
  dateFormat?: string;
}) => {
  const nav = useSearchNavigation();

  let icon: ReactNode | undefined;
  let resultDisplay: ReactNode;
  let onSelect: () => void;

  if (!item.data) {
    // This is an action item (e.g., "Create Invoice", "View Documents")
    icon = <Icons.Shortcut className="size-4 dark:text-[#666] text-primary" />;
    resultDisplay = item.title;
  } else {
    icon = null;
    resultDisplay = item.title;

    switch (item.type) {
      case "vault": {
        onSelect = () => nav.navigateToDocument({ documentId: item.id });

        icon = (
          <FilePreviewIcon
            mimetype={item.data?.metadata?.mimetype}
            className="size-4 dark:text-[#666] text-primary"
          />
        );
        resultDisplay = (
          <div className="flex items-center justify-between w-full">
            <span className="flex-grow truncate">
              {
                (item.data?.title ||
                  (item.data?.name as string)?.split("/").at(-1) ||
                  "") as string
              }
            </span>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <CopyButton path={`?documentId=${item.id}`} />
              <DownloadButton
                href={`${process.env.NEXT_PUBLIC_API_URL}/files/download/file?path=${item.data?.path_tokens?.join("/")}&filename=${
                  (item.data?.title ||
                    (item.data?.name as string)?.split("/").at(-1) ||
                    "") as string
                }`}
                filename={
                  (item.data?.title ||
                    (item.data?.name as string)?.split("/").at(-1) ||
                    "") as string
                }
              />
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );
        break;
      }
      case "student": {
        onSelect = () => nav.navigateToStudent({ studentId: item.id });

        icon = (
          <Icons.Customers className="size-4 dark:text-[#666] text-primary" />
        );
        resultDisplay = (
          <div className="flex items-center w-full">
            <div className="flex-grow truncate flex gap-2 items-center">
              <span>{item.data.name as string}</span>
              <span className="text-xs text-muted-foreground font-mono">
                {item.data.student_number as string}
              </span>
            </div>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <CopyButton path={`?studentId=${item.id}&details=true`} />
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );

        break;
      }
      case "class": {
        onSelect = () => nav.navigateToPath(`/reports/${item.id}`);

        icon = <Icons.Check className="size-4 dark:text-[#666] text-primary" />;
        resultDisplay = (
          <div className="flex items-center w-full">
            <div className="flex-grow truncate flex gap-2 items-center">
              <span>{item.data.name as string}</span>
            </div>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );
        break;
      }
      case "staff": {
        onSelect = () => nav.navigateToPath(`/staff?userId=${item.id}`);

        icon = (
          <Icons.Customers className="size-4 dark:text-[#666] text-primary" />
        );
        resultDisplay = (
          <div className="flex items-center w-full">
            <div className="flex-grow truncate flex gap-2 items-center">
              <span>{item.data.name as string}</span>
              <span className="text-xs text-muted-foreground">
                {item.data.email as string}
              </span>
              {item.data.status && (
                <Badge variant="outline" className="text-[10px] h-4">
                  {item.data.status as string}
                </Badge>
              )}
            </div>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );
        break;
      }
      case "subject": {
        onSelect = () => nav.navigateToSubject({ subjectId: item.id });

        icon = <Icons.AI className="size-4 dark:text-[#666] text-primary" />;
        resultDisplay = (
          <div className="flex items-center w-full">
            <div className="flex-grow truncate flex gap-2 items-center">
              <span>{item.data.name as string}</span>
            </div>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );
        break;
      }
      case "inbox": {
        onSelect = () => nav.navigateToPath(`/inbox?inboxId=${item.id}`);

        icon = (
          <Icons.Inbox2 size={14} className="dark:text-[#666] text-primary" />
        );
        resultDisplay = (
          <div className="flex items-center justify-between w-full">
            <div className="flex-grow truncate flex gap-2 items-center">
              <span>
                {
                  (item.data?.display_name ||
                    (item.data?.file_name as string)?.split("/").at(-1) ||
                    "") as string
                }
              </span>
              <span className="text-xs text-muted-foreground">
                {item.data?.date && formatDate(item.data.date, dateFormat)}
              </span>
            </div>
            <div className="flex items-center gap-2 invisible group-hover/item:visible group-focus/item:visible group-aria-selected/item:visible">
              <CopyButton path={`/inbox?inboxId=${item.id}`} />
              <DownloadButton
                href={`${process.env.NEXT_PUBLIC_API_URL}/files/download/file?path=${item.data?.file_path?.join("/")}&filename=${item.data?.file_name || ""}`}
                filename={item.data?.file_name || "download"}
              />
              <Icons.ArrowOutward className="size-4 dark:text-[#666] text-primary hover:!text-primary cursor-pointer" />
            </div>
          </div>
        );
        break;
      }
      case "tracker_project": {
        onSelect = () =>
          nav.navigateToTracker({ projectId: item.id, update: true });

        icon = null; // TrackerTimer will handle its own icon
        resultDisplay = (
          <div className="flex items-center w-full">
            <div className="flex-grow min-w-0 -ml-[6px]">
              <TrackerTimer
                projectId={item.id}
                projectName={item.data.name as string}
                onClick={() =>
                  nav.navigateToTracker({ projectId: item.id, update: true })
                }
                alwaysShowButton={true}
              />
            </div>
          </div>
        );
        break;
      }
      case "inbox":
      case "tracker_project":
        return null;
      default:
        // For types not explicitly handled but have data,
        // icon remains the default data icon, and resultDisplay remains item.title.
        // This is fine.
        break;
    }
  }

  const handleSelect = () => {
    item.action?.();
    onSelect?.();
  };

  return (
    <CommandItem
      key={item.id}
      value={item.id}
      onSelect={handleSelect}
      className="text-sm flex flex-col items-start gap-1 py-2 group/item"
    >
      <div className="flex items-center gap-2 w-full">
        {icon}
        {resultDisplay}
      </div>
    </CommandItem>
  );
};

export function Search() {
  const t = useI18n();
  const { data: user } = useUserQuery();
  const [debounceDelay, setDebounceDelay] = useState(200);
  const ref = useRef<HTMLDivElement>(null);
  const searchInputRef = useRef<HTMLInputElement>(null);
  const height = useRef<HTMLDivElement>(null);
  const nav = useSearchNavigation();
  const trpc = useTRPC();

  useHotkeys(
    "esc",
    () => {
      setDebouncedSearch("");
      emit("search-window-close-requested");
    },
    {
      enableOnFormTags: true,
    },
  );

  useEffect(() => {
    if (!isDesktopApp()) {
      return;
    }

    const unlistenPromise = listen("search-window-open", (event) => {
      const isOpen = event.payload as boolean;
    });

    // Cleanup function
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  });

  const [debouncedSearch, setDebouncedSearch] = useDebounceValue(
    "",
    debounceDelay,
  );

  const sectionActions: SearchItem[] = [
    {
      id: "sc-create-student",
      type: "student",
      title: t("students.create"),
      action: nav.createStudent,
    },
    {
      id: "sc-create-class",
      type: "class",
      title: t("education.create_class"),
      action: nav.createClass,
    },
    {
      id: "sc-create-subject",
      type: "subject",
      title: t("settings.subjects"),
      action: nav.createSubject,
    },
    {
      id: "sc-view-classes",
      type: "class",
      title: t("menu.classes"),
      action: () => nav.navigateToPath("/settings/classes"),
    },
    {
      id: "sc-view-staff",
      type: "staff",
      title: t("menu.staff_teachers"),
      action: () => nav.navigateToPath("/settings/staff"),
    },
    {
      id: "sc-view-grades",
      type: "shortcut",
      title: t("menu.grades"),
      action: () => nav.navigateToPath("/grades"),
    },
    {
      id: "sc-view-documents",
      type: "vault",
      title: t("menu.vault"),
      action: () => nav.navigateToPath("/vault"),
    },
    {
      id: "sc-view-certificates",
      type: "certificate",
      title: t("menu.certificates"),
      action: () => nav.navigateToPath("/certificates"),
    },
    {
      id: "sc-view-reports",
      type: "report",
      title: t("reports.extract_report"),
      action: nav.openReportsSheet,
    },
    {
      id: "sc-view-subjects",
      type: "subject",
      title: t("menu.subjects"),
      action: () => nav.navigateToPath("/settings/subjects"),
    },
    {
      id: "sc-view-promotions",
      type: "shortcut",
      title: t("menu.promotions"),
      action: () => nav.navigateToPath("/settings/promotions"),
    },
  ];

  // Fetch data using useQuery
  const {
    data: queryResult,
    isLoading,
    isFetching,
  } = useQuery({
    ...trpc.search.global.queryOptions({
      searchTerm: debouncedSearch,
    }),
    placeholderData: (previousData) => previousData,
    enabled: debouncedSearch.length >= 2,
  });

  // Extract search results array from queryResult
  const searchResults: SearchItem[] = queryResult || [];

  const combinedData = useMemo(() => {
    // Type assertion for searchResults from DB to ensure they have actions if needed,
    // or map them to include default actions. For now, assuming they come with 'type' and 'title'.
    const mappedSearchResults = searchResults
      .filter((res) => res.type !== "inbox" && res.type !== "tracker_project")
      .map((res) => ({
        ...res,
        action: () => {},
      }));
    return [...mappedSearchResults];
  }, [debouncedSearch, searchResults]);

  const groupedData = useMemo(() => {
    const groups: Record<string, SearchItem[]> = {};
    // Group search results first
    for (const item of combinedData) {
      const groupKey = item.type || "other";
      if (!groups[groupKey]) {
        groups[groupKey] = [];
      }
      groups[groupKey].push(item);
    }

    // Filter sectionActions based on debouncedSearch
    const filteredSectionActions = debouncedSearch
      ? sectionActions.filter((action) =>
          action.title.toLowerCase().includes(debouncedSearch.toLowerCase()),
        )
      : sectionActions;

    // Add filtered sectionActions to their respective groups
    for (const actionItem of filteredSectionActions) {
      const groupKey = actionItem.type;
      if (!groups[groupKey]) {
        groups[groupKey] = [];
      }
      groups[groupKey].push(actionItem);
    }

    const allGroupKeysInOrder: string[] = [];
    const addedKeys = new Set<string>();

    // Add any remaining groups that weren't in the defined order
    for (const key in groups) {
      if (groups[key] && groups[key].length > 0 && !addedKeys.has(key)) {
        allGroupKeysInOrder.push(key);
        addedKeys.add(key);
      }
    }

    const orderedGroups: Record<string, SearchItem[]> = {};
    for (const key of allGroupKeysInOrder) {
      if (groups[key] && groups[key].length > 0) {
        // Ensure group is not empty before adding
        orderedGroups[key] = groups[key];
      }
    }
    return orderedGroups;
  }, [combinedData, debouncedSearch]);

  useEffect(() => {
    if (height.current && ref.current && !isDesktopApp()) {
      const el = height.current;
      const wrapper = ref.current;
      let animationFrame: number;
      const observer = new ResizeObserver(() => {
        animationFrame = requestAnimationFrame(() => {
          const newHeight = el.offsetHeight;
          wrapper.style.setProperty("--search-list-height", `${newHeight}px`);
        });
      });
      observer.observe(el);
      return () => {
        cancelAnimationFrame(animationFrame);
        observer.unobserve(el);
      };
    }
  }, []);

  return (
    <Command
      shouldFilter={false}
      className="search-container overflow-hidden p-0 relative w-full bg-background backdrop-filter backdrop-blur-lg dark:bg-[#0C0C0C]/[99] h-auto border border-border"
    >
      <div className="border-b border-border relative">
        <CommandInput
          ref={searchInputRef}
          placeholder={t("common.find_anything")}
          onValueChange={(value: string) => {
            setDebouncedSearch(value);

            if (value.trim().split(/\s+/).length > 1) {
              setDebounceDelay(700);
            } else {
              setDebounceDelay(200);
            }
          }}
          className="px-4 h-[55px] py-0"
        />
        {isFetching && (
          <div className="absolute bottom-0 h-[2px] w-full overflow-hidden">
            <div className="absolute top-[1px] h-full w-40 animate-slide-effect bg-gradient-to-r dark:from-gray-800 dark:via-white dark:via-80% dark:to-gray-800 from-gray-200 via-black via-80% to-gray-200" />
          </div>
        )}
      </div>

      <div className="px-2 global-search-list" ref={ref}>
        <CommandList ref={height} className="scrollbar-hide">
          {!isLoading && combinedData.length === 0 && debouncedSearch && (
            <CommandEmpty>{t("common.no_data")}</CommandEmpty>
          )}
          {!isLoading &&
            Object.entries(groupedData).map(([groupName, items]) => (
              <CommandGroup
                key={groupName}
                heading={formatGroupName(groupName, t)}
              >
                {items.map((item: SearchItem) => (
                  <SearchResultItemDisplay key={item.id} item={item} />
                ))}
              </CommandGroup>
            ))}
        </CommandList>
      </div>
    </Command>
  );
}
