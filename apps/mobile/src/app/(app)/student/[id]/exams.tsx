/**
 * @module @orbis/mobile/app/(app)/student/[id]/exams
 *
 * Examination Operations, Seating Numbers, and Exam Schedule Screen.
 */

import React from "react";
import { View, Text, StyleSheet, ScrollView } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { Ionicons } from "@expo/vector-icons";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";
import { Header } from "@/components/header";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Radii, Spacing, Typography } from "@/constants/theme";

interface ExamItem {
  id: string;
  subject: string;
  code: string;
  date: string;
  time: string;
  room: string;
  duration: string;
  status: "Upcoming" | "Completed";
}

export default function StudentExamsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const exams: ExamItem[] = [
    {
      id: "ex-1",
      subject: "الرياضيات المتقدمة (تفاضل وتكامل)",
      code: "MATH-301",
      date: "الأحد، 15 نوفمبر 2026",
      time: "08:30 ص - 10:30 ص",
      duration: "ساعتان",
      room: "المسرح الأكاديمي (القاعة 1)",
      status: "Upcoming",
    },
    {
      id: "ex-2",
      subject: "الفيزياء التطبيقية",
      code: "PHYS-302",
      date: "الثلاثاء، 17 نوفمبر 2026",
      time: "08:30 ص - 10:30 ص",
      duration: "ساعتان",
      room: "المسرح الأكاديمي (القاعة 1)",
      status: "Upcoming",
    },
    {
      id: "ex-3",
      subject: "الكيمياء العضوية",
      code: "CHEM-303",
      date: "الخميس، 19 نوفمبر 2026",
      time: "08:30 ص - 10:00 ص",
      duration: "ساعة ونصف",
      room: "المسرح الأكاديمي (القاعة 1)",
      status: "Upcoming",
    },
    {
      id: "ex-4",
      subject: "اللغة الإنجليزية الأكاديمية",
      code: "ENG-304",
      date: "الأحد، 22 نوفمبر 2026",
      time: "08:30 ص - 10:00 ص",
      duration: "ساعة ونصف",
      room: "قاعة 104",
      status: "Upcoming",
    },
  ];

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="جدول الاختبارات النهائية" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}
      >
        {/* Seating Card */}
        <Card variant="elevated" style={styles.seatingCard}>
          <View style={styles.seatingHeader}>
            <View>
              <Text style={[styles.sessionTitle, { color: theme.text }]}>
                اختبارات الفصل الدراسي الثاني 1447
              </Text>
              <Text style={[styles.sessionSub, { color: theme.textSecondary }]}>
                اللجنة الامتحانية المركزية
              </Text>
            </View>
            <Badge label="معتمد" variant="success" size="sm" />
          </View>

          <View style={styles.seatingDetailsGrid}>
            <View
              style={[
                styles.seatingBox,
                { backgroundColor: theme.primaryLight },
              ]}
            >
              <Text style={[styles.seatingLbl, { color: theme.primary }]}>
                رقم الجلوس
              </Text>
              <Text style={[styles.seatingNum, { color: theme.primary }]}>
                418
              </Text>
            </View>

            <View
              style={[
                styles.seatingBox,
                { backgroundColor: theme.backgroundElement },
              ]}
            >
              <Text style={[styles.seatingLbl, { color: theme.textSecondary }]}>
                مقر الاختبار
              </Text>
              <Text
                style={[styles.seatingNum, { color: theme.text, fontSize: 16 }]}
              >
                المسرح الأكاديمي
              </Text>
            </View>
          </View>

          <View
            style={[styles.secretBox, { backgroundColor: theme.infoLight }]}
          >
            <Ionicons name="shield-checkmark" size={18} color={theme.info} />
            <Text style={[styles.secretText, { color: theme.info }]}>
              نظام التصحيح السري الإلكتروني مفعل لضمان العدالة والموضوعية
              التامة.
            </Text>
          </View>
        </Card>

        {/* Exams Timetable List */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>
            مواعيد الاختبارات
          </Text>
        </View>

        <View style={styles.examsList}>
          {exams.map((exam) => (
            <Card key={exam.id} variant="flat" style={styles.examCard}>
              <View style={styles.examHeader}>
                <View style={{ flex: 1 }}>
                  <Text style={[styles.examSubject, { color: theme.text }]}>
                    {exam.subject}
                  </Text>
                  <Text style={[styles.examDate, { color: theme.primary }]}>
                    {exam.date}
                  </Text>
                </View>
                <Badge label={exam.duration} variant="neutral" size="sm" />
              </View>

              <View
                style={[styles.examDetailsRow, { backgroundColor: theme.card }]}
              >
                <View style={styles.detailItem}>
                  <Ionicons
                    name="time-outline"
                    size={14}
                    color={theme.textSecondary}
                  />
                  <Text
                    style={[styles.detailText, { color: theme.textSecondary }]}
                  >
                    {exam.time}
                  </Text>
                </View>
                <View style={styles.detailItem}>
                  <Ionicons
                    name="location-outline"
                    size={14}
                    color={theme.textSecondary}
                  />
                  <Text
                    style={[styles.detailText, { color: theme.textSecondary }]}
                  >
                    {exam.room}
                  </Text>
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
  seatingCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  seatingHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "flex-start",
  },
  sessionTitle: {
    ...Typography.bodyBold,
  },
  sessionSub: {
    ...Typography.small,
    marginTop: 2,
  },
  seatingDetailsGrid: {
    flexDirection: "row",
    gap: Spacing.two,
  },
  seatingBox: {
    flex: 1,
    padding: Spacing.three,
    borderRadius: Radii.md,
    alignItems: "center",
    gap: 2,
  },
  seatingLbl: {
    ...Typography.small,
  },
  seatingNum: {
    ...Typography.h2,
    fontWeight: "800",
  },
  secretBox: {
    flexDirection: "row",
    alignItems: "center",
    padding: Spacing.three,
    borderRadius: Radii.md,
    gap: Spacing.two,
  },
  secretText: {
    ...Typography.small,
    flex: 1,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  examsList: {
    gap: Spacing.three,
  },
  examCard: {
    padding: Spacing.four,
    gap: Spacing.two,
  },
  examHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "flex-start",
  },
  examSubject: {
    ...Typography.bodyBold,
  },
  examDate: {
    ...Typography.captionBold,
    marginTop: 2,
  },
  examDetailsRow: {
    flexDirection: "row",
    justifyContent: "space-between",
    padding: Spacing.two,
    borderRadius: Radii.sm,
    marginTop: 4,
  },
  detailItem: {
    flexDirection: "row",
    alignItems: "center",
    gap: 4,
  },
  detailText: {
    ...Typography.small,
  },
});
