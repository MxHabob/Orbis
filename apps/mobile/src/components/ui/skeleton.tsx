/**
 * @module @orbis/mobile/components/skeleton
 *
 * High-performance loading skeleton placeholders for asynchronous views,
 * lists, bubbles, cards, and profile headers.
 */

import React, { useEffect } from "react";
import { View, type ViewStyle } from "react-native";
import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withRepeat,
  withTiming,
  Easing,
} from "react-native-reanimated";

export interface SkeletonProps {
  width?: number | `${number}%` | "auto";
  height?: number;
  borderRadius?: number;
  style?: ViewStyle;
  className?: string;
  children?: React.ReactNode;
}

export function Skeleton({
  width = "100%",
  height = 20,
  borderRadius = 6,
  style,
  className,
  children,
}: SkeletonProps) {
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
      className={`bg-background-selected ${className ?? ""}`}
      style={[{ width, height, borderRadius }, animatedStyle, style]}
    >
      {children}
    </Animated.View>
  );
}

export function SkeletonBubble({ size = 100 }: { size?: number }) {
  return (
    <View className="items-center gap-two">
      <Skeleton width={size} height={size} borderRadius={size / 2} />
      <Skeleton width={size * 0.75} height={18} borderRadius={9} />
    </View>
  );
}

export function SkeletonFeedCard({ size = 74 }: { size?: number }) {
  return (
    <View className="items-center gap-[6px]">
      <Skeleton width={size} height={size} borderRadius={22} />
      <Skeleton width={size * 0.8} height={12} borderRadius={6} />
    </View>
  );
}

export function SkeletonCard() {
  return (
    <View className="mb-three rounded-lg border border-border p-four">
      <View className="flex-row items-center">
        <Skeleton width={44} height={44} borderRadius={22} />
        <View className="ms-three flex-1">
          <Skeleton width="60%" height={16} />
          <Skeleton width="40%" height={12} style={{ marginTop: 6 }} />
        </View>
      </View>
      <Skeleton width="90%" height={14} style={{ marginTop: 14 }} />
      <Skeleton width="75%" height={14} style={{ marginTop: 8 }} />
    </View>
  );
}
