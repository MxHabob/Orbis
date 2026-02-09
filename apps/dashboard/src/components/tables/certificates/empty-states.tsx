"use client";

import { Button } from "@midday/ui/button";
import { Icons } from "@midday/ui/icons";
import Link from "next/link";

export function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center py-32 border border-dashed rounded-lg bg-muted/5">
      <div className="flex flex-col items-center justify-center gap-4">
        <div className="w-16 h-16 bg-muted rounded-full flex items-center justify-center">
          <Icons.Files className="w-8 h-8 opacity-20" />
        </div>
        <div className="space-y-1 text-center">
          <h3 className="text-lg font-bold">لا توجد قوالب</h3>
          <p className="text-sm text-muted-foreground">ابدأ بإنشاء أول قالب شهادة لطلابك.</p>
        </div>
        <Link href="/certificates/create">
          <Button className="mt-4">إنشاء أول قالب</Button>
        </Link>
      </div>
    </div>
  );
}

export function NoResults() {
  return (
    <div className="flex flex-col items-center justify-center py-32">
      <p className="text-sm text-muted-foreground">لم يتم العثور على نتائج للفلترة الحالية.</p>
    </div>
  );
}
