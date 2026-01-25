import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Grades | Entity",
};


export default async function Grades() {

  return (
    <HydrateClient>
      <h2>Grades</h2>
    </HydrateClient>
  );
}
