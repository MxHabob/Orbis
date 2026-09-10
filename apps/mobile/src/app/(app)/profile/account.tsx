/**
 * @module @orbis/mobile/app/(app)/settings/account
 *
 * Account and Personal Profile Settings Screen.
 */

import React, { useState } from "react";
import { Pressable, ScrollView, StyleSheet, Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useAuth, api } from "@/lib/api";
import { useTheme } from "@/hooks/use-theme";
import { Header } from "@/components/header";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { useToast } from "@/components/ui/toast";
import { Spacing, Typography } from "@/constants/theme";

export default function AccountSettingsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { user } = useAuth();
  const toast = useToast();

  const [fullName, setFullName] = useState(user?.full_name || "");

  const email = user?.email || "";

  const [phone, setPhone] = useState("0501234567");
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);

    try {
      if (user?.id) {
        await api.updateUser(user.id, {
          full_name: fullName.trim(),
        });
      }

      toast.success("تم حفظ التعديلات بنجاح");
    } catch (err: any) {
      toast.error(err?.message || "فشل حفظ التعديلات.");
    } finally {
      setSaving(false);
    }
  };

  return (
    <View
      style={[
        styles.container,
        {
          backgroundColor: theme.background,
        },
      ]}
    >
      <ScrollView
        showsVerticalScrollIndicator={false}
        keyboardShouldPersistTaps="handled"
        contentContainerStyle={[
          styles.content,
          {
            paddingBottom: insets.bottom + 120,
          },
        ]}
      >
        <Header title="Account" showBack showCrown={false} />

        <View style={styles.form}>
          <FieldBlock label="الاسم الكامل" theme={theme}>
            <Input
              value={fullName}
              onChangeText={setFullName}
              placeholder="الاسم الكامل"
            />
          </FieldBlock>

          <FieldBlock label="البريد الإلكتروني" theme={theme}>
            <Input
              value={email}
              editable={false}
              placeholder="البريد الإلكتروني"
            />

            <Text
              style={[
                styles.hint,
                {
                  color: theme.textMuted,
                },
              ]}
            >
              البريد الإلكتروني معتمد بواسطة المؤسسة
            </Text>
          </FieldBlock>

          <FieldBlock label="رقم الهاتف" theme={theme}>
            <Input
              value={phone}
              onChangeText={setPhone}
              keyboardType="phone-pad"
              placeholder="05XXXXXXXX"
            />
          </FieldBlock>
        </View>

        <View style={styles.saveArea}>
          <Button
            title="حفظ التغييرات"
            onPress={handleSave}
            loading={saving}
            size="lg"
            fullWidth
          />
        </View>
      </ScrollView>
    </View>
  );
}

function FieldBlock({
  label,
  theme,
  children,
}: {
  label: string;
  theme: ReturnType<typeof useTheme>;
  children: React.ReactNode;
}) {
  return (
    <View style={styles.fieldBlock}>
      <Text
        style={[
          styles.fieldLabel,
          {
            color: theme.textMuted,
          },
        ]}
      >
        {label}
      </Text>

      {children}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  content: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.two,
  },

  form: {
    width: "100%",
    maxWidth: 520,
    alignSelf: "center",
    marginTop: Spacing.six,
    gap: Spacing.five,
  },

  fieldBlock: {
    gap: Spacing.two,
  },

  fieldLabel: {
    ...Typography.small,
    fontSize: 10,
    letterSpacing: 0.4,
  },

  hint: {
    ...Typography.small,
    marginTop: 2,
  },

  saveArea: {
    width: "100%",
    maxWidth: 520,
    alignSelf: "center",
    marginTop: Spacing.six,
  },
});
