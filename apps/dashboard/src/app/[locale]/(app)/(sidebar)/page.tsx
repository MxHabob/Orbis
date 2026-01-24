import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Overview | Entity",
};


export default async function Overview() {

  return (
    <HydrateClient>
      <h2>Overview</h2>
    </HydrateClient>
  );
}
