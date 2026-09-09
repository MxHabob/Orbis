/**
 * @module @orbis/mobile/app/(app)/(tabs)/profile
 *
 * Unified Profile and Settings Hub.
 * Clean, declarative component composition powered by @orbis/sdk & TanStack Query.
 */

import React, { useCallback, useState } from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { ThemedStatusBar } from '@/components/ui/themed-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import { useUserQuery, useCurrentOrgQuery } from '@/hooks/use-api-queries';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/toast';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog';
import {
  ProfileIdentityCard,
  LinkedChildrenSection,
  SettingsMenuList,
} from '@/components/profile';
import { Spacing } from '@/constants/theme';

export default function ProfileScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { t } = useI18n();
  const { user, logout } = useAuth();
  const [showLogoutDialog, setShowLogoutDialog] = useState(false);
  const {
    role,
    isParent,
    childrenList,
    selectedChildId,
    setSelectedChildId,
  } = useEffectiveContext();

  // ── TanStack Query Hook (Zero useEffect) ───────────────────────────────
  const { data: userProfile, isLoading: isUserLoading } = useUserQuery(user?.id);
  const { data: orgData, isLoading: isOrgLoading } = useCurrentOrgQuery();

  const handleConfirmLogout = useCallback(async () => {
    setShowLogoutDialog(false);
    toast.info(t('profile_sign_out'));
    await logout();
    router.replace('/(auth)/sign-in');
  }, [logout, t, toast]);

  const fullName = userProfile?.full_name || user?.full_name || t('profile_user_default');
  const email = userProfile?.email || user?.email || 'user@orbis.edu.sa';

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <ThemedStatusBar backgroundColor={theme.background} />
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
          orgName={orgData?.name}
          isLoading={isUserLoading || isOrgLoading}
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
          onPress={() => setShowLogoutDialog(true)}
          variant="danger"
          size="lg"
          icon={<Ionicons name="log-out-outline" size={20} color="#FFFFFF" />}
          fullWidth
          style={{ marginTop: Spacing.two }}
        />
      </ScrollView>

      {/* Logout Confirmation Dialog */}
      <Dialog visible={showLogoutDialog} onClose={() => setShowLogoutDialog(false)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('profile_sign_out')}</DialogTitle>
            <DialogDescription>{t('profile_sign_out_confirm')}</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button
              title={t('common_cancel')}
              variant="outline"
              size="md"
              onPress={() => setShowLogoutDialog(false)}
            />
            <Button
              title={t('profile_sign_out')}
              variant="destructive"
              size="md"
              onPress={handleConfirmLogout}
            />
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    padding: Spacing.four,
    gap: Spacing.four,
  },
});
