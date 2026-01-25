import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Subjects | Entity",
};


export default async function Subjects() {

  return (
    <HydrateClient>
      <h2>Subjects</h2>
    </HydrateClient>
  );
}
