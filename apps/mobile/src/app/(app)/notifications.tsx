/**
 * @module @orbis/mobile/app/(app)/notifications
 *
 * Notifications & Academic Alerts Modal Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { router } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface NotificationItem {
  id: string;
  title: string;
  body: string;
  time: string;
  type: 'grade' | 'attendance' | 'exam' | 'announcement';
  read: boolean;
}

export default function NotificationsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();

  const [notifications, setNotifications] = useState<NotificationItem[]>([
    {
      id: 'notif-1',
      title: 'رصد درجات الاختبار النصفي',
      body: 'تم رصد واعتماد نتيجة اختبار مادة الرياضيات المتقدمة (29/30).',
      time: 'منذ ساعتين',
      type: 'grade',
      read: false,
    },
    {
      id: 'notif-2',
      title: 'إشعار انضباط وحضور',
      body: 'تم تسجيل حضور الطالب لجميع الحصص الدراسية اليوم بنجاح.',
      time: 'منذ 5 ساعات',
      type: 'attendance',
      read: false,
    },
    {
      id: 'notif-3',
      title: 'تحديد موعد الاختبار النهائي',
      body: 'تم اعتماد جدول الاختبارات النهائية للفصل الدراسي الثاني وتعيين رقم الجلوس (418).',
      time: 'أمس، 02:00 م',
      type: 'exam',
      read: true,
    },
    {
      id: 'notif-4',
      title: 'تعميم: المعرض العلمي السنوي',
      body: 'ندعوكم لحضور المعرض العلمي والابتكاري يوم الخميس القادم في الصالة الرئيسية.',
      time: 'قبل 3 أيام',
      type: 'announcement',
      read: true,
    },
  ]);

  const markAllAsRead = () => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  };

  const getTypeIcon = (type: NotificationItem['type']) => {
    switch (type) {
      case 'grade':
        return { icon: 'podium', color: theme.primary, bg: theme.primaryLight };
      case 'attendance':
        return { icon: 'checkmark-circle', color: theme.success, bg: theme.successLight };
      case 'exam':
        return { icon: 'document-text', color: theme.warning, bg: theme.warningLight };
      case 'announcement':
      default:
        return { icon: 'megaphone', color: theme.accent, bg: theme.accentLight };
    }
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="التنبيهات والإشعارات"
        showBack
        rightAction={
          <Pressable onPress={markAllAsRead} hitSlop={8}>
            <Text style={[styles.markReadText, { color: theme.primary }]}>قراءة الكل</Text>
          </Pressable>
        }
      />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        <View style={styles.notifList}>
          {notifications.map((item) => {
            const { icon, color, bg } = getTypeIcon(item.type);
            return (
              <Card
                key={item.id}
                variant={item.read ? 'flat' : 'elevated'}
                style={[
                  styles.notifCard,
                  !item.read && { borderColor: theme.primary, borderWidth: 1.2 },
                ]}>
                <View style={styles.notifRow}>
                  <View style={[styles.iconBox, { backgroundColor: bg }]}>
                    <Ionicons name={icon as any} size={22} color={color} />
                  </View>

                  <View style={{ flex: 1 }}>
                    <View style={styles.titleRow}>
                      <Text style={[styles.notifTitle, { color: theme.text }]}>{item.title}</Text>
                      {!item.read && <View style={[styles.dot, { backgroundColor: theme.primary }]} />}
                    </View>
                    <Text style={[styles.notifBody, { color: theme.textSecondary }]}>
                      {item.body}
                    </Text>
                    <Text style={[styles.notifTime, { color: theme.textMuted }]}>{item.time}</Text>
                  </View>
                </View>
              </Card>
            );
          })}
        </View>
      </ScrollView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  markReadText: {
    ...Typography.captionBold,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.four,
    gap: Spacing.three,
  },
  notifList: {
    gap: Spacing.three,
  },
  notifCard: {
    padding: Spacing.four,
  },
  notifRow: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    gap: Spacing.three,
  },
  iconBox: {
    width: 44,
    height: 44,
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
  },
  titleRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  notifTitle: {
    ...Typography.bodyBold,
  },
  dot: {
    width: 8,
    height: 8,
    borderRadius: 4,
  },
  notifBody: {
    ...Typography.body,
    marginTop: 4,
    lineHeight: 20,
  },
  notifTime: {
    ...Typography.small,
    marginTop: 6,
  },
});
