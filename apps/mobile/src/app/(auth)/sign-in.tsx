/**
 * @module @orbis/mobile/app/(auth)/sign-in
 *
 * Ultra-modern, elegant Sign In screen directly modeled after the Mobbin Fuse
 * and minimalist reference designs (Atmospheric radiant glow, rotating capability
 * badges, frosted glass bottom container, sleek pill inputs, and tactile buttons).
 */

import React, { useState, useEffect } from "react";
import {
  StyleSheet,
  Text,
  View,
  Pressable,
  TextInput,
  Platform,
  Dimensions,
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { useRouter } from "expo-router";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { Radii, Spacing, Typography, Fonts } from "@/constants/theme";
import {
  ArrowRight,
  ArrowLeft,
  LockClosed,
  AcademicCap,
  ChartBar,
  Bolt,
  BuildingLibrary,
  GlobeAlt,
  Eye,
  EyeSlash,
} from "@orbis/icons/native";
import { Button, Input } from "@/components/ui";

const { height: SCREEN_HEIGHT } = Dimensions.get("window");

const CAPABILITIES = [
  { labelKey: "auth_capability_teach", icon: AcademicCap, color: "#8B5CF6" },
  { labelKey: "auth_capability_analyze", icon: ChartBar, color: "#06B6D4" },
  { labelKey: "auth_capability_automate", icon: Bolt, color: "#84CC16" },
  { labelKey: "auth_capability_govern", icon: BuildingLibrary, color: "#EC4899" },
] as const;

export default function SignInScreen() {
  const router = useRouter();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t, isArabic } = useI18n();
  const toast = useToast();
  const { login, loginWithGoogle, isLoading } = useAuth();

  const [activeCapabilityIndex, setActiveCapabilityIndex] = useState(0);
  const [authMode, setAuthMode] = useState<"welcome" | "email" | "password">("welcome");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  // Rotating capability reel animation effect
  useEffect(() => {
    const interval = setInterval(() => {
      setActiveCapabilityIndex((prev) => (prev + 1) % CAPABILITIES.length);
    }, 2400);
    return () => clearInterval(interval);
  }, []);

  const handleProceedFromEmail = () => {
    setErrorMsg(null);
    const trimmed = email.trim();
    if (!trimmed || !trimmed.includes("@")) {
      setErrorMsg(t("auth_error_required"));
      return;
    }
    setAuthMode("password");
  };

  const handleSignIn = async () => {
    setErrorMsg(null);
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsSubmitting(true);
    try {
      const res: any = await login(trimmedEmail, password);
      if (res?.mfa_required && (res?.user_id || res?.user?.id)) {
        router.push({
          pathname: "/(auth)/mfa-challenge",
          params: { userId: res.user_id || res.user.id },
        });
        return;
      }
      router.replace("/");
    } catch (err: any) {
      const msg = err?.message || t("auth_error_invalid_credentials");
      setErrorMsg(msg);
      toast.error(msg);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleGoogleSignIn = async () => {
    setIsSubmitting(true);
    try {
      await loginWithGoogle("google-token");
      router.replace("/");
    } catch (err: any) {
      toast.error(err?.message || t("auth_error_google"));
    } finally {
      setIsSubmitting(false);
    }
  };

  const activeCap = CAPABILITIES[activeCapabilityIndex];
  const ActiveIcon = activeCap.icon;
  const ArrowIcon = isArabic ? ArrowLeft : ArrowRight;

  return (
    <View
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        {/* Upper Visual Area: Dynamic Floating Capability Reel */}
        <View style={styles.upperSection}>
          <View style={styles.reelContainer}>
            {CAPABILITIES.map((cap, idx) => {
              const isActive = idx === activeCapabilityIndex;
              const CapIcon = cap.icon;
              return (
                <View
                  key={cap.labelKey}
                  style={[
                    styles.reelItem,
                    isActive && styles.reelItemActive,
                  ]}
                >
                  {isActive ? (
                    <View
                      style={[
                        styles.activeCapBadge,
                        {
                          backgroundColor: isDark
                            ? "rgba(255, 255, 255, 0.1)"
                            : "rgba(255, 255, 255, 0.9)",
                          borderColor: isDark
                            ? "rgba(255, 255, 255, 0.2)"
                            : "rgba(0, 0, 0, 0.08)",
                        },
                      ]}
                    >
                      <CapIcon size={22} color={cap.color} />
                      <Text
                        style={[
                          styles.activeCapText,
                          { color: isDark ? "#FFFFFF" : "#0F172A" },
                        ]}
                      >
                        {t(cap.labelKey)}
                      </Text>
                    </View>
                  ) : (
                    <Text
                      style={[
                        styles.inactiveCapText,
                        {
                          color: isDark
                            ? "rgba(255, 255, 255, 0.18)"
                            : "rgba(15, 23, 42, 0.2)",
                        },
                      ]}
                    >
                      {t(cap.labelKey)}
                    </Text>
                  )}
                </View>
              );
            })}
          </View>
        </View>

        {/* Bottom Floating Frosted Glass Card (Directly like Reference Image 1) */}
        <View
          style={[
            styles.bottomCard,
            {
              backgroundColor: isDark
                ? "rgba(24, 24, 27, 0.88)"
                : "rgba(255, 255, 255, 0.92)",
              borderColor: isDark
                ? "rgba(255, 255, 255, 0.12)"
                : "rgba(0, 0, 0, 0.06)",
              shadowColor: isDark ? "#000000" : "#7C3AED",
            },
          ]}
        >

          {/* Inline Error if any */}
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

          {/* MODE 1: WELCOME SCREEN (Two tactile pill buttons) */}
          {authMode === "welcome" && (
            <View style={styles.actionsContainer}>
              {/* Primary White Pill Button: Google */}
              <Button
                onPress={handleGoogleSignIn}
              >
                <Text              
                style={[
                    styles.pillButtonSecondaryText,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}>
                  {t("auth_continue_google")}
                </Text>
              </Button>

              {/* Secondary Translucent Pill Button: Email */}
              <Button
                onPress={() => setAuthMode("email")}
                variant="secondary"
              >
                <Text
                  style={[
                    styles.pillButtonSecondaryText,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                >
                  {t("auth_sign_in")}
                </Text>
              </Button>
            </View>
          )}

          {/* MODE 2: EMAIL STEP (Minimalist Pill Input with embedded arrow button) */}
          {authMode === "email" && (
            <View>
              <View
                style={[
                  styles.pillInputContainer
                ]}
              >
                <Input
                  placeholder={t("auth_email_placeholder")}
                  value={email}
                  onChangeText={setEmail}
                  keyboardType="email-address"
                  autoCapitalize="none"
                  autoFocus
                  onSubmitEditing={handleProceedFromEmail}
                  returnKeyType="next"
                />

                {/* Embedded Forward Arrow Pill Button */}
                <Button onPress={handleProceedFromEmail}>
                  <ArrowIcon size={16}/>
                </Button>
              </View>

              <Button onPress={() => setAuthMode("welcome")} >
                <Text>
                  {t("header_back")}
                </Text>
              </Button>
            </View>
          )}

          {/* MODE 3: PASSWORD STEP */}
          {authMode === "password" && (
            <View>
              <View
                style={[
                  styles.pillInputContainer
                ]}
              >
                <Input
                  placeholder="••••••••"
                  placeholderTextColor={
                    isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                  }
                  value={password}
                  onChangeText={setPassword}
                  secureTextEntry={!showPassword}
                  autoFocus
                  onSubmitEditing={handleSignIn}
                  returnKeyType="go"
                />

                {/* Eye toggle */}
                <Button
                  onPress={() => setShowPassword(!showPassword)}
                >
                  {showPassword ? (
                    <EyeSlash
                      size={18}
                      color={isDark ? "rgba(255,255,255,0.6)" : "#64748B"}
                    />
                  ) : (
                    <Eye
                      size={18}
                      color={isDark ? "rgba(255,255,255,0.6)" : "#64748B"}
                    />
                  )}
                </Button>

                {/* Embedded Forward Arrow Pill Button */}
                <Button
                  onPress={handleSignIn}
                  disabled={isSubmitting}
                >
                  <ArrowIcon size={16} color="#FFFFFF" />
                </Button>
              </View>

              <View >
                <Button
                  onPress={() => setAuthMode("email")}
                >
                  <Text
                  >
                    {t("header_back")}
                  </Text>
                </Button>

                <Button onPress={() => router.push("/(auth)/forgot-password")}>
                  <Text>
                    {t("auth_forgot_password")}
                  </Text>
                </Button>
              </View>
            </View>
          )}

          {/* Bottom Link: Create Account */}
          <View style={styles.cardFooter}>
            <Text
              style={[
                styles.footerMutedText,
                {
                  color: isDark
                    ? "rgba(255, 255, 255, 0.5)"
                    : "#64748B",
                },
              ]}
            >
              {t("auth_no_account")}
            </Text>
            <Button
              onPress={() => router.push("/(auth)/register")}
              variant="link"
            >
              <Text style={[Typography.bodyMedium, { color: theme.primary }]}>
                {t("auth_register_title")}
              </Text>
            </Button>
          </View>
        </View>
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
    paddingTop: Platform.OS === "ios" ? 80 : 50,
    paddingBottom: Spacing.eight,
  },
  ambientOrb: {
    position: "absolute",
    top: SCREEN_HEIGHT * 0.15,
    alignSelf: "center",
    width: 280,
    height: 280,
    borderRadius: 140,
  },
  upperSection: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
    minHeight: 220,
  },
  reelContainer: {
    alignItems: "center",
    justifyContent: "center",
    gap: Spacing.three,
  },
  reelItem: {
    height: 48,
    alignItems: "center",
    justifyContent: "center",
  },
  reelItemActive: {
    height: 56,
  },
  activeCapBadge: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
    paddingHorizontal: Spacing.five,
    paddingVertical: Spacing.three,
    borderRadius: Radii.full,
    borderWidth: 1,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 4 },
    shadowOpacity: 0.12,
    shadowRadius: 8,
    elevation: 3,
  },
  activeCapText: {
    ...Typography.h2,
    fontFamily: Fonts.sans,
    fontWeight: "800",
  },
  inactiveCapText: {
    ...Typography.h3,
    fontFamily: Fonts.sans,
    fontWeight: "600",
  },
  bottomCard: {
    borderRadius: 32,
    borderWidth: 1,
    paddingHorizontal: Spacing.six,
    paddingTop: Spacing.six,
    paddingBottom: Spacing.five,
    shadowOffset: { width: 0, height: 12 },
    shadowOpacity: 0.15,
    shadowRadius: 24,
    elevation: 8,
  },
  cardHeader: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.three,
    marginBottom: Spacing.five,
  },
  logoIconBox: {
    width: 44,
    height: 44,
    borderRadius: Radii.xl,
    alignItems: "center",
    justifyContent: "center",
  },
  cardHeaderText: {
    flex: 1,
  },
  cardTitle: {
    ...Typography.h2,
    fontFamily: Fonts.sans,
    fontWeight: "800",
    letterSpacing: -0.3,
  },
  cardSub: {
    ...Typography.caption,
    fontFamily: Fonts.sans,
    marginTop: 2,
  },
  errorPill: {
    borderRadius: Radii.lg,
    borderWidth: 1,
    paddingHorizontal: Spacing.three,
    paddingVertical: Spacing.two,
    marginBottom: Spacing.three,
  },
  errorPillText: {
    ...Typography.caption,
    fontFamily: Fonts.sans,
    fontWeight: "600",
  },
  actionsContainer: {
    gap: Spacing.three,
  },
  pillButtonPrimary: {
    height: 52,
    borderRadius: Radii.full,
    alignItems: "center",
    justifyContent: "center",
  },
  pillButtonPrimaryText: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    fontWeight: "700",
  },
  pillButtonSecondary: {
    height: 52,
    borderRadius: Radii.full,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  pillButtonSecondaryText: {
    ...Typography.bodyBold,
    fontFamily: Fonts.sans,
    fontWeight: "600",
  },

  pillInputContainer: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.one,
    paddingHorizontal: Spacing.four,
  },
  forgotLinkText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
  },
  cardFooter: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "center",
    gap: Spacing.two,
    marginTop: Spacing.five,
    paddingTop: Spacing.three,
  },
  footerMutedText: {
    ...Typography.caption,
    fontFamily: Fonts.sans,
  },
  footerActionText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
    fontWeight: "700",
  },
});
