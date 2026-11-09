/**
 * @module @orbis/mobile/components/layout/screen-scroll-view
 *
 * Screen layout with integrated ScrollView, pull-to-refresh, safe areas, and keyboard handling.
 */

import {
  RefreshControl,
  type RefreshControlProps,
  type ScrollViewProps,
  type ViewStyle,
} from "react-native";
import { KeyboardAwareScrollView } from "react-native-keyboard-controller";
import { Screen, type ScreenProps } from "./screen";
import {
  type SpacingScale,
  paddingClasses,
  horizontalPaddingClasses,
  verticalPaddingClasses,
  resolveClass,
} from "@/lib/design-tokens";

export interface ScreenScrollViewProps extends ScreenProps {
  contentContainerStyle?: ViewStyle | ViewStyle[];
  contentPadding?: SpacingScale | number;
  horizontalPadding?: SpacingScale | number;
  verticalPadding?: SpacingScale | number;
  refreshing?: boolean;
  onRefresh?: () => void;
  refreshControlProps?: Omit<RefreshControlProps, "refreshing" | "onRefresh">;
  scrollViewProps?: Omit<
    ScrollViewProps,
    "refreshControl" | "contentContainerStyle"
  >;
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
    padding: typeof contentPadding === "number" ? contentPadding : undefined,
    paddingHorizontal:
      typeof horizontalPadding === "number" ? horizontalPadding : undefined,
    paddingVertical:
      typeof verticalPadding === "number" ? verticalPadding : undefined,
  };
  const contentPaddingClass = resolveClass(contentPadding, paddingClasses);
  const horizontalPaddingClass = resolveClass(
    horizontalPadding,
    horizontalPaddingClasses,
  );
  const verticalPaddingClass = resolveClass(
    verticalPadding,
    verticalPaddingClasses,
  );
  const contentClasses = [
    "flex-grow",
    contentPaddingClass,
    horizontalPaddingClass,
    verticalPaddingClass,
  ]
    .filter(Boolean)
    .join(" ");

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
      <KeyboardAwareScrollView
        className="flex-1"
        contentContainerClassName={contentClasses}
        contentContainerStyle={[dynamicPadding, contentContainerStyle]}
        keyboardShouldPersistTaps="handled"
        showsVerticalScrollIndicator={false}
        refreshControl={refreshControl}
        {...scrollViewProps}
      >
        {children}
      </KeyboardAwareScrollView>
    </Screen>
  );
}
