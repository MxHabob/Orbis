"use client";

import { useCustomerFilterParams } from "@/hooks/use-customer-filter-params";
import { Input } from "@midday/ui/input";
import { Icons } from "@midday/ui/icons";
import { useRef, useState, useEffect } from "react";
import { Avatar, AvatarFallback } from "@midday/ui/avatar";
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
  DropdownMenuLabel,
  DropdownMenuSeparator
} from "@midday/ui/dropdown-menu";
import { Button } from "@midday/ui/button";
import { useStudentParams } from "@/hooks/use-student-params";
import { useTRPC } from "@/trpc/client";
import { useToast } from "@midday/ui/use-toast";
import { useMutation, useQueryClient, useQuery, useSuspenseQuery } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { downloadBase64 } from "@midday/export";
import { useI18n } from "@/locales/client";
import { cn } from "@midday/ui/cn";

function StudentSearchFilter() {
  const [input, setInput] = useState("");
  const { filter, setFilter } = useCustomerFilterParams();

  useEffect(() => {
    if (filter.q) setInput(filter.q);
  }, [filter.q]);

  const handleSearch = (evt: React.ChangeEvent<HTMLInputElement>) => {
    const value = evt.target.value;
    setInput(value);
    setFilter({ q: value || null });
  };

  return (
    <div className="relative w-full sm:w-[350px]">
      <Icons.Search className="absolute pointer-events-none left-3 top-[11px] h-3.5 w-3.5 text-muted-foreground" />
      <Input
        placeholder="Search students..."
        className="pl-9 h-9 text-xs bg-background rounded-none border-border shadow-none focus-visible:ring-0"
        value={input}
        onChange={handleSearch}
        autoComplete="off"
      />
    </div>
  );
}

interface StudentActionsProps {
  classId?: string;
  className?: string;
}

function StudentActions({ classId, className }: StudentActionsProps) {
  const trpc = useTRPC();
  const queryClient = useQueryClient();
  const { toast } = useToast();
  const { setParams: setStudentParams } = useStudentParams();
  const [isExporting, setIsExporting] = useState(false);
  const t = useI18n();

  const { filter, setFilter } = useCustomerFilterParams();

  const { data: years } = useSuspenseQuery(
    trpc.academic.getYears.queryOptions()
  );

  const exportMutation = useMutation(
    trpc.students.exportByClassId.mutationOptions({
      onSuccess: (data) => {
        downloadBase64(data.data, data.filename, data.mimeType);
        toast({
          title: "Export Successful",
          description: "Report has been exported successfully.",
          variant: "success",
        });
      },
      onError: (err) => {
        toast({
          title: "Export Failed",
          description: `Failed to export report: ${err.message}`,
          variant: "destructive",
        });
      },
      onSettled: () => setIsExporting(false),
    })
  );

  const handleExport = (format: "xlsx" | "csv") => {
    if (!classId) return;
    setIsExporting(true);
    exportMutation.mutate({
      classId,
      format,
    });
  };

  const handleYearChange = (yearId: string | null) => {
    setFilter({ academicYearId: yearId });
  };

  const currentYearId = filter.academicYearId || "current";

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className="h-9 gap-2 text-xs border-border rounded-none shadow-none bg-background focus:ring-0 px-3"
          disabled={isExporting}
        >
          {isExporting ? (
            <Loader2 className="animate-spin size-3.5" />
          ) : (
             <Icons.Add size={14} />
          )}
          <span>{t("common.actions") as string || "Actions"}</span>
          <Icons.ChevronDown size={14} className="text-muted-foreground ml-1" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent sideOffset={8} align="end" className="min-w-[200px] rounded-none">
        <DropdownMenuLabel className="text-[10px] uppercase text-muted-foreground font-semibold px-2 py-1">
          {t("common.actions") as string}
        </DropdownMenuLabel>
        
        <DropdownMenuItem
          onClick={() => setStudentParams({ createStudent: true })}
          className="gap-2 cursor-pointer rounded-none text-xs px-2 py-1.5"
        >
          <Icons.Accounts size={16} className="text-muted-foreground" />
          <span>Add Student</span>
        </DropdownMenuItem>

        <DropdownMenuSeparator className="bg-border/50" />

        <DropdownMenuSub>
          <DropdownMenuSubTrigger className="gap-2 cursor-pointer rounded-none text-xs px-2 py-1.5">
            <Icons.CalendarMonth size={16} className="text-muted-foreground" />
            <span>{t("education.academic_year")}</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent sideOffset={10} className="rounded-none min-w-[150px]">
              <DropdownMenuItem
                onClick={() => handleYearChange(null)}
                className={cn(
                  "cursor-pointer rounded-none text-xs",
                  currentYearId === "current" && "bg-accent"
                )}
              >
                {t("education.current")}
              </DropdownMenuItem>
              {years?.map((year) => (
                  <DropdownMenuItem
                    key={year.id}
                    onClick={() => handleYearChange(year.id.toString())}
                    className={cn(
                      "cursor-pointer rounded-none text-xs",
                      currentYearId === year.id.toString() && "bg-accent"
                    )}
                  >
                    <div className="flex items-center gap-2">
                      <span>{year.yearName}</span>
                      {year.isCurrent && (
                        <span className="text-[9px] text-muted-foreground">(Active)</span>
                      )}
                    </div>
                  </DropdownMenuItem>
                ))}
            </DropdownMenuSubContent>
          </DropdownMenuPortal>
        </DropdownMenuSub>

        <DropdownMenuSub>
          <DropdownMenuSubTrigger className="gap-2 cursor-pointer rounded-none text-xs px-2 py-1.5" disabled={!classId}>
            <Icons.Share size={16} className="text-muted-foreground" />
            <span>Export</span>
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent sideOffset={10} className="rounded-none">
              <DropdownMenuItem
                className="gap-2 cursor-pointer rounded-none text-xs"
                onClick={() => handleExport("xlsx")}
              >
                <Icons.FolderZip size={16} className="text-muted-foreground" />
                <span>Excel (XLSX)</span>
              </DropdownMenuItem>
            </DropdownMenuSubContent>
          </DropdownMenuPortal>
        </DropdownMenuSub>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

type Props = {
  name: string;
  color?: string | null;
  id?: string;
};

export const StudentHeader = ({ name, color, id }: Props) => {
  const classInfo = (
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
      <h1 className="truncate text-xl font-serif font-medium tracking-tight whitespace-nowrap">
        {name}
      </h1>
    </div>
  );

  return (
    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 py-4">
      <div className="flex items-center gap-4 min-w-0">
        {classInfo}
      </div>

      <div className="flex items-center gap-3">
        <StudentSearchFilter />
        <StudentActions classId={id} className={name} />
      </div>
    </div>
  );
};
