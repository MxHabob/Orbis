import { auth } from "@orbis/auth";

import {  } from "react-router-dom";
import { ExportStatus } from "@/components/export-status";
import { Header } from "@/components/header";
import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";
import { Sidebar } from "@/components/sidebar";
import { HydrateClient, trpc } from "@/trpc/server";

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
        <div className="md:ms-[70px] pb-4">
          <Header />
          <div className="px-4 md:px-8">{children}</div>
        </div>
        <ExportStatus />
        <GlobalSheetsProvider />
      </div>
    </HydrateClient>
  );
}
