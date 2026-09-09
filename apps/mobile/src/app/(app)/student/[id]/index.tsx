/**
 * @module @orbis/mobile/app/(app)/student/[id]/index
 *
 * Student / Child Academic Hub Overview.
 */

import React from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { router, useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useEffectiveContext } from '@/providers/effective-context';
import { Header } from '@/components/header';
import { Avatar } from '@/components/ui/avatar';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function StudentHubScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();
  const { selectedChild, isParent } = useEffectiveContext();

  const studentName = isParent ? selectedChild?.name : 'عمر خالد المنصور';
  const studentNumber = isParent ? selectedChild?.studentNumber : 'STD-2026-081';
  const className = isParent ? selectedChild?.className : 'الصف الثالث الثانوي (علمي - أ)';

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الملف الأكاديمي" subtitle={studentName} showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Student ID Profile Card */}
        <Card variant="elevated" style={styles.profileCard}>
          <View style={styles.profileRow}>
            <Avatar name={studentName || 'Student'} size={60} />
            <View style={styles.profileInfo}>
              <Text style={[styles.name, { color: theme.text }]}>{studentName}</Text>
              <Text style={[styles.meta, { color: theme.textSecondary }]}>
                {studentNumber} • {className}
              </Text>
              <Badge label="منتظم - مسجل رسمياً" variant="success" size="sm" style={{ marginTop: 4 }} />
            </View>
          </View>
        </Card>

        {/* Academic Modules Grid */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>الأقسام والخدمات الأكاديمية</Text>
        </View>

        <View style={styles.modulesGrid}>
          {[
            {
              title: 'الدرجات والتقييم',
              sub: 'المعدل والتقييم الفصلي',
              icon: 'podium',
              color: theme.primary,
              bg: theme.primaryLight,
              route: `/(app)/student/${id}/grades`,
            },
            {
              title: 'الحضور والغياب',
              sub: 'سجل الحصص والأعذار',
              icon: 'calendar',
              color: theme.info,
              bg: theme.infoLight,
              route: `/(app)/student/${id}/attendance`,
            },
            {
              title: 'الجدول الدراسي',
              sub: 'الحصص والقاعات الأسبوعية',
              icon: 'time',
              color: theme.success,
              bg: theme.successLight,
              route: `/(app)/student/${id}/schedule`,
            },
            {
              title: 'جدول الاختبارات',
              sub: 'المواعيد وأرقام الجلوس',
              icon: 'document-text',
              color: theme.warning,
              bg: theme.warningLight,
              route: `/(app)/student/${id}/exams`,
            },
            {
              title: 'الرسوم والمستحقات',
              sub: 'الفواتير وسجل السداد',
              icon: 'receipt',
              color: theme.accent,
              bg: theme.accentLight,
              route: `/(app)/student/${id}/fees`,
            },
          ].map((item, idx) => (
            <Card
              key={idx}
              variant="flat"
              style={styles.moduleCard}
              onPress={() => router.push(item.route as any)}>
              <View style={[styles.iconBox, { backgroundColor: item.bg }]}>
                <Ionicons name={item.icon as any} size={24} color={item.color} />
              </View>
              <Text style={[styles.moduleTitle, { color: theme.text }]}>{item.title}</Text>
              <Text style={[styles.moduleSub, { color: theme.textSecondary }]}>{item.sub}</Text>
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
  profileCard: {
    padding: Spacing.four,
  },
  profileRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  profileInfo: {
    flex: 1,
  },
  name: {
    ...Typography.h3,
  },
  meta: {
    ...Typography.caption,
    marginTop: 2,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  modulesGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: Spacing.three,
  },
  moduleCard: {
    width: '47.5%',
    padding: Spacing.four,
    gap: 4,
  },
  iconBox: {
    width: 44,
    height: 44,
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: 6,
  },
  moduleTitle: {
    ...Typography.bodyBold,
  },
  moduleSub: {
    ...Typography.small,
  },
});
