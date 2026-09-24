/**
 * @module @orbis/mobile/app/(auth)/mfa-challenge
 *
 * High-end Two-Factor Authentication screen directly modeled after Reference Image 1
 * (Screen 7/8 with radiant purple top glow, subtle stepper context, clean 6-digit PIN boxes,
 * and bottom tactile pill button).
 */

import React, { useState, useRef, useEffect } from "react";
import {
  StyleSheet,
  Text,
  View,
  TextInput,
  Pressable,
  Platform,
} from "react-native";
import { LinearGradient } from "expo-linear-gradient";
import { useLocalSearchParams, useRouter } from "expo-router";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { useToast } from "@/components/ui/toast";
import { Radii, Spacing, Typography, Fonts } from "@/constants/theme";
import { XMark, ShieldCheck, KeyframePlus } from "@orbis/icons/native";

export default function MFAChallengeScreen() {
  const router = useRouter();
  const params = useLocalSearchParams<{ userId?: string }>();
  const theme = useTheme();
  const colorScheme = useColorScheme();
  const isDark = colorScheme === "dark";
  const { t } = useI18n();
  const toast = useToast();
  const { mfaChallenge, userId: sessionUserId } = useAuth();

  const targetUserId = params.userId || sessionUserId || "";

  const [digits, setDigits] = useState<string[]>(["", "", "", "", "", ""]);
  const [isBackupMode, setIsBackupMode] = useState(false);
  const [backupCode, setBackupCode] = useState("");
  const [countdown, setCountdown] = useState(55);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const inputRefs = useRef<Array<TextInput | null>>([]);

  useEffect(() => {
    if (countdown <= 0) return;
    const timer = setInterval(() => setCountdown((c) => c - 1), 1000);
    return () => clearInterval(timer);
  }, [countdown]);

  const handleDigitChange = (val: string, index: number) => {
    setErrorMsg(null);
    if (val.length > 1) {
      const sanitized = val.replace(/[^0-9]/g, "").slice(0, 6);
      const newDigits = [...digits];
      for (let i = 0; i < 6; i++) {
        newDigits[i] = sanitized[i] || "";
      }
      setDigits(newDigits);
      if (sanitized.length === 6) {
        inputRefs.current[5]?.focus();
        submitCode(sanitized);
      } else {
        const nextIdx = Math.min(sanitized.length, 5);
        inputRefs.current[nextIdx]?.focus();
      }
      return;
    }

    const single = val.replace(/[^0-9]/g, "");
    const newDigits = [...digits];
    newDigits[index] = single;
    setDigits(newDigits);

    if (single && index < 5) {
      inputRefs.current[index + 1]?.focus();
    }

    const fullCode = newDigits.join("");
    if (fullCode.length === 6 && !newDigits.includes("")) {
      submitCode(fullCode);
    }
  };

  const handleKeyPress = (e: any, index: number) => {
    if (e.nativeEvent.key === "Backspace" && !digits[index] && index > 0) {
      inputRefs.current[index - 1]?.focus();
    }
  };

  const submitCode = async (codeToSubmit: string) => {
    if (!targetUserId) {
      setErrorMsg(t("auth_error_failed"));
      return;
    }

    setIsSubmitting(true);
    setErrorMsg(null);
    try {
      await mfaChallenge({
        user_id: targetUserId,
        code: codeToSubmit,
      });

      toast.show({
        message: t("auth_verify"),
        type: "success",
      });

      router.replace("/(auth)/wallet-setup");
    } catch (err: any) {
      const msg = err?.message || t("auth_error_invalid_credentials");
      setErrorMsg(msg);
      toast.show({ message: msg, type: "error" });
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleManualSubmit = () => {
    if (isBackupMode) {
      const code = backupCode.trim();
      if (!code) {
        setErrorMsg(t("auth_error_required"));
        return;
      }
      submitCode(code);
    } else {
      const fullCode = digits.join("");
      if (fullCode.length < 6) {
        setErrorMsg(t("auth_error_required"));
        return;
      }
      submitCode(fullCode);
    }
  };

  return (
    <View
      style={[
        styles.container,
        { backgroundColor: isDark ? "#09090B" : "#F8FAFC" },
      ]}
    >
      {/* Radiant Atmospheric Purple/Violet Top Glow */}
      <LinearGradient
        colors={
          isDark
            ? (["#581C87", "#2E1065", "rgba(9,9,11,0)"] as const)
            : (["#C4B5FD", "#EDE9FE", "rgba(248,250,252,0)"] as const)
        }
        start={{ x: 0.5, y: 0 }}
        end={{ x: 0.5, y: 0.6 }}
        style={StyleSheet.absoluteFillObject}
        pointerEvents="none"
      />

      <KeyboardAwareView
        style={styles.keyboardContainer}
        contentContainerStyle={styles.scrollContent}
      >
        {/* Minimalist Top Bar (Like Reference 1) */}
        <View style={styles.topBar}>
          <Text
            style={[
              styles.topBarTitle,
              { color: isDark ? "rgba(255, 255, 255, 0.7)" : "#64748B" },
            ]}
          >
            {t("auth_setup_title")}
          </Text>
          <Pressable
            onPress={() => router.back()}
            style={[
              styles.closeBtn,
              {
                backgroundColor: isDark
                  ? "rgba(255, 255, 255, 0.1)"
                  : "rgba(0, 0, 0, 0.05)",
              },
            ]}
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <XMark size={16} color={isDark ? "#FFFFFF" : "#0F172A"} />
          </Pressable>
        </View>

        {/* Minimal Stepper Status Header */}
        <View style={styles.stepperHeader}>
          <View style={styles.stepIndicatorRow}>
            <View
              style={[
                styles.stepDotDone,
                { backgroundColor: isDark ? "#A855F7" : "#7C3AED" },
              ]}
            >
              <Text style={styles.stepCheckmark}>✓</Text>
            </View>
            <Text
              style={[
                styles.stepLabel,
                { color: isDark ? "rgba(255,255,255,0.6)" : "#64748B" },
              ]}
            >
              {t("auth_setup_step_device")}
            </Text>
          </View>
        </View>

        {/* Main Content Area */}
        <View style={styles.mainSection}>
          <Text
            style={[
              styles.heading,
              { color: isDark ? "#FFFFFF" : "#0F172A" },
            ]}
          >
            {isBackupMode ? t("auth_mfa_backup_code") : t("auth_setup_step_recovery")}
          </Text>

          <Text
            style={[
              styles.subheading,
              { color: isDark ? "rgba(255, 255, 255, 0.6)" : "#64748B" },
            ]}
          >
            {isBackupMode
              ? t("auth_mfa_backup_ph")
              : t("auth_mfa_subtitle")}
          </Text>

          {/* Resend Timer Link */}
          {!isBackupMode && (
            <Pressable
              onPress={() => countdown === 0 && setCountdown(55)}
              style={styles.resendLink}
            >
              <Text
                style={[
                  styles.resendText,
                  { color: isDark ? "#C084FC" : theme.primary },
                ]}
              >
                {countdown > 0
                  ? `${t("auth_mfa_resend")} (${countdown}s)`
                  : t("auth_mfa_resend")}
              </Text>
            </Pressable>
          )}

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

          {!isBackupMode ? (
            /* 6 Large PIN Boxes with breathing room */
            <View style={styles.pinContainer}>
              {digits.map((digit, idx) => {
                const isCurrent =
                  digits.findIndex((d) => d === "") === idx ||
                  (idx === 5 && digits[5] !== "");
                return (
                  <TextInput
                    key={idx}
                    ref={(el) => {
                      inputRefs.current[idx] = el;
                    }}
                    style={[
                      styles.pinBox,
                      {
                        backgroundColor: isDark
                          ? "rgba(255, 255, 255, 0.06)"
                          : "#FFFFFF",
                        borderColor: digit
                          ? theme.primary
                          : isCurrent
                            ? isDark
                              ? "#C084FC"
                              : theme.primaryDark
                            : isDark
                              ? "rgba(255, 255, 255, 0.15)"
                              : "rgba(0, 0, 0, 0.1)",
                        color: isDark ? "#FFFFFF" : "#0F172A",
                        shadowColor: digit ? theme.primary : "#000000",
                      },
                    ]}
                    value={digit}
                    onChangeText={(val) => handleDigitChange(val, idx)}
                    onKeyPress={(e) => handleKeyPress(e, idx)}
                    keyboardType="number-pad"
                    maxLength={idx === 0 ? 6 : 1}
                    selectTextOnFocus
                    autoFocus={idx === 0}
                  />
                );
              })}
            </View>
          ) : (
            /* Backup Code Input */
            <View style={styles.backupContainer}>
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
                    { color: isDark ? "#FFFFFF" : "#0F172A" },
                  ]}
                  placeholder="XXXX-XXXX-XXXX"
                  placeholderTextColor={
                    isDark ? "rgba(255, 255, 255, 0.4)" : "#94A3B8"
                  }
                  value={backupCode}
                  onChangeText={setBackupCode}
                  autoCapitalize="characters"
                  autoFocus
                />
              </View>
            </View>
          )}

          {/* Fallback mode toggle */}
          <Pressable
            onPress={() => {
              setIsBackupMode(!isBackupMode);
              setErrorMsg(null);
            }}
            style={styles.switchModeLink}
            hitSlop={{ top: 8, bottom: 8, left: 8, right: 8 }}
          >
            <Text
              style={[
                styles.switchModeText,
                { color: isDark ? "rgba(255,255,255,0.6)" : "#64748B" },
              ]}
            >
              {isBackupMode
                ? t("auth_mfa_standard_code")
                : t("auth_mfa_backup_code")}
            </Text>
          </Pressable>
        </View>

        {/* Bottom Tactile Pill Button */}
        <View style={styles.footer}>
          <Pressable
            onPress={handleManualSubmit}
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
              {t("auth_verify")}
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
  topBar: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: Spacing.four,
  },
  topBarTitle: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
    textTransform: "uppercase",
    letterSpacing: 0.5,
  },
  closeBtn: {
    width: 32,
    height: 32,
    borderRadius: 16,
    alignItems: "center",
    justifyContent: "center",
  },
  stepperHeader: {
    marginBottom: Spacing.six,
  },
  stepIndicatorRow: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.two,
  },
  stepDotDone: {
    width: 20,
    height: 20,
    borderRadius: 10,
    alignItems: "center",
    justifyContent: "center",
  },
  stepCheckmark: {
    color: "#FFFFFF",
    fontSize: 11,
    fontWeight: "800",
  },
  stepLabel: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
  },
  mainSection: {
    flex: 1,
    justifyContent: "center",
  },
  heading: {
    ...Typography.display,
    fontSize: 28,
    lineHeight: 34,
    fontFamily: Fonts.sans,
    fontWeight: "800",
    letterSpacing: -0.5,
    marginBottom: Spacing.two,
  },
  subheading: {
    ...Typography.bodyMedium,
    fontFamily: Fonts.sans,
    lineHeight: 20,
  },
  resendLink: {
    marginTop: Spacing.two,
    marginBottom: Spacing.four,
  },
  resendText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
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
  pinContainer: {
    flexDirection: "row",
    justifyContent: "space-between",
    marginVertical: Spacing.four,
  },
  pinBox: {
    width: 48,
    height: 58,
    borderRadius: Radii.xl,
    borderWidth: 1.5,
    fontSize: 22,
    fontWeight: "800",
    textAlign: "center",
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.08,
    shadowRadius: 4,
    elevation: 2,
  },
  backupContainer: {
    marginVertical: Spacing.four,
  },
  largePillInputContainer: {
    flexDirection: "row",
    alignItems: "center",
    height: 56,
    borderRadius: Radii.full,
    borderWidth: 1,
    paddingHorizontal: Spacing.four,
  },
  largePillTextInput: {
    flex: 1,
    fontSize: 16,
    fontFamily: Fonts.sans,
    paddingHorizontal: Spacing.two,
  },
  switchModeLink: {
    alignItems: "center",
    marginTop: Spacing.three,
  },
  switchModeText: {
    ...Typography.captionBold,
    fontFamily: Fonts.sans,
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
