"use client";

import type { RouterOutputs } from "@/ipc/query-adapter";
import { cn } from "@orbis/ui/cn";
import { useI18n } from "@/locales/client";

type Props = {
  status: NonNullable<RouterOutputs["trackerProjects"]["getById"]>["status"];
};

export function TrackerStatus({ status }: Props) {
  const t = useI18n();

  return (
    <div className="flex items-center space-x-4">
      <div
        className={cn(
          "w-[6px] h-[6px] rounded-full bg-[#FFD02B]",
          status === "completed" && "bg-primary",
        )}
      />
      {/* @ts-expect-error */}
      <span>{t(`tracker_status.${status}`)}</span>
    </div>
  );
}
