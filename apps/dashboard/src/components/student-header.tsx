"use client";

import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { Input } from "@midday/ui/input";
import { Icons } from "@midday/ui/icons";
import { useRef, useState, useEffect } from "react";
import { Avatar, AvatarFallback } from "@midday/ui/avatar";
import { AcademicYearFilter } from "./academic-year-filter";

function StudentSearchFilter() {
  const [input, setInput] = useState("");
  const { filter, setFilter } = useCustomerFilterParams();

  useEffect(() => {
    if (filter.q) setInput(filter.q);
  }, []);

  const handleSearch = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const value = evt.target.value;
    setInput(value);
    setFilter({ q: value || null });
  };

  return (
    <div className="relative w-full sm:w-[250px]">
      <Icons.Search className="absolute pointer-events-none left-3 top-[10px] h-3.5 w-3.5 text-muted-foreground" />
      <Input
        placeholder="Search students..."
        className="pl-9 h-8 text-xs bg-background rounded-none border-border shadow-none focus-visible:ring-0"
        value={input}
        onChange={handleSearch}
        autoComplete="off"
      />
    </div>
  );
}

type Props = {
  name: string;
  color?: string | null;
};

export const StudentHeader = ({
  name,
  color,
}: Props) => {
  const customerInfo = (
    <div className="flex items-center space-x-3">
      {name && (
        <Avatar className="size-6 rounded-none border border-border flex-shrink-0">
          <AvatarFallback 
            className="text-xs font-medium rounded-none"
            style={{ backgroundColor: color || "#F2F1EF" }}
          >
            {name?.[0]?.toUpperCase()}
          </AvatarFallback>
        </Avatar>
      )}
      <h1 className="truncate text-xl font-serif font-medium tracking-tight whitespace-nowrap">{name}</h1>
    </div>
  );

  return (
    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 py-2">
      <div className="flex items-center gap-4 min-w-0">
        {customerInfo}
      </div>

      <div className="flex items-center gap-3">
        <StudentSearchFilter />
        <AcademicYearFilter />
      </div>
    </div>
  );
};
