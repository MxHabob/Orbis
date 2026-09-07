/**
 * @module @orbis/mobile/app/(app)/settings/preferences
 *
 * App Preferences, Language, Theme, and Push Notification Toggles.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Switch, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useToast } from '@/components/ui/toast';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function PreferencesScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { locale, setLocale, t } = useI18n();

  const [themePref, setThemePref] = useState<'system' | 'dark' | 'light'>('system');
  const [notifyAbsence, setNotifyAbsence] = useState(true);
  const [notifyGrades, setNotifyGrades] = useState(true);
  const [notifyExams, setNotifyExams] = useState(true);
  const [notifyAnnouncements, setNotifyAnnouncements] = useState(false);

  const handleLanguageChange = (lang: 'ar' | 'en') => {
    setLocale(lang);
    toast.success(lang === 'ar' ? 'تم تحويل اللغة إلى العربية' : 'Language switched to English');
  };

  const handleThemeChange = (pref: 'system' | 'dark' | 'light') => {
    setThemePref(pref);
    toast.success(t('pref_theme') + ': ' + t(`pref_theme_${pref}`));
  };

  const handleToggle = (setter: React.Dispatch<React.SetStateAction<boolean>>, current: boolean, label: string) => {
    setter(!current);
    toast.info(`${label}: ${!current ? 'مفعّل' : 'معطّل'}`);
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title={t('pref_title')} showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Language Selection */}
        <View style={styles.section}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>{t('pref_language')}</Text>
          <Card variant="elevated" style={styles.card}>
            {[
              { code: 'ar' as const, name: t('pref_language_ar'), native: t('pref_language_ar_sub') },
              { code: 'en' as const, name: t('pref_language_en'), native: t('pref_language_en_sub') },
            ].map((l, idx) => {
              const isSelected = locale === l.code;
              return (
                <Pressable
                  key={l.code}
                  onPress={() => handleLanguageChange(l.code)}
                  style={[
                    styles.prefRow,
                    idx > 0 && { borderTopWidth: 1, borderTopColor: theme.borderSubtle },
                  ]}>
                  <View style={{ flex: 1 }}>
                    <Text style={[styles.prefTitle, { color: theme.text }]}>{l.name}</Text>
                    <Text style={[styles.prefSub, { color: theme.textSecondary }]}>{l.native}</Text>
                  </View>
                  {isSelected && (
                    <Ionicons name="checkmark-circle" size={22} color={theme.primary} />
                  )}
                </Pressable>
              );
            })}
          </Card>
        </View>

        {/* Theme Appearance */}
        <View style={styles.section}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>{t('pref_theme')}</Text>
          <Card variant="elevated" style={styles.card}>
            {[
              { key: 'system' as const, name: t('pref_theme_system'), icon: 'phone-portrait-outline' },
              { key: 'dark' as const, name: t('pref_theme_dark'), icon: 'moon-outline' },
              { key: 'light' as const, name: t('pref_theme_light'), icon: 'sunny-outline' },
            ].map((th, idx) => {
              const isSelected = themePref === th.key;
              return (
                <Pressable
                  key={th.key}
                  onPress={() => handleThemeChange(th.key)}
                  style={[
                    styles.prefRow,
                    idx > 0 && { borderTopWidth: 1, borderTopColor: theme.borderSubtle },
                  ]}>
                  <Ionicons
                    name={th.icon as any}
                    size={22}
                    color={isSelected ? theme.primary : theme.textSecondary}
                    style={{ marginRight: Spacing.two }}
                  />
                  <Text style={[styles.prefTitle, { color: theme.text, flex: 1 }]}>{th.name}</Text>
                  {isSelected && (
                    <Ionicons name="checkmark-circle" size={22} color={theme.primary} />
                  )}
                </Pressable>
              );
            })}
          </Card>
        </View>

        {/* Notification Preferences */}
        <View style={styles.section}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>{t('pref_notifications')}</Text>
          <Card variant="elevated" style={styles.card}>
            <View style={styles.switchRow}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>{t('pref_notify_absence')}</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  {t('pref_notify_absence_desc')}
                </Text>
              </View>
              <Switch
                value={notifyAbsence}
                onValueChange={() => handleToggle(setNotifyAbsence, notifyAbsence, t('pref_notify_absence'))}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>{t('pref_notify_grades')}</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  {t('pref_notify_grades_desc')}
                </Text>
              </View>
              <Switch
                value={notifyGrades}
                onValueChange={() => handleToggle(setNotifyGrades, notifyGrades, t('pref_notify_grades'))}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>{t('pref_notify_exams')}</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  {t('pref_notify_exams_desc')}
                </Text>
              </View>
              <Switch
                value={notifyExams}
                onValueChange={() => handleToggle(setNotifyExams, notifyExams, t('pref_notify_exams'))}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>{t('pref_notify_announcements')}</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  {t('pref_notify_announcements_desc')}
                </Text>
              </View>
              <Switch
                value={notifyAnnouncements}
                onValueChange={() => handleToggle(setNotifyAnnouncements, notifyAnnouncements, t('pref_notify_announcements'))}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>
          </Card>
        </View>
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
  section: {
    gap: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  card: {
    padding: 0,
    overflow: 'hidden',
  },
  prefRow: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: Spacing.three + 2,
    paddingHorizontal: Spacing.four,
  },
  prefTitle: {
    ...Typography.bodyBold,
  },
  prefSub: {
    ...Typography.small,
    marginTop: 2,
  },
  switchRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingVertical: Spacing.three + 2,
    paddingHorizontal: Spacing.four,
  },
});
