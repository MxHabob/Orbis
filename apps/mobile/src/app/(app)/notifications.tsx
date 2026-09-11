/**
 * @module @orbis/mobile/app/(app)/notifications
 *
 * Notifications & Academic Alerts Screen.
 *
 * - Full-width screen gradient.
 * - Gradient extends behind StatusBar.
 * - Header remains transparent.
 * - Content remains above the gradient.
 * - Theme-aware colors.
 */

import { useState } from "react";

import { Pressable, ScrollView, StyleSheet, Text, View } from "react-native";

import { Ionicons } from "@expo/vector-icons";

import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";

import { Header } from "@/components/header";

import { Card } from "@/components/ui/card";

import { ScreenBackground } from "@/components/screen-background";


import { Spacing, Typography } from "@/constants/theme";

// ─────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────

interface NotificationItem {
  id: string;

  title: string;

  body: string;

  time: string;

  type: "grade" | "attendance" | "exam" | "announcement";

  read: boolean;
}

// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function NotificationsScreen() {
  const insets = useSafeAreaInsets();

  const theme = useTheme();

  // ───────────────────────────────────────────────────────────────────────
  // State
  // ───────────────────────────────────────────────────────────────────────

  const [notifications, setNotifications] = useState<NotificationItem[]>([
    {
      id: "notif-1",

      title: "رصد درجات الاختبار النصفي",

      body: "تم رصد واعتماد نتيجة اختبار مادة الرياضيات المتقدمة (29/30).",

      time: "منذ ساعتين",

      type: "grade",

      read: false,
    },

    {
      id: "notif-2",

      title: "إشعار انضباط وحضور",

      body: "تم تسجيل حضور الطالب لجميع الحصص الدراسية اليوم بنجاح.",

      time: "منذ 5 ساعات",

      type: "attendance",

      read: false,
    },

    {
      id: "notif-3",

      title: "تحديد موعد الاختبار النهائي",

      body: "تم اعتماد جدول الاختبارات النهائية للفصل الدراسي الثاني وتعيين رقم الجلوس (418).",

      time: "أمس، 02:00 م",

      type: "exam",

      read: true,
    },

    {
      id: "notif-4",

      title: "تعميم: المعرض العلمي السنوي",

      body: "ندعوكم لحضور المعرض العلمي والابتكاري يوم الخميس القادم في الصالة الرئيسية.",

      time: "قبل 3 أيام",

      type: "announcement",

      read: true,
    },
  ]);

  // ───────────────────────────────────────────────────────────────────────
  // Actions
  // ───────────────────────────────────────────────────────────────────────

  const markAllAsRead = () => {
    setNotifications((previous) =>
      previous.map((notification) => ({
        ...notification,
        read: true,
      })),
    );
  };

  // ───────────────────────────────────────────────────────────────────────
  // Notification Type
  // ───────────────────────────────────────────────────────────────────────

  const getTypeIcon = (type: NotificationItem["type"]) => {
    switch (type) {
      case "grade":
        return {
          icon: "podium",

          color: theme.primary,

          bg: theme.primaryLight,
        };

      case "attendance":
        return {
          icon: "checkmark-circle",

          color: theme.success,

          bg: theme.successLight,
        };

      case "exam":
        return {
          icon: "document-text",

          color: theme.warning,

          bg: theme.warningLight,
        };

      case "announcement":
      default:
        return {
          icon: "megaphone",

          color: theme.accentForeground,

          bg: theme.accentLight,
        };
    }
  };

  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <ScreenBackground gradientHeight={0.48}>
      {/* ─────────────────────────────────────────────────────────────
          Header

          Header is transparent.
          It does NOT render its own gradient.
        ───────────────────────────────────────────────────────────── */}

      <Header
        title="التنبيهات والإشعارات"
        showBack
        rightAction={
          <Pressable onPress={markAllAsRead} hitSlop={8}>
            <Text
              style={[
                styles.markReadText,
                {
                  color: theme.primary,
                },
              ]}
            >
              قراءة الكل
            </Text>
          </Pressable>
        }
      />

      {/* ─────────────────────────────────────────────────────────────
          Notifications Content
        ───────────────────────────────────────────────────────────── */}

      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={[
          styles.scrollContent,

          {
            paddingBottom: insets.bottom + Spacing.eight,
          },
        ]}
        showsVerticalScrollIndicator={false}
      >
        <View style={styles.notifList}>
          {notifications.map((item) => {
            const { icon, color, bg } = getTypeIcon(item.type);

            return (
              <Card
                key={item.id}
                variant={item.read ? "flat" : "elevated"}
                style={[
                  styles.notifCard,

                  !item.read && {
                    borderColor: theme.primary,

                    borderWidth: 1.2,
                  },
                ]}
              >
                <View style={styles.notifRow}>
                  {/* ─────────────────────────────────────────
                        Notification Icon
                      ───────────────────────────────────────── */}

                  <View
                    style={[
                      styles.iconBox,

                      {
                        backgroundColor: bg,
                      },
                    ]}
                  >
                    <Ionicons name={icon as any} size={22} color={color} />
                  </View>

                  {/* ─────────────────────────────────────────
                        Notification Content
                      ───────────────────────────────────────── */}

                  <View style={styles.notifContent}>
                    <View style={styles.titleRow}>
                      <Text
                        style={[
                          styles.notifTitle,

                          {
                            color: theme.text,
                          },
                        ]}
                      >
                        {item.title}
                      </Text>

                      {!item.read && (
                        <View
                          style={[
                            styles.dot,

                            {
                              backgroundColor: theme.primary,
                            },
                          ]}
                        />
                      )}
                    </View>

                    <Text
                      style={[
                        styles.notifBody,

                        {
                          color: theme.textSecondary,
                        },
                      ]}
                    >
                      {item.body}
                    </Text>

                    <Text
                      style={[
                        styles.notifTime,

                        {
                          color: theme.textMuted,
                        },
                      ]}
                    >
                      {item.time}
                    </Text>
                  </View>
                </View>
              </Card>
            );
          })}
        </View>
      </ScrollView>
    </ScreenBackground>
  );
}

// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  // ───────────────────────────────────────────────────────────────────────
  // ScrollView
  // ───────────────────────────────────────────────────────────────────────

  scrollView: {
    flex: 1,

    width: "100%",
  },

  scrollContent: {
    paddingHorizontal: Spacing.four,

    paddingTop: Spacing.four,

    gap: Spacing.three,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Notification List
  // ───────────────────────────────────────────────────────────────────────

  notifList: {
    width: "100%",

    gap: Spacing.three,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Notification Card
  // ───────────────────────────────────────────────────────────────────────

  notifCard: {
    width: "100%",

    padding: Spacing.four,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Notification Row
  // ───────────────────────────────────────────────────────────────────────

  notifRow: {
    width: "100%",

    flexDirection: "row",

    alignItems: "flex-start",

    gap: Spacing.three,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Icon
  // ───────────────────────────────────────────────────────────────────────

  iconBox: {
    width: 44,

    height: 44,

    borderRadius: 22,

    alignItems: "center",

    justifyContent: "center",

    flexShrink: 0,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Content
  // ───────────────────────────────────────────────────────────────────────

  notifContent: {
    flex: 1,

    minWidth: 0,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Title
  // ───────────────────────────────────────────────────────────────────────

  titleRow: {
    flexDirection: "row",

    alignItems: "center",

    justifyContent: "space-between",

    gap: Spacing.two,
  },

  notifTitle: {
    ...Typography.bodyBold,

    flex: 1,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Unread Indicator
  // ───────────────────────────────────────────────────────────────────────

  dot: {
    width: 8,

    height: 8,

    borderRadius: 4,

    flexShrink: 0,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Body
  // ───────────────────────────────────────────────────────────────────────

  notifBody: {
    ...Typography.body,

    marginTop: 4,

    lineHeight: 20,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Time
  // ───────────────────────────────────────────────────────────────────────

  notifTime: {
    ...Typography.small,

    marginTop: 6,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Mark All Read
  // ───────────────────────────────────────────────────────────────────────

  markReadText: {
    ...Typography.captionBold,
  },
});
