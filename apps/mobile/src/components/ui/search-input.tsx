/**
 * @module @orbis/mobile/components/ui/search-input
 *
 * Search Input with debouncing support, search icon, clear button, and theme support.
 */

import React from "react";
import {
  View,
  TextInput,
  type TextInputProps,
  Pressable,
  ActivityIndicator,
  type ViewStyle,
} from "react-native";
import { Search, ChevronRight } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";

export interface SearchInputProps extends Omit<TextInputProps, "style"> {
  value: string;
  onChangeText: (text: string) => void;
  onClear?: () => void;
  loading?: boolean;
  placeholder?: string;
  containerStyle?: ViewStyle;
}

export function SearchInput({
  value,
  onChangeText,
  onClear,
  loading = false,
  placeholder = "Search...",
  containerStyle,
  ...props
}: SearchInputProps) {
  const theme = useTheme();

  const handleClear = () => {
    onChangeText("");
    onClear?.();
  };

  return (
    <View
      className="h-[42px] flex-row items-center rounded-full border border-border bg-background-element px-three"
      style={containerStyle}
    >
      <View className="me-two items-center justify-center">
        <Search size={18} color={theme.textMuted} />
      </View>

      <TextInput
        className="h-full flex-1 py-0 text-sm leading-5 text-foreground"
        value={value}
        onChangeText={onChangeText}
        placeholder={placeholder}
        placeholderTextColor={theme.textMuted}
        returnKeyType="search"
        clearButtonMode="never"
        autoCapitalize="none"
        autoCorrect={false}
        {...props}
      />

      {loading ? (
        <View className="ms-two items-center justify-center">
          <ActivityIndicator size="small" color={theme.primary} />
        </View>
      ) : value ? (
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Clear search"
          onPress={handleClear}
          hitSlop={8}
          className="ms-two items-center justify-center"
        >
          <View
            className="relative h-[18px] w-[18px] items-center justify-center rounded-full bg-muted-foreground"
          >
            <View className="absolute h-[1.5px] w-2.5 rotate-45 bg-white" />
            <View className="absolute h-[1.5px] w-2.5 -rotate-45 bg-white" />
          </View>
        </Pressable>
      ) : null}
    </View>
  );
}
