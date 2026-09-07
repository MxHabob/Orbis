/**
 * @module @orbis/mobile/app/(app)/settings/index
 *
 * Settings Hub Index Screen.
 */

import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function SettingsIndexScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الإعدادات العامة" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        <Card variant="elevated" style={styles.menuCard}>
          {[
            {
              icon: 'person-outline',
              title: 'الملف الشخصي والحساب',
              sub: 'تعديل البيانات الأساسية والاتصال',
              route: '/(app)/settings/account',
            },
            {
              icon: 'shield-checkmark-outline',
              title: 'الأمان والمصادقة',
              sub: 'كلمة المرور، MFA، الجلسات النشطة',
              route: '/(app)/settings/security',
            },
            {
              icon: 'logo-google',
              title: 'الحسابات المرتبطة',
              sub: 'إدارة حساب Google المتصل',
              route: '/(app)/settings/linked-accounts',
            },
            {
              icon: 'options-outline',
              title: 'التفضيلات والإشعارات',
              sub: 'اللغة، المظهر، خيارات التنبيه',
              route: '/(app)/settings/preferences',
            },
          ].map((item, idx) => (
            <Pressable
              key={idx}
              onPress={() => router.push(item.route as any)}
              style={({ pressed }) => [
                styles.menuItem,
                idx > 0 && { borderTopWidth: 1, borderTopColor: theme.borderSubtle },
                pressed && { backgroundColor: theme.backgroundElement },
              ]}>
              <View style={[styles.iconBox, { backgroundColor: theme.primaryLight }]}>
                <Ionicons name={item.icon as any} size={22} color={theme.primary} />
              </View>

              <View style={styles.textCol}>
                <Text style={[styles.itemTitle, { color: theme.text }]}>{item.title}</Text>
                <Text style={[styles.itemSub, { color: theme.textSecondary }]}>{item.sub}</Text>
              </View>

              <Ionicons name="chevron-forward" size={18} color={theme.textSecondary} />
            </Pressable>
          ))}
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
    gap: Spacing.four,
  },
  menuCard: {
    padding: 0,
    overflow: 'hidden',
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: Spacing.three + 2,
    paddingHorizontal: Spacing.four,
  },
  iconBox: {
    width: 40,
    height: 40,
    borderRadius: 20,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: Spacing.three,
  },
  textCol: {
    flex: 1,
  },
  itemTitle: {
    ...Typography.bodyBold,
  },
  itemSub: {
    ...Typography.small,
    marginTop: 2,
  },
});
