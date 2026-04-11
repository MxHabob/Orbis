"use client";

import { ControlSecretNumbers } from "./control-secret-numbers";
import { ControlSeatingNumbers } from "./control-seating-numbers";
export function ControlSettings() {

  return (
    <div className="flex flex-col gap-6">
      <div className="space-y-6">
        <ControlSecretNumbers />
        <ControlSeatingNumbers />
      </div>
    </div>
  );
}
