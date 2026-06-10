"use client";

import { useTRPC } from "@/trpc/client";
import { cn } from "@orbis/ui/cn";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
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
import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useGradeFilterParams } from "@/hooks/use-grade-filter-params";
import { useI18n } from "@/locales/client";

interface FilterMenuItemProps {
  icon: (typeof Icons)[keyof typeof Icons];
  label: string;
  children: React.ReactNode;
}

interface FilterCheckboxItemProps {
  id: string | number;
  name: string;
  checked?: boolean;
  className?: string;
  onCheckedChange: (checked: boolean) => void;
}

import { HIJRI_MONTHS } from "@/utils/academic-calendar";

const months = HIJRI_MONTHS.map(m => ({ id: m.id, name: m.label }));


function FilterMenuItem({ icon: Icon, label, children }: FilterMenuItemProps) {
  return (
    <DropdownMenuGroup>
      <DropdownMenuSub>
        <DropdownMenuSubTrigger>
          <Icon className="mr-2 size-4" />
          <span>{label}</span>
        </DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent
            sideOffset={14}
            alignOffset={-4}
            className="p-0"
          >
            {children}
          </DropdownMenuSubContent>
        </DropdownMenuPortal>
      </DropdownMenuSub>
    </DropdownMenuGroup>
  );
}

function FilterCheckboxItem({
  id,
  name,
  checked = false,
  onCheckedChange,
  className,
}: FilterCheckboxItemProps) {
  return (
    <DropdownMenuCheckboxItem
      key={id}
      checked={checked}
      onCheckedChange={onCheckedChange}
      className={className}
    >
      {name}
    </DropdownMenuCheckboxItem>
  );
}

export function GradesSearchFilter() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [isOpen, setIsOpen] = useState(false);
  const [isFocused, setIsFocused] = useState(false);
  const { filter, setFilter, hasFilters } = useGradeFilterParams();
  const [input, setInput] = useState(filter.q ?? "");
  const trpc = useTRPC();
  const t = useI18n();

  // Fetch filter data
  const { data: years } = useQuery(trpc.academic.getYears.queryOptions());
  const { data: terms } = useQuery(
    trpc.academic.getTerms.queryOptions(
      { yearId: filter.yearId ?? 0 },
      { enabled: !!filter.yearId }
    )
  );
  const { data: classes } = useQuery(
    trpc.classes.getAll.queryOptions({ academicYearId: filter.yearId ?? undefined })
  );
  const { data: subjects } = useQuery(trpc.subjects.getAll.queryOptions());

  useEffect(() => {
    setInput(filter.q ?? "");
  }, [filter.q]);

  useHotkeys(
    "esc",
    () => {
      setInput("");
      setFilter({ q: null });
      setIsOpen(false);
    },
    {
      enableOnFormTags: true,
      enabled: Boolean(input) && isFocused,
    }
  );

  useHotkeys("meta+s", (evt) => {
    evt.preventDefault();
    inputRef.current?.focus();
  });

  const handleSearch = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const value = evt.target.value;
    setInput(value);
  };

  const handleSubmit = (e?: React.FormEvent) => {
    e?.preventDefault();
    setFilter({ q: input.length > 0 ? input : null });
  };

  return (
    <div className="flex flex-col sm:flex-row sm:space-x-4 space-y-2 sm:space-y-0 items-stretch sm:items-center w-full md:w-auto">
      <form
        className="relative flex-1 sm:flex-initial"
        onSubmit={handleSubmit}
      >
        <Icons.Search className="absolute pointer-events-none left-3 top-[11px]" />
        <Input
          ref={inputRef}
          placeholder={t("grades.search_student_name")}
          className="pl-9 w-full sm:w-[350px] pr-8"
          value={input}
          onChange={handleSearch}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          autoComplete="off"
          autoCapitalize="none"
          autoCorrect="off"
          spellCheck="false"
        />

        <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
          <DropdownMenuTrigger asChild>
            <button
              onClick={() => setIsOpen((prev) => !prev)}
              type="button"
              className={cn(
                "absolute z-10 right-3 top-[10px] opacity-50 transition-opacity duration-300 hover:opacity-100",
                hasFilters && "opacity-100",
                isOpen && "opacity-100"
              )}
            >
              <Icons.Filter />
            </button>
          </DropdownMenuTrigger>

          <DropdownMenuContent
            className="w-[250px]"
            align="end"
            sideOffset={19}
            alignOffset={-11}
          >
            <FilterMenuItem icon={Icons.CalendarMonth} label="Academic Year">
              <div className="max-h-[300px] overflow-y-auto">
                {years?.map((year) => (
                  <FilterCheckboxItem
                    key={year.id}
                    id={year.id}
                    name={year.yearName}
                    checked={filter.yearId === year.id}
                    onCheckedChange={() =>
                      setFilter({ yearId: filter.yearId === year.id ? null : year.id, termId: null })
                    }
                  />
                ))}
              </div>
            </FilterMenuItem>

            <FilterMenuItem icon={Icons.Repeat} label="Term">
              <div className="max-h-[300px] overflow-y-auto">
                {terms?.map((term) => (
                  <FilterCheckboxItem
                    key={term.id}
                    id={term.id}
                    name={term.name}
                    checked={filter.termId === term.id}
                    onCheckedChange={() =>
                      setFilter({ termId: filter.termId === term.id ? null : term.id })
                    }
                  />
                ))}
                {!filter.yearId && (
                   <p className="text-[10px] text-muted-foreground p-2 text-center">Select year first</p>
                )}
              </div>
            </FilterMenuItem>

            <FilterMenuItem icon={Icons.Accounts} label="Class">
              <div className="max-h-[300px] overflow-y-auto">
                {classes?.data?.map((c) => (
                  <FilterCheckboxItem
                    key={c.id}
                    id={c.id}
                    name={`${c.name} ${c.division ? `- ${c.division}` : ""}`}
                    checked={filter.selectedClassId === c.id}
                    onCheckedChange={() =>
                      setFilter({ selectedClassId: filter.selectedClassId === c.id ? null : c.id })
                    }
                  />
                ))}
              </div>
            </FilterMenuItem>

            <FilterMenuItem icon={Icons.Category} label="Subject">
              <div className="max-h-[300px] overflow-y-auto">
                {subjects?.map((subject) => (
                  <FilterCheckboxItem
                    key={subject.id}
                    id={subject.id}
                    name={subject.name}
                    checked={filter.selectedSubjectId === subject.id}
                    onCheckedChange={() =>
                      setFilter({ selectedSubjectId: filter.selectedSubjectId === subject.id ? null : subject.id })
                    }
                  />
                ))}
              </div>
            </FilterMenuItem>

            <FilterMenuItem icon={Icons.CalendarMonth} label="Month">
              <div className="max-h-[300px] overflow-y-auto">
                {months.map((month) => (
                  <FilterCheckboxItem
                    key={month.id}
                    id={month.id}
                    name={month.name}
                    checked={filter.month === month.id}
                    onCheckedChange={() =>
                      setFilter({ month: filter.month === month.id ? null : month.id })
                    }
                  />
                ))}
              </div>
            </FilterMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </form>
    </div>
  );
}
