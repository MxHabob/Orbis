/**
 * @module @orbis/mobile/components/layout/screen-scroll-view
 *
 * Screen layout with integrated ScrollView, pull-to-refresh, safe areas, and keyboard handling.
 */

import React from "react";
import {
  ScrollView,
  type ScrollViewProps,
  StyleSheet,
  type ViewStyle,
  RefreshControl,
  type RefreshControlProps,
} from "react-native";
import { Screen, type ScreenProps } from "./screen";
import { Spacing } from "@/constants/theme";

export interface ScreenScrollViewProps extends ScreenProps {
  contentContainerStyle?: ViewStyle | ViewStyle[];
  contentPadding?: keyof typeof Spacing | number;
  horizontalPadding?: keyof typeof Spacing | number;
  verticalPadding?: keyof typeof Spacing | number;
  refreshing?: boolean;
  onRefresh?: () => void;
  refreshControlProps?: Omit<RefreshControlProps, "refreshing" | "onRefresh">;
  scrollViewProps?: Omit<ScrollViewProps, "refreshControl" | "contentContainerStyle">;
}

function resolvePadding(val?: keyof typeof Spacing | number): number | undefined {
  if (val === undefined) return undefined;
  if (typeof val === "number") return val;
  return Spacing[val];
}

export function ScreenScrollView({
  children,
  contentContainerStyle,
  contentPadding,
  horizontalPadding,
  verticalPadding,
  refreshing,
  onRefresh,
  refreshControlProps,
  scrollViewProps,
  ...screenProps
}: ScreenScrollViewProps) {
  const dynamicPadding: ViewStyle = {
    padding: resolvePadding(contentPadding),
    paddingHorizontal: resolvePadding(horizontalPadding),
    paddingVertical: resolvePadding(verticalPadding),
  };

  const refreshControl =
    onRefresh !== undefined ? (
      <RefreshControl
        refreshing={refreshing ?? false}
        onRefresh={onRefresh}
        {...refreshControlProps}
      />
    ) : undefined;

  return (
    <Screen {...screenProps}>
      <ScrollView
        style={styles.scroll}
        contentContainerStyle={[styles.content, dynamicPadding, contentContainerStyle]}
        keyboardShouldPersistTaps="handled"
        showsVerticalScrollIndicator={false}
        refreshControl={refreshControl}
        {...scrollViewProps}
      >
        {children}
      </ScrollView>
    </Screen>
  );
}

const styles = StyleSheet.create({
  scroll: {
    flex: 1,
  },
  content: {
    flexGrow: 1,
  },
});
