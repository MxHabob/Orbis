/**
 * @module @orbis/mobile/app/(app)/class/[id]/attendance
 *
 * Fast Class Attendance Sheet for Teachers.
 * Mark attendance with one-tap status toggles and bulk fill.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable, Alert } from 'react-native';
import { useLocalSearchParams } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Radii, Spacing, Typography } from '@/constants/theme';

type AttendanceStatus = 'Present' | 'Late' | 'Excused' | 'Absent';

interface StudentAttendanceEntry {
  studentId: string;
  studentName: string;
  studentNumber: string;
  status: AttendanceStatus;
}

export default function ClassAttendanceScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [saving, setSaving] = useState(false);
  const [entries, setEntries] = useState<StudentAttendanceEntry[]>([
    {
      studentId: 'std-1',
      studentName: 'عمر خالد المنصور',
      studentNumber: 'STD-081',
      status: 'Present',
    },
    {
      studentId: 'std-2',
      studentName: 'سلطان عبدالعزيز الراشد',
      studentNumber: 'STD-082',
      status: 'Present',
    },
    {
      studentId: 'std-3',
      studentName: 'فيصل محمد العتيبي',
      studentNumber: 'STD-083',
      status: 'Present',
    },
    {
      studentId: 'std-4',
      studentName: 'عبدالله ناصر القحطاني',
      studentNumber: 'STD-084',
      status: 'Late',
    },
    {
      studentId: 'std-5',
      studentName: 'تركي سعود الشمري',
      studentNumber: 'STD-085',
      status: 'Absent',
    },
  ]);

  const updateStatus = (studentId: string, status: AttendanceStatus) => {
    setEntries((prev) =>
      prev.map((e) => (e.studentId === studentId ? { ...e, status } : e)),
    );
  };

  const markAllPresent = () => {
    setEntries((prev) => prev.map((e) => ({ ...e, status: 'Present' })));
  };

  const handleSave = () => {
    setSaving(true);
    setTimeout(() => {
      setSaving(false);
      Alert.alert('تم الحفظ بنجاح', 'تم تسجيل واعتماد كشف الحضور للحصة الحالية وإرسال التنبيهات تلقائياً.');
    }, 800);
  };

  const presentCount = entries.filter((e) => e.status === 'Present').length;
  const lateCount = entries.filter((e) => e.status === 'Late').length;
  const absentCount = entries.filter((e) => e.status === 'Absent' || e.status === 'Excused').length;

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="رصد حضور الحصة"
        subtitle="الصف 3 علمي - أ • الحصة الأولى"
        showBack
      />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Attendance Summary & Bulk Fill */}
        <Card variant="elevated" style={styles.topCard}>
          <View style={styles.topRow}>
            <View>
              <Text style={[styles.lessonDate, { color: theme.text }]}>
                اليوم، 7 سبتمبر 2026
              </Text>
              <Text style={[styles.lessonInfo, { color: theme.textSecondary }]}>
                الرياضيات المتقدمة • القاعة 104
              </Text>
            </View>

            <Button
              title="تحضير الكل كـ حاضر"
              onPress={markAllPresent}
              variant="outline"
              size="sm"
              icon={<Ionicons name="checkmark-done" size={16} color={theme.primary} />}
            />
          </View>

          <View style={[styles.statsRow, { borderTopColor: theme.borderSubtle }]}>
            <Badge label={`حاضر: ${presentCount}`} variant="success" size="md" />
            <Badge label={`متأخر: ${lateCount}`} variant="warning" size="md" />
            <Badge label={`غياب: ${absentCount}`} variant="error" size="md" />
          </View>
        </Card>

        {/* Student Attendance List */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>كشف الطلاب ({entries.length})</Text>
        </View>

        <View style={styles.entriesList}>
          {entries.map((entry, idx) => (
            <Card key={entry.studentId} variant="flat" style={styles.entryCard}>
              <View style={styles.entryTop}>
                <Text style={[styles.entryNum, { color: theme.textSecondary }]}>{idx + 1}.</Text>
                <View style={{ flex: 1 }}>
                  <Text style={[styles.entryName, { color: theme.text }]}>
                    {entry.studentName}
                  </Text>
                  <Text style={[styles.entrySub, { color: theme.textSecondary }]}>
                    {entry.studentNumber}
                  </Text>
                </View>
              </View>

              {/* Status Selector Buttons */}
              <View style={styles.statusButtonsRow}>
                {[
                  { key: 'Present', label: 'حاضر', color: theme.success, bg: theme.successLight },
                  { key: 'Late', label: 'متأخر', color: theme.warning, bg: theme.warningLight },
                  { key: 'Excused', label: 'بعذر', color: theme.info, bg: theme.infoLight },
                  { key: 'Absent', label: 'غائب', color: theme.error, bg: theme.errorLight },
                ].map((s) => {
                  const isSelected = entry.status === s.key;
                  return (
                    <Pressable
                      key={s.key}
                      onPress={() => updateStatus(entry.studentId, s.key as AttendanceStatus)}
                      style={[
                        styles.statusBtn,
                        {
                          backgroundColor: isSelected ? s.color : theme.card,
                          borderColor: isSelected ? s.color : theme.border,
                        },
                      ]}>
                      <Text
                        style={[
                          styles.statusBtnText,
                          { color: isSelected ? '#FFFFFF' : theme.textSecondary },
                        ]}>
                        {s.label}
                      </Text>
                    </Pressable>
                  );
                })}
              </View>
            </Card>
          ))}
        </View>

        {/* Submit Attendance Button */}
        <Button
          title="اعتماد وحفظ الكشف"
          onPress={handleSave}
          loading={saving}
          size="lg"
          icon={<Ionicons name="save-outline" size={20} color="#FFFFFF" />}
          fullWidth
          style={{ marginTop: Spacing.two }}
        />
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
  topCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  topRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  lessonDate: {
    ...Typography.bodyBold,
  },
  lessonInfo: {
    ...Typography.small,
    marginTop: 2,
  },
  statsRow: {
    flexDirection: 'row',
    gap: Spacing.two,
    borderTopWidth: 1,
    paddingTop: Spacing.three,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  entriesList: {
    gap: Spacing.three,
  },
  entryCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  entryTop: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
  },
  entryNum: {
    ...Typography.bodyBold,
  },
  entryName: {
    ...Typography.bodyBold,
  },
  entrySub: {
    ...Typography.small,
    marginTop: 2,
  },
  statusButtonsRow: {
    flexDirection: 'row',
    gap: Spacing.two,
  },
  statusBtn: {
    flex: 1,
    paddingVertical: 8,
    borderRadius: Radii.md,
    borderWidth: 1,
    alignItems: 'center',
    justifyContent: 'center',
  },
  statusBtnText: {
    ...Typography.captionBold,
  },
});
