/**
 * @module @orbis/mobile/app/(app)/(tabs)/behavior
 *
 * Dedicated Behavior & AI Insights Experience.
 * Ultra-clean component orchestrator powered by @orbis/sdk & TanStack Query.
 */

import React, { useState, useMemo, useCallback } from 'react';
import { View, Text, StyleSheet, ScrollView, RefreshControl } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { StatusBar } from 'expo-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import { useAttendanceQuery, useGradesQuery } from '@/hooks/use-api-queries';
import { Button } from '@/components/ui/button';
import {
  BehaviorScoreBanner,
  BehaviorAiCard,
  BehaviorFilterBar,
  BehaviorTimelineList,
  BehaviorLogModal,
  BehaviorFilterType,
  BehaviorRecordItem,
} from '@/components/behavior';
import { Spacing, Typography } from '@/constants/theme';

export default function BehaviorScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();
  const { isTeacher, isParent, selectedChild, targetStudentId } = useEffectiveContext();

  const [filter, setFilter] = useState<BehaviorFilterType>('all');
  const [showLogModal, setShowLogModal] = useState(false);
  const [records, setRecords] = useState<BehaviorRecordItem[]>([]);

  // ── TanStack Query Hooks (Zero useEffect) ───────────────────────────────
  const { isLoading: isAttendanceLoading, refetch: refetchAttendance } = useAttendanceQuery({
    studentId: targetStudentId || undefined,
  });
  const { isLoading: isGradesLoading, refetch: refetchGrades } = useGradesQuery({
    studentId: targetStudentId || undefined,
  });

  const isQueryLoading = isAttendanceLoading || isGradesLoading;

  const onRefresh = useCallback(async () => {
    await Promise.all([refetchAttendance(), refetchGrades()]);
  }, [refetchAttendance, refetchGrades]);

  const handleSaveRecord = useCallback((record: BehaviorRecordItem) => {
    setRecords((prev) => [record, ...prev]);
  }, []);

  const filteredRecords = useMemo(() => {
    if (filter === 'all') return records;
    return records.filter((r) => r.type === filter);
  }, [records, filter]);

  const totalPoints = useMemo(() => {
    return records.reduce((acc, r) => acc + r.points, 0);
  }, [records]);

  const subtitle = isParent
    ? selectedChild?.name
      ? t('behavior_sub_parent', { name: selectedChild.name })
      : t('behavior_sub_parent_default')
    : isTeacher
      ? t('behavior_sub_teacher')
      : t('behavior_sub_student');

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <StatusBar style="auto" />
      {/* Header */}
      <View
        style={[
          styles.header,
          {
            paddingTop: insets.top + Spacing.two,
            backgroundColor: theme.card,
            borderBottomColor: theme.borderSubtle,
          },
        ]}>
        <View style={styles.headerRow}>
          <View style={styles.titleCol}>
            <Text style={[styles.screenTitle, { color: theme.text }]}>{t('behavior_title')}</Text>
            <Text style={[styles.screenSubtitle, { color: theme.textSecondary }]}>{subtitle}</Text>
          </View>

          {isTeacher && (
            <Button
              title={t('behavior_btn_log')}
              onPress={() => setShowLogModal(true)}
              size="sm"
              icon={<Ionicons name="add-circle" size={16} color="#FFFFFF" />}
            />
          )}
        </View>
      </View>

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
        }>
        {/* 1. Score Banner */}
        <BehaviorScoreBanner totalPoints={totalPoints} isLoading={isQueryLoading} />

        {/* 2. AI Card */}
        <BehaviorAiCard hasRecords={records.length > 0} isLoading={isQueryLoading} />

        {/* 3. Filter Bar */}
        <BehaviorFilterBar activeFilter={filter} onFilterChange={setFilter} />

        {/* 4. Timeline List */}
        <BehaviorTimelineList records={filteredRecords} isLoading={isQueryLoading} />
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
  header: {
    paddingHorizontal: Spacing.four,
    paddingBottom: Spacing.three,
    borderBottomWidth: 1,
  },
  headerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  titleCol: {
    gap: 2,
  },
  screenTitle: {
    ...Typography.h2,
    fontSize: 20,
  },
  screenSubtitle: {
    ...Typography.small,
  },
  scrollContent: {
    padding: Spacing.four,
    gap: Spacing.four,
  },
});
