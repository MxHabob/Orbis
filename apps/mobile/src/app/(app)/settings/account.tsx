/**
 * @module @orbis/mobile/app/(app)/settings/account
 *
 * Account and Personal Profile Settings Screen.
 */

import React, { useState } from 'react';
import { View, StyleSheet, ScrollView } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth, api } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/toast';
import { Spacing } from '@/constants/theme';

export default function AccountSettingsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { user } = useAuth();
  const toast = useToast();

  const [fullName, setFullName] = useState(user?.full_name || '');
  const [email] = useState(user?.email || '');
  const [phone, setPhone] = useState('0501234567');
  const [saving, setSaving] = useState(false);

  const handleSave = async () => {
    setSaving(true);
    try {
      if (user?.id) {
        await api.updateUser(user.id, { full_name: fullName.trim() });
      }

      toast.success('تم حفظ التعديلات بنجاح');
    } catch (err: any) {
      const msg = err?.message || 'فشل حفظ التعديلات.';
      toast.error(msg);
    } finally {
      setSaving(false);
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="المعلومات الشخصية" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        <Card variant="elevated" style={styles.card}>
          <Input
            label="الاسم الكامل"
            value={fullName}
            onChangeText={setFullName}
            placeholder="الاسم الثلاثي"
          />

          <Input
            label="البريد الإلكتروني المؤسسي"
            value={email}
            editable={false}
            hint="البريد الإلكتروني معتمد وموثق بواسطة الإدارة"
          />

          <Input
            label="رقم الهاتف الجوال"
            value={phone}
            onChangeText={setPhone}
            keyboardType="phone-pad"
            placeholder="05XXXXXXXX"
          />

          <Button
            title="حفظ التغييرات"
            onPress={handleSave}
            loading={saving}
            size="lg"
            fullWidth
            style={{ marginTop: Spacing.two }}
          />
        </Card>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
  },
  card: {
    padding: Spacing.five,
    gap: Spacing.three,
  },
});
