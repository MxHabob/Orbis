import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Exams | Entity",
};


export default async function Exams() {

  return (
    <HydrateClient>
      <h2>Exams</h2>
    </HydrateClient>
  );
}
