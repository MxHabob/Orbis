import { EmptyState } from "@/components/ui/empty-state";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { useI18n } from "@/lib/i18n";

export default function NotificationsScreen() {
  const { t } = useI18n();

  return (
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="six"
      scrollViewProps={{ contentContainerClassName: "flex-grow justify-center" }}
    >
      <EmptyState
        title={t("notifications_inbox_unavailable_title")}
        description={t("notifications_inbox_unavailable_description")}
      />
    </ScreenScrollView>
  );
}
