import React from "react";
import { View } from "react-native";
import Animated, {
  FadeInDown,
  FadeOutDown,
  useReducedMotion,
} from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Cloud } from "@orbis/icons/native";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { Text } from "@/components/ui/text";

type OfflineStatusBannerProps = {
  visible: boolean;
};

export function OfflineStatusBanner({ visible }: OfflineStatusBannerProps) {
  const { t } = useI18n();
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const reduceMotion = useReducedMotion();

  if (!visible) return null;

  const title = t("network_offline_title");
  const description = t("network_offline_description");

  return (
    <Animated.View
      accessibilityRole="alert"
      accessibilityLiveRegion="polite"
      accessibilityLabel={`${title}. ${description}`}
      entering={reduceMotion ? undefined : FadeInDown.duration(180)}
      exiting={reduceMotion ? undefined : FadeOutDown.duration(150)}
      pointerEvents="none"
      className="w-full flex-row items-center justify-center gap-two bg-foreground px-four pt-two"
      style={{
        paddingBottom: Math.max(insets.bottom, 8),
      }}
    >
      <View className="flex-row items-center gap-two">
        <Cloud size={15} strokeWidth={2} color={theme.background} />
        <Text
          numberOfLines={1}
          className="text-xs font-medium leading-4 text-background"
        >
          {title}
        </Text>
      </View>
    </Animated.View>
  );
}
