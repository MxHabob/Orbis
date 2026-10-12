import type { ReactNode } from "react";
import { EmptyState } from "@/components/ui/empty-state";
import { LoadingState } from "@/components/ui/loading-state";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { useI18n } from "@/lib/i18n";
import { Text, View } from "react-native";

interface StudentRecordLayoutProps {
  title: string;
  isPending: boolean;
  isError: boolean;
  canRetry?: boolean;
  errorDescription?: string;
  onRetry: () => void;
  children: ReactNode;
}

export function StudentRecordLayout({
  title,
  isPending,
  isError,
  canRetry = true,
  errorDescription,
  onRetry,
  children,
}: StudentRecordLayoutProps) {
  const { t } = useI18n();

  return (
    <ScreenScrollView
      horizontalPadding="five"
      verticalPadding="four"
      scrollViewProps={{ contentContainerClassName: "gap-four pb-10" }}
    >
      <View className="gap-one">
        <Text className="text-2xl font-bold text-foreground">{title}</Text>
      </View>
      {isPending ? (
        <LoadingState message={t("common_loading")} />
      ) : isError ? (
        <EmptyState
          title={t("common_error")}
          description={errorDescription}
          actionTitle={canRetry ? t("common_retry") : undefined}
          onAction={canRetry ? onRetry : undefined}
        />
      ) : (
        children
      )}
    </ScreenScrollView>
  );
}
