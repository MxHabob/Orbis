import React from "react";
import { Pressable } from "react-native";
import Animated, {
  useAnimatedStyle,
  useSharedValue,
  withTiming,
} from "react-native-reanimated";
import { Check, XMark } from "@orbis/icons/native";
import { useTheme } from "@/hooks/use-theme";

export interface SwitchProps {
  value: boolean;
  onValueChange?: (value: boolean) => void;
  disabled?: boolean;
  accessibilityLabel?: string;
}

export function Switch({
  value,
  onValueChange,
  disabled = false,
  accessibilityLabel,
}: SwitchProps) {
  const theme = useTheme();
  const progress = useSharedValue(value ? 1 : 0);

  React.useEffect(() => {
    progress.value = withTiming(value ? 1 : 0, { duration: 180 });
  }, [value, progress]);

  const thumbStyle = useAnimatedStyle(() => ({
    transform: [{ translateX: progress.value * 20 }],
  }));

  return (
    <Pressable
      accessibilityRole="switch"
      accessibilityState={{ checked: value, disabled }}
      accessibilityLabel={accessibilityLabel}
      disabled={disabled}
      onPress={() => onValueChange?.(!value)}
      hitSlop={{ top: 10, bottom: 10, left: 0, right: 0 }}
      className={`h-6 w-11 justify-start rounded-sm border ${value ? "border-primary bg-primary" : "border-border bg-background-element"} ${disabled ? "opacity-50" : ""}`}
      style={({ pressed }) => (pressed && !disabled ? { opacity: 0.8 } : undefined)}
    >
      <Animated.View
        className={`h-[22px] w-[22px] rounded-md border bg-background shadow-sm ${value ? "border-primary" : "border-border"}`}
        style={thumbStyle}
      >
        {value ? (
          <Check size={12} color={theme.primary} />
        ) : (
          <XMark size={12} color={theme.mutedForeground} />
        )}
      </Animated.View>
    </Pressable>
  );
}
