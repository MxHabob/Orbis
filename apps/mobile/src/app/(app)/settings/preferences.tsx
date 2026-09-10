/**
 * @module @orbis/mobile/app/(app)/settings/preferences
 *
 * App Preferences, Language, Theme, and Push Notification Settings.
 */

import React, { useState } from 'react';
import {
  Appearance,
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { useToast } from '@/components/ui/toast';
import { Header } from '@/components/header';
import { Switch } from '@/components/ui/switch';
import {
  Spacing,
  Typography,
} from '@/constants/theme';

export default function PreferencesScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();
  const { locale, setLocale, t } = useI18n();

  const [themePref, setThemePref] =
    useState<'system' | 'dark' | 'light'>(
      'system',
    );

  const [notifyAbsence, setNotifyAbsence] =
    useState(true);

  const [notifyGrades, setNotifyGrades] =
    useState(true);

  const [notifyExams, setNotifyExams] =
    useState(true);

  const [
    notifyAnnouncements,
    setNotifyAnnouncements,
  ] = useState(false);

  const handleLanguageChange = (
    lang: 'ar' | 'en',
  ) => {
    setLocale(lang);

    toast.success(
      lang === 'ar'
        ? 'تم تحويل اللغة إلى العربية'
        : 'Language switched to English',
    );
  };

  const handleThemeChange = (
    pref: 'system' | 'dark' | 'light',
  ) => {
    setThemePref(pref);

    Appearance.setColorScheme(
      pref === 'system'
        ? 'unspecified'
        : pref,
    );

    toast.success(
      `${t('pref_theme')}: ${t(
        `pref_theme_${pref}`,
      )}`,
    );
  };

  const handleToggle = (
    setter: React.Dispatch<
      React.SetStateAction<boolean>
    >,
    current: boolean,
    label: string,
  ) => {
    const next = !current;

    setter(next);

    toast.info(
      `${label}: ${
        next ? 'مفعّل' : 'معطّل'
      }`,
    );
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
        contentContainerStyle={[
          styles.content,
          {
            paddingBottom:
              insets.bottom + 120,
          },
        ]}
      >
        <Header
          title={t('pref_title')}
          showBack
          showCrown={false}
        />

        <View style={styles.main}>
          {/* LANGUAGE */}

          <SectionLabel
            title={t('pref_language')}
            theme={theme}
          />

          <View
            style={[
              styles.list,
              {
                borderTopColor: theme.border,
                borderBottomColor: theme.border,
              },
            ]}
          >
            <SelectionRow
              title={t('pref_language_ar')}
              subtitle={t(
                'pref_language_ar_sub',
              )}
              selected={locale === 'ar'}
              onPress={() =>
                handleLanguageChange('ar')
              }
              theme={theme}
            />

            <SelectionRow
              title={t('pref_language_en')}
              subtitle={t(
                'pref_language_en_sub',
              )}
              selected={locale === 'en'}
              onPress={() =>
                handleLanguageChange('en')
              }
              theme={theme}
              divider
            />
          </View>

          {/* APPEARANCE */}

          <SectionLabel
            title={t('pref_theme')}
            theme={theme}
            style={styles.sectionSpacing}
          />

          <View
            style={[
              styles.list,
              {
                borderTopColor: theme.border,
                borderBottomColor: theme.border,
              },
            ]}
          >
            <ThemeRow
              icon="phone-portrait-outline"
              title={t(
                'pref_theme_system',
              )}
              selected={
                themePref === 'system'
              }
              onPress={() =>
                handleThemeChange('system')
              }
              theme={theme}
            />

            <ThemeRow
              icon="sunny-outline"
              title={t(
                'pref_theme_light',
              )}
              selected={
                themePref === 'light'
              }
              onPress={() =>
                handleThemeChange('light')
              }
              theme={theme}
              divider
            />

            <ThemeRow
              icon="moon-outline"
              title={t(
                'pref_theme_dark',
              )}
              selected={
                themePref === 'dark'
              }
              onPress={() =>
                handleThemeChange('dark')
              }
              theme={theme}
              divider
            />
          </View>

          {/* NOTIFICATIONS */}

          <SectionLabel
            title={t('pref_notifications')}
            theme={theme}
            style={styles.sectionSpacing}
          />

          <View
            style={[
              styles.list,
              {
                borderTopColor: theme.border,
                borderBottomColor: theme.border,
              },
            ]}
          >
            <NotificationRow
              title={t(
                'pref_notify_absence',
              )}
              subtitle={t(
                'pref_notify_absence_desc',
              )}
              value={notifyAbsence}
              onValueChange={() =>
                handleToggle(
                  setNotifyAbsence,
                  notifyAbsence,
                  t(
                    'pref_notify_absence',
                  ),
                )
              }
              theme={theme}
            />

            <NotificationRow
              title={t(
                'pref_notify_grades',
              )}
              subtitle={t(
                'pref_notify_grades_desc',
              )}
              value={notifyGrades}
              onValueChange={() =>
                handleToggle(
                  setNotifyGrades,
                  notifyGrades,
                  t(
                    'pref_notify_grades',
                  ),
                )
              }
              theme={theme}
              divider
            />

            <NotificationRow
              title={t(
                'pref_notify_exams',
              )}
              subtitle={t(
                'pref_notify_exams_desc',
              )}
              value={notifyExams}
              onValueChange={() =>
                handleToggle(
                  setNotifyExams,
                  notifyExams,
                  t(
                    'pref_notify_exams',
                  ),
                )
              }
              theme={theme}
              divider
            />

            <NotificationRow
              title={t(
                'pref_notify_announcements',
              )}
              subtitle={t(
                'pref_notify_announcements_desc',
              )}
              value={notifyAnnouncements}
              onValueChange={() =>
                handleToggle(
                  setNotifyAnnouncements,
                  notifyAnnouncements,
                  t(
                    'pref_notify_announcements',
                  ),
                )
              }
              theme={theme}
              divider
            />
          </View>
        </View>
      </ScrollView>
    </View>
  );
}

