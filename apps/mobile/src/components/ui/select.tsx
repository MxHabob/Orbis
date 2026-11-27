/**
 * @module @orbis/mobile/components/ui/select
 *
 * S.C.R.A.P.S. Select Component for Orbis Mobile.
 * Uses DropdownMenu's measured Modal overlay so opening the option menu
 * floats over the screen and never shifts surrounding form elements.
 */

import React, { useState } from "react";
import { Pressable, View } from "react-native";
import { ChevronDown } from "@orbis/icons/native";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/drop-down-menu";
import { Text } from "@/components/ui/text";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useTheme } from "@/hooks/use-theme";

export interface SelectOption<T extends string = string> {
  value: T;
  label: string;
}

export interface SelectProps<T extends string = string> {
  label?: string;
  placeholder?: string;
  value: T;
  options: readonly SelectOption<T>[];
  onValueChange: (value: T) => void;
  error?: string | null;
}

export function Select<T extends string = string>({
  label,
  placeholder,
  value,
  options,
  onValueChange,
  error,
}: SelectProps<T>) {
  const theme = useTheme();
  const isDark = useColorScheme() === "dark";
  const [triggerWidth, setTriggerWidth] = useState(280);

  const selectedOption = options.find((option) => option.value === value);

  return (
    <View
      className="w-full gap-1"
      onLayout={(event) => {
        const nextWidth = Math.round(event.nativeEvent.layout.width);
        if (nextWidth > 0 && nextWidth !== triggerWidth) {
          setTriggerWidth(nextWidth);
        }
      }}
    >
      {label ? (
        <Text className="mb-0.5 text-start text-xs leading-4 font-semibold text-foreground">
          {label}
        </Text>
      ) : null}

      <DropdownMenu itemCount={options.length} menuWidth={triggerWidth}>
        <DropdownMenuTrigger asChild>
          <Pressable
            accessibilityRole="button"
            accessibilityLabel={
              label
                ? `${label}: ${selectedOption?.label ?? placeholder ?? ""}`
                : (selectedOption?.label ?? placeholder)
            }
            className={`min-h-12 flex-row items-center justify-between rounded-md border px-3.5 ${
              error ? "border-error" : "border-border"
            } ${isDark ? "bg-background-element" : "bg-background"}`}
          >
            <Text
              numberOfLines={1}
              className={`flex-1 text-start text-[15px] leading-5 ${
                selectedOption ? "text-foreground" : "text-muted-foreground"
              }`}
            >
              {selectedOption?.label ?? placeholder ?? ""}
            </Text>
            <ChevronDown size={18} color={theme.mutedForeground} />
          </Pressable>
        </DropdownMenuTrigger>

        <DropdownMenuContent menuWidth={triggerWidth}>
          {options.map((option) => (
            <DropdownMenuItem
              key={option.value}
              selected={option.value === value}
              onSelect={() => onValueChange(option.value)}
            >
              {option.label}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>

      {error ? (
        <Text className="mt-0.5 text-start text-xs leading-4 font-medium text-destructive">
          {error}
        </Text>
      ) : null}
    </View>
  );
}
