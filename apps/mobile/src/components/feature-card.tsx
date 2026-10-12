import { ChevronLeft, ChevronRight } from "@orbis/icons/native";
import React from "react";
import { Pressable, Text, View } from "react-native";

import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

type FeatureCardProps = {
  title: string;
  description: string;
  onPress: () => void;
};

export function FeatureCard({
  title,
  description,
  onPress,
}: FeatureCardProps) {
  const theme = useTheme();
  const { isArabic } = useI18n();
  const DirectionIcon = isArabic ? ChevronLeft : ChevronRight;

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={`${title}. ${description}`}
      onPress={onPress}
      className="min-h-[88px] flex-row items-center justify-between gap-3 rounded-xl border border-border bg-card px-four py-three"
      style={({ pressed }) => [{ opacity: pressed ? 0.82 : 1 }]}
    >
      <View className="flex-1 gap-1">
        <Text className="text-base font-semibold text-foreground">{title}</Text>
        <Text className="text-sm leading-5 text-muted-foreground">
          {description}
        </Text>
      </View>
      <DirectionIcon size={18} color={theme.textSecondary} />
    </Pressable>
  );
}
