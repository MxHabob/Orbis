/**
 * @module @orbis/mobile/app/(app)/notifications
 *
 * Notification center — displays the user's notification inbox with
 * category filtering, read/dismiss actions, and mark-all-read.
 */

import {
  AcademicCap,
  Bell,
  BookOpen,
  Calendar,
  CheckCircle,
  CreditCard,
  ExclamationCircle,
  SoundHigh,
  Computer,
  ShieldCheck,
  UserPlus,
  XMark,
} from "@orbis/icons/native";
import { useRouter } from "expo-router";
import { useCallback, useMemo, useState } from "react";
import {
  Pressable,
  RefreshControl,
  ScrollView,
  Text,
  View,
} from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { HEADER_CONTENT_OFFSET, Header } from "@/components/header";
import {
  useDismissNotificationMutation,
  useMarkAllReadMutation,
  useMarkNotificationReadMutation,
  useNotificationsQuery,
  useUnreadCountQuery,
} from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n";
import type { NotificationDto } from "@/lib/api/types";

// ── Category helpers ───────────────────────────────────────────

const CATEGORIES = [
  "all",
  "enrollment",
  "grades",
  "attendance",
  "finance",
  "behavior",
  "schedule",
  "approval",
  "system",
  "announcement",
] as const;

type Category = (typeof CATEGORIES)[number];

function getCategoryIcon(category: string, size: number, color: string) {
  switch (category) {
    case "enrollment":
      return <UserPlus size={size} color={color} />;
    case "grades":
      return <AcademicCap size={size} color={color} />;
    case "attendance":
      return <Calendar size={size} color={color} />;
    case "finance":
      return <CreditCard size={size} color={color} />;
    case "behavior":
      return <ShieldCheck size={size} color={color} />;
    case "schedule":
      return <BookOpen size={size} color={color} />;
    case "approval":
      return <CheckCircle size={size} color={color} />;
    case "system":
      return <Computer size={size} color={color} />;
    case "announcement":
      return <SoundHigh size={size} color={color} />;
    default:
      return <Bell size={size} color={color} />;
  }
}

const CATEGORY_LABEL_MAP: Record<Category, TranslationKey> = {
  all: "filter_all",
  enrollment: "notifications_category_enrollment",
  grades: "notifications_category_grades",
  attendance: "notifications_category_attendance",
  finance: "notifications_category_finance",
  behavior: "notifications_category_behavior",
  schedule: "notifications_category_schedule",
  approval: "notifications_category_approval",
  system: "notifications_category_system",
  announcement: "notifications_category_announcement",
};

// ── Time formatting ────────────────────────────────────────────

function formatRelativeTime(isoDate: string): string {
  const now = Date.now();
  const then = new Date(isoDate).getTime();
  const diffMs = now - then;
  const diffMin = Math.floor(diffMs / 60000);
  if (diffMin < 1) return "now";
  if (diffMin < 60) return `${diffMin}m`;
  const diffHrs = Math.floor(diffMin / 60);
  if (diffHrs < 24) return `${diffHrs}h`;
  const diffDays = Math.floor(diffHrs / 24);
  if (diffDays < 30) return `${diffDays}d`;
  return new Date(isoDate).toLocaleDateString();
}

// ── Main Screen ────────────────────────────────────────────────

