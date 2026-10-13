/**
 * @module @orbis/mobile/app/(app)/(tabs)/behavior
 *
 * Behavior AI & Conduct Analytics Hub for Orbis Mobile.
 * Shows net conduct score, positive merit vs violation breakdown,
 * filterable timeline of incidents, and parent dependent switching.
 */

import React, { useState } from "react";
import { View, Text, Pressable } from "react-native";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Chip } from "@/components/ui/chip";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useLinkedStudentId,
  useStudentBehaviorQuery,
} from "@/hooks/use-api-queries";
import { useAuth } from "@/lib/api";
import { useI18n, formatNumber } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import {
  Sparkles,
  CheckCircle,
  ExclamationCircle,
  ShieldCheck,
  User,
} from "@orbis/icons/native";

const behaviorTypeKeys: Record<string, TranslationKey> = {
  merit: "behavior_record_merit",
  violation: "behavior_record_violation",
};

export default function BehaviorTabScreen() {
  const { user, effectiveContext } = useAuth();
  const theme = useTheme();
  const { t, isArabic, formatDate } = useI18n();

  const [selectedDependentId, setSelectedDependentId] = useState<string>();
  const [filterType, setFilterType] = useState<"all" | "merit" | "violation">("all");

  const dependents = effectiveContext?.dependents ?? [];
  const roles = [
    ...(user?.roles ?? []),
    effectiveContext?.effective_role ?? "",
    ...(effectiveContext?.roles ?? []),
  ].map((role) => role.toLowerCase().replace(/[^a-z]/g, ""));

  const isStudent = roles.some((role) => role.includes("student"));
  const isGuardian = roles.some(
    (role) =>
      role.includes("parent") ||
      role.includes("guardian") ||
      role.includes("family"),
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
    identityError,
    identityMissing,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);

  const behaviorQuery = useStudentBehaviorQuery(studentId);

  const rawRecords = behaviorQuery.data ?? [];
  const records = [...rawRecords].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );

  const filteredRecords = records.filter((rec) => {
    if (filterType === "all") return true;
    return rec.record_type.toLowerCase() === filterType;
  });

  const netPoints = records.reduce((total, rec) => total + rec.points, 0);
  const meritsCount = records.filter((r) => r.points > 0).length;
  const violationsCount = records.filter((r) => r.points < 0).length;

  const hasStudentContext = isStudent || isGuardian;
  const hasSelectedStudent = isStudent || Boolean(selectedDependent);
  const missingDependent = isGuardian && dependents.length === 0;
  const missingStudentProfile = isStudent && identityMissing;

  return (
    <StudentRecordLayout
      title={t("behavior_title")}
      showBack={false}
      contentContainerClassName="gap-five px-four pb-[130px]"
      isPending={
        hasSelectedStudent &&
        (isResolving || (isLinked && behaviorQuery.isPending))
      }
      isError={
        !hasStudentContext ||
        missingDependent ||
        missingStudentProfile ||
        (hasSelectedStudent && !isResolving && !isLinked) ||
        behaviorQuery.isError
      }
      canRetry={
        !missingDependent &&
        !missingStudentProfile &&
        (isLinked || identityError || behaviorQuery.isError)
      }
      errorTitle={
        missingDependent
          ? t("home_no_dependents")
          : missingStudentProfile
            ? t("student_profile_missing")
            : undefined
      }
      errorDescription={
        missingDependent
          ? undefined
          : missingStudentProfile
            ? undefined
            : hasStudentContext
          ? isLinked
            ? undefined
            : identityError
              ? t("home_student_load_error")
              : t("student_access_denied")
          : t("behavior_student_context_required")
      }
      onRetry={() =>
        void (isLinked ? behaviorQuery.refetch() : retryIdentity())
      }
    >
      {/* 1. Guardian Dependent Switcher */}
      {isGuardian && dependents.length > 1 ? (
        <View className="gap-two">
          <Text className="text-xs font-bold text-muted-foreground uppercase tracking-wider">
            {t("home_dependents")}
          </Text>
          <View className="flex-row gap-two">
            {dependents.map((dep) => {
              const selected = dep.student_id === selectedDependent?.student_id;
              const depName = isArabic
                ? dep.full_name_ar || dep.full_name
                : dep.full_name;
              return (
                <Pressable
                  key={dep.student_id}
                  accessibilityRole="button"
                  accessibilityState={{ selected }}
                  onPress={() => setSelectedDependentId(dep.student_id)}
                  className={`rounded-full px-four py-two ${
                    selected
                      ? "bg-primary"
                      : "border border-border bg-card"
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

      {/* 2. Conduct Score & Analytics Banner */}
      <Card
        variant="default"
        padding="four"
      >
        <View className="flex-row items-center justify-between pb-three">
          <View className="gap-half">
            <Text className="text-xs font-semibold text-muted-foreground">
              {t("behavior_score_net")}
            </Text>
            <View className="flex-row items-baseline gap-two">
              <Text
                className={`text-3xl font-extrabold ${
                  netPoints >= 0 ? "text-primary" : "text-destructive"
                }`}
              >
                {netPoints > 0 ? `+${formatNumber(netPoints)}` : formatNumber(netPoints)}
              </Text>
              <Text className="text-xs font-medium text-muted-foreground">
                {t("behavior_points_label") || "Points"}
              </Text>
            </View>
          </View>

          <View className="h-12 w-12 items-center justify-center rounded-md bg-primary/10">
            <ShieldCheck size={28} color={theme.primary} />
          </View>
        </View>

        {/* Breakdown Stats */}
        <View className="flex-row items-center justify-between border-t border-border/60 pt-three">
          <View className="flex-row items-center gap-two">
            <View className="h-7 w-7 items-center justify-center rounded-full bg-success/15">
              <CheckCircle size={16} color={theme.success} />
            </View>
            <View>
              <Text className="text-xs font-bold text-foreground">
                {formatNumber(meritsCount)}
              </Text>
              <Text className="text-[10px] text-muted-foreground">
                {t("behavior_record_merit")}
              </Text>
            </View>
          </View>

          <View className="flex-row items-center gap-two">
            <View className="h-7 w-7 items-center justify-center rounded-full bg-destructive/15">
              <ExclamationCircle size={16} color={theme.destructive} />
            </View>
            <View>
              <Text className="text-xs font-bold text-foreground">
                {formatNumber(violationsCount)}
              </Text>
              <Text className="text-[10px] text-muted-foreground">
                {t("behavior_record_violation")}
              </Text>
            </View>
          </View>
        </View>
      </Card>

      {/* 3. Filter Chips */}
      <View className="flex-row items-center gap-two pt-one">
        <Chip
          label={t("filter_all") || "All"}
          selected={filterType === "all"}
          onPress={() => setFilterType("all")}
        />
        <Chip
          label={t("behavior_record_merit")}
          selected={filterType === "merit"}
          onPress={() => setFilterType("merit")}
        />
        <Chip
          label={t("behavior_record_violation")}
          selected={filterType === "violation"}
          onPress={() => setFilterType("violation")}
        />
      </View>

      {/* 4. Timeline Records */}
      {filteredRecords.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("behavior_empty_title")}
            description={t("behavior_empty_desc")}
            icon={<Sparkles size={32} className="text-muted-foreground" />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {filteredRecords.map((record) => {
            const isPositive = record.points >= 0;
            const recordType = record.record_type.toLowerCase();
            const typeKey = behaviorTypeKeys[recordType];
            return (
              <Card
                key={record.id}
                variant="outline"
                padding="four"
              >
                <View className="flex-row items-center justify-between pb-two">
                  <View className="flex-row items-center gap-two">
                    <Badge
                      variant={isPositive ? "default" : "destructive"}
                      className="px-two py-half"
                    >
                      <Text
                        className={`text-xs font-bold ${
                          isPositive
                            ? "text-primary-foreground"
                            : "text-destructive-foreground"
                        }`}
                      >
                        {typeKey ? t(typeKey) : t("behavior_record_other")}
                      </Text>
                    </Badge>
                  </View>

                  <Text
                    className={`text-sm font-extrabold ${
                      isPositive ? "text-primary" : "text-destructive"
                    }`}
                  >
                    {isPositive ? `+${formatNumber(record.points)}` : formatNumber(record.points)}
                  </Text>
                </View>

                {record.description ? (
                  <Text className="text-sm font-medium leading-5 text-foreground">
                    {record.description}
                  </Text>
                ) : null}

                <View className="mt-two flex-row items-center justify-between border-t border-border/40 pt-two">
                  <Text className="text-xs text-muted-foreground">
                    {formatDate(record.created_at)}
                  </Text>
                </View>
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
