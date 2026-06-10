"use client";


import { FilterList } from "@/components/filter-list";
import { useDocumentFilterParams } from "@/hooks/use-document-filter-params";
import { useTRPC } from "@/trpc/client";
import { authClient } from "@orbis/auth/auth-client";
import { Calendar } from "@orbis/ui/calendar";
import { cn } from "@orbis/ui/cn";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuPortal,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@orbis/ui/dropdown-menu";
import { Icons } from "@orbis/ui/icons";
import { Input } from "@orbis/ui/input";
import { formatISO, parseISO } from "date-fns";
import { useRef, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useI18n } from "@/locales/client";

export function VaultSearchFilter() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [isOpen, setIsOpen] = useState(false);
  const [isFocused, setIsFocused] = useState(false);
  const trpc = useTRPC();
  const { data: session } = authClient.useSession();
  const user = session?.user;
  const t = useI18n();

  const { filter, setFilter } = useDocumentFilterParams();
  const [input, setInput] = useState(filter.q ?? "");

  useHotkeys(
    "esc",
    () => {
      setInput("");
      setFilter(null);
      setIsOpen(false);
    },
    {
      enableOnFormTags: true,
      enabled: Boolean(input) && isFocused,
    },
  );

  useHotkeys("meta+s", (evt) => {
    evt.preventDefault();
    inputRef.current?.focus();
  });

  const handleSearch = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const value = evt.target.value;

    if (value) {
      setInput(value);
    } else {
      setFilter(null);
      setInput("");
    }
  };

  const handleSubmit = async (e?: React.FormEvent) => {
    e?.preventDefault();
    setFilter({ q: input.length > 0 ? input : null });
  };

  const validFilters = Object.fromEntries(
    Object.entries(filter).filter(([key]) => key !== "q"),
  );

  const hasValidFilters = Object.values(validFilters).some(
    (value) => value !== null,
  );

  return (
    <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
      <div className="flex gap-4 items-center">
        <form
          className="relative"
          onSubmit={handleSubmit}
        >
          <Icons.Search className="absolute pointer-events-none start-3 top-[11px]" />
          <Input
            ref={inputRef}
            placeholder={t("vault.search_placeholder")}
            className="ps-9 w-full md:w-[350px] pe-8"
            value={input}
            onChange={handleSearch}
            autoComplete="off"
            autoCapitalize="none"
            autoCorrect="off"
            spellCheck="false"
            onFocus={() => setIsFocused(true)}
            onBlur={() => setIsFocused(false)}
          />

          <DropdownMenuTrigger asChild>
            <button
              onClick={() => setIsOpen((prev) => !prev)}
              type="button"
              className={cn(
                "absolute z-10 end-3 top-[10px] opacity-50 transition-opacity duration-300 hover:opacity-100",
                hasValidFilters && "opacity-100",
                isOpen && "opacity-100",
              )}
            >
              <Icons.Filter />
            </button>
          </DropdownMenuTrigger>
        </form>

        <FilterList
          filters={validFilters}
          loading={false}
          onRemove={setFilter}
        />
      </div>

      <DropdownMenuContent
        className="w-[350px]"
        align="end"
        sideOffset={19}
        alignOffset={-11}
        side="bottom"
      >
        <DropdownMenuGroup>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger>
              <Icons.CalendarMonth className="me-2 h-4 w-4" />
              <span>{t("vault.date")}</span>
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent
                sideOffset={14}
                alignOffset={-4}
                className="p-0"
              >
                <Calendar
                  mode="range"
                  initialFocus
                  weekStartsOn={session?.user?.weekStartsOnMonday ? 1 : 0}
                  toDate={new Date()}
                  selected={
                    filter.start || filter.end
                      ? {
                          from: filter.start
                            ? parseISO(filter.start)
                            : undefined,
                          to: filter.end ? parseISO(filter.end) : undefined,
                        }
                      : undefined
                  }
                  onSelect={(range) => {
                    if (!range) return;

                    const newRange = {
                      start: range.from
                        ? formatISO(range.from, { representation: "date" })
                        : filter.start,
                      end: range.to
                        ? formatISO(range.to, { representation: "date" })
                        : filter.end,
                    };

                    setFilter(newRange);
                  }}
                />
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
