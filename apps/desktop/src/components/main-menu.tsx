"use client";

import { cn } from "@orbis/ui/cn";
import { Icons } from "@orbis/ui/icons";
import { Link } from "react-router-dom";
import { useLocation } from "react-router-dom";
import { useEffect, useState } from "react";
import { useI18n } from "@/locales/client";

const icons = {
  "/": () => <Icons.Overview size={20} />,
  "/grades": () => <Icons.Bolt size={20} />,
  "/settings": () => <Icons.Settings size={20} />,
  "/vault": () => <Icons.Vault size={20} />,
  "/reports": () => <Icons.Customers size={20} />,
  "/certificates": () => <Icons.Files size={20} />,
  "/promotions": () => <Icons.TrendingUp size={20} />,
} as const;

// Known menu base paths that should not be treated as chat IDs
const KNOWN_MENU_PATHS = [
  "/grades",
  "/exams",
  "/fees",
  "/settings",
  "/reports",
  "/vault",
  "/certificates",
  "/promotions",
];

interface ItemProps {
  item: {
    path: string;
    name: string;
    children?: { path: string; name: string }[];
  };
  isActive: boolean;
  isExpanded: boolean;
  isItemExpanded: boolean;
  onToggle: (path: string) => void;
  onSelect?: () => void;
}

const ChildItem = ({
  child,
  isActive,
  isExpanded,
  shouldShow,
  onSelect,
  index,
}: {
  child: { path: string; name: string };
  isActive: boolean;
  isExpanded: boolean;
  shouldShow: boolean;
  onSelect?: () => void;
  index: number;
}) => {
  const showChild = isExpanded && shouldShow;

  return (
    <Link
      prefetch
      to={child.path}
      onClick={() => onSelect?.()}
      className="block group/child"
    >
      <div className="relative">
        {/* Child item text */}
        <div
          className={cn(
            "ms-[35px] me-[15px] h-[32px] flex items-center",
            "border-s border-[#e6e6e6] dark:border-[#1d1d1d] ps-3",
            "transition-all duration-200 ease-out",
            showChild
              ? "opacity-100 translate-x-0"
              : "opacity-0 -translate-x-2 rtl:translate-x-2",
          )}
          style={{
            transitionDelay: showChild
              ? `${40 + index * 20}ms`
              : `${index * 20}ms`,
          }}
        >
          <span
            className={cn(
              "text-xs font-medium transition-colors duration-200",
              "text-[#888] group-hover/child:text-primary",
              "whitespace-nowrap overflow-hidden",
              isActive && "text-primary",
            )}
          >
            {child.name}
          </span>
        </div>
      </div>
    </Link>
  );
};

