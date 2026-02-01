import { TeamMembers } from "@/components/team-members";
import { prefetch, trpc } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Staff | Midday",
};

export default function Staff() {
  prefetch(trpc.user.getStaff.queryOptions());

  return <TeamMembers />;
}
