/**
 * @module @orbis/mobile/app/(app)/student/[id]/attendance
 *
 * Detailed Attendance History & Record Logs for Students.
 * Features attendance rate statistics, filter chips, and status badges.
 */

import React, { useState } from "react";
import { View, Text } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Chip } from "@/components/ui/chip";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentAttendanceQuery,
} from "@/hooks/use-api-queries";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import {
  ClipboardDocumentCheck,
  CheckCircle,
  ExclamationCircle,
  Clock,
  Sparkles,
} from "@orbis/icons/native";

const attendanceStatusKeys: Record<string, TranslationKey> = {
  present: "student_attendance_present",
  absent: "student_attendance_absent",
  late: "student_attendance_late",
  excused: "student_attendance_excused",
};

export default function StudentAttendanceScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const theme = useTheme();
  const { t, formatDate } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityError,
    identityMissing,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);

  const attendanceQuery = useStudentAttendanceQuery(studentId);
  const [filterStatus, setFilterStatus] = useState<string>("all");

  const rawRecords = attendanceQuery.data ?? [];
  const records = [...rawRecords].sort((a, b) => b.date.localeCompare(a.date));

  const presentCount = records.filter((r) => r.status.toLowerCase() === "present").length;
  const absentCount = records.filter((r) => r.status.toLowerCase() === "absent").length;
  const lateCount = records.filter((r) => r.status.toLowerCase() === "late").length;
  const attendancePercentage = records.length > 0 ? Math.round((presentCount / records.length) * 100) : 100;

  const filteredRecords = records.filter((r) => {
    if (filterStatus === "all") return true;
    return r.status.toLowerCase() === filterStatus;
  });

  return (
    <StudentRecordLayout
      title={t("student_attendance") || "Attendance"}
      showBack={true}
      isPending={isResolving || (isLinked && attendanceQuery.isPending)}
      isError={!isResolving && (!isLinked || attendanceQuery.isError)}
      canRetry={isLinked || identityError}
      errorTitle={identityMissing ? t("student_profile_missing") : undefined}
      errorDescription={
        isLinked || isResolving || identityMissing
          ? undefined
          : identityError
            ? t("home_student_load_error")
            : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? attendanceQuery.refetch() : retryIdentity())
      }
    >
      {/* 1. Overall Attendance Performance Widget */}
      <Card
        variant="default"
        padding="four"
      >
        <View className="flex-row items-center justify-between pb-three">
          <View className="gap-half">
            <Text className="text-xs font-semibold text-muted-foreground">
              {t("attendance_rate_label") || "Attendance Rate"}
            </Text>
            <Text className="text-3xl font-extrabold text-foreground">
              {formatNumber(attendancePercentage)}%
            </Text>
          </View>

          <View className="h-12 w-12 items-center justify-center rounded-md bg-primary/10">
            <ClipboardDocumentCheck size={28} color={theme.primary} />
          </View>
        </View>

        {/* Breakdown Counts */}
        <View className="flex-row items-center justify-between border-t border-border/60 pt-three">
          <View className="flex-row items-center gap-two">
            <View className="h-7 w-7 items-center justify-center rounded-full bg-success/15">
              <CheckCircle size={15} color={theme.success} />
            </View>
            <View>
              <Text className="text-xs font-bold text-foreground">{formatNumber(presentCount)}</Text>
              <Text className="text-[10px] text-muted-foreground">{t("student_attendance_present")}</Text>
            </View>
          </View>

          <View className="flex-row items-center gap-two">
            <View className="h-7 w-7 items-center justify-center rounded-full bg-destructive/15">
              <ExclamationCircle size={15} color={theme.destructive} />
            </View>
            <View>
              <Text className="text-xs font-bold text-foreground">{formatNumber(absentCount)}</Text>
              <Text className="text-[10px] text-muted-foreground">{t("student_attendance_absent")}</Text>
            </View>
          </View>

          <View className="flex-row items-center gap-two">
            <View className="h-7 w-7 items-center justify-center rounded-full bg-warning/15">
              <Clock size={15} color={theme.warning} />
            </View>
            <View>
              <Text className="text-xs font-bold text-foreground">{formatNumber(lateCount)}</Text>
              <Text className="text-[10px] text-muted-foreground">{t("student_attendance_late")}</Text>
            </View>
          </View>
        </View>
      </Card>

      {/* 2. Filter Chips */}
      <View className="flex-row items-center gap-two pt-one">
        <Chip
          label={t("filter_all") || "All"}
          selected={filterStatus === "all"}
          onPress={() => setFilterStatus("all")}
        />
        <Chip
          label={t("student_attendance_present")}
          selected={filterStatus === "present"}
          onPress={() => setFilterStatus("present")}
        />
        <Chip
          label={t("student_attendance_absent")}
          selected={filterStatus === "absent"}
          onPress={() => setFilterStatus("absent")}
        />
        <Chip
          label={t("student_attendance_late")}
          selected={filterStatus === "late"}
          onPress={() => setFilterStatus("late")}
        />
      </View>

      {/* 3. Detailed Attendance Records */}
      {filteredRecords.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("student_no_attendance")}
            icon={<ClipboardDocumentCheck size={32} className="text-muted-foreground" />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {filteredRecords.map((record) => {
            const status = record.status.toLowerCase();
            const statusKey = attendanceStatusKeys[status];
            const isAbsent = status === "absent";
            const isLate = status === "late";

            return (
              <Card
                key={record.id}
                variant="outline"
                padding="four"
              >
                <View className="flex-row items-center justify-between pb-two">
                  <Text className="text-sm font-bold text-foreground">
                    {formatDate(record.date)}
                  </Text>

                  <Badge
                    variant={isAbsent ? "destructive" : isLate ? "secondary" : "default"}
                    className={`px-two py-half ${
                      !isAbsent && !isLate ? "bg-success/20" : ""
                    }`}
                  >
                    <Text
                      className={`text-[10px] font-bold ${
                        isAbsent
                          ? "text-destructive-foreground"
                          : isLate
                            ? "text-warning"
                            : "text-success"
                      }`}
                    >
                      {statusKey ? t(statusKey) : record.status}
                    </Text>
                  </Badge>
                </View>

                {record.notes ? (
                  <Text className="mt-one text-xs text-muted-foreground">
                    {record.notes}
                  </Text>
                ) : null}
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
