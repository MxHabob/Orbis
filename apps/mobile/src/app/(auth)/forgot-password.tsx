/**
 * @module @orbis/mobile/app/(auth)/forgot-password
 *
 * Minimalist, elegant Password Recovery screen directly inspired by Reference Image 3.
 */

import React, { useState } from "react";
import {
  Text,
  View,
  Pressable,
  TextInput,
  Platform,
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { useRouter } from "expo-router";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import {
  ArrowLeft,
  ArrowRight,
  CheckCircle,
} from "@orbis/icons/native";

export default function ForgotPasswordScreen() {
  const router = useRouter();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t, isArabic } = useI18n();
  const toast = useToast();

  const [email, setEmail] = useState("");
  const [isSubmitted, setIsSubmitted] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const handleSend = async () => {
    setErrorMsg(null);
    const trimmed = email.trim();
    if (!trimmed || !trimmed.includes("@")) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsLoading(true);
    try {
      await new Promise((resolve) => setTimeout(resolve, 600));
      setIsSubmitted(true);
      toast.success(t("auth_forgot_success_title"));
    } catch {
      setErrorMsg(t("auth_error_failed"));
    } finally {
      setIsLoading(false);
    }
  };

  const BackIcon = isArabic ? ArrowRight : ArrowLeft;
  const ArrowIcon = isArabic ? ArrowLeft : ArrowRight;

  return (
    <View
      className="flex-1 bg-background"
    >
      <LinearGradient
        colors={
          isDark
            ? (["#3B0764", "#1E1035", "rgba(9,9,11,0)"] as const)
            : (["#DDD6FE", "#EDE9FE", "rgba(248,250,252,0)"] as const)
        }
        start={{ x: 0.5, y: 0 }}
        end={{ x: 0.5, y: 0.5 }}
        className="absolute inset-0"
        pointerEvents="none"
      />

      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 20,
          paddingTop: Platform.OS === "ios" ? 54 : 36,
          paddingBottom: 32,
        }}
      >
        {/* Minimal Nav Header */}
        <View className="mb-6 flex-row items-center">
          <Pressable
            onPress={() => router.back()}
            className="h-10 w-10 items-center justify-center rounded-full border"
            style={{
              backgroundColor: isDark
                ? "rgba(255, 255, 255, 0.08)"
                : "rgba(255, 255, 255, 0.8)",
              borderColor: isDark
                ? "rgba(255, 255, 255, 0.12)"
                : "rgba(0, 0, 0, 0.06)",
            }}
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={18} color={isDark ? "#FFFFFF" : "#0F172A"} />
          </Pressable>
        </View>

        {!isSubmitted ? (
          <View className="flex-1 justify-center">
            {/* Title Block */}
            <View className="mb-6">
              <Text
                className="font-sans text-[28px] leading-8.5 font-extrabold tracking-[-0.5px] text-foreground"
              >
                {t("auth_forgot_title")}
              </Text>
              <Text
                className="mt-one font-sans text-sm leading-5 text-muted-foreground"
              >
                {t("auth_forgot_subtitle")}
              </Text>
            </View>

            {errorMsg ? (
              <View className="mb-four rounded-lg border border-destructive bg-destructive/10 px-three py-two">
                <Text
                  className="font-sans text-xs leading-4 font-semibold text-destructive"
                >
                  {errorMsg}
                </Text>
              </View>
            ) : null}

            {/* Large Pill Input with embedded arrow */}
            <View
              className="h-14 flex-row items-center rounded-full border border-border bg-background-element px-four"
              style={{
                shadowColor: "#000000",
                shadowOffset: { width: 0, height: 2 },
                shadowOpacity: 0.06,
                shadowRadius: 6,
                elevation: 2,
              }}
            >
              <TextInput
                className="flex-1 px-2 font-sans text-base text-foreground"
                style={{
                  textAlign: isArabic ? "right" : "left",
                }}
                placeholder={t("auth_email_placeholder")}
                placeholderTextColor={theme.textMuted}
                value={email}
                onChangeText={setEmail}
                keyboardType="email-address"
                autoCapitalize="none"
                autoFocus
                onSubmitEditing={handleSend}
                returnKeyType="go"
              />

              <Pressable
                onPress={handleSend}
                disabled={isLoading}
                className="h-9.5 w-9.5 items-center justify-center rounded-full bg-foreground"
                style={({ pressed }) => ({
                  transform: [{ scale: pressed ? 0.9 : 1 }],
                  opacity: isLoading ? 0.6 : 1,
                })}
              >
                <ArrowIcon
                  size={16}
                  color={theme.background}
                />
              </Pressable>
            </View>
          </View>
        ) : (
          /* Confirmation State */
          <View className="flex-1 items-center justify-center px-4">
            <View
              className="mb-five h-20 w-20 items-center justify-center rounded-full bg-success/10"
            >
              <CheckCircle size={44} color={theme.success} />
            </View>

            <Text
              className="mb-two text-center font-sans text-2xl leading-7.5 font-extrabold text-foreground"
            >
              {t("auth_forgot_success_title")}
            </Text>

            <Text
              className="mb-eight text-center font-sans text-sm leading-5.5 text-muted-foreground"
            >
              {t("auth_forgot_success_msg")}
            </Text>

            <View className="w-full">
              <Pressable
                onPress={() => router.replace("/(auth)/sign-in")}
                className="h-13.5 items-center justify-center rounded-full bg-foreground shadow-md"
                style={({ pressed }) => ({
                  transform: [{ scale: pressed ? 0.98 : 1 }],
                })}
              >
                <Text
                  className="font-sans text-base leading-5 font-bold text-background"
                >
                  {t("auth_back_to_sign_in")}
                </Text>
              </Pressable>
            </View>
          </View>
        )}
      </KeyboardAwareView>
    </View>
  );
}
