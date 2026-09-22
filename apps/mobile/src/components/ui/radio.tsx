/**
 * @module @orbis/mobile/components/ui/radio
 *
 * Radio button and RadioGroup components for single-selection lists.
 */

import React from "react";
import {
  View,
  Text,
  Pressable,
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";

export interface RadioProps {
  selected: boolean;
  onSelect: () => void;
  label?: string;
  description?: string;
  disabled?: boolean;
  style?: ViewStyle;
}

export function Radio({
  selected,
  onSelect,
  label,
  description,
  disabled = false,
  style,
}: RadioProps) {
  const theme = useTheme();

  return (
    <Pressable
      accessibilityRole="radio"
      accessibilityState={{ selected, disabled }}
      disabled={disabled}
      onPress={disabled ? undefined : onSelect}
      style={({ pressed }) => [
        styles.container,
        { opacity: disabled ? 0.5 : 1 },
        pressed && !disabled && styles.pressed,
        style,
      ]}
    >
      <View
        style={[
          styles.outerCircle,
          {
            borderColor: selected ? theme.primary : theme.border,
            backgroundColor: selected ? theme.accentLight : "transparent",
          },
        ]}
      >
        {selected && (
          <View
            style={[
              styles.innerDot,
              { backgroundColor: theme.primary },
            ]}
          />
        )}
      </View>

      {(label || description) && (
        <View style={styles.textContainer}>
          {label && (
            <Text style={[styles.label, { color: theme.foreground }]}>
              {label}
            </Text>
          )}
          {description && (
            <Text style={[styles.description, { color: theme.textSecondary }]}>
              {description}
            </Text>
          )}
        </View>
      )}
    </Pressable>
  );
}

export interface RadioOption<T extends string = string> {
  value: T;
  label: string;
  description?: string;
  disabled?: boolean;
}

export interface RadioGroupProps<T extends string = string> {
  options: RadioOption<T>[];
  value: T;
  onChange: (value: T) => void;
  style?: ViewStyle;
}

export function RadioGroup<T extends string = string>({
  options,
  value,
  onChange,
  style,
}: RadioGroupProps<T>) {
  return (
    <View style={[styles.group, style]}>
      {options.map((opt) => (
        <Radio
          key={opt.value}
          selected={opt.value === value}
          onSelect={() => onChange(opt.value)}
          label={opt.label}
          description={opt.description}
          disabled={opt.disabled}
        />
      ))}
    </View>
  );
}

const styles = StyleSheet.create({
  group: {
    gap: Spacing.three,
  },
  container: {
    flexDirection: "row",
    alignItems: "flex-start",
    gap: Spacing.two,
    paddingVertical: Spacing.one,
  },
  outerCircle: {
    width: 22,
    height: 22,
    borderRadius: Radii.full,
    borderWidth: 2,
    alignItems: "center",
    justifyContent: "center",
    marginTop: 1,
  },
  innerDot: {
    width: 10,
    height: 10,
    borderRadius: Radii.full,
  },
  textContainer: {
    flex: 1,
  },
  label: {
    ...Typography.bodyBold,
  },
  description: {
    ...Typography.caption,
    marginTop: Spacing.half,
  },
  pressed: {
    opacity: 0.75,
  },
});
