import { PropsWithChildren, useState } from "react";
import { Pressable, Text, View } from "react-native";
import { ChevronUp, ChevronDown } from "@orbis/icons/native";
import Animated, { FadeIn, FadeOut } from "react-native-reanimated";
import { useTheme } from "@/hooks/use-theme";

export interface AccordionProps extends PropsWithChildren {
  title: string;
  defaultOpen?: boolean;
  disabled?: boolean;
  onOpenChange?: (open: boolean) => void;
}

export function Accordion({
  title,
  children,
  defaultOpen = false,
  disabled = false,
  onOpenChange,
}: AccordionProps) {
  const theme = useTheme();
  const [open, setOpen] = useState(defaultOpen);

  const toggle = () => {
    if (disabled) return;
    const nextOpen = !open;
    setOpen(nextOpen);
    onOpenChange?.(nextOpen);
  };

  return (
    <View className="border-b" style={{ borderBottomColor: theme.borderSubtle }}>
      <Pressable
        accessibilityRole="button"
        accessibilityState={{ expanded: open, disabled }}
        disabled={disabled}
        onPress={toggle}
        className="min-h-12 flex-row items-center justify-between gap-3 py-3"
        style={({ pressed }) => pressed ? { opacity: 0.7 } : undefined}
      >
        <Text className="flex-1 text-base font-semibold">{title}</Text>
        {open ? (
          <ChevronUp size={18} color={theme.textSecondary} />
        ) : (
          <ChevronDown size={18} color={theme.textSecondary} />
        )}
      </Pressable>
      {open && (
        <Animated.View
          entering={FadeIn.duration(160)}
          exiting={FadeOut.duration(120)}
        >
          <View className="pb-4 pr-2">{children}</View>
        </Animated.View>
      )}
    </View>
  );
}
