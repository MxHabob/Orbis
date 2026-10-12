/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Sign in with email/password, optional MFA, and supported account links.
 */

import React, { useState } from "react";
import {
  ImageBackground,
  Pressable,
  StyleSheet,
  Text,
  View,
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { ThemedStatusBar } from "@/components/ui/themed-status-bar";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { Language, Logo } from "@orbis/icons/native";

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
      <ImageBackground
        source={require("../../../assets/Dreamy Arrival at the Historic Campus.png")}
        resizeMode="cover"
        className="flex-1"
      >
        <LinearGradient
          pointerEvents="none"
          colors={[
            "rgba(21, 16, 39, 0.24)",
            "rgba(21, 16, 39, 0.1)",
            "rgba(21, 16, 39, 0.42)",
            "rgba(21, 16, 39, 0.56)",
          ]}
          locations={[0, 0.28, 0.58, 1]}
          style={StyleSheet.absoluteFill}
        />
        <ThemedStatusBar backgroundColor="transparent" style="light" />
        <KeyboardAwareView
          style={{ flex: 1 }}
          contentContainerStyle={{ flexGrow: 1 }}
        >
          <View
            className="flex-1 px-5"
            style={{
              paddingTop: insets.top + 8,
              paddingBottom: Math.max(insets.bottom, 12) + 4,
            }}
          >
            <View className="h-11 flex-row items-center justify-center">
              <View className="flex-row items-center gap-2">
                <Logo size={27} color="#FFFFFF" dotColor="#A821D9" />
                <Text className="text-lg font-bold tracking-tight text-white">
                  Orbis
                </Text>
              </View>
              <Pressable
                accessibilityRole="button"
                accessibilityLabel={
                  locale === "ar" ? "Switch to English" : "التبديل إلى العربية"
                }
                onPress={toggleLanguage}
                hitSlop={8}
                className="absolute end-0 min-h-11 flex-row items-center gap-2 rounded-full border border-white/30 bg-black/10 px-3"
              >
                <Language size={16} color="#FFFFFF" />
                <Text className="text-sm font-medium text-white">
                  {locale === "ar" ? "English" : "العربية"}
                </Text>
              </Pressable>
            </View>

            <View className="min-h-40 flex-1 justify-end gap-3 pb-5 pt-5">
              <Text className="text-[32px] leading-9.75 font-semibold tracking-tight text-white">
                {t("visitor_title")}
              </Text>
              <Text className="text-base leading-6 text-white/90">
                {t("visitor_description")}
              </Text>
            </View>

            <View className="gap-2 bg-card px-4 py-3 shadow-xl">
              <Text className="text-xl font-semibold tracking-tight text-foreground">
                {t("auth_sign_in")}
              </Text>

              <Input
                placeholder={t("auth_username_or_email")}
                value={email}
                onChangeText={setEmail}
                keyboardType="email-address"
                autoCapitalize="none"
                autoComplete="email"
                textContentType="username"
                returnKeyType="next"
                style={{ minHeight: 32, fontSize: 12 }}
              />

              <View>
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
                  style={{ minHeight: 32, fontSize: 12 }}
                />
              </View>

              <Button
                title={t("auth_sign_in")}
                onPress={() => void handleSignIn()}
                disabled={isSubmitting}
                loading={isSubmitting}
                fullWidth
                size="md"
              />

              <View className="flex-row items-center gap-3">
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
                size="md"
              />

              <View className="items-center gap-1 pt-1">
                <View className="flex-row items-center gap-1">
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
      </ImageBackground>
    </View>
  );
}
