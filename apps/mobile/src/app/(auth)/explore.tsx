import { useRouter } from "expo-router";
import { ScrollView, Text, View } from "react-native";

import { Button } from "@/components/ui/button";
import { useI18n } from "@/lib/i18n";

const VISITOR_FEATURES = [
  ["visitor_academic_title", "visitor_academic_description"],
  ["visitor_family_title", "visitor_family_description"],
  ["visitor_teacher_title", "visitor_teacher_description"],
  ["visitor_admission_title", "visitor_admission_description"],
] as const;

export default function VisitorExploreScreen() {
  const router = useRouter();
  const { t } = useI18n();

  return (
    <ScrollView
      className="flex-1 bg-background"
      contentContainerClassName="flex-grow justify-between gap-six px-five pb-eight pt-twelve"
    >
      <View className="gap-six">
        <View className="gap-two">
          <Text className="text-sm font-semibold uppercase tracking-widest text-primary">
            {t("nav_app_name")}
          </Text>
          <Text className="text-3xl font-extrabold text-foreground">
            {t("visitor_title")}
          </Text>
          <Text className="text-base leading-6 text-muted-foreground">
            {t("visitor_description")}
          </Text>
        </View>

        <View className="gap-three">
          {VISITOR_FEATURES.map(([titleKey, descriptionKey]) => (
            <View
              key={titleKey}
              className="gap-one rounded-xl border border-border bg-card p-four"
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
    </ScrollView>
  );
}
