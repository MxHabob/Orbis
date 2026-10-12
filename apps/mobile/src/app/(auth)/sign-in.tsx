/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Sign in with email/password, optional MFA, and supported account links.
 */

import React, { useState } from "react";
import { Pressable, Text, View } from "react-native";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { AcademicCap, Language } from "@orbis/icons/native";

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
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { t, locale, setLocale } = useI18n();
  const toast = useToast();
  const { login, loginWithGoogle } = useAuth();

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSignIn = async () => {
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      toast.error(t("auth_error_required") || "Please enter your email and password.");
      return;
    }

    setIsSubmitting(true);
    try {
      const response = await login(trimmedEmail, password);
      const mfaUserId = getMfaUserId(response);
      if (mfaUserId) {
        router.push({
          pathname: "/(auth)/mfa-challenge",
          params: { userId: mfaUserId },
        });
        return;
      }
      router.replace("/(app)/(tabs)");
    } catch (err: unknown) {
      const message =
        err instanceof Error ? err.message : t("auth_error_invalid_credentials");
      toast.error(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setIsSubmitting(true);
    try {
      await loginWithGoogle("google-token");
      router.replace("/(app)/(tabs)");
    } catch (err: unknown) {
      const message = err instanceof Error ? err.message : t("auth_error_google");
      toast.error(message);
    } finally {
      setIsSubmitting(false);
    }
  };

  const toggleLanguage = () => {
    void setLocale(locale === "ar" ? "en" : "ar");
  };

  return (
    <View className="flex-1 bg-primary-dark">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          paddingTop: insets.top,
        }}
      >
        <ThemedStatusBar
          backgroundColor={theme.brandPurpleDark}
          style="light"
        />
        <View className="flex-1">
          <View className="min-h-[320px] flex-1 justify-between overflow-hidden bg-brand-purple-dark px-six pb-eight pt-four">
            <View
              pointerEvents="none"
              className="absolute -right-16 top-10 h-[280px] w-[280px] items-center justify-center rounded-full border border-white/15"
            >
              <View className="h-[218px] w-[218px] items-center justify-center rounded-full border border-white/15">
                <AcademicCap size={120} color="#FFFFFF" />
              </View>
            </View>

            <View className="flex-row items-center justify-between">
              <View className="flex-row items-center gap-two">
                <View className="h-10 w-10 items-center justify-center rounded-md border border-white/25 bg-white/10">
                  <AcademicCap size={23} color="#FFFFFF" />
                </View>
                <Text className="text-lg font-bold tracking-tight text-white">
                  Orbis
                </Text>
              </View>
              <Pressable
                accessibilityRole="button"
                accessibilityLabel={locale === "ar" ? "Switch to English" : "التبديل إلى العربية"}
                onPress={toggleLanguage}
                hitSlop={8}
                className="min-h-11 flex-row items-center gap-two rounded-full border border-white/20 px-three"
              >
                <Language size={16} color="#FFFFFF" />
                <Text className="text-sm font-medium text-white">
                  {locale === "ar" ? "English" : "العربية"}
                </Text>
              </Pressable>
            </View>

            <View className="max-w-[520px] gap-three">
              <Text className="text-[34px] leading-[40px] font-semibold tracking-tight text-white">
                {t("visitor_title")}
              </Text>
              <Text className="text-base leading-6 text-white/80">
                {t("visitor_description")}
              </Text>
            </View>
          </View>

          <View
            className="gap-four rounded-t-3xl bg-card px-six pb-eight pt-six"
            style={{ paddingBottom: Math.max(insets.bottom, 24) }}
          >
            <View className="gap-one">
              <Text className="text-xl font-semibold tracking-tight text-foreground">
                {t("auth_sign_in")}
              </Text>
              <Text className="text-sm text-muted-foreground">
                {t("auth_sign_in_subtitle")}
              </Text>
            </View>

            <Input
              label={t("auth_username_or_email")}
              placeholder="name@school.org"
              value={email}
              onChangeText={setEmail}
              keyboardType="email-address"
              autoCapitalize="none"
              autoComplete="email"
              textContentType="username"
              returnKeyType="next"
            />

            <View className="gap-one">
              <View className="flex-row items-center justify-between">
                <Text className="text-sm font-semibold text-foreground">
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
                value={password}
                onChangeText={setPassword}
                secureTextEntry
                autoComplete="current-password"
                textContentType="password"
                returnKeyType="go"
                onSubmitEditing={() => void handleSignIn()}
              />
            </View>

            <View className="gap-three pt-one">
              <Button
                title={t("auth_sign_in")}
                onPress={() => void handleSignIn()}
                disabled={isSubmitting}
                loading={isSubmitting}
                fullWidth
                size="lg"
              />
              <Button
                title={t("auth_continue_google")}
                onPress={() => void handleGoogleSignIn()}
                disabled={isSubmitting}
                variant="outline"
                fullWidth
                size="lg"
              />
            </View>

            <View className="items-center gap-two pt-two">
              <View className="flex-row items-center gap-one">
                <Text className="text-sm text-muted-foreground">
                  {t("auth_no_account")}
                </Text>
                <Pressable
                  accessibilityRole="button"
                  onPress={() => router.push("/(auth)/register")}
                  hitSlop={8}
                >
                  <Text className="text-sm font-semibold text-primary">
                    {t("auth_register_title")}
                  </Text>
                </Pressable>
              </View>
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
          </View>
        </View>
      </KeyboardAwareView>
    </View>
  );
}