/* -------------------------------------------------------------------------- */
/* Section Label                                                              */
/* -------------------------------------------------------------------------- */

function SectionLabel({
  title,
  theme,
  style,
}: {
  title: string;
  theme: ReturnType<typeof useTheme>;
  style?: any;
}) {
  return (
    <Text
      style={[
        styles.sectionLabel,
        {
          color: theme.textMuted,
        },
        style,
      ]}
    >
      {title}
    </Text>
  );
}

/* -------------------------------------------------------------------------- */
/* Selection Row                                                              */
/* -------------------------------------------------------------------------- */

function SelectionRow({
  title,
  subtitle,
  selected,
  onPress,
  theme,
  divider = false,
}: {
  title: string;
  subtitle: string;
  selected: boolean;
  onPress: () => void;
  theme: ReturnType<typeof useTheme>;
  divider?: boolean;
}) {
  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.selectionRow,
        divider && {
          borderTopWidth:
            StyleSheet.hairlineWidth,
          borderTopColor: theme.border,
        },
        {
          opacity: pressed ? 0.55 : 1,
        },
      ]}
    >
      <View style={styles.rowText}>
        <Text
          style={[
            styles.rowTitle,
            {
              color: theme.text,
            },
          ]}
        >
          {title}
        </Text>

        <Text
          style={[
            styles.rowSubtitle,
            {
              color: theme.textMuted,
            },
          ]}
        >
          {subtitle}
        </Text>
      </View>

      {selected && (
        <Ionicons
          name="checkmark"
          size={20}
          color={theme.primary}
        />
      )}
    </Pressable>
  );
}

/* -------------------------------------------------------------------------- */
/* Theme Row                                                                  */
/* -------------------------------------------------------------------------- */

function ThemeRow({
  icon,
  title,
  selected,
  onPress,
  theme,
  divider = false,
}: {
  icon: keyof typeof Ionicons.glyphMap;
  title: string;
  selected: boolean;
  onPress: () => void;
  theme: ReturnType<typeof useTheme>;
  divider?: boolean;
}) {
  return (
    <Pressable
      onPress={onPress}
      style={({ pressed }) => [
        styles.themeRow,
        divider && {
          borderTopWidth:
            StyleSheet.hairlineWidth,
          borderTopColor: theme.border,
        },
        {
          opacity: pressed ? 0.55 : 1,
        },
      ]}
    >
      <Ionicons
        name={icon}
        size={19}
        color={
          selected
            ? theme.primary
            : theme.textSecondary
        }
      />

      <Text
        style={[
          styles.rowTitle,
          {
            color: theme.text,
          },
        ]}
      >
        {title}
      </Text>

      {selected && (
        <Ionicons
          name="checkmark"
          size={20}
          color={theme.primary}
        />
      )}
    </Pressable>
  );
}

/* -------------------------------------------------------------------------- */
/* Notification Row                                                           */
/* -------------------------------------------------------------------------- */

function NotificationRow({
  title,
  subtitle,
  value,
  onValueChange,
  theme,
  divider = false,
}: {
  title: string;
  subtitle: string;
  value: boolean;
  onValueChange: () => void;
  theme: ReturnType<typeof useTheme>;
  divider?: boolean;
}) {
  return (
    <View
      style={[
        styles.notificationRow,
        divider && {
          borderTopWidth:
            StyleSheet.hairlineWidth,
          borderTopColor: theme.border,
        },
      ]}
    >
      <View style={styles.rowText}>
        <Text
          style={[
            styles.rowTitle,
            {
              color: theme.text,
            },
          ]}
        >
          {title}
        </Text>

        <Text
          style={[
            styles.rowSubtitle,
            {
              color: theme.textMuted,
            },
          ]}
        >
          {subtitle}
        </Text>
      </View>

      <Switch
        value={value}
        onValueChange={onValueChange}
        accessibilityLabel={title}
      />
    </View>
  );
}

/* -------------------------------------------------------------------------- */
/* Styles                                                                     */
/* -------------------------------------------------------------------------- */

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },

  content: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.two,
  },

  main: {
    width: '100%',
    maxWidth: 520,
    alignSelf: 'center',
    marginTop: Spacing.six,
  },

  sectionLabel: {
    ...Typography.small,
    fontSize: 10,
    letterSpacing: 0.5,
    marginBottom: Spacing.two,
  },

  sectionSpacing: {
    marginTop: Spacing.six,
  },

  list: {
    borderTopWidth:
      StyleSheet.hairlineWidth,
    borderBottomWidth:
      StyleSheet.hairlineWidth,
  },

  selectionRow: {
    minHeight: 66,

    flexDirection: 'row',
    alignItems: 'center',

    paddingVertical: Spacing.three,
  },

  themeRow: {
    minHeight: 56,

    flexDirection: 'row',
    alignItems: 'center',

    gap: Spacing.three,
  },

  notificationRow: {
    minHeight: 76,

    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',

    gap: Spacing.four,
  },

  rowText: {
    flex: 1,
    minWidth: 0,
  },

  rowTitle: {
    ...Typography.bodyBold,
    fontSize: 14,
  },

  rowSubtitle: {
    ...Typography.small,
    lineHeight: 17,
    marginTop: 3,
  },
});