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
  type ViewStyle,
  Pressable,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";

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
    const theme = useTheme();

    const getVariantStyle = (): ViewStyle => {
      switch (variant) {
        case "elevated":
        case "interactive":
          return {
            backgroundColor: theme.card,
            borderWidth: 0,
            shadowOpacity: 0,
            elevation: 0,
          };
        case "outline":
          return {
            backgroundColor: "transparent",
            borderColor: theme.border,
            borderWidth: 1,
          };
        case "flat":
          return { backgroundColor: "transparent", borderWidth: 0 };
        case "default":
        default:
          return {
            backgroundColor: theme.card,
            borderWidth: 0,
            shadowOpacity: 0,
            elevation: 0,
          };
      }
    };

    const containerStyle: ViewStyle = {
      ...getVariantStyle(),
    };
    const className = `rounded-lg ${paddingClasses[padding]}`;

    if (onPress) {
      return (
        <Pressable
          className={className}
          onPress={onPress}
          style={({ pressed }) => [
            containerStyle,
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
      <View className={className} style={[containerStyle, style]} {...props}>
        {children}
      </View>
    );
  },
);

Card.displayName = "Card";
