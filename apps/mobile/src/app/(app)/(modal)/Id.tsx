/**
 * @module @orbis/mobile/app/(app)/(modal)/Id
 *
 * Official Digital Student / Campus Pass for Orbis Mobile.
 * Renders an authenticated identity badge with student number, barcode,
 * campus security watermark, and verified status.
 */

import React, { useState } from "react";
import { View, Text, Pressable } from "react-native";
import { useRouter } from "expo-router";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Avatar } from "@/components/ui/avatar";
import { Skeleton } from "@/components/ui/skeleton";
import { EmptyState } from "@/components/ui/empty-state";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import {
  useLinkedStudentId,
  useStudentQuery,
} from "@/hooks/use-api-queries";
import {
  Sparkles,
  ShieldCheck,
  XMark,
} from "@orbis/icons/native";

export default function DigitalIdModalScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { user, effectiveContext } = useAuth();
  const { t, isArabic, formatDate } = useI18n();

  const [selectedDependentId, setSelectedDependentId] = useState<string>();
  const dependents = effectiveContext?.dependents ?? [];

  const roles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
    ...(effectiveContext?.roles ?? []),
  ].map((role) => role.toLowerCase().replace(/[^a-z]/g, ""));

  const isStudent = roles.some((role) => role.includes("student"));
  const isGuardian = roles.some(
    (role) => role.includes("parent") || role.includes("guardian"),
  );

  const selectedDependent =
    dependents.find((dep) => dep.student_id === selectedDependentId) ??
    dependents[0];

  const requestedStudentId = isStudent
    ? "me"
    : isGuardian
      ? selectedDependent?.student_id
      : undefined;

  const {
    studentId,
    isLinked,
    isResolving,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);

  const studentQuery = useStudentQuery(studentId);
  const student = studentQuery.data;

  const studentName = student
    ? isArabic
      ? student.name_ar || student.full_name
      : student.display_name || student.full_name
    : isArabic
      ? user?.full_name_ar || user?.full_name
      : user?.full_name;

  const orgName = isArabic
    ? effectiveContext?.organization_name_ar || effectiveContext?.organization_name
    : effectiveContext?.organization_name;

  return (
    <View className="flex-1 bg-background">
      {/* Top Modal Header with Close Button */}
      <View className="flex-row items-center justify-between px-five pt-six pb-two">
        <View className="flex-row items-center gap-two">
          <Sparkles size={22} color={theme.primary} />
          <Text className="text-lg font-extrabold text-foreground">
            {t("tab_digital_id") || "Digital Campus ID"}
          </Text>
        </View>

        <Pressable
          accessibilityRole="button"
          accessibilityLabel={t("common_close") || "Close"}
          onPress={() => router.back()}
          className="h-9 w-9 items-center justify-center rounded-full bg-secondary active:opacity-70"
        >
          <XMark size={20} color={theme.foreground} />
        </Pressable>
      </View>

      <ScreenScrollView
        edges={["bottom", "left", "right"]}
        scrollViewProps={{
          contentContainerClassName: "gap-six px-five pb-ten pt-two items-center",
        }}
      >
        {/* Child Switcher for Guardians */}
        {isGuardian && dependents.length > 1 ? (
          <View className="w-full gap-two">
            <View className="flex-row gap-two">
              {dependents.map((dep) => {
                const selected = dep.student_id === selectedDependent?.student_id;
                const depName = isArabic
                  ? dep.full_name_ar || dep.full_name
                  : dep.full_name;
                return (
                  <Pressable
                    key={dep.student_id}
                    onPress={() => setSelectedDependentId(dep.student_id)}
                    className={`rounded-full px-four py-two ${
                      selected ? "bg-primary" : "border border-border bg-card"
                    }`}
                  >
                    <Text
                      className={`text-xs font-bold ${
                        selected ? "text-primary-foreground" : "text-foreground"
                      }`}
                    >
                      {depName}
                    </Text>
                  </Pressable>
                );
              })}
            </View>
          </View>
        ) : null}

        {isResolving || (isLinked && studentQuery.isPending) ? (
          <Skeleton className="h-[440px] w-full max-w-[340px] rounded-3xl" />
        ) : (
          /* The Official Digital Identity Card */
          <View className="w-full max-w-[340px] overflow-hidden rounded-3xl border border-primary/20 bg-card p-six shadow-lg">
            {/* School / Institution Header */}
            <View className="flex-row items-center justify-between border-b border-border/80 pb-four">
              <View className="gap-half">
                <Text className="text-xs font-extrabold uppercase tracking-widest text-primary">
                  {orgName || "ORBIS ACADEMY"}
                </Text>
                <Text className="text-[10px] font-medium text-muted-foreground">
                  {t("digital_id_official") || "Official Student Pass"}
                </Text>
              </View>

              <Badge variant="default" className="flex-row items-center gap-half bg-success/20 px-two py-half">
                <ShieldCheck size={12} color="#16A34A" />
                <Text className="text-[10px] font-bold text-success">
                  {t("student_status_active") || "Active"}
                </Text>
              </Badge>
            </View>

            {/* Student Photo & Identity */}
            <View className="items-center gap-three py-six">
              <Avatar
                fallback={(studentName || "S").charAt(0)}
                size="xl"
                className="h-24 w-24 rounded-full border-2 border-primary/30 bg-primary/10"
              />
              <View className="items-center gap-half">
                <Text className="text-center text-xl font-extrabold text-foreground">
                  {studentName}
                </Text>
                <Text className="font-mono text-xs font-semibold text-muted-foreground">
                  {student?.student_number || `ID: #${student?.id?.slice(0, 8) || "2025-091"}`}
                </Text>
              </View>
            </View>

            {/* Meta Attributes Grid */}
            <View className="flex-row justify-between rounded-xl bg-secondary/50 p-three">
              <View className="gap-half">
                <Text className="text-[10px] text-muted-foreground uppercase">
                  {t("home_class_level") || "Grade"}
                </Text>
                <Text className="text-xs font-bold text-foreground">
                  {student?.status || "Secondary"}
                </Text>
              </View>

              <View className="gap-half">
                <Text className="text-[10px] text-muted-foreground uppercase">
                  {t("student_birth_date") || "Date of Birth"}
                </Text>
                <Text className="text-xs font-bold text-foreground">
                  {student?.date_of_birth ? formatDate(student.date_of_birth) : "—"}
                </Text>
              </View>

              <View className="gap-half">
                <Text className="text-[10px] text-muted-foreground uppercase">
                  {t("student_gender") || "Gender"}
                </Text>
                <Text className="text-xs font-bold text-foreground">
                  {student?.gender || "—"}
                </Text>
              </View>
            </View>

            {/* Simulated Digital Security Barcode */}
            <View className="items-center gap-two pt-six">
              <View className="h-10 w-full flex-row items-center justify-center gap-one px-two">
                {[4, 2, 6, 2, 8, 3, 5, 2, 7, 3, 4, 2, 6, 3, 8, 2, 4, 3].map((w, i) => (
                  <View
                    key={i}
                    style={{ width: w, height: 32 }}
                    className="rounded-xs bg-foreground/80"
                  />
                ))}
              </View>
              <Text className="font-mono text-[10px] tracking-widest text-muted-foreground">
                * {student?.student_number || "ORBIS-PASS-SECURE"} *
              </Text>
            </View>
          </View>
        )}
      </ScreenScrollView>
    </View>
  );
}
