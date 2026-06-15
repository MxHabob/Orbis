import { Suspense } from "react";
import { NotificationsSettingsList } from "@/components/notifications-settings-list";

export default function NotificationsSettingsPage() {
  return (
    <Suspense>
      <NotificationsSettingsList />
    </Suspense>
  );
}
