import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Fees & Payments | Entity",
};


export default async function Fees() {

  return (
    <HydrateClient>
      <h2>Fees & Payments</h2>
    </HydrateClient>
  );
}
