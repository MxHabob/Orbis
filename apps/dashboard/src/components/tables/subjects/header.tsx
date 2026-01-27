"use client";

import { useCategoryParams } from "@/hooks/use-category-params";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import type { Table } from "@tanstack/react-table";
import type { Subject } from "./columns";

type Props = {
  table?: Table<Subject>;
};

export function Header({ table }: Props) {
  const { setParams } = useCategoryParams();

  const handleCreateSubject = () => {
    setParams({ createCategory: true });
  };

  const meta = table?.options.meta as
    | { searchValue?: string; setSearchValue?: (value: string) => void }
    | undefined;

  const searchValue = meta?.searchValue ?? "";
  const setSearchValue = meta?.setSearchValue;

  return (
    <div className="flex items-center py-4 justify-between">
      <Input
        placeholder="Search..."
        value={searchValue}
        onChange={(event) => setSearchValue?.(event.target.value)}
        className="max-w-sm"
      />

      <Button onClick={handleCreateSubject}>Create</Button>
    </div>
  );
}
