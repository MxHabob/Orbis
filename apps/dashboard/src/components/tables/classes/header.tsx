"use client";

import { useClassParams } from "@/hooks/use-class-params";
import { Button } from "@midday/ui/button";
import { Input } from "@midday/ui/input";
import type { Table } from "@tanstack/react-table";
import type { Class } from "./columns";

type Props = {
  table?: Table<Class>;
};

export function Header({ table }: Props) {
  const { setParams } = useClassParams();

  const handleCreateClass = () => {
    setParams({ createClass: true });
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

      <div className="flex items-center space-x-2">
        <Button
          variant="outline"
          onClick={() => setParams({ step: "import", type: "students" })}
        >
          Import
        </Button>
        <Button onClick={handleCreateClass}>Create</Button>
      </div>
    </div>
  );
}
