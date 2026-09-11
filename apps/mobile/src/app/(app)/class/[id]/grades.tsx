/**
 * @module @orbis/mobile/app/(app)/class/[id]/grades
 *
 * Class Gradebook & Score Entry Screen for Teachers.
 *
 * - Full-width screen gradient.
 * - Gradient extends behind StatusBar.
 * - Header remains transparent.
 * - Fast score entry.
 * - Theme-aware UI.
 */

import { useState } from "react";

import {
  Alert,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  TextInput,
  View,
} from "react-native";

import { useLocalSearchParams } from "expo-router";

import { Ionicons } from "@expo/vector-icons";

import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";

import { Header } from "@/components/header";

import { Card } from "@/components/ui/card";

import { Button } from "@/components/ui/button";

import { ScreenBackground } from "@/components/screen-background";


import { Radii, Spacing, Typography } from "@/constants/theme";

// ─────────────────────────────────────────────────────────────────────────
// Types
// ─────────────────────────────────────────────────────────────────────────

interface StudentGradeRow {
  id: string;

  name: string;

  studentNumber: string;

  score: string;

  notes?: string;

  isExcused?: boolean;
}

type AssessmentScheme = "midterm" | "coursework" | "practical";

// ─────────────────────────────────────────────────────────────────────────
// Screen
// ─────────────────────────────────────────────────────────────────────────

export default function ClassGradesScreen() {
  const insets = useSafeAreaInsets();

  const theme = useTheme();

  const { id } = useLocalSearchParams<{
    id: string;
  }>();

  // ───────────────────────────────────────────────────────────────────────
  // State
  // ───────────────────────────────────────────────────────────────────────

  const [activeScheme, setActiveScheme] = useState<AssessmentScheme>("midterm");

  const [saving, setSaving] = useState(false);

  const [grades, setGrades] = useState<StudentGradeRow[]>([
    {
      id: "std-1",

      name: "عمر خالد المنصور",

      studentNumber: "STD-081",

      score: "29",
    },

    {
      id: "std-2",

      name: "سلطان عبدالعزيز الراشد",

      studentNumber: "STD-082",

      score: "30",
    },

    {
      id: "std-3",

      name: "فيصل محمد العتيبي",

      studentNumber: "STD-083",

      score: "26",
    },

    {
      id: "std-4",

      name: "عبدالله ناصر القحطاني",

      studentNumber: "STD-084",

      score: "24",
    },

    {
      id: "std-5",

      name: "تركي سعود الشمري",

      studentNumber: "STD-085",

      score: "28",
    },
  ]);

  // ───────────────────────────────────────────────────────────────────────
  // Actions
  // ───────────────────────────────────────────────────────────────────────

  const updateScore = (studentId: string, score: string) => {
    setGrades((previous) =>
      previous.map((grade) =>
        grade.id === studentId
          ? {
              ...grade,
              score,
            }
          : grade,
      ),
    );
  };

  const handleSave = () => {
    setSaving(true);

    setTimeout(() => {
      setSaving(false);

      Alert.alert(
        "تم حفظ الدرجات",
        "تم تسجيل درجات التقييم بنجاح وتحديث السجلات الأكاديمية للطلاب.",
      );
    }, 700);
  };

  // ───────────────────────────────────────────────────────────────────────
  // Maximum Score
  // ───────────────────────────────────────────────────────────────────────

  const maxScore =
    activeScheme === "midterm" ? 30 : activeScheme === "coursework" ? 20 : 10;

  // ───────────────────────────────────────────────────────────────────────
  // Render
  // ───────────────────────────────────────────────────────────────────────

  return (
    <ScreenBackground gradientHeight={0.48}>
      {/* ─────────────────────────────────────────────────────────────
          Header

          Header itself does not contain a gradient.
        ───────────────────────────────────────────────────────────── */}

      <Header
        title="دفتر رصد الدرجات"
        subtitle="الصف 3 علمي - أ • الرياضيات"
        showBack
      />

      {/* ─────────────────────────────────────────────────────────────
          Main Content
        ───────────────────────────────────────────────────────────── */}

      <ScrollView
        style={styles.scrollView}
        contentContainerStyle={[
          styles.scrollContent,

          {
            paddingBottom: insets.bottom + Spacing.eight,
          },
        ]}
        showsVerticalScrollIndicator={false}
      >
        {/* ─────────────────────────────────────────────────────────
            Assessment Scheme Selector
          ───────────────────────────────────────────────────────── */}

        <View style={styles.schemeSelector}>
          {[
            {
              key: "midterm" as const,

              label: "الاختبار النصفي (30 درجة)",
            },

            {
              key: "coursework" as const,

              label: "الأعمال والمشاريع (20 درجة)",
            },

            {
              key: "practical" as const,

              label: "التقييم العملي (10 درجات)",
            },
          ].map((scheme) => {
            const isSelected = activeScheme === scheme.key;

            return (
              <Pressable
                key={scheme.key}
                onPress={() => setActiveScheme(scheme.key)}
                style={[
                  styles.schemeTab,

                  {
                    backgroundColor: isSelected ? theme.primary : theme.card,

                    borderColor: isSelected ? theme.primary : theme.border,
                  },
                ]}
              >
                <Text
                  style={[
                    styles.schemeText,

                    {
                      color: isSelected
                        ? theme.primaryForeground
                        : theme.textSecondary,
                    },
                  ]}
                >
                  {scheme.label}
                </Text>
              </Pressable>
            );
          })}
        </View>

        {/* ─────────────────────────────────────────────────────────
            Section Header
          ───────────────────────────────────────────────────────── */}

        <View style={styles.sectionHeader}>
          <Text
            style={[
              styles.sectionTitle,

              {
                color: theme.text,
              },
            ]}
          >
            درجات الطلاب
          </Text>

          <Text
            style={[
              styles.sectionSub,

              {
                color: theme.textSecondary,
              },
            ]}
          >
            الدرجة العظمى: {maxScore}
          </Text>
        </View>

        {/* ─────────────────────────────────────────────────────────
            Student Score List
          ───────────────────────────────────────────────────────── */}

        <View style={styles.gradesList}>
          {grades.map((item, index) => {
            return (
              <Card key={item.id} variant="elevated" style={styles.gradeCard}>
                <View style={styles.gradeRow}>
                  {/* ───────────────────────────────────────
                        Student
                      ─────────────────────────────────────── */}

                  <View style={styles.stdCol}>
                    <Text
                      style={[
                        styles.stdName,

                        {
                          color: theme.text,
                        },
                      ]}
                      numberOfLines={1}
                    >
                      {index + 1}. {item.name}
                    </Text>

                    <Text
                      style={[
                        styles.stdNum,

                        {
                          color: theme.textSecondary,
                        },
                      ]}
                    >
                      {item.studentNumber}
                    </Text>
                  </View>

                  {/* ───────────────────────────────────────
                        Score
                      ─────────────────────────────────────── */}

                  <View style={styles.scoreInputBox}>
                    <TextInput
                      value={item.score}
                      onChangeText={(value) => updateScore(item.id, value)}
                      keyboardType="numeric"
                      maxLength={3}
                      selectTextOnFocus
                      style={[
                        styles.scoreInput,

                        {
                          backgroundColor: theme.backgroundElement,

                          color: theme.text,

                          borderColor: theme.border,
                        },
                      ]}
                    />

                    <Text
                      style={[
                        styles.maxScoreLbl,

                        {
                          color: theme.textSecondary,
                        },
                      ]}
                    >
                      / {maxScore}
                    </Text>
                  </View>
                </View>
              </Card>
            );
          })}
        </View>

        {/* ─────────────────────────────────────────────────────────
            Save
          ───────────────────────────────────────────────────────── */}

        <Button
          title="حفظ واعتماد الدرجات"
          onPress={handleSave}
          loading={saving}
          size="lg"
          icon={
            <Ionicons
              name="cloud-upload-outline"
              size={20}
              color={theme.primaryForeground}
            />
          }
          fullWidth
          style={styles.saveButton}
        />
      </ScrollView>
    </ScreenBackground>
  );
}

