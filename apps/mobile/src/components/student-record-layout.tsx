/**
 * @module @orbis/mobile/components/student-record-layout
 *
 * Universal layout wrapper for Student, Class, and Academic Record sub-pages.
 * Provides consistent Header with back-navigation, safe areas, pull-to-refresh,
 * loading skeleton, and error retry states.
 */

import React, { type ReactNode } from "react";
import { View, Text } from "react-native";
import { Header, HEADER_CONTENT_OFFSET } from "@/components/header";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { EmptyState } from "@/components/ui/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { Button } from "@/components/ui/button";
import { useI18n } from "@/lib/i18n";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";

interface StudentRecordLayoutProps {
  title: string;
  subtitle?: string;
  showBack?: boolean;
  onBack?: () => void;
  rightAction?: ReactNode;
  isPending: boolean;
  isError: boolean;
  canRetry?: boolean;
  errorTitle?: string;
  errorDescription?: string;
  onRetry: () => void;
  children: ReactNode;
  contentContainerClassName?: string;
}

export function StudentRecordLayout({
  title,
  subtitle,
  showBack = true,
  onBack,
  rightAction,
  isPending,
  isError,
  canRetry = true,
  errorTitle,
  errorDescription,
  onRetry,
  children,
  contentContainerClassName = "gap-five pb-[120px] px-four",
}: StudentRecordLayoutProps) {
  const { t } = useI18n();
  const router = useRouter();
  const insets = useSafeAreaInsets();

  const handleBack = onBack ?? (() => router.back());

  return (
    <View className="flex-1 bg-background">
      {showBack && (
        <Header
          title={title}
          subtitle={subtitle}
          showBack={true}
          onBack={handleBack}
          rightAction={rightAction}
        />
      )}

      <ScreenScrollView
        edges={showBack ? ["bottom", "left", "right"] : ["top", "bottom", "left", "right"]}
        contentContainerStyle={{
          paddingTop:
            (showBack ? insets.top : 0) + HEADER_CONTENT_OFFSET,
        }}
        scrollViewProps={{
          contentContainerClassName,
        }}
      >
        {isPending ? (
          <View className="gap-four pt-two">
            <Skeleton width="100%" height={112} borderRadius={6} />
            <Skeleton width="100%" height={176} borderRadius={6} />
            <Skeleton width="100%" height={128} borderRadius={6} />
          </View>
        ) : isError ? (
          <View className="flex-1 items-center justify-center py-twelve">
            <EmptyState
              title={errorTitle ?? t("common_error")}
              description={errorDescription || t("common_retry")}
              actionTitle={canRetry ? t("common_retry") : undefined}
              onAction={canRetry ? onRetry : undefined}
            />
          </View>
        ) : (
          children
        )}
      </ScreenScrollView>
    </View>
  );
}
