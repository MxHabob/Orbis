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
  Pressable,
} from "react-native";

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
      interactive: "bg-card",
    };
    const className = `rounded-lg ${paddingClasses[padding]} ${variantClasses[variant]}`;

    if (onPress) {
      return (
        <Pressable
          className={className}
          onPress={onPress}
          style={({ pressed }) => [
            style,
            pressed && { opacity: 0.88, transform: [{ translateY: 1 }] },
          ]}
          {...props}
        >
          {children}
        </Pressable>
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
