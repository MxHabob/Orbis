/**
 * @module @orbis/mobile/app/(app)/class/[id]/schedule
 *
 * Class Timetable Grid Screen for Teachers.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function ClassScheduleScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [selectedDay, setSelectedDay] = useState(0);

  const days = [
    { index: 0, name: 'الأحد' },
    { index: 1, name: 'الإثنين' },
    { index: 2, name: 'الثلاثاء' },
    { index: 3, name: 'الأربعاء' },
    { index: 4, name: 'الخميس' },
  ];

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="جدول حصص الفصل"
        subtitle="الصف 3 علمي - أ • القاعة 104"
        showBack
      />

      <View style={[styles.dayTabBar, { backgroundColor: theme.card }]}>
        {days.map((d) => {
          const isSelected = selectedDay === d.index;
          return (
            <Pressable
              key={d.index}
              onPress={() => setSelectedDay(d.index)}
              style={[styles.dayTab, isSelected && { backgroundColor: theme.primary }]}>
              <Text
                style={[
                  styles.dayTabText,
                  { color: isSelected ? '#FFFFFF' : theme.textSecondary },
                ]}>
                {d.name}
              </Text>
            </Pressable>
          );
        })}
      </View>

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {[
          {
            slot: 1,
            time: '08:00 - 08:45',
            subject: 'الرياضيات المتقدمة',
            teacher: 'أنت (معلم المادة)',
            room: 'قاعة 104',
          },
          {
            slot: 2,
            time: '08:50 - 09:35',
            subject: 'الفيزياء التطبيقية',
            teacher: 'د. أحمد العمري',
            room: 'مختبر العلوم 2',
          },
          {
            slot: 3,
            time: '09:40 - 10:25',
            subject: 'اللغة الإنجليزية',
            teacher: 'أ. مايكل براون',
            room: 'قاعة 104',
          },
          {
            slot: 4,
            time: '10:55 - 11:40',
            subject: 'الكيمياء العضوية',
            teacher: 'أ. عبدالعزيز الغامدي',
            room: 'مختبر الكيمياء',
          },
        ].map((item) => (
          <Card key={item.slot} variant="elevated" style={styles.slotCard}>
            <View style={styles.slotRow}>
              <View style={styles.timeCol}>
                <Text style={[styles.timeText, { color: theme.text }]}>{item.time}</Text>
                <Badge label={`الحصة ${item.slot}`} variant="neutral" size="sm" />
              </View>

              <View style={styles.detailsCol}>
                <Text style={[styles.subjectName, { color: theme.text }]}>{item.subject}</Text>
                <Text style={[styles.metaText, { color: theme.textSecondary }]}>
                  {item.teacher} • {item.room}
                </Text>
              </View>
            </View>
          </Card>
        ))}
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  dayTabBar: {
    flexDirection: 'row',
    padding: Spacing.two,
    gap: Spacing.one,
    borderBottomWidth: 1,
    borderBottomColor: '#E2E8F030',
  },
  dayTab: {
    flex: 1,
    paddingVertical: Spacing.two,
    borderRadius: Radii.md,
    alignItems: 'center',
  },
  dayTabText: {
    ...Typography.captionBold,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.three,
  },
  slotCard: {
    padding: Spacing.four,
  },
  slotRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  timeCol: {
    width: 100,
    gap: 4,
  },
  timeText: {
    ...Typography.small,
    fontWeight: '600',
  },
  detailsCol: {
    flex: 1,
    marginLeft: Spacing.three,
    gap: 2,
  },
  subjectName: {
    ...Typography.bodyBold,
  },
  metaText: {
    ...Typography.small,
  },
});