// ─────────────────────────────────────────────────────────────────────────
// Styles
// ─────────────────────────────────────────────────────────────────────────

const styles = StyleSheet.create({
  // ───────────────────────────────────────────────────────────────────────
  // Scroll
  // ───────────────────────────────────────────────────────────────────────

  scrollView: {
    flex: 1,

    width: "100%",
  },

  scrollContent: {
    paddingHorizontal: Spacing.four,

    paddingTop: Spacing.four,

    gap: Spacing.four,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Assessment Scheme
  // ───────────────────────────────────────────────────────────────────────

  schemeSelector: {
    width: "100%",

    gap: Spacing.two,
  },

  schemeTab: {
    width: "100%",

    paddingVertical: Spacing.three,

    paddingHorizontal: Spacing.four,

    borderRadius: Radii.md,

    borderWidth: 1,

    alignItems: "center",

    justifyContent: "center",
  },

  schemeText: {
    ...Typography.captionBold,

    textAlign: "center",
  },

  // ───────────────────────────────────────────────────────────────────────
  // Section Header
  // ───────────────────────────────────────────────────────────────────────

  sectionHeader: {
    flexDirection: "row",

    justifyContent: "space-between",

    alignItems: "center",

    marginTop: Spacing.two,

    gap: Spacing.two,
  },

  sectionTitle: {
    ...Typography.h3,
  },

  sectionSub: {
    ...Typography.captionBold,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Grades
  // ───────────────────────────────────────────────────────────────────────

  gradesList: {
    width: "100%",

    gap: Spacing.two,
  },

  gradeCard: {
    padding: Spacing.three,
  },

  gradeRow: {
    width: "100%",

    flexDirection: "row",

    justifyContent: "space-between",

    alignItems: "center",

    gap: Spacing.three,
  },

  stdCol: {
    flex: 1,

    minWidth: 0,

    gap: 2,
  },

  stdName: {
    ...Typography.bodyBold,
  },

  stdNum: {
    ...Typography.small,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Score
  // ───────────────────────────────────────────────────────────────────────

  scoreInputBox: {
    flexDirection: "row",

    alignItems: "center",

    gap: 6,

    flexShrink: 0,
  },

  scoreInput: {
    width: 60,

    height: 44,

    borderRadius: Radii.md,

    borderWidth: 1.5,

    textAlign: "center",

    fontSize: 18,

    fontWeight: "700",
  },

  maxScoreLbl: {
    ...Typography.captionBold,

    minWidth: 28,
  },

  // ───────────────────────────────────────────────────────────────────────
  // Save
  // ───────────────────────────────────────────────────────────────────────

  saveButton: {
    marginTop: Spacing.two,
  },
});
