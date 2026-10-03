/**
 * @module @orbis/mobile/app/(auth)/forgot-password
 *
 * Minimalist, elegant Password Recovery screen directly inspired by Reference Image 3.
 */

import React, { useState } from "react";
import {
  StyleSheet,
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
import { Radii, Spacing, Typography, Fonts } from "@/constants/theme";
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
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >
      <LinearGradient
        colors={
          isDark
            ? (["#3B0764", "#1E1035", "rgba(9,9,11,0)"] as const)
            : (["#DDD6FE", "#EDE9FE", "rgba(248,250,252,0)"] as const)
        }
        start={{ x: 0.5, y: 0 }}
        end={{ x: 0.5, y: 0.5 }}
        style={StyleSheet.absoluteFill}
        pointerEvents="none"
      />

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        {/* Minimal Nav Header */}
        <View style={styles.navHeader}>
          <Pressable
            onPress={() => router.back()}
            style={[
              styles.navIconBtn,
              {
                backgroundColor: isDark
                  ? "rgba(255, 255, 255, 0.08)"
                  : "rgba(255, 255, 255, 0.8)",
                borderColor: isDark
                  ? "rgba(255, 255, 255, 0.12)"
                  : "rgba(0, 0, 0, 0.06)",
              },
            ]}
            hitSlop={{ top: 12, bottom: 12, left: 12, right: 12 }}
          >
            <BackIcon size={18} color={isDark ? "#FFFFFF" : "#0F172A"} />
          </Pressable>
        </View>

        {!isSubmitted ? (
          <View style={styles.mainContent}>
            {/* Title Block */}
            <View style={styles.titleBlock}>
              <Text
                style={[
                  styles.stepQuestion,
                  { color: isDark ? "#FFFFFF" : "#0F172A" },
                ]}
              >
                {t("auth_forgot_title")}
              </Text>
              <Text
                style={[
                  styles.stepHint,
                  {
                    color: isDark
                      ? "rgba(255, 255, 255, 0.6)"
                      : "#64748B",
                  },
                ]}
              >
                {t("auth_forgot_subtitle")}
              </Text>
            </View>

            {errorMsg ? (
              <View
                style={[
                  styles.errorPill,
                  {
                    backgroundColor: isDark
                      ? "rgba(239, 68, 68, 0.18)"
                      : "#FEF2F2",
                    borderColor: theme.error,
                  },
                ]}
              >
                <Text style={[styles.errorPillText, { color: theme.error }]}>
                  {errorMsg}
                </Text>
              </View>
            ) : null}

            {/* Large Pill Input with embedded arrow */}
            <View
              style={[
                styles.largePillInputContainer,
                {
                  backgroundColor: isDark
                    ? "rgba(255, 255, 255, 0.06)"
                    : "#FFFFFF",
                  borderColor: isDark
                    ? "rgba(255, 255, 255, 0.18)"
                    : "rgba(0, 0, 0, 0.1)",
                },
              ]}
            >
              <TextInput
                style={[
                  styles.largePillTextInput,
                  {
                    color: isDark ? "#FFFFFF" : "#0F172A",
                    textAlign: isArabic ? "right" : "left",
                  },
                ]}
                placeholder={t("auth_email_placeholder")}
                placeholderTextColor={
                  isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                }
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
                style={({ pressed }) => [
                  styles.arrowCircleBtn,
                  {
                    backgroundColor: isDark ? "#FFFFFF" : "#0F172A",
                    transform: [{ scale: pressed ? 0.9 : 1 }],
                    opacity: isLoading ? 0.6 : 1,
                  },
                ]}
              >
                <ArrowIcon
                  size={16}
                  color={isDark ? "#0F172A" : "#FFFFFF"}
                />
              </Pressable>
            </View>
          </View>
        ) : (
          /* Confirmation State */
          <View style={styles.successContent}>
            <View
              style={[
                styles.successIconBox,
                {
                  backgroundColor: isDark
                    ? "rgba(16, 185, 129, 0.15)"
                    : theme.successLight,
                },
              ]}
            >
              <CheckCircle size={44} color={theme.success} />
            </View>

            <Text
              style={[
                styles.successTitle,
                { color: isDark ? "#FFFFFF" : "#0F172A" },
              ]}
            >
              {t("auth_forgot_success_title")}
            </Text>

            <Text
              style={[
                styles.successSub,
                {
                  color: isDark
                    ? "rgba(255, 255, 255, 0.6)"
                    : "#64748B",
                },
              ]}
            >
              {t("auth_forgot_success_msg")}
            </Text>

            <View style={styles.successFooter}>
              <Pressable
                onPress={() => router.replace("/(auth)/sign-in")}
                style={({ pressed }) => [
                  styles.continuePillBtn,
                  {
                    backgroundColor: isDark ? "#FFFFFF" : "#0F172A",
                    transform: [{ scale: pressed ? 0.98 : 1 }],
                  },
                ]}
              >
                <Text
                  style={[
                    styles.continuePillBtnText,
                    { color: isDark ? "#0F172A" : "#FFFFFF" },
                  ]}
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

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  keyboardContainer: {
    flex: 1,
  },
  scrollContent: {
    flexGrow: 1,
    justifyContent: "space-between",
    paddingHorizontal: Spacing.five,
    paddingTop: Platform.OS === "ios" ? 54 : 36,
    paddingBottom: Spacing.eight,
  },
  navHeader: {
    flexDirection: "row",
    alignItems: "center",
    marginBottom: Spacing.six,
  },
  navIconBtn: {
    width: 40,
    height: 40,
    borderRadius: Radii.full,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  mainContent: {
    flex: 1,
    justifyContent: "center",
  },
  titleBlock: {
    marginBottom: Spacing.six,
  },
  stepQuestion: {
    ...Typography.display,
    fontSize: 28,
    lineHeight: 34,
    fontFamily: Fonts.sans,
    fontWeight: "800",
    letterSpacing: -0.5,
  },
  stepHint: {
    ...Typography.bodyMedium,
    fontFamily: Fonts.sans,
    marginTop: Spacing.one,
  },
  errorPill: {
    borderRadius: Radii.lg,
    borderWidth: 1,
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.two,
    marginBottom: Spacing.four,
  },
  errorPillText: {
    ...Typography.caption,
    fontFamily: Fonts.sans,
    fontWeight: "600",
  },
  largePillInputContainer: {
    flexDirection: "row",
    alignItems: "center",
    height: 56,
    borderRadius: Radii.full,
    borderWidth: 1,
    paddingHorizontal: Spacing.four,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.06,
    shadowRadius: 6,
    elevation: 2,
  },
  largePillTextInput: {
    flex: 1,
    fontSize: 16,
    fontFamily: Fonts.sans,
    paddingHorizontal: Spacing.two,
  },
  arrowCircleBtn: {
    width: 38,
    height: 38,
    borderRadius: 19,
    alignItems: "center",
    justifyContent: "center",
  },
  successContent: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    paddingHorizontal: Spacing.four,
  },
  successIconBox: {
    width: 80,
    height: 80,
    borderRadius: 40,
    alignItems: "center",
    justifyContent: "center",
    marginBottom: Spacing.five,
  },
  successTitle: {
    ...Typography.h1,
    fontFamily: Fonts.sans,
    fontWeight: "800",
    textAlign: "center",
    marginBottom: Spacing.two,
  },
  successSub: {
    ...Typography.bodyMedium,
    fontFamily: Fonts.sans,
    textAlign: "center",
    lineHeight: 22,
    marginBottom: Spacing.eight,
  },
  successFooter: {
    width: "100%",
  },
  continuePillBtn: {
    height: 54,
    borderRadius: Radii.full,
    alignItems: "center",
    justifyContent: "center",
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.15,
    shadowRadius: 8,
    elevation: 4,
  },
  continuePillBtnText: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    fontSize: 16,
    fontWeight: "700",
  },
});
