"use client";

import { useRouter } from "next/navigation";
import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@midday/ui/dropdown-menu";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import type { ClassReport } from "./columns";

type Props = {
  row: ClassReport;
};

export function ActionsMenu({ row }: Props) {
  const router = useRouter();

  const handleViewReport = () => {
    router.push(`/reports/${row.id}`);
  };

  return (
    <div className="flex items-center justify-center w-full">
      <DropdownMenu>
        <DropdownMenuTrigger asChild className="relative">
          <Button variant="ghost" className="h-8 w-8 p-0">
            <DotsHorizontalIcon className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent align="end">
          <DropdownMenuItem onClick={handleViewReport}>
            View Report
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
