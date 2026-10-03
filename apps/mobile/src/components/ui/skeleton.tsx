/**
 * @module @orbis/mobile/components/skeleton
 *
 * High-performance loading skeleton placeholders for asynchronous views,
 * lists, bubbles, cards, and profile headers.
 */

import React, { useEffect } from "react";
import { View, StyleSheet, type ViewStyle } from "react-native";
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withRepeat,
  withTiming,
  Easing,
} from "react-native-reanimated";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing } from "@/constants/theme";

export interface SkeletonProps {
  width?: number | `${number}%` | "auto";
  height?: number;
  borderRadius?: number;
  style?: ViewStyle;
}

export function Skeleton({
  width = "100%",
  height = 20,
  borderRadius = Radii.md,
  style,
}: SkeletonProps) {
  const theme = useTheme();
  const opacity = useSharedValue(0.35);

  useEffect(() => {
    opacity.value = withRepeat(
      withTiming(0.8, { duration: 750, easing: Easing.inOut(Easing.ease) }),
      -1,
      true,
    );
  }, [opacity]);

  const animatedStyle = useAnimatedStyle(() => ({
    opacity: opacity.value,
  }));

  return (
    <Animated.View
      style={[
        {
          width,
          height,
          borderRadius,
          backgroundColor: theme.backgroundSelected || "#E4E4E7",
        },
        animatedStyle,
        style,
      ]}
    />
  );
}

export function SkeletonBubble({ size = 100 }: { size?: number }) {
  return (
    <View style={{ alignItems: "center", gap: 8 }}>
      <Skeleton width={size} height={size} borderRadius={size / 2} />
      <Skeleton width={size * 0.75} height={18} borderRadius={9} />
    </View>
  );
}

export function SkeletonFeedCard({ size = 74 }: { size?: number }) {
  return (
    <View style={{ alignItems: "center", gap: 6 }}>
      <Skeleton width={size} height={size} borderRadius={22} />
      <Skeleton width={size * 0.8} height={12} borderRadius={6} />
    </View>
  );
}

export function SkeletonCard() {
  return (
    <View style={styles.cardContainer}>
      <View style={styles.row}>
        <Skeleton width={44} height={44} borderRadius={22} />
        <View style={styles.col}>
          <Skeleton width="60%" height={16} />
          <Skeleton width="40%" height={12} style={{ marginTop: 6 }} />
        </View>
      </View>
      <Skeleton width="90%" height={14} style={{ marginTop: 14 }} />
      <Skeleton width="75%" height={14} style={{ marginTop: 8 }} />
    </View>
  );
}

const styles = StyleSheet.create({
  cardContainer: {
    padding: Spacing.four,
    borderRadius: Radii.lg,
    borderWidth: 1,
    borderColor: "#E2E8F030",
    marginBottom: Spacing.three,
  },
  row: {
    flexDirection: "row",
    alignItems: "center",
  },
  col: {
    marginLeft: Spacing.three,
    flex: 1,
  },
});
