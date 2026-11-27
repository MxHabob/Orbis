/**
 * @module @orbis/mobile/app/(auth)/forgot-password
 *
 * Minimal password recovery screen aligned with Orbis S.C.R.A.P.S. surfaces.
 */

import React, { useState } from "react";
import { Pressable, View } from "react-native";
import { useRouter } from "expo-router";
import { ArrowLeft, ArrowRight, CheckCircle } from "@orbis/icons/native";
import { Screen } from "@/components/layout/screen";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { Text } from "@/components/ui/text";
import { useFormValidation } from "@/hooks/use-form-validation";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

export default function ForgotPasswordScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();

  const [isSubmitted, setIsSubmitted] = useState(false);
  const [isLoading, setIsLoading] = useState(false);

  const {
    values,
    errors,
    formError,
    setFormError,
    setValue,
    markTouched,
    validateAll,
  } = useFormValidation({
    initialValues: { email: "" },
    validators: {
      email: (value) => {
        const trimmed = value.trim();
        if (!trimmed || !trimmed.includes("@")) {
          return t("auth_error_required");
        }
        return null;
      },
    },
  });

  const handleSend = async () => {
    if (!validateAll()) {
      return;
    }

    setIsLoading(true);
    setFormError(null);
    try {
      await new Promise((resolve) => setTimeout(resolve, 600));
      setIsSubmitted(true);
    } catch {
      setFormError(t("auth_error_failed"));
    } finally {
      setIsLoading(false);
    }
  };

  const BackIcon = isArabic ? ArrowRight : ArrowLeft;

  return (
    <Screen edges={["top", "bottom", "left", "right"]}>
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{ flexGrow: 1 }}
      >
        <View className="flex-1 justify-between px-five pb-four pt-two">
          <View className="flex-row items-center">
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={t("auth_back_to_sign_in")}
              onPress={() => router.back()}
              className="h-11 w-11 items-center justify-center rounded-full border border-border bg-card"
              hitSlop={10}
            >
              <BackIcon size={22} color={theme.foreground} />
            </Pressable>
          </View>

          {!isSubmitted ? (
            <View className="flex-1 justify-center gap-four">
              <Text className="text-[28px] leading-9 font-semibold tracking-tight text-foreground">
                {t("auth_forgot_title")}
              </Text>

              <Card padding="four" className="gap-three">
                <Input
                  label={t("auth_email_placeholder")}
                  placeholder={t("auth_email_placeholder")}
                  value={values.email}
                  onChangeText={(value) => setValue("email", value)}
                  onBlur={() => markTouched("email")}
                  error={errors.email}
                  keyboardType="email-address"
                  autoCapitalize="none"
                  autoComplete="email"
                  autoFocus
                  onSubmitEditing={() => void handleSend()}
                  returnKeyType="go"
                />

                {formError ? (
                  <Text
                    accessibilityRole="alert"
                    className="text-start text-xs leading-4 font-medium text-destructive"
                  >
                    {formError}
                  </Text>
                ) : null}

                <Button
                  title={t("auth_forgot_title")}
                  onPress={() => void handleSend()}
                  disabled={isLoading}
                  loading={isLoading}
                  fullWidth
                  size="lg"
                />
              </Card>
            </View>
          ) : (
            <View className="flex-1 items-center justify-center gap-four px-four">
              <View className="h-14 w-14 items-center justify-center rounded-full bg-success/10">
                <CheckCircle size={32} color={theme.success} />
              </View>

              <Text className="text-center text-2xl leading-8 font-semibold text-foreground">
                {t("auth_forgot_success_title")}
              </Text>

              <Text className="text-center text-sm leading-5 text-muted-foreground">
                {t("auth_forgot_success_msg")}
              </Text>

              <View className="w-full pt-two">
                <Button
                  title={t("auth_back_to_sign_in")}
                  onPress={() => router.replace("/(auth)/sign-in")}
                  fullWidth
                  size="lg"
                />
              </View>
            </View>
          )}
        </View>
      </KeyboardAwareView>
    </Screen>
  );
}
