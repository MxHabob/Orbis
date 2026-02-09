"use client";

import { useCertificateFilterParams } from "@/hooks/use-certificate-filter-params";
import { useCertificateParams } from "@/hooks/use-certificate-params";
import { Input } from "@midday/ui/input";
import { Plus, Search } from "lucide-react";
import { useDebounceCallback } from "usehooks-ts";
import { useState } from "react";
import Link from "next/link";
import { Button } from "@midday/ui/button";

export function TableToolbar() {
  const { filter, setFilter } = useCertificateFilterParams();
  const { setParams } = useCertificateParams();
  const [value, setValue] = useState(filter.q ?? "");

  const debounced = useDebounceCallback((q: string) => {
    setFilter({ q: q || null });
  }, 300);

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newVal = e.target.value;
    setValue(newVal);
    debounced(newVal);
  };

  return (
    <div className="flex items-center justify-between py-4">
      <div className="flex flex-1 items-center space-x-2">
        <div className="relative w-[300px]">
          <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search templates..."
            className="pl-8"
            value={value}
            onChange={onChange}
          />
        </div>
      </div>
      <Button size="sm" onClick={() => setParams({ type: "create" })}>
        <Plus className="w-4 h-4" />
      </Button>
    </div>
  );
}
