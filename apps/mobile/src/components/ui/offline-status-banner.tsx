import React, { useEffect, useState } from "react";
import { Text, View } from "react-native";
import Animated, { FadeInDown, FadeOutUp } from "react-native-reanimated";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Cloud } from "@orbis/icons/native";
import {
  getNetworkStatus,
  subscribeToNetworkStatus,
  type NetworkStatus,
} from "@/lib/network/online-manager";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";

export function OfflineStatusBanner() {
  const { t } = useI18n();
  const insets = useSafeAreaInsets();
    const theme = useTheme();
  const [networkStatus, setNetworkStatus] = useState<NetworkStatus>(
    getNetworkStatus,
  );

  useEffect(
    () =>
      subscribeToNetworkStatus(setNetworkStatus),
    [],
  );

  if (!networkStatus.isKnown || networkStatus.isOnline) return null;

  return (
    <Animated.View
      accessibilityRole="alert"
      accessibilityLiveRegion="polite"
      accessibilityLabel={t("network_offline_read_only")}
      entering={FadeInDown.duration(180)}
      exiting={FadeOutUp.duration(150)}
      pointerEvents="none"
      className="h-8 items-center justify-center"
      style={{ top: insets.top }}
    >
      <View
        className={`flex-row items-center gap-2`}
      >
          <Cloud size={18} strokeWidth={1.8}  color={theme.foreground} />
        <Text
          className="text-[13px] leading-4.25 font-bold tracking-tight text-foreground"
        >
          {t("network_offline_read_only")}
        </Text>
      </View>
    </Animated.View>
  );
}
