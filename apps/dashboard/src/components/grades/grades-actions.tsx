"use client";

import { useGradesParams } from "@/hooks/use-grades-params";
import { Button } from "@midday/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  DropdownMenuSub,
  DropdownMenuSubTrigger,
  DropdownMenuPortal,
  DropdownMenuSubContent,
  DropdownMenuGroup,
} from "@midday/ui/dropdown-menu";
import { Icons } from "@midday/ui/icons";
import { parseAsBoolean, parseAsString, useQueryStates } from "nuqs";

export function GradesActions() {
  const [_, setParams] = useQueryStates({
    step: parseAsString,
    hide: parseAsBoolean,
  });

  const { setParams: setGradesParams } = useGradesParams();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon">
          <Icons.Add size={17} />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent sideOffset={10} align="end">
        <DropdownMenuItem
          onClick={() => setParams({ step: "connect" })}
          className="space-x-2"
        >
          <Icons.Accounts size={18} />
          <span>Connect accounts</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setParams({ step: "import", hide: true })}
          className="space-x-2"
        >
          <Icons.Import size={18} />
          <span>Backfill</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() => setGradesParams({ createGrade: true })}
          className="space-x-2"
        >
          <Icons.CreateTransaction size={18} />
          <span>Upload grades</span>
        </DropdownMenuItem>
        <DropdownMenuItem
          onClick={() =>
            document.getElementById("upload-transaction-files")?.click()
          }
          className="space-x-2"
        >
          <Icons.ReceiptLong size={18} />
          <span>Upload receipts</span>
        </DropdownMenuItem>

        <DropdownMenuGroup>
          <DropdownMenuSub>
            <DropdownMenuSubTrigger className="space-x-2 cursor-pointer">
              <Icons.Share size={18} />
              <span>Export</span>
            </DropdownMenuSubTrigger>
            <DropdownMenuPortal>
              <DropdownMenuSubContent sideOffset={14} alignOffset={-4}>
                <DropdownMenuItem className="space-x-2 cursor-pointer">
                  <Icons.FolderZip size={18} />
                  <span>Excel (CSV)</span>
                </DropdownMenuItem>
                <DropdownMenuItem className="space-x-2 cursor-pointer">
                  <Icons.Pdf size={18} />
                  <span>PDF</span>
                </DropdownMenuItem>
              </DropdownMenuSubContent>
            </DropdownMenuPortal>
          </DropdownMenuSub>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
