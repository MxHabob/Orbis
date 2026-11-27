import React, { useEffect, useRef, useState } from "react";
import {
  Pressable,
  TextInput,
  View,
  type TextInputInstance,
  type TextInputKeyPressEvent,
} from "react-native";
import { useLocalSearchParams, useRouter } from "expo-router";
import { XMark } from "@orbis/icons/native";
import { Screen } from "@/components/layout/screen";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { Text } from "@/components/ui/text";
import { useFormValidation } from "@/hooks/use-form-validation";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function MFAChallengeScreen() {
  const router = useRouter();
  const params = useLocalSearchParams<{ userId?: string; mfaToken?: string }>();
  const theme = useTheme();
  const { t } = useI18n();
  const { mfaChallenge, userId: sessionUserId } = useAuth();

  const targetUserId = params.userId || sessionUserId || "";

  const [digits, setDigits] = useState<string[]>(["", "", "", "", "", ""]);
  const [isBackupMode, setIsBackupMode] = useState(false);
  const [countdown, setCountdown] = useState(55);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const {
    values: backupValues,
    errors: backupErrors,
    formError,
    setFormError,
    setValue: setBackupValue,
    markTouched: markBackupTouched,
    validateAll: validateBackupAll,
    reset: resetBackup,
  } = useFormValidation({
    initialValues: { backupCode: "" },
    validators: {
      backupCode: (value) => (!value.trim() ? t("auth_error_required") : null),
    },
  });

  const inputRefs = useRef<(TextInputInstance | null)[]>([]);

  useEffect(() => {
    if (countdown <= 0) return;
    const timer = setInterval(() => setCountdown((c) => c - 1), 1000);
    return () => clearInterval(timer);
  }, [countdown]);

  const submitCode = async (codeToSubmit: string) => {
    if (!targetUserId) {
      setFormError(t("auth_error_failed"));
      return;
    }

    setIsSubmitting(true);
    setFormError(null);
    try {
      await mfaChallenge({
        code: codeToSubmit,
        mfa_token: params.mfaToken || targetUserId,
      });
    } catch (err: unknown) {
      const msg =
        err instanceof Error
          ? err.message
          : t("auth_error_invalid_credentials");
      setFormError(msg);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDigitChange = (val: string, index: number) => {
    setFormError(null);
    if (val.length > 1) {
      const sanitized = val.replace(/[^0-9]/g, "").slice(0, 6);
      const newDigits = [...digits];
      for (let i = 0; i < 6; i++) {
        newDigits[i] = sanitized[i] || "";
      }
      setDigits(newDigits);
      if (sanitized.length === 6) {
        inputRefs.current[5]?.focus();
        void submitCode(sanitized);
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
      void submitCode(fullCode);
    }
  };

  const handleKeyPress = (e: TextInputKeyPressEvent, index: number) => {
    if (e.nativeEvent.key === "Backspace" && !digits[index] && index > 0) {
      inputRefs.current[index - 1]?.focus();
    }
  };

  const handleManualSubmit = () => {
    if (isBackupMode) {
      if (!validateBackupAll()) {
        return;
      }
      void submitCode(backupValues.backupCode.trim());
    } else {
      const fullCode = digits.join("");
      if (fullCode.length < 6) {
        setFormError(t("auth_error_required"));
        return;
      }
      void submitCode(fullCode);
    }
  };

  return (
    <Screen edges={["top", "bottom", "left", "right"]}>
      <KeyboardAwareView
        style={{ flex: 1 }}
        contentContainerStyle={{ flexGrow: 1 }}
      >
        <View className="flex-1 justify-between px-five pb-four pt-two">
          <View className="flex-row justify-end">
            <Pressable
              accessibilityRole="button"
              accessibilityLabel={t("common_cancel")}
              onPress={() => router.back()}
              className="h-11 w-11 items-center justify-center rounded-full"
              hitSlop={10}
            >
              <XMark size={20} color={theme.foreground} />
            </Pressable>
          </View>

          <View className="flex-1 justify-center gap-four">
            <Text className="text-[28px] leading-9 font-semibold tracking-tight text-foreground">
              {isBackupMode ? t("auth_mfa_backup_code") : t("auth_mfa_title")}
            </Text>

            <Card padding="four" className="gap-three">
              {!isBackupMode ? (
                <View className="flex-row justify-between py-two">
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
                        className={`h-14 w-12 rounded-md border bg-background text-center text-[22px] font-bold text-foreground ${digit ? "border-primary" : isCurrent ? "border-primary-dark" : "border-border"}`}
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
                <Input
                  label={t("auth_mfa_backup_code")}
                  placeholder="XXXX-XXXX-XXXX"
                  value={backupValues.backupCode}
                  onChangeText={(val) => setBackupValue("backupCode", val)}
                  onBlur={() => markBackupTouched("backupCode")}
                  error={backupErrors.backupCode}
                  autoCapitalize="characters"
                  autoFocus
                  returnKeyType="go"
                  onSubmitEditing={handleManualSubmit}
                />
              )}

              {formError ? (
                <Text
                  accessibilityRole="alert"
                  className="text-start text-xs leading-4 font-medium text-destructive"
                >
                  {formError}
                </Text>
              ) : null}

              <Button
                title={t("auth_verify")}
                onPress={handleManualSubmit}
                disabled={isSubmitting}
                loading={isSubmitting}
                fullWidth
                size="lg"
              />

              <View className="flex-row items-center justify-between pt-one">
                {!isBackupMode ? (
                  <Pressable
                    accessibilityRole="button"
                    disabled={countdown > 0}
                    onPress={() => countdown === 0 && setCountdown(55)}
                    className="min-h-11 justify-center"
                    hitSlop={8}
                  >
                    <Text className="text-xs leading-4 font-semibold text-primary">
                      {countdown > 0
                        ? `${t("auth_mfa_resend")} (${countdown}s)`
                        : t("auth_mfa_resend")}
                    </Text>
                  </Pressable>
                ) : (
                  <View />
                )}

                <Pressable
                  accessibilityRole="button"
                  onPress={() => {
                    setIsBackupMode(!isBackupMode);
                    setFormError(null);
                    resetBackup();
                  }}
                  className="min-h-11 justify-center"
                  hitSlop={8}
                >
                  <Text className="text-xs leading-4 font-semibold text-muted-foreground">
                    {isBackupMode
                      ? t("auth_mfa_standard_code")
                      : t("auth_mfa_backup_code")}
                  </Text>
                </Pressable>
              </View>
            </Card>
          </View>

          <View className="h-4" />
        </View>
      </KeyboardAwareView>
    </Screen>
  );
}
