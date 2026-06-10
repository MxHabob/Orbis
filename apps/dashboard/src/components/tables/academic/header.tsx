"use client";

import { useAcademicParams } from "@/hooks/use-academic-params";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import type { Table } from "@tanstack/react-table";
import { Icons } from "@orbis/ui/icons";
import { useI18n } from "@/locales/client";

type Props = {
  table?: Table<any>;
};

export function Header({ table }: Props) {
  const t = useI18n();
  const { setParams } = useAcademicParams();

  const handleCreateYear = () => {
    setParams({ createYear: true });
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
        <Button onClick={handleCreateYear}>
          <Icons.Add className="w-4 h-4 mr-2" />
          {t("settings.create_year")}
        </Button>
      </div>
    </div>
  );
}
