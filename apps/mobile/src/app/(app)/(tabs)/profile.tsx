/**
 * @module @orbis/mobile/app/(app)/(tabs)/profile
 *
 * Unified Profile and Settings Hub.
 * Clean, declarative component composition powered by @orbis/sdk & TanStack Query.
 */

import React, { useCallback } from 'react';
import { View, Text, StyleSheet, ScrollView, Alert } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { ThemedStatusBar } from '@/components/ui/themed-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import { useUserQuery } from '@/hooks/use-api-queries';
import { Button } from '@/components/ui/button';
import {
  ProfileIdentityCard,
  LinkedChildrenSection,
  SettingsMenuList,
} from '@/components/profile';
import { Spacing, Typography } from '@/constants/theme';

export default function ProfileScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();
  const { user, logout } = useAuth();
  const {
    role,
    isParent,
    childrenList,
    selectedChildId,
    setSelectedChildId,
  } = useEffectiveContext();

  // ── TanStack Query Hook (Zero useEffect) ───────────────────────────────
  const { data: userProfile, isLoading: isUserLoading } = useUserQuery(user?.id);

  const handleLogout = useCallback(() => {
    Alert.alert(
      t('profile_sign_out'),
      t('profile_sign_out_confirm'),
      [
        { text: t('common_cancel'), style: 'cancel' },
        {
          text: t('profile_sign_out'),
          style: 'destructive',
          onPress: async () => {
            await logout();
            router.replace('/(auth)/sign-in');
          },
        },
      ],
    );
  }, [logout, t]);

  const fullName = userProfile?.full_name || user?.full_name || t('profile_user_default');
  const email = userProfile?.email || user?.email || 'user@orbis.edu.sa';

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <ThemedStatusBar backgroundColor={theme.card} />
      {/* Header */}
      <View
        style={[
          styles.header,
          {
            paddingTop: insets.top + Spacing.two,
            backgroundColor: theme.card,
            borderBottomColor: theme.borderSubtle,
          },
        ]}>
        <Text style={[styles.headerTitle, { color: theme.text }]}>{t('profile_title')}</Text>
      </View>

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: Math.max(insets.bottom, 16) + 90 },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* 1. Identity Card */}
        <ProfileIdentityCard
          fullName={fullName}
          email={email}
          role={role as any}
          isLoading={isUserLoading}
        />

        {/* 2. Linked Children for Parent */}
        {isParent && (
          <LinkedChildrenSection
            childrenList={childrenList}
            selectedChildId={selectedChildId}
            onSelectChild={setSelectedChildId}
          />
        )}

        {/* 3. Settings Menu List */}
        <SettingsMenuList />

        {/* 4. Logout Button */}
        <Button
          title={t('profile_sign_out_button')}
          onPress={handleLogout}
          variant="danger"
          size="lg"
          icon={<Ionicons name="log-out-outline" size={20} color="#FFFFFF" />}
          fullWidth
          style={{ marginTop: Spacing.two }}
        />
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  header: {
    paddingHorizontal: Spacing.four,
    paddingBottom: Spacing.three,
    borderBottomWidth: 1,
  },
  headerTitle: {
    ...Typography.h2,
    fontSize: 20,
  },
  scrollContent: {
    padding: Spacing.four,
    gap: Spacing.four,
  },
});
