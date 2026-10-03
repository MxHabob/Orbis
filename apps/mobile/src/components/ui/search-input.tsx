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
  StyleSheet,
  type ViewStyle,
} from "react-native";
import { Search, ChevronRight } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";
import { Radii, Spacing, Typography } from "@/constants/theme";

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
      style={[
        styles.container,
        {
          backgroundColor: theme.backgroundElement,
          borderColor: theme.border,
        },
        containerStyle,
      ]}
    >
      <View style={styles.iconBox}>
        <Search size={18} color={theme.textMuted} />
      </View>

      <TextInput
        value={value}
        onChangeText={onChangeText}
        placeholder={placeholder}
        placeholderTextColor={theme.textMuted}
        returnKeyType="search"
        clearButtonMode="never"
        autoCapitalize="none"
        autoCorrect={false}
        style={[
          styles.input,
          {
            color: theme.foreground,
          },
        ]}
        {...props}
      />

      {loading ? (
        <View style={styles.rightBox}>
          <ActivityIndicator size="small" color={theme.primary} />
        </View>
      ) : value ? (
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Clear search"
          onPress={handleClear}
          hitSlop={8}
          style={styles.rightBox}
        >
          <View
            style={[
              styles.clearCircle,
              { backgroundColor: theme.mutedForeground },
            ]}
          >
            <View style={styles.clearBar1} />
            <View style={styles.clearBar2} />
          </View>
        </Pressable>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    height: 42,
    borderRadius: Radii.full,
    borderWidth: 1,
    flexDirection: "row",
    alignItems: "center",
    paddingHorizontal: Spacing.three,
  },
  iconBox: {
    marginRight: Spacing.two,
    alignItems: "center",
    justifyContent: "center",
  },
  input: {
    flex: 1,
    height: "100%",
    ...Typography.body,
    paddingVertical: 0,
  },
  rightBox: {
    marginLeft: Spacing.two,
    alignItems: "center",
    justifyContent: "center",
  },
  clearCircle: {
    width: 18,
    height: 18,
    borderRadius: 9,
    alignItems: "center",
    justifyContent: "center",
    position: "relative",
  },
  clearBar1: {
    position: "absolute",
    width: 10,
    height: 1.5,
    backgroundColor: "#FFFFFF",
    transform: [{ rotate: "45deg" }],
  },
  clearBar2: {
    position: "absolute",
    width: 10,
    height: 1.5,
    backgroundColor: "#FFFFFF",
    transform: [{ rotate: "-45deg" }],
  },
});
