/**
 * @module @orbis/mobile/app/(app)/settings/security
 *
 * Security, Password, MFA, and Active Devices Management Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView, Alert, Pressable } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

interface DeviceItem {
  id: string;
  name: string;
  type: string;
  lastActive: string;
  isCurrent: boolean;
}

export default function SecuritySettingsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { user } = useAuth();

  const [currentPass, setCurrentPass] = useState('');
  const [newPass, setNewPass] = useState('');
  const [confirmPass, setConfirmPass] = useState('');
  const [mfaEnabled, setMfaEnabled] = useState(user?.mfa_enabled ?? true);
  const [devices, setDevices] = useState<DeviceItem[]>([
    {
      id: 'dev-1',
      name: 'iPhone 15 Pro (هذا الجهاز)',
      type: 'mobile',
      lastActive: 'نشط الآن',
      isCurrent: true,
    },
    {
      id: 'dev-2',
      name: 'Orbis Desktop App (macOS)',
      type: 'desktop',
      lastActive: 'منذ ساعتين',
      isCurrent: false,
    },
    {
      id: 'dev-3',
      name: 'Chrome Web (Windows)',
      type: 'web',
      lastActive: 'أمس، 04:30 م',
      isCurrent: false,
    },
  ]);

  const handleRevokeDevice = (deviceId: string) => {
    Alert.alert('إنهاء الجلسة', 'هل أنت متأكد من رغبتك في تسجيل الخروج من هذا الجهاز؟', [
      { text: 'إلغاء', style: 'cancel' },
      {
        text: 'إنهاء الجلسة',
        style: 'destructive',
        onPress: () => {
          setDevices(devices.filter((d) => d.id !== deviceId));
          Alert.alert('تم', 'تم إنهاء الجلسة عن بعد بنجاح.');
        },
      },
    ]);
  };

  const handleUpdatePassword = () => {
    if (!currentPass || !newPass) {
      Alert.alert('تنبيه', 'يرجى إدخال كلمة المرور الحالية والجديدة.');
      return;
    }
    if (newPass !== confirmPass) {
      Alert.alert('تنبيه', 'كلمات المرور الجديدة غير متطابقة.');
      return;
    }
    Alert.alert('تم التحديث', 'تم تغيير كلمة المرور بنجاح.');
    setCurrentPass('');
    setNewPass('');
    setConfirmPass('');
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الأمان والمصادقة" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}>
        {/* Two-Factor Authentication (MFA) Card */}
        <Card variant="elevated" style={styles.card}>
          <View style={styles.cardHeaderRow}>
            <View style={[styles.iconBox, { backgroundColor: theme.primaryLight }]}>
              <Ionicons name="shield-checkmark" size={24} color={theme.primary} />
            </View>
            <View style={{ flex: 1 }}>
              <Text style={[styles.cardTitle, { color: theme.text }]}>التحقق بخطوتين (MFA)</Text>
              <Text style={[styles.cardSubtitle, { color: theme.textSecondary }]}>
                حماية إضافية عبر تطبيق Google Authenticator
              </Text>
            </View>
            <Badge
              label={mfaEnabled ? 'مفعل ومحمي' : 'غير مفعل'}
              variant={mfaEnabled ? 'success' : 'warning'}
              size="sm"
            />
          </View>

          <Button
            title={mfaEnabled ? 'إعادة ضبط مفاتيح المصادقة' : 'تفعيل التحقق بخطوتين'}
            onPress={() => {
              setMfaEnabled(!mfaEnabled);
              Alert.alert('حالة الأمان', mfaEnabled ? 'تم تعطيل التحقق بخطوتين.' : 'تم تفعيل التحقق بخطوتين.');
            }}
            variant="outline"
            size="sm"
          />
        </Card>

        {/* Change Password Card */}
        <Card variant="elevated" style={styles.card}>
          <Text style={[styles.cardTitle, { color: theme.text }]}>تغيير كلمة المرور</Text>

          <Input
            label="كلمة المرور الحالية"
            value={currentPass}
            onChangeText={setCurrentPass}
            secureTextEntry
            placeholder="••••••••"
          />

          <Input
            label="كلمة المرور الجديدة"
            value={newPass}
            onChangeText={setNewPass}
            secureTextEntry
            placeholder="••••••••"
          />

          <Input
            label="تأكيد كلمة المرور الجديدة"
            value={confirmPass}
            onChangeText={setConfirmPass}
            secureTextEntry
            placeholder="••••••••"
          />

          <Button title="تحديث كلمة المرور" onPress={handleUpdatePassword} size="md" fullWidth />
        </Card>

        {/* Active Devices Section */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>الأجهزة والجلسات النشطة</Text>
        </View>

        <View style={styles.devicesList}>
          {devices.map((dev) => (
            <Card key={dev.id} variant="flat" style={styles.devCard}>
              <View style={styles.devRow}>
                <Ionicons
                  name={dev.type === 'mobile' ? 'phone-portrait' : dev.type === 'desktop' ? 'laptop' : 'globe'}
                  size={24}
                  color={dev.isCurrent ? theme.primary : theme.textSecondary}
                />

                <View style={{ flex: 1, marginLeft: Spacing.two }}>
                  <Text style={[styles.devName, { color: theme.text }]}>{dev.name}</Text>
                  <Text style={[styles.devSub, { color: theme.textSecondary }]}>
                    {dev.lastActive}
                  </Text>
                </View>

                {dev.isCurrent ? (
                  <Badge label="الجهاز الحالي" variant="primary" size="sm" />
                ) : (
                  <Button
                    title="إنهاء"
                    onPress={() => handleRevokeDevice(dev.id)}
                    variant="ghost"
                    size="sm"
                    textStyle={{ color: theme.error }}
                  />
                )}
              </View>
            </Card>
          ))}
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
  card: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  cardHeaderRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  iconBox: {
    width: 44,
    height: 44,
    borderRadius: 22,
    alignItems: 'center',
    justifyContent: 'center',
  },
  cardTitle: {
    ...Typography.bodyBold,
  },
  cardSubtitle: {
    ...Typography.small,
    marginTop: 2,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  devicesList: {
    gap: Spacing.two,
  },
  devCard: {
    padding: Spacing.three,
  },
  devRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  devName: {
    ...Typography.bodyBold,
  },
  devSub: {
    ...Typography.small,
    marginTop: 2,
  },
});
