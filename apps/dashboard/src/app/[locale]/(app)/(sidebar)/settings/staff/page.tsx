import type { Metadata } from "next";
import { StaffUsers } from "@/components/staff-users";
import { prefetch, trpc } from "@/trpc/server";

export const metadata: Metadata = {
  title: "Staff | Orbis",
};

export default function Staff() {
  prefetch(trpc.user.getStaff.queryOptions());
  prefetch(trpc.user.getParents.queryOptions());

  return <StaffUsers />;
}
