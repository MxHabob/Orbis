import { View } from "react-native";
import { LanguageToggle } from "@/components/ui/language-toggle";

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
  return (
    <View className="min-h-11 flex-row items-center justify-end px-5">
      <LanguageToggle
        accessibilityLabel={accessibilityLabel}
        onPress={onToggleLanguage}
        label={languageLabel}
      />
    </View>
  );
}
