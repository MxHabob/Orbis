"use client";

import { useRouter } from "next/navigation";
import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSeparator,
} from "@midday/ui/dropdown-menu";
import { DotsHorizontalIcon, FileTextIcon } from "@radix-ui/react-icons";
import type { ClassReport } from "./columns";
import { ExportStudentsModal } from "@/components/modals/export-students-modal";
import { useState } from "react";

type Props = {
  row: ClassReport;
};

export function ActionsMenu({ row }: Props) {
  const router = useRouter();
  const [isExportModalOpen, setIsExportModalOpen] = useState(false);

  const handleViewReport = () => {
    router.push(`/reports/${row.id}`);
  };

  const handleExport = () => {
    setIsExportModalOpen(true);
  };

  return (
    <>
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
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={handleExport}>
              Export to Excel
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>

      <ExportStudentsModal
        isOpen={isExportModalOpen}
        onOpenChange={setIsExportModalOpen}
        classId={row.id.toString()}
        className={row.name}
      />
    </>
  );
}
