/**
 * @module @orbis/mobile/components/behavior/behavior-timeline-list
 *
 * Timeline list of behavioral events or empty state placeholder.
 */

import React from "react";
import { View, Text, StyleSheet } from "react-native";
import { Ionicons } from "@expo/vector-icons";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import { SkeletonCard } from "@/components/ui/skeleton";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";
import { Spacing, Typography } from "@/constants/theme";

export interface BehaviorRecordItem {
  id: string;
  type: "merit" | "violation";
  category: string;
  points: number;
  description: string;
  reportedBy: string;
  date: string;
}

interface BehaviorTimelineListProps {
  records: BehaviorRecordItem[];
  isLoading?: boolean;
}

export const BehaviorTimelineList = React.memo<BehaviorTimelineListProps>(
  ({ records, isLoading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    if (isLoading) {
      return (
        <View style={styles.container}>
          <SkeletonCard />
          <SkeletonCard />
        </View>
      );
    }

    if (records.length === 0) {
      return (
        <EmptyState
          icon="shield-checkmark-outline"
          title={t("behavior_empty_title")}
          description={t("behavior_empty_desc")}
        />
      );
    }

    return (
      <View style={styles.container}>
        {records.map((item) => {
          const isMerit = item.type === "merit";
          return (
            <Card key={item.id} variant="flat" style={styles.card}>
              <View style={styles.row}>
                <View
                  style={[
                    styles.iconBox,
                    {
                      backgroundColor: isMerit
                        ? theme.successLight
                        : theme.errorLight,
                    },
                  ]}
                >
                  <Ionicons
                    name={isMerit ? "star" : "warning"}
                    size={20}
                    color={isMerit ? theme.success : theme.error}
                  />
                </View>

                <View style={styles.details}>
                  <View style={styles.topRow}>
                    <Text style={[styles.category, { color: theme.text }]}>
                      {item.category}
                    </Text>
                    <Badge
                      label={
                        item.points > 0 ? `+${item.points}` : `${item.points}`
                      }
                      variant={isMerit ? "success" : "error"}
                      size="sm"
                    />
                  </View>

                  <Text style={[styles.desc, { color: theme.textSecondary }]}>
                    {item.description}
                  </Text>

                  <View style={styles.metaRow}>
                    <Text style={[styles.meta, { color: theme.textMuted }]}>
                      {item.reportedBy} • {item.date}
                    </Text>
                  </View>
                </View>
              </View>
            </Card>
          );
        })}
      </View>
    );
  },
);

BehaviorTimelineList.displayName = "BehaviorTimelineList";

const styles = StyleSheet.create({
  container: {
    gap: Spacing.three,
  },
  card: {
    padding: Spacing.four,
  },
  row: {
    flexDirection: "row",
    gap: Spacing.three,
  },
  iconBox: {
    width: 42,
    height: 42,
    borderRadius: 21,
    alignItems: "center",
    justifyContent: "center",
  },
  details: {
    flex: 1,
    gap: 4,
  },
  topRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },
  category: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
  desc: {
    ...Typography.small,
    lineHeight: 18,
  },
  metaRow: {
    flexDirection: "row",
    alignItems: "center",
    marginTop: 2,
  },
  meta: {
    ...Typography.small,
    fontSize: 11,
  },
});
