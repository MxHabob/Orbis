/**
 * @module @orbis/mobile/components/layout/section
 *
 * Page section layout wrapper with title, subtitle, action button, and content slots.
 */

import React from "react";
import {
  View,
  Text,
  type ViewProps,
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

const marginBottomClasses: Record<SpacingScale, string> = {
  half: "mb-half",
  one: "mb-one",
  two: "mb-two",
  three: "mb-three",
  four: "mb-four",
  five: "mb-five",
  six: "mb-six",
  eight: "mb-eight",
  ten: "mb-ten",
  twelve: "mb-twelve",
  sixteen: "mb-sixteen",
};

export interface SectionProps extends ViewProps {
  title?: string;
  subtitle?: string;
  action?: React.ReactNode;
  spacing?: SpacingScale | number;
  headerSpacing?: SpacingScale | number;
  children?: React.ReactNode;
}

export function Section({
  title,
  subtitle,
  action,
  spacing = "four",
  headerSpacing = "two",
  style,
  children,
  ...props
}: SectionProps) {

  const hasHeader = title || subtitle || action;
  const sectionSpacingClass =
    typeof spacing === "string" ? marginBottomClasses[spacing] : undefined;
  const headerSpacingClass =
    typeof headerSpacing === "string"
      ? marginBottomClasses[headerSpacing]
      : undefined;

  return (
    <View
      style={[
        { marginBottom: typeof spacing === "number" ? spacing : undefined },
        style,
      ]}
      className={`w-full ${sectionSpacingClass ?? ""}`}
      {...props}
    >
      {hasHeader && (
        <View
          style={{
            marginBottom:
              typeof headerSpacing === "number" ? headerSpacing : undefined,
          }}
          className={`flex-row items-end justify-between ${headerSpacingClass ?? ""}`}
        >
          <View className="flex-1">
            {title && (
              <Text className="text-[20px] font-semibold text-foreground">
                {title}
              </Text>
            )}
            {subtitle && (
              <Text className="mt-0.5 text-xs text-muted-foreground">
                {subtitle}
              </Text>
            )}
          </View>
          {action && <View className="ml-2">{action}</View>}
        </View>
      )}
      {children}
    </View>
  );
}
