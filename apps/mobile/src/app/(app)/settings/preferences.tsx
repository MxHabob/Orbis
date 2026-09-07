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
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function PreferencesScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const [lang, setLang] = useState<'ar' | 'en'>('ar');
  const [themePref, setThemePref] = useState<'system' | 'dark' | 'light'>('system');
  const [notifyAbsence, setNotifyAbsence] = useState(true);
  const [notifyGrades, setNotifyGrades] = useState(true);
  const [notifyExams, setNotifyExams] = useState(true);
  const [notifyAnnouncements, setNotifyAnnouncements] = useState(false);

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="التفضيلات والإشعارات" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Language Selection */}
        <View style={styles.section}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>لغة التطبيق (Language)</Text>
          <Card variant="elevated" style={styles.card}>
            {[
              { code: 'ar', name: 'العربية (Arabic)', native: 'اللغة الأساسية' },
              { code: 'en', name: 'English', native: 'English Interface' },
            ].map((l, idx) => {
              const isSelected = lang === l.code;
              return (
                <Pressable
                  key={l.code}
                  onPress={() => setLang(l.code as any)}
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
          <Text style={[styles.sectionTitle, { color: theme.text }]}>مظهر التطبيق</Text>
          <Card variant="elevated" style={styles.card}>
            {[
              { key: 'system', name: 'تلقائي (حسب إعدادات النظام)', icon: 'phone-portrait-outline' },
              { key: 'dark', name: 'الوضع الداكن (Dark Mode)', icon: 'moon-outline' },
              { key: 'light', name: 'الوضع الفاتح (Light Mode)', icon: 'sunny-outline' },
            ].map((t, idx) => {
              const isSelected = themePref === t.key;
              return (
                <Pressable
                  key={t.key}
                  onPress={() => setThemePref(t.key as any)}
                  style={[
                    styles.prefRow,
                    idx > 0 && { borderTopWidth: 1, borderTopColor: theme.borderSubtle },
                  ]}>
                  <Ionicons
                    name={t.icon as any}
                    size={22}
                    color={isSelected ? theme.primary : theme.textSecondary}
                    style={{ marginRight: Spacing.two }}
                  />
                  <Text style={[styles.prefTitle, { color: theme.text, flex: 1 }]}>{t.name}</Text>
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
          <Text style={[styles.sectionTitle, { color: theme.text }]}>إشعارات وتنبيهات النظام</Text>
          <Card variant="elevated" style={styles.card}>
            <View style={styles.switchRow}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>تنبيهات الغياب والتأخر</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  إشعار فوري عند رصد غياب أو تأخر
                </Text>
              </View>
              <Switch
                value={notifyAbsence}
                onValueChange={setNotifyAbsence}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>إعلان الدرجات والتقييمات</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  إشعار عند رصد أو نشر نتائج اختبار
                </Text>
              </View>
              <Switch
                value={notifyGrades}
                onValueChange={setNotifyGrades}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>تذكيرات مواعيد الاختبارات</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  تذكير قبل موعد الاختبار بـ 24 ساعة
                </Text>
              </View>
              <Switch
                value={notifyExams}
                onValueChange={setNotifyExams}
                trackColor={{ false: theme.border, true: theme.primary }}
              />
            </View>

            <View style={[styles.switchRow, { borderTopWidth: 1, borderTopColor: theme.borderSubtle }]}>
              <View style={{ flex: 1 }}>
                <Text style={[styles.prefTitle, { color: theme.text }]}>التعاميم والأخبار العامة</Text>
                <Text style={[styles.prefSub, { color: theme.textSecondary }]}>
                  إشعارات الفعاليات والمناسبات المدرسية
                </Text>
              </View>
              <Switch
                value={notifyAnnouncements}
                onValueChange={setNotifyAnnouncements}
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
