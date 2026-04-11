"use client";

import { useI18n } from "@/locales/client";
import { ControlSecretNumbers } from "./control-secret-numbers";
import { ControlSeatingNumbers } from "./control-seating-numbers";

export function ControlSettings() {
  const t = useI18n();

  return (
    <div className="flex flex-col gap-8">
      <div className="space-y-12">
        <ControlSecretNumbers />
        <ControlSeatingNumbers />
      </div>
    </div>
  );
}
