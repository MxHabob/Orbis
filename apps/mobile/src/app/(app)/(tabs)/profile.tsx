/**
 * @module @orbis/mobile/app/(app)/(tabs)/profile
 *
 * Unified Profile and Settings Hub.
 * Built with @orbis/sdk & TanStack Query:
 * - Zero useEffect data fetching
 * - Skeletons for user profile details
 * - High performance & native speed
 */

import React, { useCallback } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Pressable,
  Alert,
} from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { StatusBar } from 'expo-status-bar';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { useI18n, l, t } from '@/lib/i18n';
import { useEffectiveContext } from '@/providers/effective-context';
import { useUserQuery } from '@/hooks/use-api-queries';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Skeleton } from '@/components/ui/skeleton';
import { Radii, Spacing, Typography } from '@/constants/theme';

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
      <StatusBar style="auto" />
      {/* Profile Header */}
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
        {/* User Identity Card */}
        <Card variant="elevated" style={styles.userCard}>
          {isUserLoading ? (
            <View style={styles.userRow}>
              <Skeleton width={68} height={68} borderRadius={34} />
              <View style={styles.userInfo}>
                <Skeleton width={140} height={20} />
                <Skeleton width={180} height={14} style={{ marginTop: 6 }} />
                <Skeleton width={80} height={18} style={{ marginTop: 8 }} />
              </View>
            </View>
          ) : (
            <View style={styles.userRow}>
              <View style={[styles.avatarCircle, { backgroundColor: theme.primaryLight }]}>
                <Ionicons name="person" size={36} color={theme.primary} />
              </View>

              <View style={styles.userInfo}>
                <Text style={[styles.userName, { color: theme.text }]}>{fullName}</Text>
                <Text style={[styles.userEmail, { color: theme.textSecondary }]}>{email}</Text>

                <View style={styles.badgeRow}>
                  <Badge
                    label={
                      role === 'student'
                        ? t('profile_role_student')
                        : role === 'parent'
                          ? t('profile_role_parent')
                          : role === 'teacher'
                            ? t('profile_role_teacher')
                            : t('profile_role_admin')
                    }
                    variant="primary"
                    size="sm"
                  />
                  <Text style={[styles.orgName, { color: theme.textMuted }]}>
                    {t('nav_campus_default')}
                  </Text>
                </View>
              </View>
            </View>
          )}
        </Card>

        {/* Linked Children Card (for Parent) */}
        {isParent && childrenList.length > 0 && (
          <View style={styles.section}>
            <View style={styles.sectionHeader}>
              <Text style={[styles.sectionTitle, { color: theme.text }]}>
                {t('profile_linked_children')}
              </Text>
            </View>

            <View style={styles.childrenGrid}>
              {childrenList.map((child) => {
                const isSelected = child.id === selectedChildId;
                return (
                  <Card
                    key={child.id}
                    onPress={() => setSelectedChildId(child.id)}
                    variant={isSelected ? 'elevated' : 'flat'}
                    style={[
                      styles.childItemCard,
                      isSelected && { borderColor: theme.primary, borderWidth: 1.5 },
                    ]}>
                    <View style={styles.childItemRow}>
                      <View
                        style={[
                          styles.childAvatar,
                          {
                            backgroundColor: isSelected
                              ? theme.primaryLight
                              : theme.backgroundElement,
                          },
                        ]}>
                        <Ionicons
                          name="person"
                          size={22}
                          color={isSelected ? theme.primary : theme.textSecondary}
                        />
                      </View>

                      <View style={styles.childInfo}>
                        <Text style={[styles.childName, { color: theme.text }]}>{child.name}</Text>
                        <Text style={[styles.childClass, { color: theme.textSecondary }]}>
                          {child.className || child.gradeLevel}
                        </Text>
                      </View>

                      {isSelected && (
                        <Ionicons name="checkmark-circle" size={22} color={theme.primary} />
                      )}
                    </View>
                  </Card>
                );
              })}
            </View>
          </View>
        )}

        {/* Quick Settings Navigation Menu */}
        <View style={styles.section}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>
            {t('profile_settings_title')}
          </Text>
          <Card variant="flat" style={styles.menuCard}>
            {[
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
            ].map((item, idx, arr) => (
              <Pressable
                key={item.id}
                onPress={() => router.push(item.route as any)}
                style={({ pressed }) => [
                  styles.menuItem,
                  idx < arr.length - 1 && {
                    borderBottomWidth: 1,
                    borderBottomColor: theme.borderSubtle,
                  },
                  pressed && { backgroundColor: theme.backgroundElement },
                ]}>
                <View style={styles.menuItemLeft}>
                  <View
                    style={[styles.menuIconBox, { backgroundColor: theme.backgroundElement }]}>
                    <Ionicons name={item.icon as any} size={20} color={theme.primary} />
                  </View>
                  <Text style={[styles.menuItemTitle, { color: theme.text }]}>{item.title}</Text>
                </View>

                <Ionicons name="chevron-forward" size={18} color={theme.textSecondary} />
              </Pressable>
            ))}
          </Card>
        </View>

        {/* Logout Button */}
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
  userCard: {
    padding: Spacing.four,
  },
  userRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.four,
  },
  avatarCircle: {
    width: 68,
    height: 68,
    borderRadius: 34,
    alignItems: 'center',
    justifyContent: 'center',
  },
  userInfo: {
    flex: 1,
    gap: 3,
  },
  userName: {
    ...Typography.bodyBold,
    fontSize: 18,
  },
  userEmail: {
    ...Typography.small,
  },
  badgeRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    marginTop: 4,
  },
  orgName: {
    ...Typography.small,
    fontSize: 11,
  },
  section: {
    gap: Spacing.two,
  },
  sectionHeader: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  sectionTitle: {
    ...Typography.bodyBold,
    fontSize: 16,
  },
  childrenGrid: {
    gap: Spacing.two,
  },
  childItemCard: {
    padding: Spacing.three,
    borderRadius: Radii.lg,
  },
  childItemRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
  },
  childAvatar: {
    width: 44,
    height: 44,
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
  },
  childInfo: {
    flex: 1,
    gap: 2,
  },
  childName: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
  childClass: {
    ...Typography.small,
  },
  menuCard: {
    overflow: 'hidden',
    padding: 0,
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingVertical: Spacing.three,
    paddingHorizontal: Spacing.four,
  },
  menuItemLeft: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.three,
  },
  menuIconBox: {
    width: 36,
    height: 36,
    borderRadius: 18,
    alignItems: 'center',
    justifyContent: 'center',
  },
  menuItemTitle: {
    ...Typography.bodyBold,
    fontSize: 14,
  },
});
