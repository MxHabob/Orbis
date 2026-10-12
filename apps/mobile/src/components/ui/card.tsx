/**
 * @module @orbis/mobile/components/ui/card
 *
 * S.C.R.A.P.S. Tactile Card Component for Orbis Mobile.
 * 100% synchronized with @orbis/ui (packages/ui/globals.css .card-tactile & .card-tactile-interactive).
 */

import React from "react";
import {
  View,
  type ViewProps,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { AnimatedPressable, useTactilePress } from "./tactile-press";

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

const paddingClasses: Record<SpacingScale, string> = {
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

export interface CardProps extends ViewProps {
  variant?: "default" | "elevated" | "outline" | "flat" | "interactive";
  onPress?: () => void;
  padding?: SpacingScale;
  children?: React.ReactNode;
}

export const Card = React.memo<CardProps>(
  ({
    variant = "default",
    onPress,
    padding = "four",
    style,
    children,
    ...props
  }) => {
    const variantClasses: Record<NonNullable<CardProps["variant"]>, string> = {
      default: "bg-card",
      elevated: "bg-card",
      outline: "border border-border bg-transparent",
      flat: "bg-transparent",
      interactive:
        "border border-border border-b-2 border-b-btn-shadow-secondary bg-card",
    };
    const theme = useTheme();
    const tactilePress = useTactilePress(
      Boolean(onPress),
      false,
      theme.btnShadowSecondary,
    );
    const tactileClass =
      onPress && variant !== "interactive"
        ? "border border-border border-b-2 border-b-btn-shadow-secondary"
        : "";
    const className = `rounded-lg ${paddingClasses[padding]} ${variantClasses[variant]} ${tactileClass}`;

    if (onPress) {
      return (
        <AnimatedPressable
          className={className}
          onPress={onPress}
          onPressIn={tactilePress.onPressIn}
          onPressOut={tactilePress.onPressOut}
          style={({ pressed }) => [
            tactilePress.style,
            style,
            pressed && { opacity: 0.92 },
          ]}
          {...props}
        >
          {children}
        </AnimatedPressable>
      );
    }

    return (
      <View className={className} style={style} {...props}>
        {children}
      </View>
    );
  },
);

Card.displayName = "Card";
