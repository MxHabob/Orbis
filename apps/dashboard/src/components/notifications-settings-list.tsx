import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { ErrorBoundary } from "@/components/error-boundary";
import { Suspense } from "react";
import { ErrorFallback } from "./error-fallback";
import {
  NotificationSettings,
  NotificationSettingsSkeleton,
} from "./notification-settings";

export async function NotificationsSettingsList() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Notifications</CardTitle>
        <CardDescription>
          Manage your personal notification settings for this team.
        </CardDescription>
      </CardHeader>

      <CardContent>
        <ErrorBoundary fallback={<ErrorFallback />}>
          <Suspense fallback={<NotificationSettingsSkeleton />}>
            <NotificationSettings />
          </Suspense>
        </ErrorBoundary>
      </CardContent>
    </Card>
  );
}
