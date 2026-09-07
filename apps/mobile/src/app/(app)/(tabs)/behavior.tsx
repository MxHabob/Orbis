/**
 * @module @orbis/mobile/app/(app)/(tabs)/behavior
 *
 * Dedicated Behavior & AI Insights Experience.
 * Powered by @orbis/sdk & TanStack Query:
 * - Zero useEffect data fetching
 * - Skeletons for initial query load
 * - Performance memoization
 */

import React, { useState, useMemo, useCallback } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Pressable,
  RefreshControl,
  Modal,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { StatusBar } from 'expo-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useI18n, l, t } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import { useAttendanceQuery, useGradesQuery } from '@/hooks/use-api-queries';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { EmptyState } from '@/components/ui/empty-state';
import { Skeleton, SkeletonCard } from '@/components/ui/skeleton';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface BehaviorRecordItem {
  id: string;
  type: 'merit' | 'violation';
  category: string;
  points: number;
  description: string;
  reportedBy: string;
  date: string;
}

export default function BehaviorScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();
  const { isTeacher, isParent, selectedChild, targetStudentId } = useEffectiveContext();

  const [filter, setFilter] = useState<'all' | 'merit' | 'violation'>('all');
  const [showLogModal, setShowLogModal] = useState(false);

  // New behavior logging form state (for teachers)
  const [newStudent, setNewStudent] = useState('');
  const [newType, setNewType] = useState<'merit' | 'violation'>('merit');
  const [newCategory, setNewCategory] = useState('');
  const [newPoints, setNewPoints] = useState('2');
  const [newDesc, setNewDesc] = useState('');

  // ── TanStack Query Hooks (Zero useEffect) ───────────────────────────────
  const {
    data: attendanceData,
    isLoading: isAttendanceLoading,
    refetch: refetchAttendance,
  } = useAttendanceQuery({ studentId: targetStudentId || undefined });

  const {
    data: gradesData,
    isLoading: isGradesLoading,
    refetch: refetchGrades,
  } = useGradesQuery({ studentId: targetStudentId || undefined });

  const isQueryLoading = isAttendanceLoading || isGradesLoading;

  const onRefresh = useCallback(async () => {
    await Promise.all([refetchAttendance(), refetchGrades()]);
  }, [refetchAttendance, refetchGrades]);

  const [records, setRecords] = useState<BehaviorRecordItem[]>([]);

  const handleSaveRecord = useCallback(() => {
    if (!newDesc.trim()) return;
    const item: BehaviorRecordItem = {
      id: `rec-${Date.now()}`,
      type: newType,
      category: newCategory || t('behavior_modal_type_merit'),
      points:
        newType === 'merit'
          ? Math.abs(parseInt(newPoints, 10) || 1)
          : -Math.abs(parseInt(newPoints, 10) || 1),
      description: newDesc.trim(),
      reportedBy: t('common_you_teacher'),
      date: t('common_now'),
    };
    setRecords((prev) => [item, ...prev]);
    setShowLogModal(false);
    setNewDesc('');
  }, [newDesc, newType, newCategory, newPoints, t]);

  const filteredRecords = useMemo(() => {
    return records.filter((r) => {
      if (filter === 'all') return true;
      return r.type === filter;
    });
  }, [records, filter]);

  const totalPoints = useMemo(() => {
    return records.reduce((acc, r) => acc + r.points, 0);
  }, [records]);

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <StatusBar style="auto" />
      {/* Screen Header */}
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
            <Text style={[styles.screenTitle, { color: theme.text }]}>
              {t('behavior_title')}
            </Text>
            <Text style={[styles.screenSubtitle, { color: theme.textSecondary }]}>
              {isParent
                ? (selectedChild?.name ? t('behavior_sub_parent', { name: selectedChild.name }) : t('behavior_sub_parent_default'))
                : isTeacher
                  ? t('behavior_sub_teacher')
                  : t('behavior_sub_student')}
            </Text>
          </View>

          {isTeacher ? (
            <Button
              title={t('behavior_btn_log')}
              onPress={() => setShowLogModal(true)}
              size="sm"
              icon={<Ionicons name="add-circle" size={16} color="#FFFFFF" />}
            />
          ) : null}
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
        {/* Points & Score Summary Banner */}
        {isQueryLoading ? (
          <SkeletonCard />
        ) : (
          <Card variant="elevated" style={styles.scoreBanner}>
            <View style={styles.scoreRow}>
              <View style={styles.scoreCol}>
                <Text style={[styles.scoreLabel, { color: theme.textSecondary }]}>
                  {t('behavior_score_net')}
                </Text>
                <Text
                  style={[
                    styles.scoreValue,
                    { color: totalPoints >= 0 ? theme.success : theme.error },
                  ]}>
                  {t('behavior_score_unit', { count: totalPoints > 0 ? `+${totalPoints}` : totalPoints })}
                </Text>
                <Text style={[styles.scoreStatus, { color: theme.textSecondary }]}>
                  {t('behavior_status_regular')}
                </Text>
              </View>

              <View
                style={[
                  styles.badgeCircle,
                  { backgroundColor: totalPoints >= 0 ? theme.successLight : theme.errorLight },
                ]}>
                <Ionicons
                  name={totalPoints >= 0 ? 'shield-checkmark' : 'alert-circle'}
                  size={36}
                  color={totalPoints >= 0 ? theme.success : theme.error}
                />
              </View>
            </View>
          </Card>
        )}

        {/* AI Insight Analysis Card */}
        {isQueryLoading ? (
          <SkeletonCard />
        ) : (
          <Card variant="outline" style={styles.aiCard}>
            <View style={styles.aiHeader}>
              <View style={styles.aiTitleBox}>
                <Ionicons name="sparkles" size={18} color="#7C3AED" />
                <Text style={[styles.aiTitle, { color: theme.text }]}>
                  {t('behavior_ai_title')}
                </Text>
              </View>
              <Badge label={t('behavior_badge_live')} variant="primary" size="sm" />
            </View>

            <Text style={[styles.aiSummary, { color: theme.textSecondary }]}>
              {records.length > 0
                ? t('behavior_ai_summary_active')
                : t('behavior_ai_summary_empty')}
            </Text>
          </Card>
        )}

        {/* Filter Tabs */}
        <View style={styles.filterSection}>
          <View style={[styles.filterBar, { backgroundColor: theme.backgroundElement }]}>
            {[
              { key: 'all', label: t('behavior_filter_all') },
              { key: 'merit', label: t('behavior_filter_merit') },
              { key: 'violation', label: t('behavior_filter_violation') },
            ].map((f) => {
              const isSelected = filter === f.key;
              return (
                <Pressable
                  key={f.key}
                  onPress={() => setFilter(f.key as any)}
                  style={[
                    styles.filterPill,
                    isSelected && { backgroundColor: theme.primary },
                  ]}>
                  <Text
                    style={[
                      styles.filterLabel,
                      { color: isSelected ? '#FFFFFF' : theme.textSecondary },
                    ]}>
                    {f.label}
                  </Text>
                </Pressable>
              );
            })}
          </View>
        </View>

        {/* Behavioral Event Timeline */}
        <View style={styles.timelineList}>
          {isQueryLoading ? (
            <>
              <SkeletonCard />
              <SkeletonCard />
            </>
          ) : filteredRecords.length === 0 ? (
            <EmptyState
              icon="shield-checkmark-outline"
              title={t('behavior_empty_title')}
              description={t('behavior_empty_desc')}
            />
          ) : (
            filteredRecords.map((item) => {
              const isMerit = item.type === 'merit';
              return (
                <Card key={item.id} variant="flat" style={styles.eventCard}>
                  <View style={styles.eventRow}>
                    <View
                      style={[
                        styles.eventIcon,
                        { backgroundColor: isMerit ? theme.successLight : theme.errorLight },
                      ]}>
                      <Ionicons
                        name={isMerit ? 'star' : 'warning'}
                        size={20}
                        color={isMerit ? theme.success : theme.error}
                      />
                    </View>

                    <View style={styles.eventDetails}>
                      <View style={styles.eventTopRow}>
                        <Text style={[styles.eventCategory, { color: theme.text }]}>
                          {item.category}
                        </Text>
                        <Badge
                          label={item.points > 0 ? `+${item.points}` : `${item.points}`}
                          variant={isMerit ? 'success' : 'error'}
                          size="sm"
                        />
                      </View>

                      <Text style={[styles.eventDesc, { color: theme.textSecondary }]}>
                        {item.description}
                      </Text>

                      <View style={styles.eventMetaRow}>
                        <Text style={[styles.eventMeta, { color: theme.textMuted }]}>
                          {item.reportedBy} • {item.date}
                        </Text>
                      </View>
                    </View>
                  </View>
                </Card>
              );
            })
          )}
        </View>
      </ScrollView>

      {/* Teacher Add Behavior Modal */}
      <Modal visible={showLogModal} transparent animationType="slide">
        <View style={styles.modalOverlay}>
          <View style={[styles.modalContainer, { backgroundColor: theme.card }]}>
            <View style={styles.modalHeader}>
              <Text style={[styles.modalTitle, { color: theme.text }]}>
                {t('behavior_modal_title')}
              </Text>
              <Pressable onPress={() => setShowLogModal(false)}>
                <Ionicons name="close" size={24} color={theme.textSecondary} />
              </Pressable>
            </View>

            <ScrollView style={{ maxHeight: 400 }} showsVerticalScrollIndicator={false}>
              <View style={styles.modalFields}>
                <Input
                  label={t('behavior_modal_student_label')}
                  placeholder={t('behavior_modal_student_placeholder')}
                  value={newStudent}
                  onChangeText={setNewStudent}
                />

                <View style={{ gap: 6 }}>
                  <Text style={[Typography.captionBold, { color: theme.text }]}>
                    {t('behavior_modal_type_label')}
                  </Text>
                  <View style={{ flexDirection: 'row', gap: Spacing.two }}>
                    <Button
                      title={t('behavior_modal_type_merit')}
                      onPress={() => setNewType('merit')}
                      variant={newType === 'merit' ? 'primary' : 'outline'}
                      size="sm"
                      style={{ flex: 1 }}
                    />
                    <Button
                      title={t('behavior_modal_type_violation')}
                      onPress={() => setNewType('violation')}
                      variant={newType === 'violation' ? 'danger' : 'outline'}
                      size="sm"
                      style={{ flex: 1 }}
                    />
                  </View>
                </View>

                <Input
                  label={t('behavior_modal_category_label')}
                  placeholder={t('behavior_modal_category_placeholder')}
                  value={newCategory}
                  onChangeText={setNewCategory}
                />

                <Input
                  label={t('behavior_modal_points_label')}
                  placeholder="2"
                  keyboardType="numeric"
                  value={newPoints}
                  onChangeText={setNewPoints}
                />

                <Input
                  label={t('behavior_modal_desc_label')}
                  placeholder={t('behavior_modal_desc_placeholder')}
                  multiline
                  numberOfLines={3}
                  value={newDesc}
                  onChangeText={setNewDesc}
                />
              </View>
            </ScrollView>

            <View style={styles.modalFooter}>
              <Button
                title={t('behavior_modal_cancel')}
                onPress={() => setShowLogModal(false)}
                variant="ghost"
                style={{ flex: 1 }}
              />
              <Button
                title={t('behavior_modal_submit')}
                onPress={handleSaveRecord}
                variant="primary"
                style={{ flex: 2 }}
                disabled={!newDesc.trim()}
              />
            </View>
          </View>
        </View>
      </Modal>
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
  scoreBanner: {
    padding: Spacing.four,
  },
  scoreRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  scoreCol: {
    gap: 2,
  },
  scoreLabel: {
    ...Typography.small,
    fontSize: 12,
  },
  scoreValue: {
    ...Typography.h1,
    fontSize: 32,
    marginTop: 2,
  },
  scoreStatus: {
    ...Typography.small,
    marginTop: 4,
  },
  badgeCircle: {
    width: 60,
    height: 60,
    borderRadius: 30,
    alignItems: 'center',
    justifyContent: 'center',
  },
  aiCard: {
    padding: Spacing.four,
    gap: Spacing.two,
  },
  aiHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  aiTitleBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
  },
  aiTitle: {
    ...Typography.bodyBold,
    fontSize: 15,
  },
  aiSummary: {
    ...Typography.body,
    fontSize: 13,
    lineHeight: 19,
  },
  filterSection: {
    alignItems: 'center',
  },
  filterBar: {
    flexDirection: 'row',
    borderRadius: Radii.full,
    padding: 3,
    width: '100%',
  },
  filterPill: {
    flex: 1,
    paddingVertical: 7,
    borderRadius: Radii.full,
    alignItems: 'center',
    justifyContent: 'center',
  },
  filterLabel: {
    ...Typography.small,
    fontWeight: '700',
    fontSize: 12,
  },
  timelineList: {
    gap: Spacing.three,
  },
  eventCard: {
    padding: Spacing.three,
  },
  eventRow: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    gap: Spacing.three,
  },
  eventIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    alignItems: 'center',
    justifyContent: 'center',
    marginTop: 2,
  },
  eventDetails: {
    flex: 1,
    gap: 4,
  },
  eventTopRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  eventCategory: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
  eventDesc: {
    ...Typography.body,
    fontSize: 12,
    lineHeight: 18,
  },
  eventMetaRow: {
    marginTop: 2,
  },
  eventMeta: {
    ...Typography.small,
    fontSize: 10,
  },
  modalOverlay: {
    flex: 1,
    backgroundColor: 'rgba(0,0,0,0.5)',
    justifyContent: 'flex-end',
  },
  modalContainer: {
    borderTopLeftRadius: Radii['2xl'],
    borderTopRightRadius: Radii['2xl'],
    padding: Spacing.four,
    gap: Spacing.four,
    maxHeight: '90%',
  },
  modalHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingBottom: Spacing.two,
  },
  modalTitle: {
    ...Typography.h2,
    fontSize: 18,
  },
  modalFields: {
    gap: Spacing.three,
  },
  modalFooter: {
    flexDirection: 'row',
    gap: Spacing.two,
    paddingTop: Spacing.two,
  },
});
