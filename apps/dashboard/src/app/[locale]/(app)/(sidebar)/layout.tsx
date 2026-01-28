import { ExportStatus } from "@/components/export-status";
import { GlobalTimerProvider } from "@/components/global-timer-provider";
import { Header } from "@/components/header";
import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";
import { Sidebar } from "@/components/sidebar";
import { TimezoneDetector } from "@/components/timezone-detector";
import {
  HydrateClient,
  batchPrefetch,
  getQueryClient,
  trpc,
} from "@/trpc/server";
import { shouldShowUpgradeContent } from "@/utils/trial";
import { auth } from "@midday/auth"
import { headers } from "next/headers";
import { redirect } from "next/navigation";

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {

  // NOTE: These are used in the global sheets
  // batchPrefetch([
  //   trpc.search.global.queryOptions({ searchTerm: "" }),
  // ]);

  // NOTE: Right now we want to fetch the user and hydrate the client
  // Next steps would be to prefetch and suspense
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  if (!session?.user) {
    redirect("/login");
  }

  if (!session.user.name) {
    redirect("/setup");
  }


  const headersList = await headers();
  const pathname = headersList.get("x-pathname") || "";

  return (
    <HydrateClient>
      <div className="relative">
        <Sidebar />

        <div className="md:ml-[70px] pb-4">
          <Header />
          <div className="px-4 md:px-8">{children}</div>
        </div>

        {/* <ExportStatus /> */}
        <GlobalSheetsProvider />
        {/* <GlobalTimerProvider /> */}
        {/* <TimezoneDetector /> */}
      </div>
    </HydrateClient>
  );
}
