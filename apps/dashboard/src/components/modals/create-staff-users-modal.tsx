"use client";

import {
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@orbis/ui/dialog";
import { InviteForm } from "../forms/invite-form";

type InviteTeamMembersModalProps = {
  onOpenChange: (open: boolean) => void;
};

export function InviteTeamMembersModal({
  onOpenChange,
}: InviteTeamMembersModalProps) {
  return (
    <DialogContent className="max-w-[780px]">
      <div className="p-2">
        <DialogHeader className="mb-4">
          <DialogTitle className="text-xl font-semibold">User Creation</DialogTitle>
          <DialogDescription>
            Add new members to your institution by providing their details below.
          </DialogDescription>
        </DialogHeader>

        <InviteForm onSuccess={() => onOpenChange(false)} skippable={false} />
      </div>
    </DialogContent>
  );
}
