/**
 * @module @orbis/mobile/components/behavior/behavior-ai-card
 *
 * Card displaying automated AI insights on student conduct and engagement.
 */

import React from "react";
import { View, Text, StyleSheet } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { SkeletonCard } from "@/components/ui/skeleton";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Spacing, Typography } from "@/constants/theme";

interface BehaviorAiCardProps {
  hasRecords: boolean;
  isLoading?: boolean;
}

export const BehaviorAiCard = React.memo<BehaviorAiCardProps>(
  ({ hasRecords, isLoading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    if (isLoading) {
      return <SkeletonCard />;
    }

    return (
      <Card
        variant="outline"
        style={[
          styles.card,
          {
            borderColor: theme.primaryLight,
            backgroundColor: theme.accentLight,
          },
        ]}
      >
        <View style={styles.header}>
          <View style={styles.titleBox}>
            <Ionicons name="sparkles" size={18} color={theme.primary} />
            <Text style={[styles.title, { color: theme.text }]}>
              {t("behavior_ai_title")}
            </Text>
          </View>
          <Badge label={t("behavior_badge_live")} variant="primary" size="sm" />
        </View>

        <Text style={[styles.summary, { color: theme.textSecondary }]}>
          {hasRecords
            ? t("behavior_ai_summary_active")
            : t("behavior_ai_summary_empty")}
        </Text>
      </Card>
    );
  },
);

BehaviorAiCard.displayName = "BehaviorAiCard";

const styles = StyleSheet.create({
  card: {
    padding: Spacing.four,
    borderWidth: 1.5,
    gap: Spacing.two,
  },
  header: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },
  titleBox: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
  },
  title: {
    ...Typography.bodyBold,
    fontSize: 15,
  },
  summary: {
    ...Typography.small,
    lineHeight: 20,
  },
});
