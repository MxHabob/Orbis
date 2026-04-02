"use client";

import { useClassesFilterParams } from "@/hooks/use-classes-filter-params";
import { cn } from "@midday/ui/cn";
import { Icons } from "@midday/ui/icons";
import { Input } from "@midday/ui/input";
import { useRef, useState } from "react";
import { useHotkeys } from "react-hotkeys-hook";
import { useI18n } from "@/locales/client";

export function ReportsSearchFilter() {
  const [input, setInput] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);
  const t = useI18n();

  const { setFilter } = useClassesFilterParams();

  useHotkeys(
    "esc",
    () => {
      setInput("");
      setFilter(null);
    },
    {
      enableOnFormTags: true,
      enabled: Boolean(input),
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
      setFilter({ q: value });
    } else {
      setFilter(null);
      setInput("");
    }
  };

  const handleSubmit = (e?: React.FormEvent) => {
    e?.preventDefault();
    setFilter({ q: input.length > 0 ? input : null });
  };

  return (
    <div className="flex flex-col sm:flex-row sm:space-x-4 space-y-4 sm:space-y-0 items-start sm:items-center w-full">
      <form
        className="relative w-full sm:w-auto"
        onSubmit={handleSubmit}
      >
        <Icons.Search className="absolute pointer-events-none inset-inline-start-3 top-[11px]" />
        <Input
          ref={inputRef}
          placeholder={t("education.search_classes")}
          className="ps-9 w-full sm:w-[350px] pe-8"
          value={input}
          onChange={handleSearch}
          autoComplete="off"
          autoCapitalize="none"
          autoCorrect="off"
          spellCheck="false"
        />
      </form>
    </div>
  );
}
