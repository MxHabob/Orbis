/**
 * @module @orbis/mobile/components/behavior/behavior-score-banner
 *
 * Summary card presenting net behavior score and discipline indicator.
 */

import React from "react";
import { View, Text, StyleSheet } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { SkeletonCard } from "@/components/ui/skeleton";
import { MetricProgress } from "@/components/ui/metric-progress";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Spacing, Typography } from "@/constants/theme";

interface BehaviorScoreBannerProps {
  totalPoints: number;
  isLoading?: boolean;
}

export const BehaviorScoreBanner = React.memo<BehaviorScoreBannerProps>(
  ({ totalPoints, isLoading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    if (isLoading) {
      return <SkeletonCard />;
    }

    const isPositive = totalPoints >= 0;

    return (
      <View style={styles.container}>
        <View style={styles.row}>
          <View style={styles.metric}>
            <MetricProgress
              title={t("behavior_score_net")}
              displayValue={t("behavior_score_unit", {
                count: totalPoints > 0 ? `+${totalPoints}` : totalPoints,
              })}
              segments={[
                {
                  label: t("behavior_status_regular"),
                  value: Math.max(totalPoints, 0),
                  tone: "success",
                },
                {
                  label: t("behavior_score_net"),
                  value: Math.max(-totalPoints, 0),
                  tone: "destructive",
                },
              ]}
            />
          </View>

          <View
            style={[
              styles.badgeCircle,
              {
                backgroundColor: isPositive
                  ? theme.successLight
                  : theme.errorLight,
              },
            ]}
          >
            <Ionicons
              name={isPositive ? "shield-checkmark" : "alert-circle"}
              size={36}
              color={isPositive ? theme.success : theme.error}
            />
          </View>
        </View>
      </View>
    );
  },
);

BehaviorScoreBanner.displayName = "BehaviorScoreBanner";

const styles = StyleSheet.create({
  container: { gap: Spacing.four },
  row: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },
  metric: { flex: 1 },
  badgeCircle: {
    width: 64,
    height: 64,
    borderRadius: 32,
    alignItems: "center",
    justifyContent: "center",
  },
});
