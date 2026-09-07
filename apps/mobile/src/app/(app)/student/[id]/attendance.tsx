/**
 * @module @orbis/mobile/app/(app)/student/[id]/attendance
 *
 * Student Attendance & Absence History Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, RefreshControl } from 'react-native';
import { useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface AttendanceLog {
  date: string;
  dayName: string;
  status: 'Present' | 'Late' | 'Excused' | 'Absent';
  periodOrSubject: string;
  notes?: string;
}

export default function StudentAttendanceScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [refreshing, setRefreshing] = useState(false);

  const logs: AttendanceLog[] = [
    {
      date: '7 سبتمبر 2026',
      dayName: 'اليوم',
      status: 'Present',
      periodOrSubject: 'جميع الحصص (اليوم كامل)',
    },
    {
      date: '6 سبتمبر 2026',
      dayName: 'أمس',
      status: 'Present',
      periodOrSubject: 'جميع الحصص (اليوم كامل)',
    },
    {
      date: '3 سبتمبر 2026',
      dayName: 'الأربعاء',
      status: 'Late',
      periodOrSubject: 'الحصة الأولى - الرياضيات المتقدمة',
      notes: 'تأخر 10 دقائق بسبب الازدحام المروري',
    },
    {
      date: '27 أغسطس 2026',
      dayName: 'الخميس',
      status: 'Excused',
      periodOrSubject: 'اليوم كامل',
      notes: 'إجازة مرضية معتمدة من منصة صحتي',
    },
    {
      date: '26 أغسطس 2026',
      dayName: 'الأربعاء',
      status: 'Present',
      periodOrSubject: 'جميع الحصص (اليوم كامل)',
    },
  ];

  const onRefresh = () => {
    setRefreshing(true);
    setTimeout(() => setRefreshing(false), 500);
  };

  const getStatusBadge = (status: AttendanceLog['status']) => {
    switch (status) {
      case 'Present':
        return <Badge label="حاضر" variant="success" size="md" />;
      case 'Late':
        return <Badge label="متأخر" variant="warning" size="md" />;
      case 'Excused':
        return <Badge label="غياب بعذر" variant="info" size="md" />;
      case 'Absent':
        return <Badge label="غياب غير مبرر" variant="error" size="md" />;
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="سجل الحضور والمواظبة" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} tintColor={theme.primary} />
        }>
        {/* Attendance Percentage Banner */}
        <Card variant="elevated" style={styles.summaryCard}>
          <View style={styles.summaryRow}>
            <View style={styles.summaryCol}>
              <Text style={[styles.summaryLabel, { color: theme.textSecondary }]}>
                نسبة الالتزام بالحضور
              </Text>
              <Text style={[styles.summaryRate, { color: theme.success }]}>98.5%</Text>
              <Text style={[styles.summarySub, { color: theme.textSecondary }]}>
                حالة الحضور: ممتازة جداً
              </Text>
            </View>

            <View style={[styles.iconBox, { backgroundColor: theme.successLight }]}>
              <Ionicons name="checkmark-circle" size={40} color={theme.success} />
            </View>
          </View>

          <View style={[styles.statsRow, { borderTopColor: theme.borderSubtle }]}>
            <View style={styles.statItem}>
              <Text style={[styles.statVal, { color: theme.text }]}>65</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>أيام حضور</Text>
            </View>
            <View style={styles.statItem}>
              <Text style={[styles.statVal, { color: theme.info }]}>1</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>غياب بعذر</Text>
            </View>
            <View style={styles.statItem}>
              <Text style={[styles.statVal, { color: theme.warning }]}>2</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>تأخر</Text>
            </View>
            <View style={styles.statItem}>
              <Text style={[styles.statVal, { color: theme.error }]}>0</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>غير مبرر</Text>
            </View>
          </View>
        </Card>

        {/* Detailed Timeline */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>سجل الأيام والحصص</Text>
        </View>

        <View style={styles.logsList}>
          {logs.map((log, idx) => (
            <Card key={idx} variant="flat" style={styles.logCard}>
              <View style={styles.logHeader}>
                <View>
                  <Text style={[styles.logDate, { color: theme.text }]}>
                    {log.date} ({log.dayName})
                  </Text>
                  <Text style={[styles.logSubject, { color: theme.textSecondary }]}>
                    {log.periodOrSubject}
                  </Text>
                </View>
                {getStatusBadge(log.status)}
              </View>

              {log.notes && (
                <View style={[styles.notesBox, { backgroundColor: theme.card }]}>
                  <Ionicons name="information-circle-outline" size={16} color={theme.textSecondary} />
                  <Text style={[styles.notesText, { color: theme.textSecondary }]}>{log.notes}</Text>
                </View>
              )}
            </Card>
          ))}
        </View>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.four,
  },
  summaryCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  summaryRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  summaryCol: {
    flex: 1,
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
    borderRadius: 32,
    alignItems: 'center',
    justifyContent: 'center',
  },
  statsRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    borderTopWidth: 1,
    paddingTop: Spacing.three,
  },
  statItem: {
    alignItems: 'center',
    flex: 1,
  },
  statVal: {
    ...Typography.bodyBold,
  },
  statLbl: {
    ...Typography.small,
    marginTop: 2,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  logsList: {
    gap: Spacing.two,
  },
  logCard: {
    padding: Spacing.four,
    gap: Spacing.two,
  },
  logHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  logDate: {
    ...Typography.bodyBold,
  },
  logSubject: {
    ...Typography.small,
    marginTop: 2,
  },
  notesBox: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: Spacing.two,
    borderRadius: Radii.sm,
    gap: 6,
  },
  notesText: {
    ...Typography.small,
    flex: 1,
  },
});
