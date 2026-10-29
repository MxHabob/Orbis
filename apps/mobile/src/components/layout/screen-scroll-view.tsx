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

type SpacingScale =
  | "half"
  | "one"
  | "two"
  | "three"
  | "four"
  | "five"
  | "six"
  | "eight"
  | "ten"
  | "twelve"
  | "sixteen";

const contentPaddingClasses: Record<SpacingScale, string> = {
  half: "p-half",
  one: "p-one",
  two: "p-two",
  three: "p-three",
  four: "p-four",
  five: "p-five",
  six: "p-six",
  eight: "p-eight",
  ten: "p-ten",
  twelve: "p-twelve",
  sixteen: "p-sixteen",
};

const horizontalPaddingClasses: Record<SpacingScale, string> = {
  half: "px-half",
  one: "px-one",
  two: "px-two",
  three: "px-three",
  four: "px-four",
  five: "px-five",
  six: "px-six",
  eight: "px-eight",
  ten: "px-ten",
  twelve: "px-twelve",
  sixteen: "px-sixteen",
};

const verticalPaddingClasses: Record<SpacingScale, string> = {
  half: "py-half",
  one: "py-one",
  two: "py-two",
  three: "py-three",
  four: "py-four",
  five: "py-five",
  six: "py-six",
  eight: "py-eight",
  ten: "py-ten",
  twelve: "py-twelve",
  sixteen: "py-sixteen",
};

function resolveClass(
  value: SpacingScale | number | undefined,
  classes: Record<SpacingScale, string>,
) {
  return typeof value === "string" ? classes[value] : undefined;
}

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
  const contentPaddingClass = resolveClass(
    contentPadding,
    contentPaddingClasses,
  );
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
