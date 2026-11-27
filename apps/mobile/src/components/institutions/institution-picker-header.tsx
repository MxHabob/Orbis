import { View } from "react-native";
import { LanguageToggle } from "@/components/ui/language-toggle";
import { Text } from "@/components/ui/text";

interface InstitutionPickerHeaderProps {
  title: string;
  languageLabel: string;
  accessibilityLabel: string;
  onToggleLanguage: () => void;
}

export function InstitutionPickerHeader({
  title,
  languageLabel,
  accessibilityLabel,
  onToggleLanguage,
}: InstitutionPickerHeaderProps) {
  return (
    <View className="gap-two px-four pb-two pt-one">
      <View className="min-h-11 flex-row items-center justify-end">
        <LanguageToggle
          accessibilityLabel={accessibilityLabel}
          onPress={onToggleLanguage}
          label={languageLabel}
        />
      </View>
      <Text
        accessibilityRole="header"
        className="text-start text-[28px] leading-9 font-bold tracking-tight text-foreground"
      >
        {title}
      </Text>
    </View>
  );
}
