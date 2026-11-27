/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Sign in with email/password, optional MFA, and supported account links.
 */

import React, { useState } from "react";
import { Pressable, StyleSheet, View } from "react-native";
import { Image } from "expo-image";
import { Redirect, useRouter } from "expo-router";
import { Screen } from "@/components/layout/screen";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { LanguageToggle } from "@/components/ui/language-toggle";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useFormValidation } from "@/hooks/use-form-validation";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { Text } from "@/components/ui/text";

function getMfaUserId(response: unknown): string | undefined {
  if (typeof response !== "object" || response === null) return undefined;

  const value = response as {
    mfa_required?: unknown;
    user_id?: unknown;
    user?: unknown;
  };
  if (value.mfa_required !== true) return undefined;

  const nestedUserId =
    typeof value.user === "object" &&
    value.user !== null &&
    "id" in value.user &&
    typeof value.user.id === "string"
      ? value.user.id
      : undefined;

  return typeof value.user_id === "string" ? value.user_id : nestedUserId;
}

export default function SignInScreen() {
  const router = useRouter();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t, locale, setLocale, isArabic } = useI18n();
  const { login, loginWithGoogle, selectedInstitution, institutions } =
    useAuth();

  const [isSubmitting, setIsSubmitting] = useState(false);

  const {
    values,
    errors,
    formError,
    setFormError,
    setValue,
    markTouched,
    validateAll,
  } = useFormValidation({
    initialValues: { email: "", password: "" },
    validators: {
      email: (value) => (!value.trim() ? t("auth_error_required") : null),
      password: (value) => (!value ? t("auth_error_required") : null),
    },
  });

  if (!selectedInstitution) {
    return <Redirect href="/institution-select" />;
  }

  const handleSignIn = async () => {
    if (!validateAll()) {
      return;
    }

    const trimmedEmail = values.email.trim();
    setIsSubmitting(true);
    setFormError(null);
    try {
      const response = await login(trimmedEmail, values.password);
      const mfaUserId = getMfaUserId(response);
      if (mfaUserId) {
        router.push({
          pathname: "/(auth)/mfa-challenge",
          params: { userId: mfaUserId },
        });
        return;
      }
    } catch (err: unknown) {
      const message =
        err instanceof Error
          ? err.message
          : t("auth_error_invalid_credentials");
      setFormError(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setIsSubmitting(true);
    setFormError(null);
    try {
      await loginWithGoogle("google-token");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : t("auth_error_google");
      setFormError(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const toggleLanguage = () => {
    void setLocale(locale === "ar" ? "en" : "ar");
  };

  return (
    <View className="flex-1 bg-background">
      <Image
        source={require("../../../assets/dreamy-historic-campus.png")}
        style={StyleSheet.absoluteFill}
        contentFit="cover"
      />
      <View
        style={[
          StyleSheet.absoluteFill,
          {
            backgroundColor: isDark
              ? "rgba(18, 18, 18, 0.78)"
              : "rgba(252, 252, 252, 0.78)",
          },
        ]}
      />
      <Screen
        backgroundColor="transparent"
        edges={["top", "bottom", "left", "right"]}
      >
        <KeyboardAwareView
          style={{ flex: 1 }}
          contentContainerStyle={{ flexGrow: 1 }}
        >
          <View className="flex-1 gap-four px-five pb-three pt-two">
            <View className="flex-row justify-end">
              <LanguageToggle
                accessibilityLabel={
                  locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
                }
                onPress={toggleLanguage}
                label={locale === "ar" ? "English" : "العربية"}
              />
            </View>

            <View className="flex-1 justify-end pb-two">
              <Text className="text-[28px] leading-9 font-semibold tracking-tight text-foreground">
                {t("auth_sign_in")}
              </Text>
            </View>

            <Card padding="four" className="gap-three">
              <Pressable
                accessibilityRole="button"
                onPress={() => router.push("/institution-select")}
                hitSlop={8}
                className="min-h-11 justify-center"
              >
                <Text className="text-sm font-semibold text-primary">
                  {institutions.length > 1
                    ? t("auth_institution_switch")
                    : `${t("auth_institution_current")}: ${isArabic && selectedInstitution.name_ar ? selectedInstitution.name_ar : selectedInstitution.name}`}
                </Text>
              </Pressable>

              <Input
                label={t("auth_username_or_email")}
                placeholder={t("auth_username_or_email")}
                value={values.email}
                onChangeText={(value) => setValue("email", value)}
                onBlur={() => markTouched("email")}
                error={errors.email}
                keyboardType="email-address"
                autoCapitalize="none"
                autoComplete="email"
                textContentType="username"
                returnKeyType="next"
              />

              <View>
                <View className="mb-one flex-row items-center justify-between">
                  <Text className="text-sm font-medium text-foreground">
                    {t("auth_password")}
                  </Text>
                  <Pressable
                    accessibilityRole="button"
                    onPress={() => router.push("/(auth)/forgot-password")}
                    hitSlop={8}
                    className="min-h-11 justify-center"
                  >
                    <Text className="text-sm font-semibold text-primary">
                      {t("auth_forgot_password")}
                    </Text>
                  </Pressable>
                </View>
                <Input
                  placeholder="••••••••"
                  value={values.password}
                  onChangeText={(value) => setValue("password", value)}
                  onBlur={() => markTouched("password")}
                  error={errors.password}
                  secureTextEntry
                  autoComplete="current-password"
                  textContentType="password"
                  returnKeyType="go"
                  onSubmitEditing={() => void handleSignIn()}
                />
              </View>

              {formError ? (
                <Text
                  accessibilityRole="alert"
                  className="text-start text-xs leading-4 font-medium text-destructive"
                >
                  {formError}
                </Text>
              ) : null}

              <Button
                title={t("auth_sign_in")}
                onPress={() => void handleSignIn()}
                disabled={isSubmitting}
                loading={isSubmitting}
                fullWidth
                size="lg"
              />

              <View className="flex-row items-center gap-three">
                <View className="h-px flex-1 bg-border" />
                <Text className="text-sm text-muted-foreground">
                  {t("auth_or_divider")}
                </Text>
                <View className="h-px flex-1 bg-border" />
              </View>

              <Button
                title={t("auth_continue_google")}
                onPress={() => void handleGoogleSignIn()}
                disabled={isSubmitting}
                loading={isSubmitting}
                variant="outline"
                fullWidth
                size="lg"
              />

              <View className="items-center pt-one">
                <Pressable
                  accessibilityRole="button"
                  onPress={() => router.push("/(auth)/explore")}
                  hitSlop={8}
                  className="min-h-11 justify-center"
                >
                  <Text className="text-sm text-muted-foreground">
                    {t("visitor_explore")}
                  </Text>
                </Pressable>
              </View>
            </Card>
          </View>
        </KeyboardAwareView>
      </Screen>
    </View>
  );
}
