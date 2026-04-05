import { Tabs, TabsContent, TabsList, TabsTrigger } from "@midday/ui/tabs";
import { Suspense } from "react";
import { DataTable as MembersTable } from "./tables/staff";
import { DataTable as ParentsTable } from "./tables/parents";
import { PendingInvitesSkeleton } from "./tables/parents/skeleton";

export function StaffUsers() {
  return (
    <Tabs defaultValue="users">
      <TabsList className="bg-transparent border-b-[1px] w-full justify-start rounded-none mb-1 p-0 h-auto pb-4">
        <TabsTrigger value="users" className="p-0 m-0 mr-4">
          Staff & Teachers
        </TabsTrigger>
        <TabsTrigger value="parents" className="p-0 m-0">
          Parents
        </TabsTrigger>
      </TabsList>

      <TabsContent value="users">
        <Suspense fallback={<PendingInvitesSkeleton />}>
          <MembersTable />
        </Suspense>
      </TabsContent>

      <TabsContent value="parents">
        <Suspense fallback={<PendingInvitesSkeleton />}>
          <ParentsTable />
        </Suspense>
      </TabsContent>
    </Tabs>
  );
}
