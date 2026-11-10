import React, { useState } from "react";
import { View } from "react-native";
import { ScreenScrollView } from "@/components/layout/screen-scroll-view";
import { Button } from "@/components/ui/button";
import { LanguageToggle } from "@/components/ui/language-toggle";
import { OrganizationLogo } from "@/components/ui/organization-logo";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

export function WelcomeOnboarding({
  onContinue,
}: {
  onContinue: () => Promise<void>;
}) {
  const { t, locale, setLocale } = useI18n();
  const [isContinuing, setIsContinuing] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  async function handleContinue() {
    setIsContinuing(true);
    setErrorMessage(null);
    try {
      await onContinue();
    } catch (error) {
      console.error(
        "[Mobile Onboarding] Failed to save completion state.",
        error,
      );
      setErrorMessage(t("onboarding_continue_error"));
    } finally {
      setIsContinuing(false);
    }
  }

  return (
    <ScreenScrollView
      edges={["top", "bottom", "left", "right"]}
      scrollViewProps={{
        contentContainerClassName:
          "flex-grow justify-between gap-eight px-six pb-six pt-four",
      }}
    >
      <View className="flex-row justify-end">
        <LanguageToggle
          accessibilityLabel={
            locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
          }
          label={locale === "ar" ? "English" : "العربية"}
          onPress={() =>
            void setLocale(locale === "ar" ? "en" : "ar").catch(
              (error: unknown) => {
                console.error(
                  "[Mobile Onboarding] Failed to save the selected language.",
                  error,
                );
              },
            )
          }
        />
      </View>

      <View className="items-center gap-eight py-eight">
        <OrganizationLogo size={76} />
        <View className="max-w-sm gap-four">
          <Text className="text-center text-[28px] leading-9 font-bold tracking-tight text-foreground">
            {t("onboarding_title")}
          </Text>
          <Text className="text-center text-[15px] leading-6 text-muted-foreground">
            {t("onboarding_description")}
          </Text>
        </View>
      </View>

      <View className="gap-three">
        {errorMessage ? (
          <Text
            accessibilityRole="alert"
            className="text-center text-sm text-destructive"
          >
            {errorMessage}
          </Text>
        ) : null}
        <Button
          title={t("onboarding_continue")}
          onPress={() => void handleContinue()}
          loading={isContinuing}
          disabled={isContinuing}
          fullWidth
          size="lg"
        />
      </View>
    </ScreenScrollView>
  );
}
