/**
 * @module @orbis/mobile/app/(app)/settings/linked-accounts
 *
 * Linked Identity Providers & Google Account Management Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Alert } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function LinkedAccountsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const [googleLinked, setGoogleLinked] = useState(true);

  const toggleGoogle = () => {
    if (googleLinked) {
      Alert.alert('فصل الحساب', 'هل أنت متأكد من إلغاء ربط حساب Google؟', [
        { text: 'إلغاء', style: 'cancel' },
        {
          text: 'فصل',
          style: 'destructive',
          onPress: () => {
            setGoogleLinked(false);
            Alert.alert('تم', 'تم إلغاء ربط الحساب.');
          },
        },
      ]);
    } else {
      setGoogleLinked(true);
      Alert.alert('تم الربط', 'تم ربط حساب Google بنجاح.');
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الحسابات المرتبطة" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        <Card variant="elevated" style={styles.card}>
          <View style={styles.providerRow}>
            <View style={[styles.providerIcon, { backgroundColor: theme.backgroundElement }]}>
              <Ionicons name="logo-google" size={26} color={theme.text} />
            </View>

            <View style={{ flex: 1 }}>
              <Text style={[styles.providerTitle, { color: theme.text }]}>حساب Google</Text>
              <Text style={[styles.providerEmail, { color: theme.textSecondary }]}>
                {googleLinked ? 'user@gmail.com' : 'غير متصل'}
              </Text>
            </View>

            <Badge
              label={googleLinked ? 'متصل' : 'غير متصل'}
              variant={googleLinked ? 'success' : 'neutral'}
              size="sm"
            />
          </View>

          <Text style={[styles.desc, { color: theme.textSecondary }]}>
            يتيح لك ربط حساب Google تسجيل الدخول السريع بلمسة واحدة دون الحاجة لإدخال كلمة المرور في كل مرة.
          </Text>

          <Button
            title={googleLinked ? 'فصل حساب Google' : 'ربط حساب Google'}
            onPress={toggleGoogle}
            variant={googleLinked ? 'secondary' : 'primary'}
            size="md"
            fullWidth
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
  providerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  providerIcon: {
    width: 48,
    height: 48,
    borderRadius: 24,
    alignItems: 'center',
    justifyContent: 'center',
  },
  providerTitle: {
    ...Typography.bodyBold,
  },
  providerEmail: {
    ...Typography.small,
    marginTop: 2,
  },
  desc: {
    ...Typography.caption,
    lineHeight: 20,
  },
});
