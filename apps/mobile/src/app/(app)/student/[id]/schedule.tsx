/**
 * @module @orbis/mobile/app/(app)/student/[id]/schedule
 *
 * Student / Child Weekly Timetable and Class Schedule Screen.
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

interface ScheduleSlot {
  slotIndex: number;
  time: string;
  subject: string;
  code: string;
  room: string;
  teacher: string;
  type: 'Lecture' | 'Lab' | 'Activity';
}

export default function StudentScheduleScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [selectedDay, setSelectedDay] = useState<number>(0); // 0 = Sunday

  const days = [
    { index: 0, name: 'الأحد', short: 'أحد' },
    { index: 1, name: 'الإثنين', short: 'إثنين' },
    { index: 2, name: 'الثلاثاء', short: 'ثلاثاء' },
    { index: 3, name: 'الأربعاء', short: 'أربعاء' },
    { index: 4, name: 'الخميس', short: 'خميس' },
  ];

  const scheduleData: Record<number, ScheduleSlot[]> = {
    0: [
      {
        slotIndex: 1,
        time: '08:00 - 08:45',
        subject: 'الرياضيات المتقدمة',
        code: 'MATH-301',
        room: 'قاعة 104',
        teacher: 'أ. فهد الدوسري',
        type: 'Lecture',
      },
      {
        slotIndex: 2,
        time: '08:50 - 09:35',
        subject: 'الفيزياء التطبيقية',
        code: 'PHYS-302',
        room: 'مختبر العلوم 2',
        teacher: 'د. أحمد العمري',
        type: 'Lab',
      },
      {
        slotIndex: 3,
        time: '09:40 - 10:25',
        subject: 'اللغة الإنجليزية الأكاديمية',
        code: 'ENG-304',
        room: 'قاعة 104',
        teacher: 'أ. مايكل براون',
        type: 'Lecture',
      },
      {
        slotIndex: 4,
        time: '10:55 - 11:40',
        subject: 'الكيمياء العضوية',
        code: 'CHEM-303',
        room: 'مختبر الكيمياء',
        teacher: 'أ. عبدالعزيز الغامدي',
        type: 'Lab',
      },
      {
        slotIndex: 5,
        time: '11:45 - 12:30',
        subject: 'علوم الحاسب والذكاء الاصطناعي',
        code: 'CS-305',
        room: 'معمل الحاسب 1',
        teacher: 'م. سالم القحطاني',
        type: 'Lecture',
      },
    ],
    1: [
      {
        slotIndex: 1,
        time: '08:00 - 08:45',
        subject: 'الكيمياء العضوية',
        code: 'CHEM-303',
        room: 'قاعة 104',
        teacher: 'أ. عبدالعزيز الغامدي',
        type: 'Lecture',
      },
      {
        slotIndex: 2,
        time: '08:50 - 09:35',
        subject: 'الرياضيات المتقدمة',
        code: 'MATH-301',
        room: 'قاعة 104',
        teacher: 'أ. فهد الدوسري',
        type: 'Lecture',
      },
      {
        slotIndex: 3,
        time: '09:40 - 10:25',
        subject: 'التربية البدنية والرياضية',
        code: 'PE-101',
        room: 'الصالة الرياضية',
        teacher: 'كابتن ياسر',
        type: 'Activity',
      },
    ],
    2: [
      {
        slotIndex: 1,
        time: '08:00 - 08:45',
        subject: 'الفيزياء التطبيقية',
        code: 'PHYS-302',
        room: 'قاعة 104',
        teacher: 'د. أحمد العمري',
        type: 'Lecture',
      },
      {
        slotIndex: 2,
        time: '08:50 - 09:35',
        subject: 'علوم الحاسب والبرمجة',
        code: 'CS-305',
        room: 'معمل الحاسب 1',
        teacher: 'م. سالم القحطاني',
        type: 'Lab',
      },
    ],
    3: [
      {
        slotIndex: 1,
        time: '08:00 - 08:45',
        subject: 'اللغة الإنجليزية الأكاديمية',
        code: 'ENG-304',
        room: 'قاعة 104',
        teacher: 'أ. مايكل براون',
        type: 'Lecture',
      },
      {
        slotIndex: 2,
        time: '08:50 - 09:35',
        subject: 'الرياضيات المتقدمة',
        code: 'MATH-301',
        room: 'قاعة 104',
        teacher: 'أ. فهد الدوسري',
        type: 'Lecture',
      },
    ],
    4: [
      {
        slotIndex: 1,
        time: '08:00 - 08:45',
        subject: 'الكيمياء العضوية',
        code: 'CHEM-303',
        room: 'مختبر الكيمياء',
        teacher: 'أ. عبدالعزيز الغامدي',
        type: 'Lab',
      },
      {
        slotIndex: 2,
        time: '08:50 - 09:35',
        subject: 'ندوة التوجيه الأكاديمي والمهني',
        code: 'GUIDE-301',
        room: 'المسرح الرئيسي',
        teacher: 'المرشد الطلابي',
        type: 'Activity',
      },
    ],
  };

  const currentSlots = scheduleData[selectedDay] || scheduleData[0];

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الجدول الدراسي الأسبوعي" showBack />

      {/* Day Switcher Tab Bar */}
      <View style={[styles.dayTabBar, { backgroundColor: theme.card }]}>
        {days.map((d) => {
          const isSelected = selectedDay === d.index;
          return (
            <Pressable
              key={d.index}
              onPress={() => setSelectedDay(d.index)}
              style={[
                styles.dayTab,
                isSelected && {
                  backgroundColor: theme.primary,
                },
              ]}>
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
        <View style={styles.slotsList}>
          {currentSlots.map((slot) => (
            <Card key={slot.slotIndex} variant="elevated" style={styles.slotCard}>
              <View style={styles.slotHeader}>
                <View style={styles.slotTimeRow}>
                  <Ionicons name="time-outline" size={16} color={theme.primary} />
                  <Text style={[styles.slotTime, { color: theme.text }]}>{slot.time}</Text>
                  <Badge
                    label={`الحصة ${slot.slotIndex}`}
                    variant="neutral"
                    size="sm"
                  />
                </View>
                <Badge
                  label={slot.type === 'Lab' ? 'معملي' : slot.type === 'Activity' ? 'نشاط' : 'نظري'}
                  variant={slot.type === 'Lab' ? 'info' : 'primary'}
                  size="sm"
                />
              </View>

              <View style={styles.slotBody}>
                <Text style={[styles.subjectTitle, { color: theme.text }]}>{slot.subject}</Text>
                <Text style={[styles.slotMeta, { color: theme.textSecondary }]}>
                  {slot.teacher} • {slot.room}
                </Text>
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
    justifyContent: 'center',
  },
  dayTabText: {
    ...Typography.captionBold,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.three,
  },
  slotsList: {
    gap: Spacing.three,
  },
  slotCard: {
    padding: Spacing.four,
    gap: Spacing.two,
  },
  slotHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  slotTimeRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 6,
  },
  slotTime: {
    ...Typography.captionBold,
  },
  slotBody: {
    gap: 2,
    marginTop: 2,
  },
  subjectTitle: {
    ...Typography.bodyBold,
  },
  slotMeta: {
    ...Typography.small,
  },
});
