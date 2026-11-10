import React from "react";
import { View } from "react-native";
import Animated, { FadeInDown, FadeOutUp } from "react-native-reanimated";
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
  if (!visible) return null;
  const title = t("network_offline_title");
  const description = t("network_offline_description");

  return (
    <Animated.View
      accessibilityRole="alert"
      accessibilityLiveRegion="polite"
      accessibilityLabel={`${title}. ${description}`}
      entering={FadeInDown.duration(180)}
      exiting={FadeOutUp.duration(150)}
      pointerEvents="none"
      className="flex-row items-center gap-3 rounded-2xl px-3 py-2"
      style={{
        position: "absolute",
        top: insets.top + 8,
        left: 16,
        right: 16,
        zIndex: 1000,
        elevation: 1000,
        backgroundColor: theme.backgroundElement,
        shadowColor: theme.foreground,
        shadowOffset: { width: 0, height: 3 },
        shadowOpacity: 0.1,
        shadowRadius: 8,
      }}
    >
      <View className="size-9 items-center justify-center rounded-full bg-primary/10">
        <Cloud size={19} strokeWidth={1.8} color={theme.primary} />
      </View>
      <View className="min-w-0 flex-1 gap-0.5">
        <Text className="text-[13px] font-semibold leading-4 text-foreground">
          {title}
        </Text>
        <Text
          numberOfLines={1}
          className="text-xs leading-4 text-muted-foreground"
        >
          {description}
        </Text>
      </View>
    </Animated.View>
  );
}
