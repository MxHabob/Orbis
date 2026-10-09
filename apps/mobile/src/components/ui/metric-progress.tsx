import React from "react";
import { StyleSheet, Text, View } from "react-native";
import { useTheme } from "@/hooks/use-theme";

export type MetricTone =
  | "primary"
  | "success"
  | "warning"
  | "destructive"
  | "muted"
  | "info"
  | "purple";

export interface MetricSegment {
  label: string;
  value: number;
  tone?: MetricTone;
  displayValue?: string;
}

export interface MetricProgressProps {
  title?: string;
  subtitle?: string;
  value?: number;
  max?: number;
  displayValue?: string | number;
  segments?: readonly MetricSegment[];
  tone?: MetricTone;
  showProgress?: boolean;
}

export const MetricProgress = React.memo<MetricProgressProps>(
  ({
    title,
    subtitle,
    value = 0,
    max = 100,
    displayValue,
    segments = [],
    tone = "primary",
    showProgress = true,
  }) => {
    const theme = useTheme();
    const percentage = Math.min(
      100,
      Math.max(0, (value / Math.max(max, 1)) * 100),
    );
    const segmentTotal = Math.max(
      segments.reduce(
        (total, segment) => total + Math.max(0, segment.value),
        0,
      ),
      1,
    );

    return (
      <View style={styles.container}>
        {(title || subtitle) && (
          <View style={styles.header}>
            <View style={styles.labels}>
              {title && (
                <Text style={[styles.title, { color: theme.text }]}>
                  {title}
                </Text>
              )}
              {subtitle && (
                <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
                  {subtitle}
                </Text>
              )}
            </View>
            {displayValue !== undefined && (
              <Text style={[styles.value, { color: theme.text }]}>
                {displayValue}
              </Text>
            )}
          </View>
        )}

        {showProgress &&
          (segments.length > 0 ? (
            <View style={styles.progressGroup}>
              <View
                style={[styles.progressTrack, { backgroundColor: theme.muted }]}
              >
                {segments.map((segment) => (
                  <View
                    key={segment.label}
                    style={[
                      styles.segment,
                      {
                        width: `${(Math.max(0, segment.value) / segmentTotal) * 100}%`,
                        backgroundColor: getToneColor(segment.tone, theme),
                      },
                    ]}
                  />
                ))}
              </View>
              <View style={styles.legend}>
                {segments.map((segment) => (
                  <View key={segment.label} style={styles.legendItem}>
                    <View
                      style={[
                        styles.dot,
                        { backgroundColor: getToneColor(segment.tone, theme) },
                      ]}
                    />
                    <Text
                      style={[
                        styles.legendText,
                        { color: theme.textSecondary },
                      ]}
                    >
                      {segment.label}
                      {segment.displayValue ? ` ${segment.displayValue}` : ""}
                    </Text>
                  </View>
                ))}
              </View>
            </View>
          ) : (
            <View
              style={[styles.progressTrack, { backgroundColor: theme.muted }]}
            >
              <View
                style={[
                  styles.segment,
                  {
                    width: `${percentage}%`,
                    backgroundColor: getToneColor(tone, theme),
                  },
                ]}
              />
            </View>
          ))}
      </View>
    );
  },
);

function getToneColor(
  tone: MetricTone = "primary",
  theme: ReturnType<typeof useTheme>,
) {
  switch (tone) {
    case "success":
      return theme.success;
    case "warning":
      return theme.warning;
    case "destructive":
      return theme.destructive;
    case "muted":
      return theme.mutedForeground;
    case "info":
      return theme.info;
    case "purple":
      return theme.brandPurple;
    default:
      return theme.primary;
  }
}

const styles = StyleSheet.create({
  container: { width: "100%", gap: 12 },
  header: {
    flexDirection: "row",
    alignItems: "flex-start",
    justifyContent: "space-between",
    gap: 8,
  },
  labels: { flex: 1, gap: 2 },
  title: { ...{"fontSize":14,"lineHeight":20,"fontWeight":"600"} },
  subtitle: { ...{"fontSize":11,"lineHeight":14,"fontWeight":"500"} },
  value: { ...{"fontSize":18,"lineHeight":24,"fontWeight":"700"} },
  progressGroup: { gap: 8 },
  progressTrack: {
    height: 8,
    flexDirection: "row",
    gap: 2,
    overflow: "hidden",
    borderRadius: 9999,
  },
  segment: { height: "100%", minWidth: 2, borderRadius: 9999 },
  legend: { flexDirection: "row", flexWrap: "wrap", gap: 12 },
  legendItem: { flexDirection: "row", alignItems: "center", gap: 4 },
  dot: { width: 6, height: 6, borderRadius: 9999 },
  legendText: { ...{"fontSize":11,"lineHeight":14,"fontWeight":"500"} },
});
