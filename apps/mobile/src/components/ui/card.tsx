/**
 * @module @orbis/mobile/components/ui/card
 *
 * Native counterpart to the @orbis/ui Card root: a compact, bordered surface
 * with an optional tactile press affordance for interactive cards.
 */

import React from "react";
import {
  View,
  type PressableStateCallbackType,
  type ViewProps,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { AnimatedPressable, useTactilePress } from "./tactile-press";

type SpacingScale =
  | "none"
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
  none: "p-0",
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
  className?: string;
  children?: React.ReactNode;
}

export const Card = React.memo<CardProps>(
  ({
    variant = "default",
    onPress,
    padding = "four",
    className: customClassName,
    style,
    children,
    ...props
  }) => {
    const variantClasses: Record<NonNullable<CardProps["variant"]>, string> = {
      default: "overflow-hidden rounded-md border border-border bg-card",
      elevated:
        "overflow-hidden rounded-md border border-border bg-card shadow-sm",
      outline: "overflow-hidden rounded-md border border-border bg-transparent",
      flat: "bg-transparent",
      interactive:
        "overflow-hidden rounded-md border border-border border-b-2 border-b-btn-shadow-secondary bg-card",
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
    const className = [
      paddingClasses[padding],
      variantClasses[variant],
      tactileClass,
      customClassName,
    ]
      .filter(Boolean)
      .join(" ");

    if (onPress) {
      return (
        <AnimatedPressable
          className={className}
          onPress={onPress}
          onPressIn={tactilePress.onPressIn}
          onPressOut={tactilePress.onPressOut}
          style={({ pressed }: PressableStateCallbackType) => [
            tactilePress.shadowStyle,
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
