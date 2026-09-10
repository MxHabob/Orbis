/**
 * @module @orbis/mobile/app/(auth)/forgot-password
 *
 * Password recovery screen with verification instructions.
 */

import React, { useState } from "react";
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  KeyboardAvoidingView,
  Platform,
} from "react-native";
import { router } from "expo-router";
import { Ionicons } from "@expo/vector-icons";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Header } from "@/components/header";
import { KeyboardAwareView } from "@/components/ui/keyboard-aware-view";
import { Radii, Spacing, Typography } from "@/constants/theme";

export default function ForgotPasswordScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const [email, setEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async () => {
    if (!email.trim()) {
      setError("يرجى إدخال البريد الإلكتروني");
      return;
    }

    setLoading(true);
    setError(null);
    try {
      await new Promise((resolve) => setTimeout(resolve, 800));
      setSubmitted(true);
    } catch (err: any) {
      setError(err?.message || "حدث خطأ أثناء إرسال الرابط.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="استعادة كلمة المرور" showBack />

      <KeyboardAwareView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
      >
        <Card variant="elevated" style={styles.card}>
          {submitted ? (
            <View style={styles.successBox}>
              <View
                style={[
                  styles.successIconCircle,
                  { backgroundColor: theme.successLight },
                ]}
              >
                <Ionicons
                  name="mail-unread-outline"
                  size={40}
                  color={theme.success}
                />
              </View>
              <Text style={[styles.successTitle, { color: theme.text }]}>
                تم إرسال الرابط!
              </Text>
              <Text
                style={[styles.successDesc, { color: theme.textSecondary }]}
              >
                تم إرسال تعليمات إعادة تعيين كلمة المرور إلى البريد الإلكتروني (
                {email}). يرجى التحقق من صندوق الوارد.
              </Text>
              <Button
                title="العودة لتسجيل الدخول"
                onPress={() => router.replace("/(auth)/sign-in")}
                fullWidth
                size="lg"
                style={{ marginTop: Spacing.three }}
              />
            </View>
          ) : (
            <>
              <Text style={[styles.cardTitle, { color: theme.text }]}>
                إعادة تعيين كلمة المرور
              </Text>
              <Text
                style={[styles.cardSubtitle, { color: theme.textSecondary }]}
              >
                أدخل البريد الإلكتروني المرتبط بحسابك وسنرسل لك رابطاً لإعادة
                تعيين كلمة المرور
              </Text>

              {error && (
                <View
                  style={[
                    styles.errorBox,
                    { backgroundColor: theme.errorLight },
                  ]}
                >
                  <Ionicons name="alert-circle" size={18} color={theme.error} />
                  <Text style={[styles.errorText, { color: theme.error }]}>
                    {error}
                  </Text>
                </View>
              )}

              <Input
                label="البريد الإلكتروني"
                placeholder="name@school.edu.sa"
                value={email}
                onChangeText={(t) => {
                  setEmail(t);
                  setError(null);
                }}
                autoCapitalize="none"
                keyboardType="email-address"
                icon={
                  <Ionicons
                    name="mail-outline"
                    size={20}
                    color={theme.textMuted}
                  />
                }
              />

              <Button
                title="إرسال رابط الاستعادة"
                onPress={handleSubmit}
                loading={loading}
                fullWidth
                size="lg"
              />
            </>
          )}
        </Card>
      </KeyboardAwareView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    alignItems: "center",
    paddingTop: Spacing.four,
  },
  card: {
    width: "100%",
    maxWidth: 440,
    padding: Spacing.five,
    gap: Spacing.three,
  },
  cardTitle: {
    ...Typography.h2,
  },
  cardSubtitle: {
    ...Typography.caption,
    marginBottom: Spacing.two,
  },
  errorBox: {
    flexDirection: "row",
    alignItems: "center",
    padding: Spacing.three,
    borderRadius: Radii.md,
    gap: Spacing.two,
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  successBox: {
    alignItems: "center",
    gap: Spacing.three,
    paddingVertical: Spacing.three,
  },
  successIconCircle: {
    width: 72,
    height: 72,
    borderRadius: 36,
    alignItems: "center",
    justifyContent: "center",
  },
  successTitle: {
    ...Typography.h2,
    textAlign: "center",
  },
  successDesc: {
    ...Typography.body,
    textAlign: "center",
  },
});
