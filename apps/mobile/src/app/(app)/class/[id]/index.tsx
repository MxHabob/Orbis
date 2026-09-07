/**
 * @module @orbis/mobile/app/(app)/class/[id]/index
 *
 * Class Hub and Student Roster Screen for Teachers.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { router, useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface StudentRosterItem {
  id: string;
  name: string;
  studentNumber: string;
  attendanceRate: string;
  averageGrade: string;
}

export default function ClassHubScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const students: StudentRosterItem[] = [
    {
      id: 'std-1',
      name: 'عمر خالد المنصور',
      studentNumber: 'STD-2026-081',
      attendanceRate: '98.5%',
      averageGrade: '96.4%',
    },
    {
      id: 'std-2',
      name: 'سلطان عبدالعزيز الراشد',
      studentNumber: 'STD-2026-082',
      attendanceRate: '100%',
      averageGrade: '98.0%',
    },
    {
      id: 'std-3',
      name: 'فيصل محمد العتيبي',
      studentNumber: 'STD-2026-083',
      attendanceRate: '95.0%',
      averageGrade: '91.2%',
    },
    {
      id: 'std-4',
      name: 'عبدالله ناصر القحطاني',
      studentNumber: 'STD-2026-084',
      attendanceRate: '92.0%',
      averageGrade: '88.5%',
    },
    {
      id: 'std-5',
      name: 'تركي سعود الشمري',
      studentNumber: 'STD-2026-085',
      attendanceRate: '97.0%',
      averageGrade: '94.0%',
    },
  ];

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="الصف 3 علمي - أ"
        subtitle="الرياضيات المتقدمة • 28 طالباً"
        showBack
      />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Class Overview Card */}
        <Card variant="elevated" style={styles.classCard}>
          <View style={styles.classHeader}>
            <View style={[styles.classIcon, { backgroundColor: theme.primaryLight }]}>
              <Ionicons name="people" size={28} color={theme.primary} />
            </View>
            <View style={styles.classInfo}>
              <Text style={[styles.className, { color: theme.text }]}>
                الصف الثالث الثانوي (علمي - أ)
              </Text>
              <Text style={[styles.classMeta, { color: theme.textSecondary }]}>
                المادة: الرياضيات المتقدمة • القاعة 104
              </Text>
            </View>
          </View>

          <View style={[styles.statsRow, { borderTopColor: theme.borderSubtle }]}>
            <View style={styles.statCol}>
              <Text style={[styles.statVal, { color: theme.text }]}>28</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>الطلاب المسجلين</Text>
            </View>
            <View style={styles.statCol}>
              <Text style={[styles.statVal, { color: theme.success }]}>96.2%</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>متوسط الحضور</Text>
            </View>
            <View style={styles.statCol}>
              <Text style={[styles.statVal, { color: theme.primary }]}>93.8%</Text>
              <Text style={[styles.statLbl, { color: theme.textSecondary }]}>متوسط الفصل</Text>
            </View>
          </View>
        </Card>

        {/* Action Buttons */}
        <View style={styles.actionGrid}>
          <Card
            variant="flat"
            style={styles.actionCard}
            onPress={() => router.push(`/(app)/class/${id}/attendance` as any)}>
            <View style={[styles.actionIconBox, { backgroundColor: theme.primaryLight }]}>
              <Ionicons name="checkmark-done-circle" size={24} color={theme.primary} />
            </View>
            <Text style={[styles.actionTitle, { color: theme.text }]}>رصد الحضور</Text>
            <Text style={[styles.actionSub, { color: theme.textSecondary }]}>تسجيل حضور الحصة</Text>
          </Card>

          <Card
            variant="flat"
            style={styles.actionCard}
            onPress={() => router.push(`/(app)/class/${id}/grades` as any)}>
            <View style={[styles.actionIconBox, { backgroundColor: theme.successLight }]}>
              <Ionicons name="create" size={24} color={theme.success} />
            </View>
            <Text style={[styles.actionTitle, { color: theme.text }]}>دفتر الدرجات</Text>
            <Text style={[styles.actionSub, { color: theme.textSecondary }]}>رصد وتقييم الاختبارات</Text>
          </Card>

          <Card
            variant="flat"
            style={styles.actionCard}
            onPress={() => router.push(`/(app)/class/${id}/schedule` as any)}>
            <View style={[styles.actionIconBox, { backgroundColor: theme.infoLight }]}>
              <Ionicons name="time" size={24} color={theme.info} />
            </View>
            <Text style={[styles.actionTitle, { color: theme.text }]}>جدول الحصص</Text>
            <Text style={[styles.actionSub, { color: theme.textSecondary }]}>المواعيد والقاعات</Text>
          </Card>
        </View>

        {/* Student Roster List */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>قائمة طلاب الفصل</Text>
        </View>

        <View style={styles.rosterList}>
          {students.map((std, idx) => (
            <Card key={std.id} variant="elevated" style={styles.rosterCard}>
              <View style={styles.rosterRow}>
                <View style={[styles.numBadge, { backgroundColor: theme.backgroundElement }]}>
                  <Text style={[styles.numText, { color: theme.textSecondary }]}>{idx + 1}</Text>
                </View>

                <View style={styles.stdInfo}>
                  <Text style={[styles.stdName, { color: theme.text }]}>{std.name}</Text>
                  <Text style={[styles.stdNum, { color: theme.textSecondary }]}>
                    {std.studentNumber}
                  </Text>
                </View>

                <View style={styles.stdStats}>
                  <Badge label={`حضور ${std.attendanceRate}`} variant="neutral" size="sm" />
                  <Badge label={`معدل ${std.averageGrade}`} variant="success" size="sm" />
                </View>
              </View>
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
  classCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  classHeader: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  classIcon: {
    width: 52,
    height: 52,
    borderRadius: 26,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: Spacing.three,
  },
  classInfo: {
    flex: 1,
  },
  className: {
    ...Typography.h3,
  },
  classMeta: {
    ...Typography.caption,
    marginTop: 2,
  },
  statsRow: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    borderTopWidth: 1,
    paddingTop: Spacing.three,
  },
  statCol: {
    alignItems: 'center',
  },
  statVal: {
    ...Typography.bodyBold,
  },
  statLbl: {
    ...Typography.small,
    marginTop: 2,
  },
  actionGrid: {
    flexDirection: 'row',
    gap: Spacing.two,
  },
  actionCard: {
    flex: 1,
    padding: Spacing.three,
    gap: 2,
  },
  actionIconBox: {
    width: 38,
    height: 38,
    borderRadius: 19,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: 4,
  },
  actionTitle: {
    ...Typography.captionBold,
  },
  actionSub: {
    ...Typography.small,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  rosterList: {
    gap: Spacing.two,
  },
  rosterCard: {
    padding: Spacing.three,
  },
  rosterRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  numBadge: {
    width: 28,
    height: 28,
    borderRadius: 14,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: Spacing.three,
  },
  numText: {
    ...Typography.small,
    fontWeight: '700',
  },
  stdInfo: {
    flex: 1,
  },
  stdName: {
    ...Typography.bodyBold,
  },
  stdNum: {
    ...Typography.small,
    marginTop: 2,
  },
  stdStats: {
    alignItems: 'flex-end',
    gap: 4,
  },
});
