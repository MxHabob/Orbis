import {
  ALL_AVATAR_HATS,
  AvatarController,
  type AvatarHat,
  Avatar as CompanionAvatar,
} from "@orbis/avatar-engine/native";
import { memo, useCallback, useEffect, useState } from "react";
import { Text, View } from "react-native";
import { useI18n } from "@/lib/i18n";

function pickNextHat(
  currentHat?: AvatarHat,
): Exclude<AvatarHat, "random" | "none"> {
  const candidates = ALL_AVATAR_HATS.filter(
    (hat): hat is Exclude<AvatarHat, "random" | "none"> =>
      hat !== currentHat && hat !== "none",
  );
  return (
    candidates[Math.floor(Math.random() * candidates.length)] ?? "graduation"
  );
}

type HomeHeroProps = {
  firstName?: string;
};

export const HomeHero = memo(function HomeHero({ firstName }: HomeHeroProps) {
  const { t, isArabic } = useI18n();
  const [avatarHat, setAvatarHat] = useState<AvatarHat>(() => pickNextHat());
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
    setAvatarHat((current) => pickNextHat(current));
    const mood = Math.random();
    if (mood < 0.34) avatarController.happy();
    else if (mood < 0.67) avatarController.think();
    else avatarController.blink();
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
