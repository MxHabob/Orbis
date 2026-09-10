/**
 * @module @orbis/mobile/app/(app)/student/[id]/attendance
 *
 * Student Attendance & Absence History Screen.
 */

import React, { useState } from "react";
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  RefreshControl,
} from "react-native";

import { useLocalSearchParams } from "expo-router";
import { Ionicons } from "@expo/vector-icons";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";
import { Header } from "@/components/header";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { useToast } from "@/components/ui/toast";

import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
} from "@/components/ui/drop-down-menu";

import { ScreenBackground } from "@/components/screen-background";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";

import { Radii, Spacing, Typography } from "@/constants/theme";

// ─────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────

interface AttendanceLog {
  date: string;
  dayName: string;
  status: "Present" | "Late" | "Excused" | "Absent";
  periodOrSubject: string;
  notes?: string;
}

// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function StudentAttendanceScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();

  const { id } = useLocalSearchParams<{
    id: string;
  }>();

  const [refreshing, setRefreshing] = useState(false);

  const [statusFilter, setStatusFilter] = useState<
    "ALL" | AttendanceLog["status"]
  >("ALL");

  // ───────────────────────────────────────────────────────────────────────
  // Attendance Data
  // ───────────────────────────────────────────────────────────────────────

  const logs: AttendanceLog[] = [
    {
      date: "7 سبتمبر 2026",
      dayName: "اليوم",
      status: "Present",
      periodOrSubject: "جميع الحصص (اليوم كامل)",
    },

    {
      date: "6 سبتمبر 2026",
      dayName: "أمس",
      status: "Present",
      periodOrSubject: "جميع الحصص (اليوم كامل)",
    },

    {
      date: "3 سبتمبر 2026",
      dayName: "الأربعاء",
      status: "Late",
      periodOrSubject: "الحصة الأولى - الرياضيات المتقدمة",
      notes: "تأخر 10 دقائق بسبب الازدحام المروري",
    },

    {
      date: "27 أغسطس 2026",
      dayName: "الخميس",
      status: "Excused",
      periodOrSubject: "اليوم كامل",
      notes: "إجازة مرضية معتمدة من منصة صحتي",
    },

    {
      date: "26 أغسطس 2026",
      dayName: "الأربعاء",
      status: "Present",
      periodOrSubject: "جميع الحصص (اليوم كامل)",
    },
  ];

  // ───────────────────────────────────────────────────────────────────────
  // Refresh
  // ───────────────────────────────────────────────────────────────────────

  const onRefresh = () => {
    setRefreshing(true);

    setTimeout(() => {
      setRefreshing(false);
    }, 500);
  };

  // ───────────────────────────────────────────────────────────────────────
  // Status Badge
  // ───────────────────────────────────────────────────────────────────────

  const getStatusBadge = (status: AttendanceLog["status"]) => {
    switch (status) {
      case "Present":
        return <Badge label="حاضر" variant="success" size="md" />;

      case "Late":
        return <Badge label="متأخر" variant="warning" size="md" />;

      case "Excused":
        return <Badge label="غياب بعذر" variant="info" size="md" />;

      case "Absent":
        return <Badge label="غياب غير مبرر" variant="error" size="md" />;
    }
  };

  // ───────────────────────────────────────────────────────────────────────
  // Filter
  // ───────────────────────────────────────────────────────────────────────

  const handleFilterSelect = (filter: "ALL" | AttendanceLog["status"]) => {
    setStatusFilter(filter);

    const filterLabel =
      filter === "ALL"
        ? "عرض جميع السجلات"
        : filter === "Present"
          ? "تصفية: حاضر"
          : filter === "Late"
            ? "تصفية: متأخر"
            : filter === "Excused"
              ? "تصفية: غياب بعذر"
              : "تصفية: غير مبرر";

    toast.info(filterLabel);
  };

  const filteredLogs =
    statusFilter === "ALL"
      ? logs
      : logs.filter((log) => log.status === statusFilter);

  // ───────────────────────────────────────────────────────────────────────
  // Filter Label
  // ───────────────────────────────────────────────────────────────────────

  const currentFilterLabel =
    statusFilter === "ALL"
      ? "الكل"
      : statusFilter === "Present"
        ? "حاضر"
        : statusFilter === "Late"
          ? "متأخر"
          : statusFilter === "Excused"
            ? "بعذر"
            : "غير مبرر";

  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <ScreenBackground gradientHeight={0.48}>
      {/* ─────────────────────────────────────────────────────────────
          Status Bar
        ───────────────────────────────────────────────────────────── */}

      <ThemedStatusBar />

      {/* ─────────────────────────────────────────────────────────────
          Header
        ───────────────────────────────────────────────────────────── */}

      <Header title="سجل الحضور والمواظبة" showBack />

      {/* ─────────────────────────────────────────────────────────────
          Scroll Content
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
        refreshControl={
          <RefreshControl
            refreshing={refreshing}
            onRefresh={onRefresh}
            tintColor={theme.primary}
          />
        }
      >
        {/* ─────────────────────────────────────────────────────────
            Attendance Summary
          ───────────────────────────────────────────────────────── */}

        <Card variant="elevated" style={styles.summaryCard}>
          <View style={styles.summaryRow}>
            <View style={styles.summaryCol}>
              <Text
                style={[
                  styles.summaryLabel,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                نسبة الالتزام بالحضور
              </Text>

              <Text
                style={[
                  styles.summaryRate,
                  {
                    color: theme.success,
                  },
                ]}
              >
                98.5%
              </Text>

              <Text
                style={[
                  styles.summarySub,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                حالة الحضور: ممتازة جداً
              </Text>
            </View>

            <View
              style={[
                styles.iconBox,
                {
                  backgroundColor: theme.successLight,
                },
              ]}
            >
              <Ionicons
                name="checkmark-circle"
                size={40}
                color={theme.success}
              />
            </View>
          </View>

          {/* ───────────────────────────────────────────────────────
              Statistics
            ─────────────────────────────────────────────────────── */}

          <View
            style={[
              styles.statsRow,
              {
                borderTopColor: theme.borderSubtle,
              },
            ]}
          >
            <View style={styles.statItem}>
              <Text
                style={[
                  styles.statVal,
                  {
                    color: theme.text,
                  },
                ]}
              >
                65
              </Text>

              <Text
                style={[
                  styles.statLbl,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                أيام حضور
              </Text>
            </View>

            <View style={styles.statItem}>
              <Text
                style={[
                  styles.statVal,
                  {
                    color: theme.info,
                  },
                ]}
              >
                1
              </Text>

              <Text
                style={[
                  styles.statLbl,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                غياب بعذر
              </Text>
            </View>

            <View style={styles.statItem}>
              <Text
                style={[
                  styles.statVal,
                  {
                    color: theme.warning,
                  },
                ]}
              >
                2
              </Text>

              <Text
                style={[
                  styles.statLbl,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                تأخر
              </Text>
            </View>

            <View style={styles.statItem}>
              <Text
                style={[
                  styles.statVal,
                  {
                    color: theme.error,
                  },
                ]}
              >
                0
              </Text>

              <Text
                style={[
                  styles.statLbl,
                  {
                    color: theme.textSecondary,
                  },
                ]}
              >
                غير مبرر
              </Text>
            </View>
          </View>
        </Card>

        {/* ─────────────────────────────────────────────────────────
            Section Header
          ───────────────────────────────────────────────────────── */}

        <View style={styles.sectionHeader}>
          <Text
            style={[
              styles.sectionTitle,
              {
                color: theme.text,
              },
            ]}
          >
            سجل الأيام والحصص
          </Text>

          <DropdownMenu itemCount={5} menuWidth={190}>
            <DropdownMenuTrigger>
              <View
                style={[
                  styles.filterTrigger,
                  {
                    backgroundColor: theme.card,

                    borderColor: theme.border,
                  },
                ]}
              >
                <Ionicons
                  name="filter-outline"
                  size={15}
                  color={theme.primary}
                />

                <Text
                  style={[
                    styles.filterTriggerText,
                    {
                      color: theme.text,
                    },
                  ]}
                >
                  {currentFilterLabel}
                </Text>

                <Ionicons
                  name="chevron-down"
                  size={14}
                  color={theme.textSecondary}
                />
              </View>
            </DropdownMenuTrigger>

            <DropdownMenuContent>
              <DropdownMenuLabel>تصفية الحالات</DropdownMenuLabel>

              <DropdownMenuItem
                selected={statusFilter === "ALL"}
                onSelect={() => handleFilterSelect("ALL")}
              >
                جميع الحالات
              </DropdownMenuItem>

              <DropdownMenuItem
                selected={statusFilter === "Present"}
                onSelect={() => handleFilterSelect("Present")}
              >
                حاضر
              </DropdownMenuItem>

              <DropdownMenuItem
                selected={statusFilter === "Late"}
                onSelect={() => handleFilterSelect("Late")}
              >
                متأخر
              </DropdownMenuItem>

              <DropdownMenuItem
                selected={statusFilter === "Excused"}
                onSelect={() => handleFilterSelect("Excused")}
              >
                غياب بعذر
              </DropdownMenuItem>

              <DropdownMenuItem
                selected={statusFilter === "Absent"}
                onSelect={() => handleFilterSelect("Absent")}
              >
                غياب غير مبرر
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </View>

        {/* ─────────────────────────────────────────────────────────
            Attendance Timeline
          ───────────────────────────────────────────────────────── */}

        <View style={styles.logsList}>
          {filteredLogs.map((log, index) => (
            <Card
              key={`${log.date}-${index}`}
              variant="flat"
              style={styles.logCard}
            >
              <View style={styles.logHeader}>
                <View style={styles.logInfo}>
                  <Text
                    style={[
                      styles.logDate,
                      {
                        color: theme.text,
                      },
                    ]}
                  >
                    {log.date}
                    {" ("}
                    {log.dayName}
                    {")"}
                  </Text>

                  <Text
                    style={[
                      styles.logSubject,
                      {
                        color: theme.textSecondary,
                      },
                    ]}
                  >
                    {log.periodOrSubject}
                  </Text>
                </View>

                {getStatusBadge(log.status)}
              </View>

              {/* ─────────────────────────────────────────────────
                    Notes
                  ───────────────────────────────────────────────── */}

              {log.notes ? (
                <View
                  style={[
                    styles.notesBox,
                    {
                      backgroundColor: theme.card,
                    },
                  ]}
                >
                  <Ionicons
                    name="information-circle-outline"
                    size={16}
                    color={theme.textSecondary}
                  />

                  <Text
                    style={[
                      styles.notesText,
                      {
                        color: theme.textSecondary,
                      },
                    ]}
                  >
                    {log.notes}
                  </Text>
                </View>
              ) : null}
            </Card>
          ))}
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
  // Scroll
  // ───────────────────────────────────────────────────────────────────────

  scrollView: {
    flex: 1,
    width: "100%",
  },

  scrollContent: {
    width: "100%",
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.four,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Summary
  // ───────────────────────────────────────────────────────────────────────

  summaryCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },

  summaryRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
  },

  summaryCol: {
    flex: 1,
    minWidth: 0,
  },

  summaryLabel: {
    ...Typography.caption,
  },

  summaryRate: {
    ...Typography.h1,
    marginTop: 2,
  },

  summarySub: {
    ...Typography.small,
    marginTop: 4,
  },

  iconBox: {
    width: 64,
    height: 64,
    borderRadius: Radii.full,
    alignItems: "center",
    justifyContent: "center",
  },

  // ───────────────────────────────────────────────────────────────────────
  // Statistics
  // ───────────────────────────────────────────────────────────────────────

  statsRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    borderTopWidth: 1,
    paddingTop: Spacing.three,
  },

  statItem: {
    flex: 1,
    alignItems: "center",
  },

  statVal: {
    ...Typography.bodyBold,
  },

  statLbl: {
    ...Typography.small,
    marginTop: 2,
    textAlign: "center",
  },

  // ───────────────────────────────────────────────────────────────────────
  // Section
  // ───────────────────────────────────────────────────────────────────────

  sectionHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginTop: Spacing.two,
    gap: Spacing.two,
  },

  sectionTitle: {
    ...Typography.h3,
    flex: 1,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Filter
  // ───────────────────────────────────────────────────────────────────────

  filterTrigger: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    paddingVertical: 5,
    paddingHorizontal: 10,
    borderRadius: Radii.full,
    borderWidth: 1,
  },

  filterTriggerText: {
    ...Typography.small,
    fontWeight: "700",
  },

  // ───────────────────────────────────────────────────────────────────────
  // Logs
  // ───────────────────────────────────────────────────────────────────────

  logsList: {
    gap: Spacing.two,
  },

  logCard: {
    padding: Spacing.four,
    gap: Spacing.two,
  },

  logHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    gap: Spacing.three,
  },

  logInfo: {
    flex: 1,
    minWidth: 0,
  },

  logDate: {
    ...Typography.bodyBold,
  },

  logSubject: {
    ...Typography.small,
    marginTop: 2,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Notes
  // ───────────────────────────────────────────────────────────────────────

  notesBox: {
    flexDirection: "row",
    alignItems: "center",
    padding: Spacing.two,
    borderRadius: Radii.sm,
    gap: 6,
  },

  notesText: {
    ...Typography.small,
    flex: 1,
  },
});
