import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Reports | Entity",
};


export default async function Reports() {

  return (
    <HydrateClient>
      <h2>Reports</h2>
    </HydrateClient>
  );
}
