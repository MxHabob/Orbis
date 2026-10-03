/**
 * @module @orbis/mobile/app/(auth)/register
 *
 * Minimalist, elegant step-by-step registration directly inspired by Reference Image 3
 * (Single focused question per step, large pill inputs, tactile selection chips,
 * and bottom pill continue buttons).
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
import { api, useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { Radii, Spacing, Typography, Fonts } from "@/constants/theme";
import {
  ArrowLeft,
  ArrowRight,
  AcademicCap,
  User,
  Users,
  ShieldCheck,
  Eye,
  EyeSlash,
} from "@orbis/icons/native";

type RoleOption = "teacher" | "student" | "parent" | "admin";

export default function RegisterScreen() {
  const router = useRouter();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t, isRTL } = useI18n();
  const toast = useToast();
  const { login } = useAuth();

  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [fullName, setFullName] = useState("");
  const [selectedRole, setSelectedRole] = useState<RoleOption>("teacher");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const roles: { key: RoleOption; label: string; icon: any }[] = [
    { key: "teacher", label: t("auth_role_teacher"), icon: AcademicCap },
    { key: "student", label: t("auth_role_student"), icon: User },
    { key: "parent", label: t("auth_role_parent"), icon: Users },
    { key: "admin", label: t("auth_role_admin"), icon: ShieldCheck },
  ];

  const handleNextStep1 = () => {
    setErrorMsg(null);
    if (!fullName.trim()) {
      setErrorMsg(t("auth_error_required"));
      return;
    }
    setStep(2);
  };

  const handleNextStep2 = () => {
    setErrorMsg(null);
    setStep(3);
  };

  const handleFinalSubmit = async () => {
    setErrorMsg(null);
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
      setErrorMsg(t("auth_error_required"));
      return;
    }

    setIsSubmitting(true);
    try {
      await api.registerUser({
        body: {
          full_name: fullName.trim(),
          email: trimmedEmail,
          password,
          role: selectedRole,
        },
        throwOnError: true,
      });

      toast.show({
        message: t("auth_register_success"),
        type: "success",
      });

      try {
        await login(trimmedEmail, password);
        router.replace("/(auth)/wallet-setup");
      } catch {
        router.replace("/(auth)/sign-in");
      }
    } catch (err: any) {
      const msg = err?.message || t("auth_error_failed");
      setErrorMsg(msg);
      toast.show({ message: msg, type: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleBack = () => {
    setErrorMsg(null);
    if (step === 3) setStep(2);
    else if (step === 2) setStep(1);
    else router.back();
  };

  const BackIcon = isRTL ? ArrowRight : ArrowLeft;

  return (
    <View
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >
      {/* Radiant Top Glow */}
      <LinearGradient
        colors={
          isDark
            ? (["#3B0764", "#1E1035", "rgba(9,9,11,0)"] as const)
            : (["#DDD6FE", "#EDE9FE", "rgba(248,250,252,0)"] as const)
        }
        start={{ x: 0.5, y: 0 }}
        end={{ x: 0.5, y: 0.5 }}
        style={StyleSheet.absoluteFillObject}
        pointerEvents="none"
      />

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        {/* Minimal Nav Bar */}
        <View style={styles.navHeader}>
          <Pressable
            onPress={handleBack}
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

          {/* Minimal Step Dots Indicator */}
          <View style={styles.stepDots}>
            {[1, 2, 3].map((s) => (
              <View
                key={s}
                style={[
                  styles.dot,
                  {
                    backgroundColor:
                      s <= step
                        ? theme.primary
                        : isDark
                          ? "rgba(255, 255, 255, 0.2)"
                          : "rgba(0, 0, 0, 0.1)",
                    width: s === step ? 20 : 6,
                  },
                ]}
              />
            ))}
          </View>

          <Pressable
            onPress={() => router.push("/(auth)/sign-in")}
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <Text
              style={[
                styles.navLinkText,
                { color: isDark ? "rgba(255, 255, 255, 0.6)" : "#64748B" },
              ]}
            >
              {t("auth_sign_in")}
            </Text>
          </Pressable>
        </View>

        {/* Dynamic Step Content */}
        <View style={styles.stepContent}>
          {/* STEP 1: NAME */}
          {step === 1 && (
            <View style={styles.stepInner}>
              <View style={styles.titleBlock}>
                <Text
                  style={[
                    styles.stepQuestion,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                >
                  {t("auth_full_name")}
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
                  {t("auth_register_subtitle")}
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
                      textAlign: isRTL ? "right" : "left",
                    },
                  ]}
                  placeholder={t("auth_full_name_ph")}
                  placeholderTextColor={
                    isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                  }
                  value={fullName}
                  onChangeText={setFullName}
                  autoFocus
                  onSubmitEditing={handleNextStep1}
                  returnKeyType="next"
                />
              </View>
            </View>
          )}

          {/* STEP 2: ROLE SELECTION */}
          {step === 2 && (
            <View style={styles.stepInner}>
              <View style={styles.titleBlock}>
                <Text
                  style={[
                    styles.stepQuestion,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                >
                  {t("auth_role_label")}
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
                  {t("auth_register_subtitle")}
                </Text>
              </View>

              {/* Chips Grid */}
              <View style={styles.rolesGrid}>
                {roles.map((r) => {
                  const isSelected = selectedRole === r.key;
                  const IconComp = r.icon;
                  return (
                    <Pressable
                      key={r.key}
                      onPress={() => setSelectedRole(r.key)}
                      style={({ pressed }) => [
                        styles.roleCard,
                        {
                          backgroundColor: isSelected
                            ? isDark
                              ? "rgba(124, 58, 237, 0.2)"
                              : "rgba(124, 58, 237, 0.08)"
                            : isDark
                              ? "rgba(255, 255, 255, 0.05)"
                              : "#FFFFFF",
                          borderColor: isSelected
                            ? theme.primary
                            : isDark
                              ? "rgba(255, 255, 255, 0.1)"
                              : "rgba(0, 0, 0, 0.08)",
                          transform: [{ scale: pressed ? 0.98 : 1 }],
                        },
                      ]}
                    >
                      <View
                        style={[
                          styles.roleIconCircle,
                          {
                            backgroundColor: isSelected
                              ? theme.primary
                              : isDark
                                ? "rgba(255, 255, 255, 0.08)"
                                : "rgba(0, 0, 0, 0.04)",
                          },
                        ]}
                      >
                        <IconComp
                          size={20}
                          color={isSelected ? "#FFFFFF" : isDark ? "#FFFFFF" : "#0F172A"}
                        />
                      </View>
                      <Text
                        style={[
                          styles.roleTitle,
                          {
                            color: isSelected
                              ? theme.primary
                              : isDark
                                ? "#FFFFFF"
                                : "#0F172A",
                            fontWeight: isSelected ? "700" : "500",
                          },
                        ]}
                      >
                        {r.label}
                      </Text>
                    </Pressable>
                  );
                })}
              </View>
            </View>
          )}

          {/* STEP 3: CREDENTIALS */}
          {step === 3 && (
            <View style={styles.stepInner}>
              <View style={styles.titleBlock}>
                <Text
                  style={[
                    styles.stepQuestion,
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                >
                  {t("auth_email_label")}
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
                  {t("auth_card_subtitle")}
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

              <View style={styles.credentialsInputs}>
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
                        textAlign: isRTL ? "right" : "left",
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
                  />
                </View>

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
                      marginTop: Spacing.three,
                    },
                  ]}
                >
                  <TextInput
                    style={[
                      styles.largePillTextInput,
                      {
                        color: isDark ? "#FFFFFF" : "#0F172A",
                        textAlign: isRTL ? "right" : "left",
                      },
                    ]}
                    placeholder="••••••••"
                    placeholderTextColor={
                      isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                    }
                    value={password}
                    onChangeText={setPassword}
                    secureTextEntry={!showPassword}
                    onSubmitEditing={handleFinalSubmit}
                    returnKeyType="go"
                  />
                  <Pressable
                    onPress={() => setShowPassword(!showPassword)}
                    style={{ paddingHorizontal: Spacing.two }}
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
                  </Pressable>
                </View>
              </View>
            </View>
          )}
        </View>

        {/* Bottom Tactile Pill Button */}
        <View style={styles.footer}>
          <Pressable
            onPress={
              step === 1
                ? handleNextStep1
                : step === 2
                  ? handleNextStep2
                  : handleFinalSubmit
            }
            disabled={isSubmitting}
            style={({ pressed }) => [
              styles.continuePillBtn,
              {
                backgroundColor: isDark ? "#FFFFFF" : "#0F172A",
                transform: [{ scale: pressed ? 0.98 : 1 }],
                opacity: isSubmitting ? 0.7 : 1,
              },
            ]}
          >
            <Text
              style={[
                styles.continuePillBtnText,
                { color: isDark ? "#0F172A" : "#FFFFFF" },
              ]}
            >
              {step === 3
                ? t("auth_create_account_btn")
                : t("admission_next_step")}
            </Text>
          </Pressable>
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
    paddingTop: Platform.OS === "ios" ? 54 : 36,
    paddingBottom: Spacing.eight,
  },
  navHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
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
  stepDots: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
  },
  dot: {
    height: 6,
    borderRadius: 3,
  },
  navLinkText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
  },
  stepContent: {
    flex: 1,
    justifyContent: "center",
  },
  stepInner: {
    width: "100%",
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
    paddingHorizontal: Spacing.five,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.06,
    shadowRadius: 6,
    elevation: 2,
  },
  largePillTextInput: {
    flex: 1,
    fontSize: 17,
    fontFamily: Fonts.sans,
    fontWeight: "500",
  },
  rolesGrid: {
    gap: Spacing.three,
  },
  roleCard: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.four,
    paddingHorizontal: Spacing.five,
    paddingVertical: Spacing.four,
    borderRadius: Radii.full,
    borderWidth: 1.5,
    shadowColor: "#000000",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.04,
    shadowRadius: 4,
    elevation: 1,
  },
  roleIconCircle: {
    width: 40,
    height: 40,
    borderRadius: 20,
    alignItems: "center",
    justifyContent: "center",
  },
  roleTitle: {
    ...Typography.bodyLarge,
    fontFamily: Fonts.sans,
    fontSize: 16,
  },
  credentialsInputs: {
    gap: Spacing.two,
  },
  footer: {
    marginTop: Spacing.six,
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
