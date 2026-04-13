import { GlobalSheetsProvider } from "@/components/sheets/global-sheets-provider";
import {
  HydrateClient,
} from "@/trpc/server";
import { auth } from "@midday/auth"
import { headers } from "next/headers";
import { redirect } from "next/navigation";

export default async function EditorLayout({
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

  return (
    <HydrateClient>
      <div className="fixed inset-0 overflow-hidden bg-background">
        {children}
        <GlobalSheetsProvider />
      </div>
    </HydrateClient>
  );
}
