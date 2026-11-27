import {
  AvatarController,
  type AvatarHat,
  Avatar as CompanionAvatar,
  pickNextAvatarHat,
} from "@orbis/avatar-engine/native";
import { memo, useCallback, useEffect, useState } from "react";
import { View } from "react-native";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

type HomeHeroProps = {
  firstName?: string;
};

export const HomeHero = memo(function HomeHero({ firstName }: HomeHeroProps) {
  const { t, isArabic } = useI18n();
  const [avatarHat, setAvatarHat] = useState<AvatarHat>(() =>
    pickNextAvatarHat(),
  );
  const [avatarController] = useState(() => new AvatarController());
  const hour = new Date().getHours();
  const greetingKey =
    hour >= 4 && hour < 12
      ? "home_greeting_morning"
      : hour >= 12 && hour < 17
        ? "home_greeting_afternoon"
        : "home_greeting_evening";
  const greeting = `${t(greetingKey)}${
    firstName ? `${isArabic ? "، " : ", "}${firstName}` : ""
  }`;

  useEffect(
    () => () => {
      avatarController.destroy();
    },
    [avatarController],
  );

  const handleAvatarPress = useCallback(() => {
    setAvatarHat((current) => pickNextAvatarHat(current));
    avatarController.tapReaction();
  }, [avatarController]);

  return (
    <View className="items-center gap-two px-two pb-one pt-three">
      <CompanionAvatar
        size={112}
        controller={avatarController}
        hat={avatarHat}
        label={t("home_avatar_change_hat")}
        accessibilityHint={t("home_avatar_hint")}
        onPress={handleAvatarPress}
      />
      <Text
        numberOfLines={2}
        className="text-center text-[26px] font-bold tracking-tight text-foreground"
      >
        {greeting}
      </Text>
    </View>
  );
});