const Item = ({
  item,
  isActive,
  isExpanded,
  isItemExpanded,
  onToggle,
  onSelect,
}: ItemProps) => {
  const Icon = icons[item.path as keyof typeof icons];
  const { pathname } = useLocation();
  const hasChildren = item.children && item.children.length > 0;

  // Children should be visible when: expanded sidebar AND this item is expanded
  const shouldShowChildren = isExpanded && isItemExpanded;

  const handleChevronClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    onToggle(item.path);
  };

  return (
    <div className="group">
      <Link
        prefetch
        to={item.path}
        onClick={() => onSelect?.()}
        className="group"
      >
        <div className="relative">
          {/* Background that expands */}
          <div
            className={cn(
              "border border-transparent h-[40px] transition-all duration-200 ease-&lsqb;cubic-bezier(0.4,0,0.2,1)&rsqb; ms-[15px] me-[15px]",
              isActive &&
                "bg-[#f7f7f7] dark:bg-[#131313] border-[#e6e6e6] dark:border-[#1d1d1d]",
              isExpanded ? "w-[calc(100%-30px)]" : "w-[40px]",
            )}
          />

          {/* Icon - always in same position from sidebar edge */}
          <div className="absolute top-0 start-[15px] w-[40px] h-[40px] flex items-center justify-center dark:text-[#666666] text-black group-hover:!text-primary pointer-events-none">
            <div className={cn(isActive && "dark:!text-white")}>
              <Icon />
            </div>
          </div>

          {isExpanded && (
            <div className="absolute top-0 start-[55px] end-[4px] h-[40px] flex items-center pointer-events-none">
              <span
                className={cn(
                  "text-sm font-medium transition-opacity duration-200 ease-in-out text-[#666] group-hover:text-primary",
                  "whitespace-nowrap overflow-hidden",
                  hasChildren ? "pe-2" : "",
                  isActive && "text-primary",
                )}
              >
                {item.name}
              </span>
              {hasChildren && (
                <button
                  type="button"
                  onClick={handleChevronClick}
                  className={cn(
                    "w-8 h-8 flex items-center justify-center transition-all duration-200 ms-auto me-3",
                    "text-[#888] hover:text-primary pointer-events-auto",
                    isActive && "text-primary/60",
                    shouldShowChildren && "rotate-180",
                  )}
                >
                  <Icons.ChevronDown size={16} />
                </button>
              )}
            </div>
          )}
        </div>
      </Link>

      {/* Children */}
      {hasChildren && (
        <div
          className={cn(
            "transition-all duration-300 ease-out overflow-hidden",
            shouldShowChildren ? "max-h-96 mt-1" : "max-h-0",
          )}
        >
          {item.children!.map((child, index) => {
            const isChildActive = pathname === child.path;
            return (
              <ChildItem
                key={child.path}
                child={child}
                isActive={isChildActive}
                isExpanded={isExpanded}
                shouldShow={shouldShowChildren}
                onSelect={onSelect}
                index={index}
              />
            );
          })}
        </div>
      )}
    </div>
  );
};

type Props = {
  onSelectAction?: () => void;
  isExpanded?: boolean;
};

export function MainMenu({ onSelectAction, isExpanded = false }: Props) {
  const t = useI18n();
  const { pathname } = useLocation();
  const part = pathname?.split("/")[1];
  const [expandedItem, setExpandedItem] = useState<string | null>(null);

  const items = [
    {
      path: "/",
      name: t("menu.overview"),
    },
    {
      path: "/grades",
      name: t("menu.grades"),
      children: [{ path: "/grades/reports", name: t("menu.reports") }],
    },
    {
      path: "/reports",
      name: t("menu.reports"),
      children: [
        { path: "/reports/academic", name: t("menu.academic_reports") },
        { path: "/reports/attendance", name: t("menu.attendance_reports") },
      ],
    },
    {
      path: "/certificates",
      name: t("menu.certificates"),
    },
    {
      path: "/vault",
      name: t("menu.vault"),
    },
    {
      path: "/settings",
      name: t("menu.settings"),
      children: [
        { path: "/settings", name: t("menu.general") },
        { path: "/settings/staff", name: t("menu.staff_teachers") },
        { path: "/settings/academic", name: t("settings.academic") },
        { path: "/settings/classes", name: t("menu.classes") },
        { path: "/settings/subjects", name: t("menu.subjects") },
      ],
    },
  ];

  // Check if current pathname is a known menu path (including sub-paths)
  const pathnameWithoutQuery = pathname?.split("?")[0] || "";
  const isKnownMenuPath = KNOWN_MENU_PATHS.some((knownPath) =>
    pathnameWithoutQuery.startsWith(knownPath),
  );

  // Reset expanded item when sidebar expands/collapses
  useEffect(() => {
    setExpandedItem(null);
  }, [isExpanded]);

  return (
    <div className="mt-6 w-full">
      <nav className="w-full">
        <div className="flex flex-col gap-2">
          {items.map((item) => {
            // Check if current path matches item path or is a child of it
            const isActive =
              (pathname === "/" && item.path === "/") ||
              (pathname !== "/" && item.path.startsWith(`/${part}`));

            return (
              <Item
                key={item.path}
                item={item}
                isActive={isActive}
                isExpanded={isExpanded}
                isItemExpanded={expandedItem === item.path}
                onToggle={(path) => {
                  setExpandedItem(expandedItem === path ? null : path);
                }}
                onSelect={onSelectAction}
              />
            );
          })}
        </div>
      </nav>
    </div>
  );
}
