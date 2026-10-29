/**
 * @module @orbis/mobile/components/ui/radio
 *
 * Radio button and RadioGroup components for single-selection lists.
 */

import React from "react";
import { View, Text, Pressable, type ViewStyle } from "react-native";

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
  return (
    <Pressable
      accessibilityRole="radio"
      accessibilityState={{ selected, disabled }}
      disabled={disabled}
      onPress={disabled ? undefined : onSelect}
      className="flex-row items-start gap-two py-one"
      style={({ pressed }) => [
        { opacity: disabled ? 0.5 : 1 },
        pressed && !disabled && { opacity: 0.75 },
        style,
      ]}
    >
      <View
        className={`mt-px h-[22px] w-[22px] items-center justify-center rounded-full border-2 ${selected ? "border-primary bg-accent-light" : "border-border bg-transparent"}`}
      >
        {selected && <View className="h-2.5 w-2.5 rounded-full bg-primary" />}
      </View>

      {(label || description) && (
        <View className="flex-1">
          {label && (
            <Text className="text-sm leading-5 font-semibold text-foreground">
              {label}
            </Text>
          )}
          {description && (
            <Text className="mt-0.5 text-xs leading-4 text-muted-foreground">
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
    <View className="gap-three" style={style}>
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
