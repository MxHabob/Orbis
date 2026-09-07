/**
 * @module @orbis/mobile/components/profile/settings-menu-list
 *
 * Menu list for Account, Security, Linked accounts, and Preferences.
 */

import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Spacing, Typography } from '@/constants/theme';

export const SettingsMenuList = React.memo(() => {
  const theme = useTheme();
  const { t } = useI18n();

  const menuItems = [
    {
      id: 'account',
      title: t('profile_menu_account'),
      icon: 'person-circle-outline',
      route: '/(app)/settings/account',
    },
    {
      id: 'security',
      title: t('profile_menu_security'),
      icon: 'shield-checkmark-outline',
      route: '/(app)/settings/security',
    },
    {
      id: 'linked',
      title: t('profile_menu_linked'),
      icon: 'link-outline',
      route: '/(app)/settings/linked-accounts',
    },
    {
      id: 'preferences',
      title: t('profile_menu_preferences'),
      icon: 'notifications-outline',
      route: '/(app)/settings/preferences',
    },
  ];

  return (
    <View style={styles.section}>
      <Text style={[styles.title, { color: theme.text }]}>
        {t('profile_settings_title')}
      </Text>

      <Card variant="flat" style={styles.card}>
        {menuItems.map((item, idx) => (
          <Pressable
            key={item.id}
            onPress={() => router.push(item.route as any)}
            style={({ pressed }) => [
              styles.item,
              idx < menuItems.length - 1 && {
                borderBottomWidth: 1,
                borderBottomColor: theme.borderSubtle,
              },
              pressed && { backgroundColor: theme.backgroundElement },
            ]}>
            <View style={styles.itemLeft}>
              <View
                style={[styles.iconBox, { backgroundColor: theme.backgroundElement }]}>
                <Ionicons name={item.icon as any} size={20} color={theme.primary} />
              </View>
              <Text style={[styles.itemTitle, { color: theme.text }]}>{item.title}</Text>
            </View>

            <Ionicons name="chevron-forward" size={18} color={theme.textSecondary} />
          </Pressable>
        ))}
      </Card>
    </View>
  );
});

SettingsMenuList.displayName = 'SettingsMenuList';

const styles = StyleSheet.create({
  section: {
    gap: Spacing.two,
  },
  title: {
    ...Typography.h3,
    fontSize: 16,
    paddingHorizontal: Spacing.one,
  },
  card: {
    padding: 0,
    overflow: 'hidden',
  },
  item: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingVertical: Spacing.three + 2,
    paddingHorizontal: Spacing.four,
  },
  itemLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
  },
  iconBox: {
    width: 36,
    height: 36,
    borderRadius: 18,
    alignItems: 'center',
    justifyContent: 'center',
  },
  itemTitle: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
});
