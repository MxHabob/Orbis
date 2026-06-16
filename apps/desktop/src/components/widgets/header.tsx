import { TZDate } from "@date-fns/tz";
import { authClient } from "@/lib/auth-client";
import { cn } from "@orbis/ui/cn";
import { TabsList, TabsTrigger } from "@orbis/ui/tabs";
import { useEffect, useState } from "react";
import { MetricsFilter } from "@/components/metrics/components/metrics-filter";
import { Customize } from "@/components/widgets/customize";
import { useUserQuery } from "@/hooks/use-user";
import { useI18n } from "@/locales/client";
import { useIsCustomizing } from "./widget-provider";

function getTimeBasedGreeting(t: any, timezone?: string): string {
  const userTimezone =
    timezone || Intl.DateTimeFormat().resolvedOptions().timeZone;
  const now = new TZDate(new Date(), userTimezone);
  const hour = now.getHours();

  if (hour >= 5 && hour < 12) {
    return t("widgets.morning");
  }
  if (hour >= 12 && hour < 17) {
    return t("widgets.afternoon");
  }

  return t("widgets.evening");
}

export function WidgetsHeader() {
  const { data: session } = authClient.useSession();
  const isCustomizing = useIsCustomizing();
  const t = useI18n();
  const [greeting, setGreeting] = useState(() =>
    getTimeBasedGreeting(t, undefined),
  );

  useEffect(() => {
    // Update greeting immediately
    setGreeting(getTimeBasedGreeting(t, undefined));

    // Set up interval to update greeting every 5 minutes
    const interval = setInterval(
      () => {
        const newGreeting = getTimeBasedGreeting(t, undefined);
        setGreeting(newGreeting);
      },
      5 * 60 * 1000,
    );

    return () => clearInterval(interval);
  }, [t]);

  return (
    <div className="flex justify-between items-end mb-6">
      <div>
        <h1 className="text-[30px] font-serif leading-normal mb-1">
          <span>{greeting} </span>
          <span className="text-muted-foreground">
            {session?.user?.name?.split(" ")[0]},
          </span>
        </h1>
        <p className="text-muted-foreground text-[14px]">
          {isCustomizing
            ? t("widgets.customizing_description")
            : t("widgets.normal_description")}
        </p>
      </div>

      <div className="flex items-center gap-2" data-no-close>
        <div className="hidden md:block">
          <Customize />
        </div>
        <MetricsFilter />
        <div className="ms-2 relative flex items-stretch w-fit">
          <TabsList className="flex items-stretch h-auto">
            <TabsTrigger
              value="overview"
              className="px-3 py-1.5 text-[14px] h-9 min-h-9"
            >
              {t("widgets.overview")}
            </TabsTrigger>
            <TabsTrigger
              value="calendar"
              className="px-3 py-1.5 text-[14px] h-9 min-h-9"
            >
              {t("widgets.calendar")}
            </TabsTrigger>
          </TabsList>
        </div>
      </div>
    </div>
  );
}
