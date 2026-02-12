import { ExportStatus } from "@/components/export-status";
import { Header } from "@/components/header";
import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";
import { Sidebar } from "@/components/sidebar";
import {
  HydrateClient,
  trpc,
} from "@/trpc/server";
import { auth } from "@midday/auth"
import { headers } from "next/headers";
import { redirect } from "next/navigation";

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {

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

        <ExportStatus />
        <GlobalSheetsProvider />
      </div>
    </HydrateClient>
  );
}
