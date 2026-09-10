/**
 * @module @orbis/mobile/components/auth/student-registration-success
 *
 * Celebratory confirmation card for completed Student Admission applications.
 * Details the physical school visit instructions and next steps for student activation.
 */

import React from "react";
import { View, Text, StyleSheet, Pressable } from "react-native";
import { router } from "expo-router";
import { Ionicons } from "@expo/vector-icons";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Radii, Spacing, Typography } from "@/constants/theme";

interface StudentRegistrationSuccessProps {
  referenceNumber: string;
  studentName: string;
  onReset: () => void;
}

export const StudentRegistrationSuccess =
  React.memo<StudentRegistrationSuccessProps>(
    ({ referenceNumber, studentName, onReset }) => {
      const theme = useTheme();
      const { t } = useI18n();

      return (
        <View style={styles.container}>
          <Card variant="elevated" style={styles.card}>
            {/* Celebratory Badge */}
            <View style={styles.header}>
              <View
                style={[
                  styles.iconCircle,
                  { backgroundColor: theme.successLight },
                ]}
              >
                <Ionicons
                  name="checkmark-circle"
                  size={48}
                  color={theme.success}
                />
              </View>

              <Text style={[styles.title, { color: theme.text }]}>
                {t("admission_success_title")}
              </Text>

              <Text style={[styles.studentName, { color: theme.primary }]}>
                {studentName}
              </Text>
            </View>

            {/* Reference Number Badge */}
            <View
              style={[
                styles.refBadge,
                {
                  backgroundColor: theme.backgroundElement,
                  borderColor: theme.border,
                },
              ]}
            >
              <Text style={[styles.refLabel, { color: theme.textMuted }]}>
                {t("admission_success_ref")}
              </Text>
              <Text style={[styles.refCode, { color: theme.text }]}>
                {referenceNumber}
              </Text>
            </View>

            {/* School Visit Instructions */}
            <View
              style={[
                styles.instructionBox,
                {
                  backgroundColor: theme.primaryLight,
                  borderColor: theme.primary,
                },
              ]}
            >
              <Ionicons
                name="information-circle"
                size={22}
                color={theme.primary}
              />
              <Text style={[styles.instructionText, { color: theme.text }]}>
                {t("admission_success_notice")}
              </Text>
            </View>

            {/* Steps List */}
            <View style={styles.stepsSection}>
              <Text style={[styles.stepsTitle, { color: theme.text }]}>
                {t("admission_success_steps_title")}
              </Text>

              <View style={styles.stepRow}>
                <View
                  style={[styles.stepDot, { backgroundColor: theme.primary }]}
                />
                <Text style={[styles.stepText, { color: theme.textSecondary }]}>
                  {t("admission_success_step1")}
                </Text>
              </View>

              <View style={styles.stepRow}>
                <View
                  style={[styles.stepDot, { backgroundColor: theme.primary }]}
                />
                <Text style={[styles.stepText, { color: theme.textSecondary }]}>
                  {t("admission_success_step2")}
                </Text>
              </View>

              <View style={styles.stepRow}>
                <View
                  style={[styles.stepDot, { backgroundColor: theme.primary }]}
                />
                <Text style={[styles.stepText, { color: theme.textSecondary }]}>
                  {t("admission_success_step3")}
                </Text>
              </View>
            </View>

            {/* Action Buttons */}
            <View style={styles.actions}>
              <Button
                title={t("admission_return_signin")}
                onPress={() => router.replace("/(auth)/sign-in")}
                fullWidth
                size="lg"
              />
              <Button
                title={t("admission_new_another")}
                onPress={onReset}
                variant="outline"
                fullWidth
              />
            </View>
          </Card>
        </View>
      );
    },
  );

StudentRegistrationSuccess.displayName = "StudentRegistrationSuccess";

const styles = StyleSheet.create({
  container: {
    width: "100%",
  },
  card: {
    padding: Spacing.four,
    gap: Spacing.four,
    alignItems: "center",
  },
  header: {
    alignItems: "center",
    gap: Spacing.two,
  },
  iconCircle: {
    width: 80,
    height: 80,
    borderRadius: 40,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: Spacing.one,
  },
  title: {
    ...Typography.h2,
    fontSize: 22,
    textAlign: "center",
  },
  studentName: {
    ...Typography.bodyBold,
    fontSize: 16,
    textAlign: "center",
  },
  refBadge: {
    width: "100%",
    padding: Spacing.three,
    borderRadius: Radii.lg,
    borderWidth: 1.5,
    borderStyle: "dashed",
    alignItems: "center",
    gap: 4,
  },
  refLabel: {
    ...Typography.caption,
    fontSize: 12,
  },
  refCode: {
    ...Typography.h2,
    fontSize: 20,
    letterSpacing: 1.5,
    fontFamily: "monospace",
  },
  instructionBox: {
    flexDirection: "row",
    alignItems: "flex-start",
    gap: Spacing.two,
    padding: Spacing.three,
    borderRadius: Radii.md,
    borderWidth: 1,
    width: "100%",
  },
  instructionText: {
    ...Typography.captionBold,
    flex: 1,
    lineHeight: 20,
  },
  stepsSection: {
    width: "100%",
    gap: Spacing.two,
  },
  stepsTitle: {
    ...Typography.captionBold,
    fontSize: 14,
    marginBottom: 4,
  },
  stepRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
  },
  stepDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
  },
  stepText: {
    ...Typography.caption,
    flex: 1,
  },
  actions: {
    width: "100%",
    gap: Spacing.two,
    marginTop: Spacing.two,
  },
});
