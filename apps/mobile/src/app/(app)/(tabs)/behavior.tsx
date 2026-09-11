/**
 * @module @orbis/mobile/app/(app)/(tabs)/behavior
 *
 * Dedicated Behavior & AI Insights Experience.
 * Ultra-clean component orchestrator powered by @orbis/sdk & TanStack Query.
 */

import { Ionicons } from "@expo/vector-icons";
import { useCallback, useMemo, useState } from "react";
import { RefreshControl, ScrollView, StyleSheet, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import {
  BehaviorAiCard,
  BehaviorFilterBar,
  BehaviorFilterType,
  BehaviorLogModal,
  BehaviorRecordItem,
  BehaviorScoreBanner,
  BehaviorTimelineList,
} from "@/components/behavior";
import { Button } from "@/components/ui/button";
import { Spacing } from "@/constants/theme";
import { useAttendanceQuery, useGradesQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { useEffectiveContext } from "@/providers/effective-context";

export default function BehaviorScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();
  const { isTeacher, isParent, selectedChild, targetStudentId } =
    useEffectiveContext();

  const [filter, setFilter] = useState<BehaviorFilterType>("all");
  const [showLogModal, setShowLogModal] = useState(false);
  const [records, setRecords] = useState<BehaviorRecordItem[]>([]);

  // ── TanStack Query Hooks (Zero useEffect) ───────────────────────────────
  const { isLoading: isAttendanceLoading, refetch: refetchAttendance } =
    useAttendanceQuery({
      studentId: targetStudentId || undefined,
    });
  const { isLoading: isGradesLoading, refetch: refetchGrades } = useGradesQuery(
    {
      studentId: targetStudentId || undefined,
    },
  );

  const isQueryLoading = isAttendanceLoading || isGradesLoading;

  const onRefresh = useCallback(async () => {
    await Promise.all([refetchAttendance(), refetchGrades()]);
  }, [refetchAttendance, refetchGrades]);

  const handleSaveRecord = useCallback((record: BehaviorRecordItem) => {
    setRecords((prev) => [record, ...prev]);
  }, []);

  const filteredRecords = useMemo(() => {
    if (filter === "all") return records;
    return records.filter((r) => r.type === filter);
  }, [records, filter]);

  const totalPoints = useMemo(() => {
    return records.reduce((acc, r) => acc + r.points, 0);
  }, [records]);

  const subtitle = isParent
    ? selectedChild?.name
      ? t("behavior_sub_parent", { name: selectedChild.name })
      : t("behavior_sub_parent_default")
    : isTeacher
      ? t("behavior_sub_teacher")
      : t("behavior_sub_student");

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      {isTeacher && (
        <View
          style={[styles.actionBar, { paddingTop: insets.top + Spacing.two }]}
        >
          <Button
            title={t("behavior_btn_log")}
            onPress={() => setShowLogModal(true)}
            size="sm"
            icon={<Ionicons name="add-circle" size={16} color="#FFFFFF" />}
          />
        </View>
      )}

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: Math.max(insets.bottom, 16) + 90 },
        ]}
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl
            refreshing={isQueryLoading}
            onRefresh={onRefresh}
            tintColor={theme.primary}
          />
        }
      >
        {/* 1. Score Banner */}
        <BehaviorScoreBanner
          totalPoints={totalPoints}
          isLoading={isQueryLoading}
        />

        {/* 2. AI Card */}
        <BehaviorAiCard
          hasRecords={records.length > 0}
          isLoading={isQueryLoading}
        />

        {/* 3. Filter Bar */}
        <BehaviorFilterBar activeFilter={filter} onFilterChange={setFilter} />

        {/* 4. Timeline List */}
        <BehaviorTimelineList
          records={filteredRecords}
          isLoading={isQueryLoading}
        />
      </ScrollView>

      {/* 5. Teacher Modal */}
      <BehaviorLogModal
        visible={showLogModal}
        onClose={() => setShowLogModal(false)}
        onSave={handleSaveRecord}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  actionBar: {
    paddingHorizontal: Spacing.four,
    paddingBottom: Spacing.three,
  },
  scrollContent: {
    padding: Spacing.four,
    gap: Spacing.four,
  },
});
