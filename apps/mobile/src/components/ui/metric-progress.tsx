import React from "react";
import { Text, View } from "react-native";
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

export const MetricProgress = React.memo(function MetricProgress({
    title,
    subtitle,
    value = 0,
    max = 100,
    displayValue,
    segments = [],
    tone = "primary",
    showProgress = true,
}: MetricProgressProps) {
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
      <View className="w-full gap-three">
        {(title || subtitle) && (
          <View className="flex-row items-start justify-between gap-two">
            <View className="flex-1 gap-half">
              {title && (
                <Text className="text-sm leading-5 font-semibold text-foreground">
                  {title}
                </Text>
              )}
              {subtitle && (
                <Text className="text-[11px] leading-[14px] font-medium text-muted-foreground">
                  {subtitle}
                </Text>
              )}
            </View>
            {displayValue !== undefined && (
              <Text className="text-lg leading-6 font-bold text-foreground">
                {displayValue}
              </Text>
            )}
          </View>
        )}

        {showProgress &&
          (segments.length > 0 ? (
            <View className="gap-two">
              <View className="h-2 flex-row gap-half overflow-hidden rounded-full bg-muted">
                {segments.map((segment) => (
                  <View
                    key={segment.label}
                    className="h-full min-w-[2px] rounded-full"
                    style={{
                      width: `${(Math.max(0, segment.value) / segmentTotal) * 100}%`,
                      backgroundColor: getToneColor(segment.tone, theme),
                    }}
                  />
                ))}
              </View>
              <View className="flex-row flex-wrap gap-three">
                {segments.map((segment) => (
                  <View
                    key={segment.label}
                    className="flex-row items-center gap-one"
                  >
                    <View
                      className="h-1.5 w-1.5 rounded-full"
                      style={{
                        backgroundColor: getToneColor(segment.tone, theme),
                      }}
                    />
                    <Text className="text-[11px] leading-[14px] font-medium text-muted-foreground">
                      {segment.label}
                      {segment.displayValue ? ` ${segment.displayValue}` : ""}
                    </Text>
                  </View>
                ))}
              </View>
            </View>
          ) : (
            <View className="h-2 flex-row gap-half overflow-hidden rounded-full bg-muted">
              <View
                className="h-full min-w-[2px] rounded-full"
                style={{
                  width: `${percentage}%`,
                  backgroundColor: getToneColor(tone, theme),
                }}
              />
            </View>
          ))}
      </View>
    );
});

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
