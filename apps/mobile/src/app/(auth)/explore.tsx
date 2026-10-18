import { useRouter } from "expo-router";
import { Text, View } from "react-native";

import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Button } from "@/components/ui/button";
import { useI18n } from "@/lib/i18n";

const VISITOR_FEATURES = [
  ["visitor_academic_title", "visitor_academic_description"],
  ["visitor_family_title", "visitor_family_description"],
  ["visitor_teacher_title", "visitor_teacher_description"],
] as const;

export default function VisitorExploreScreen() {
  const router = useRouter();
  const { t } = useI18n();

  return (
    <ScreenScrollView
      edges={["top", "bottom", "left", "right"]}
      scrollViewProps={{
        contentContainerClassName: "flex-grow justify-between gap-eight px-six pb-eight pt-eight",
      }}
    >
      <View className="gap-eight">
        <View className="gap-three">
          <Text className="text-xs font-semibold uppercase tracking-widest text-primary">
            {t("nav_app_name")}
          </Text>
          <Text className="text-[32px] leading-[38px] font-bold tracking-tight text-foreground">
            {t("visitor_title")}
          </Text>
          <Text className="text-base leading-6 text-muted-foreground">
            {t("visitor_description")}
          </Text>
        </View>

        <View className="overflow-hidden border-y border-border">
          {VISITOR_FEATURES.map(([titleKey, descriptionKey], index) => (
            <View
              key={titleKey}
              className={`gap-one py-four ${index < VISITOR_FEATURES.length - 1 ? "border-b border-border" : ""}`}
            >
              <Text className="text-base font-semibold text-foreground">
                {t(titleKey)}
              </Text>
              <Text className="text-sm leading-5 text-muted-foreground">
                {t(descriptionKey)}
              </Text>
            </View>
          ))}
        </View>
      </View>

      <View className="gap-three">
        <Button
          title={t("visitor_sign_in")}
          onPress={() => router.push("/(auth)/sign-in")}
          fullWidth
          size="lg"
        />
        <Button
          title={t("visitor_register")}
          onPress={() => router.push("/(auth)/register")}
          variant="secondary"
          fullWidth
          size="lg"
        />
      </View>
    </ScreenScrollView>
  );
}
