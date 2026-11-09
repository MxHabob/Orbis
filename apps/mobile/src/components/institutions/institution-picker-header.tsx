import { Language, Logo } from "@orbis/icons/native";
import { Pressable, Text, View } from "react-native";
import { useTheme } from "@/hooks/use-theme";

interface InstitutionPickerHeaderProps {
  languageLabel: string;
  accessibilityLabel: string;
  onToggleLanguage: () => void;
}

export function InstitutionPickerHeader({
  languageLabel,
  accessibilityLabel,
  onToggleLanguage,
}: InstitutionPickerHeaderProps) {
  const theme = useTheme();

  return (
    <View className="h-11 flex-row items-center justify-between px-5">
      <View className="flex-row items-center gap-2">
        <Logo size={25} color={theme.primary} dotColor={theme.brandPink} />
        <Text className="text-lg font-bold tracking-tight text-foreground">
          Orbis
        </Text>
      </View>
      <Pressable
        accessibilityRole="button"
        accessibilityLabel={accessibilityLabel}
        onPress={onToggleLanguage}
        hitSlop={8}
        className="min-h-11 flex-row items-center gap-2 rounded-full border border-border bg-card px-3"
      >
        <Language size={15} color={theme.textMuted} />
        <Text className="text-sm font-medium text-foreground">
          {languageLabel}
        </Text>
      </Pressable>
    </View>
  );
}
