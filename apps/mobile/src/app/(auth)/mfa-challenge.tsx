/**
 * @module @orbis/mobile/app/(auth)/mfa-challenge
 *
 * High-end Two-Factor Authentication screen directly modeled after Reference Image 1
 * (Screen 7/8 with radiant purple top glow, subtle stepper context, clean 6-digit PIN boxes,
 * and bottom tactile pill button).
 */

import React, { useState, useRef, useEffect } from "react";
import {
  Text,
  View,
  TextInput,
  Pressable,
  type TextInputInstance,
} from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { Button } from "@/components/ui/button";
import { XMark } from "@orbis/icons/native";

export default function MFAChallengeScreen() {
  const router = useRouter();
  const params = useLocalSearchParams<{ userId?: string }>();
  const theme = useTheme();
  const { t } = useI18n();
  const { mfaChallenge, userId: sessionUserId } = useAuth();

  const targetUserId = params.userId || sessionUserId || "";

  const [digits, setDigits] = useState<string[]>(["", "", "", "", "", ""]);
  const [isBackupMode, setIsBackupMode] = useState(false);
  const [backupCode, setBackupCode] = useState("");
  const [countdown, setCountdown] = useState(55);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errorMsg, setErrorMsg] = useState<string | null>(null);

  const inputRefs = useRef<(TextInputInstance | null)[]>([]);

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
        code: codeToSubmit,
        mfa_token: (params as any)?.mfaToken || targetUserId,
      });
    } catch (err: any) {
      const msg = err?.message || t("auth_error_invalid_credentials");
      setErrorMsg(msg);
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
    <View className="flex-1 bg-background">
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{
          flexGrow: 1,
          justifyContent: "space-between",
          paddingHorizontal: 24,
          paddingTop: 48,
          paddingBottom: 32,
        }}
      >
        {/* Minimalist Top Bar (Like Reference 1) */}
        <View className="mb-four flex-row items-center justify-between">
          <Text className="text-xs leading-4 font-semibold tracking-[0.5px] text-muted-foreground uppercase">
            {t("auth_setup_title")}
          </Text>
          <Pressable
            onPress={() => router.back()}
            className="h-11 w-11 items-center justify-center"
            hitSlop={{ top: 10, bottom: 10, left: 10, right: 10 }}
          >
            <XMark size={16} color={theme.foreground} />
          </Pressable>
        </View>

        {/* Minimal Stepper Status Header */}
        <View className="mb-six">
          <View className="flex-row items-center gap-two">
            <View className="h-5 w-5 items-center justify-center rounded-full bg-primary">
              <Text className="text-[11px] font-extrabold text-white">✓</Text>
            </View>
            <Text className="text-xs leading-4 font-semibold text-muted-foreground">
              {t("auth_setup_step_device")}
            </Text>
          </View>
        </View>

        {/* Main Content Area */}
        <View className="flex-1 justify-center">
          <Text className="mb-one text-[28px] leading-8.5 font-extrabold tracking-[-0.5px] text-foreground">
            {isBackupMode
              ? t("auth_mfa_backup_code")
              : t("auth_setup_step_recovery")}
          </Text>

          <Text className="text-sm leading-5 text-muted-foreground">
            {isBackupMode ? t("auth_mfa_backup_ph") : t("auth_mfa_subtitle")}
          </Text>

          {/* Resend Timer Link */}
          {!isBackupMode && (
            <Pressable
              onPress={() => countdown === 0 && setCountdown(55)}
              className="mb-four mt-two"
            >
              <Text className="text-xs leading-4 font-semibold text-primary">
                {countdown > 0
                  ? `${t("auth_mfa_resend")} (${countdown}s)`
                  : t("auth_mfa_resend")}
              </Text>
            </Pressable>
          )}

          {errorMsg ? (
            <View className="mb-four rounded-lg border border-destructive bg-destructive/10 px-three py-two">
              <Text className="text-xs leading-4 font-semibold text-destructive">
                {errorMsg}
              </Text>
            </View>
          ) : null}

          {!isBackupMode ? (
            /* 6 Large PIN Boxes with breathing room */
            <View className="my-four flex-row justify-between">
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
                    className={`h-14.5 w-12 rounded-md border bg-background-element text-center text-[22px] font-bold text-foreground ${digit ? "border-primary" : isCurrent ? "border-primary-dark" : "border-border"}`}
                    style={{
                      shadowColor: digit ? theme.primary : "#000000",
                      shadowOffset: { width: 0, height: 2 },
                      shadowOpacity: 0.08,
                      shadowRadius: 4,
                      elevation: 2,
                    }}
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
            <View className="my-four">
              <View className="h-14 flex-row items-center rounded-full border border-border bg-background-element px-four">
                <TextInput
                  className="flex-1 px-two text-base text-foreground"
                  placeholder="XXXX-XXXX-XXXX"
                  placeholderTextColor={theme.textMuted}
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
            className="mt-three items-center"
            hitSlop={{ top: 8, bottom: 8, left: 8, right: 8 }}
          >
            <Text className="text-xs leading-4 font-semibold text-muted-foreground">
              {isBackupMode
                ? t("auth_mfa_standard_code")
                : t("auth_mfa_backup_code")}
            </Text>
          </Pressable>
        </View>

        {/* Bottom Tactile Pill Button */}
        <View className="mt-six">
          <Button
            title={t("auth_verify")}
            onPress={handleManualSubmit}
            disabled={isSubmitting}
            loading={isSubmitting}
            fullWidth
            size="lg"
          />
        </View>
      </KeyboardAwareView>
    </View>
  );
}
