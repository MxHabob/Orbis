import { Widgets } from "@/components/widgets";
import { useTauriQuery } from "@/hooks/use-tauri-query";
import { commands } from "@orbis/desktop-client/commands";
import { queryKeys } from "@/lib/query-keys";
import { useAuth } from "@/hooks/use-auth";
import { Spinner } from "@orbis/ui/spinner";

export default function OverviewPage() {
  const { user } = useAuth();
  const weeklyCalendar = localStorage.getItem("weeklyCalendar") === "true";

  const { data: widgetPreferences, isLoading } = useTauriQuery(
    queryKeys.widgets.preferences(user?.id || ""),
    () => commands.widgets.getWidgetPreferences(user?.id || ""),
    { enabled: !!user?.id }
  );

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-[calc(100vh-200px)]">
        <Spinner />
      </div>
    );
  }

  return (
    <Widgets
      initialPreferences={widgetPreferences}
      weeklyCalendar={weeklyCalendar}
    />
  );
}
