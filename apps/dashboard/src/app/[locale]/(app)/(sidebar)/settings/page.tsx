import { SchoolAcademicYear } from "@/components/company-fiscal-year";
import { CompanyLogo } from "@/components/company-logo";
import { CompanyName } from "@/components/company-name";
import { DeleteTeam } from "@/components/delete-team";
import { HydrateClient } from "@/trpc/server";
import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Settings | Entity",
};

export default async function Account() {

  return (
    <HydrateClient>
      <div className="space-y-12">
        <CompanyLogo />
        <CompanyName />
        <SchoolAcademicYear />
        <DeleteTeam />
      </div>
    </HydrateClient>
  );
}
