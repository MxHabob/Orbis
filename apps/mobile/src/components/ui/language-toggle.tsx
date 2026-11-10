import { Pressable } from "react-native";
import { Text } from "@/components/ui/text";

interface LanguageToggleProps {
  label: string;
  accessibilityLabel: string;
  onPress: () => void;
}

export function LanguageToggle({
  label,
  accessibilityLabel,
  onPress,
}: LanguageToggleProps) {
  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={accessibilityLabel}
      onPress={onPress}
      hitSlop={8}
      className="min-h-11 items-center justify-center px-2"
    >
      <Text className="text-sm font-medium text-primary">{label}</Text>
    </Pressable>
  );
}