export default function NotificationsScreen() {
  const { t, isArabic } = useI18n();
  const router = useRouter();
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const [activeCategory, setActiveCategory] = useState<Category>("all");

  const filter = useMemo(
    () => (activeCategory === "all" ? undefined : { category: activeCategory }),
    [activeCategory],
  );

  const {
    data: notifications,
    isLoading,
    isError,
    refetch,
    isRefetching,
  } = useNotificationsQuery(filter);
  const { data: unreadCount } = useUnreadCountQuery();

  const markRead = useMarkNotificationReadMutation();
  const markAllRead = useMarkAllReadMutation();
  const dismiss = useDismissNotificationMutation();

  const handlePress = useCallback(
    (item: NotificationDto) => {
      if (!item.is_read) {
        markRead.mutate(item.id);
      }
      if (item.action_route) {
        router.push(item.action_route as never);
      }
    },
    [markRead, router],
  );

  const handleDismiss = useCallback(
    (id: string) => {
      dismiss.mutate(id);
    },
    [dismiss],
  );

  const handleMarkAllRead = useCallback(() => {
    markAllRead.mutate(undefined as never);
  }, [markAllRead]);

  const items: NotificationDto[] = notifications ?? [];

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("home_notifications")}
        showBack
        onBack={() => router.back()}
        rightAction={
          (unreadCount ?? 0) > 0 ? (
            <Pressable
              onPress={handleMarkAllRead}
              hitSlop={8}
              accessibilityRole="button"
              accessibilityLabel={t("notifications_mark_all_read")}
              className="items-center justify-center rounded-full px-three py-one"
              style={{ backgroundColor: theme.primary + "15" }}
            >
              <Text className="text-xs font-semibold text-primary">
                {t("notifications_mark_all_read")}
              </Text>
            </Pressable>
          ) : undefined
        }
      />

      {/* Category filter chips */}
      <View style={{ paddingTop: insets.top + HEADER_CONTENT_OFFSET }}>
        <ScrollView
          horizontal
          showsHorizontalScrollIndicator={false}
          contentContainerClassName="px-four py-two gap-two"
        >
          {CATEGORIES.map((cat) => {
            const isActive = cat === activeCategory;
            return (
              <Pressable
                key={cat}
                onPress={() => setActiveCategory(cat)}
                className="items-center justify-center rounded-full px-four py-one"
                style={{
                  backgroundColor: isActive ? theme.primary : theme.secondary,
                }}
              >
                <Text
                  className="text-xs font-medium"
                  style={{
                    color: isActive ? "#fff" : theme.foreground,
                  }}
                >
                  {t(CATEGORY_LABEL_MAP[cat])}
                </Text>
              </Pressable>
            );
          })}
        </ScrollView>
      </View>

      {/* Notification list */}
      {isError ? (
        <Pressable
          onPress={() => refetch()}
          className="flex-1 items-center justify-center px-four"
        >
          <View className="mb-four h-16 w-16 items-center justify-center rounded-full bg-destructive/10">
            <ExclamationCircle size={28} color={theme.destructive} />
          </View>
          <Text className="mb-two text-center text-base font-semibold text-foreground">
            {t("notifications_error_title")}
          </Text>
          <Text className="text-center text-sm text-muted-foreground">
            {t("notifications_error_retry")}
          </Text>
        </Pressable>
      ) : (
        <ScrollView
          contentContainerClassName="flex-grow"
          refreshControl={
            <RefreshControl
              refreshing={isRefetching}
              onRefresh={refetch}
              tintColor={theme.primary}
            />
          }
        >
          {!isLoading && items.length === 0 ? (
            <View className="flex-1 items-center justify-center px-four py-sixteen">
              <View className="mb-six h-20 w-20 items-center justify-center rounded-full bg-primary/10">
                <Bell size={36} color={theme.primary} />
              </View>
              <Text className="mb-two text-center text-lg font-bold text-foreground">
                {t("notifications_empty_title")}
              </Text>
              <Text className="max-w-[300px] text-center text-sm leading-6 text-muted-foreground">
                {t("notifications_empty_description")}
              </Text>
            </View>
          ) : (
            items.map((item, index) => {
              const title =
                isArabic && item.title_ar ? item.title_ar : item.title;
              const body = isArabic && item.body_ar ? item.body_ar : item.body;

              return (
                <View key={item.id}>
                  {index > 0 && <View className="h-px bg-border mx-four" />}
                  <Pressable
                    onPress={() => handlePress(item)}
                    style={({ pressed }) => ({
                      opacity: pressed ? 0.7 : 1,
                      backgroundColor: item.is_read
                        ? "transparent"
                        : theme.primaryLight + "10",
                    })}
                    className="flex-row items-start px-four py-three"
                  >
                    {/* Unread indicator dot */}
                    <View
                      className="mr-three items-center justify-center pt-one"
                      style={{ width: 8 }}
                    >
                      {!item.is_read && (
                        <View
                          style={{
                            width: 8,
                            height: 8,
                            borderRadius: 4,
                            backgroundColor: theme.primary,
                          }}
                        />
                      )}
                    </View>

                    {/* Category icon */}
                    <View
                      className="mr-three items-center justify-center rounded-full"
                      style={{
                        width: 36,
                        height: 36,
                        backgroundColor: theme.muted,
                      }}
                    >
                      {getCategoryIcon(
                        item.category,
                        18,
                        theme.mutedForeground,
                      )}
                    </View>

                    {/* Content */}
                    <View className="flex-1 mr-two">
                      <View className="flex-row items-center justify-between mb-half">
                        <Text
                          className="flex-1 text-sm font-semibold text-foreground"
                          numberOfLines={1}
                          style={{
                            textAlign: isArabic ? "right" : "left",
                          }}
                        >
                          {title}
                        </Text>
                        <Text className="ml-two text-xs text-muted-foreground">
                          {formatRelativeTime(item.created_at)}
                        </Text>
                      </View>
                      <Text
                        className="text-sm leading-5 text-muted-foreground"
                        numberOfLines={2}
                        style={{
                          textAlign: isArabic ? "right" : "left",
                        }}
                      >
                        {body}
                      </Text>
                    </View>

                    {/* Dismiss button */}
                    <Pressable
                      onPress={() => handleDismiss(item.id)}
                      hitSlop={10}
                      className="ml-one items-center justify-center pt-one"
                      accessibilityRole="button"
                      accessibilityLabel={t("notifications_dismiss")}
                    >
                      <XMark size={14} color={theme.mutedForeground} />
                    </Pressable>
                  </Pressable>
                </View>
              );
            })
          )}
          {/* Bottom safe area spacing */}
          <View style={{ height: insets.bottom + 20 }} />
        </ScrollView>
      )}
    </View>
  );
}
