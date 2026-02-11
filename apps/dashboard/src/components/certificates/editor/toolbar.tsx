"use client";

import { memo } from "react";
import { Button } from "@midday/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@midday/ui/select";
import {
  Maximize,
  Type,
  Eye,
  Table as TableIcon,
} from "lucide-react";

interface ToolbarProps {
  paperSize: string;
  orientation: string;
  isPreview: boolean;
  setIsPreview: (isPreview: boolean) => void;
  previewStudentId: string | null;
  setPreviewStudentId: (id: string) => void;
  students: any;
}

export const Toolbar = memo(({
  paperSize,
  orientation,
  isPreview,
  setIsPreview,
  previewStudentId,
  setPreviewStudentId,
  students,
}: ToolbarProps) => {
  return (
    <div className="h-16 border-b flex items-center justify-between px-6 bg-background shrink-0">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <Maximize className="w-3 h-3" />
          {paperSize.toUpperCase()} · {orientation === 'portrait' ? 'Portrait' : 'Landscape'}
        </div>

        {isPreview && (
          <div className="flex items-center gap-2 border-r pr-4 mr-4" dir="ltr">
            <TableIcon className="w-3 h-3 text-muted-foreground" />
            <Select dir="ltr" value={previewStudentId || students?.[0]?.userId || ""} onValueChange={setPreviewStudentId}>
              <SelectTrigger className="h-7 w-[200px] text-[10px] bg-accent/50 border-0 shadow-none focus:ring-0">
                <SelectValue placeholder="Select a record for preview..." />
              </SelectTrigger>
              <SelectContent className="max-h-[400px]">
                {students?.map((s: any) => (
                  <SelectItem key={s.userId} value={s.userId} className="text-xs">
                    <div className="flex flex-col items-start gap-0.5" dir="ltr">
                      <span className="font-bold">{s.name || "No Name"}</span>
                      <span className="text-[9px] text-muted-foreground opacity-70">{s.studentNumber}</span>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}
      </div>

      <div className="flex items-center gap-2">
        <Button 
          variant={!isPreview ? "secondary" : "ghost"} 
          size="sm"
          onClick={() => setIsPreview(false)}
        >
          <Type className="w-4 h-4 mr-2" />
          Edit
        </Button>
        <Button 
          variant={isPreview ? "secondary" : "ghost"} 
          size="sm"
          onClick={() => setIsPreview(true)}
        >
          <Eye className="w-4 h-4 mr-2" />
          Preview
        </Button>
      </div>
    </div>
  );
});

Toolbar.displayName = "Toolbar";
