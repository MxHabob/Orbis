/**
 * @module @orbis/mobile/components/home/personalized-features-grid
 *
 * 4-card squircle features grid for student academic domains.
 */

import React from "react";
import { View, Text, StyleSheet, Pressable, Dimensions } from "react-native";
import {
  AttendanceFeatureIllustration,
  GradesFeatureIllustration,
  BehaviorFeatureIllustration,
  ScheduleFeatureIllustration,
} from "@/components/glovo-illustrations";
import { Skeleton, SkeletonFeedCard } from "@/components/ui/skeleton";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";

const { width: SCREEN_WIDTH } = Dimensions.get("window");

interface PersonalizedFeaturesGridProps {
  isLoading?: boolean;
  onAttendancePress: () => void;
  onGradesPress: () => void;
  onBehaviorPress: () => void;
  onSchedulePress: () => void;
}

export const PersonalizedFeaturesGrid =
  React.memo<PersonalizedFeaturesGridProps>(
    ({
      isLoading = false,
      onAttendancePress,
      onGradesPress,
      onBehaviorPress,
      onSchedulePress,
    }) => {
      const theme = useTheme();
      const { t } = useI18n();
      const cardSize = (SCREEN_WIDTH - 56) / 4;

      return (
        <View
          style={[
            styles.card,
          ]}
        >
          <View style={[styles.content, { backgroundColor: theme.background }]}>
            {/* 4 Feature Cards */}
            {isLoading ? (
              <View style={styles.cardsRow}>
                <SkeletonFeedCard size={cardSize} />
                <SkeletonFeedCard size={cardSize} />
                <SkeletonFeedCard size={cardSize} />
                <SkeletonFeedCard size={cardSize} />
              </View>
            ) : (
              <View style={styles.cardsRow}>
                {/* 1. Attendance */}
                <Pressable
                  onPress={onAttendancePress}
                  style={({ pressed }) => [
                    styles.featureItem,
                    pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                  ]}
                >
                  <View
                    style={[
                      styles.squircleBox,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.border,
                      },
                    ]}
                  >
                    <AttendanceFeatureIllustration size={44} />
                  </View>
                  <Text
                    style={[styles.featureLabel, { color: theme.text }]}
                    numberOfLines={1}
                  >
                    {t("domain_attendance_log")}
                  </Text>
                </Pressable>

                {/* 2. Grades */}
                <Pressable
                  onPress={onGradesPress}
                  style={({ pressed }) => [
                    styles.featureItem,
                    pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                  ]}
                >
                  <View
                    style={[
                      styles.squircleBox,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.border,
                      },
                    ]}
                  >
                    <GradesFeatureIllustration size={44} />
                  </View>
                  <Text
                    style={[styles.featureLabel, { color: theme.text }]}
                    numberOfLines={1}
                  >
                    {t("domain_grades")}
                  </Text>
                </Pressable>

                {/* 3. Behavior */}
                <Pressable
                  onPress={onBehaviorPress}
                  style={({ pressed }) => [
                    styles.featureItem,
                    pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                  ]}
                >
                  <View
                    style={[
                      styles.squircleBox,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.border,
                      },
                    ]}
                  >
                    <BehaviorFeatureIllustration size={44} />
                  </View>
                  <Text
                    style={[styles.featureLabel, { color: theme.text }]}
                    numberOfLines={1}
                  >
                    {t("domain_excellence_behavior")}
                  </Text>
                </Pressable>

                {/* 4. Schedule */}
                <Pressable
                  onPress={onSchedulePress}
                  style={({ pressed }) => [
                    styles.featureItem,
                    pressed && { opacity: 0.85, transform: [{ scale: 0.96 }] },
                  ]}
                >
                  <View
                    style={[
                      styles.squircleBox,
                      {
                        backgroundColor: theme.card,
                        borderColor: theme.border,
                      },
                    ]}
                  >
                    <ScheduleFeatureIllustration size={44} />
                  </View>
                  <Text
                    style={[styles.featureLabel, { color: theme.text }]}
                    numberOfLines={1}
                  >
                    {t("domain_schedule")}
                  </Text>
                </Pressable>
              </View>
            )}

            {/* Pagination Dots */}
            <View style={styles.paginationRow}>
              <View
                style={[
                  styles.dot,
                  styles.dotActive,
                  { backgroundColor: theme.primary },
                ]}
              />
              <View style={[styles.dot, { backgroundColor: theme.border }]} />
            </View>
          </View>
        </View>
      );
    },
  );

PersonalizedFeaturesGrid.displayName = "PersonalizedFeaturesGrid";

const styles = StyleSheet.create({
  card: {
    marginHorizontal: -Spacing.four,
  },
  content: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.three,
    gap: Spacing.three,
  },
  cardsRow: {
    flexDirection: "row",
    alignItems: "flex-start",
    justifyContent: "space-between",
    gap: Spacing.two,
  },
  featureItem: {
    flex: 1,
    alignItems: "center",
    gap: Spacing.one,
  },
  squircleBox: {
    width: "100%",
    aspectRatio: 1,
    maxWidth: 76,
    maxHeight: 76,
    borderRadius: Radii.xl,
    backgroundColor: "transparent",
    borderWidth: 1,
    borderColor: "transparent",
    alignItems: "center",
    justifyContent: "center",
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.04,
    shadowRadius: 6,
    elevation: 2,
  },
  featureLabel: {
    ...Typography.small,
    ...Typography.captionBold,
    textAlign: "center",
  },
  paginationRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    gap: 6,
    marginTop: 2,
  },
  dot: {
    width: 6,
    height: 6,
    borderRadius: 3,
    backgroundColor: "#E4E4E7",
  },
  dotActive: {
    width: 16,
    backgroundColor: "#18181B",
  },
});
