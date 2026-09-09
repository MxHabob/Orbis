/**
 * @module @orbis/mobile/app/(app)/settings/security
 *
 * Security, Password, MFA, and Active Devices Management Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { useToast } from '@/components/ui/toast';
import { Spacing, Typography } from '@/constants/theme';

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
  const toast = useToast();

  const [currentPass, setCurrentPass] = useState('');
  const [newPass, setNewPass] = useState('');
  const [confirmPass, setConfirmPass] = useState('');
  const [mfaEnabled, setMfaEnabled] = useState(user?.mfa_enabled ?? true);
  const [dialogVisible, setDialogVisible] = useState(false);
  const [dialogMode, setDialogMode] = useState<'message' | 'confirm'>('message');
  const [dialogTitle, setDialogTitle] = useState('');
  const [dialogDescription, setDialogDescription] = useState('');
  const [pendingDeviceId, setPendingDeviceId] = useState<string | null>(null);
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

  const openDialog = (title: string, description: string, mode: 'message' | 'confirm' = 'message') => {
    setDialogTitle(title);
    setDialogDescription(description);
    setDialogMode(mode);
    setDialogVisible(true);
  };

  const handleRevokeDevice = (deviceId: string) => {
    setPendingDeviceId(deviceId);
    openDialog('إنهاء الجلسة', 'هل أنت متأكد من رغبتك في تسجيل الخروج من هذا الجهاز؟', 'confirm');
  };

  const confirmRevokeDevice = () => {
    if (pendingDeviceId) {
      setDevices((currentDevices) => currentDevices.filter((d) => d.id !== pendingDeviceId));
      toast.success('تم إنهاء الجلسة عن بعد بنجاح.');
    }

    setDialogVisible(false);
    setPendingDeviceId(null);
  };

  const handleUpdatePassword = () => {
    if (!currentPass || !newPass) {
      openDialog('تنبيه', 'يرجى إدخال كلمة المرور الحالية والجديدة.');
      return;
    }
    if (newPass !== confirmPass) {
      openDialog('تنبيه', 'كلمات المرور الجديدة غير متطابقة.');
      return;
    }

    openDialog('تم التحديث', 'تم تغيير كلمة المرور بنجاح.');
    toast.success('تم تغيير كلمة المرور بنجاح.');
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
              const nextMfa = !mfaEnabled;
              setMfaEnabled(nextMfa);
              openDialog(
                'حالة الأمان',
                nextMfa ? 'تم تفعيل التحقق بخطوتين.' : 'تم تعطيل التحقق بخطوتين.',
              );
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

      <Dialog visible={dialogVisible} onClose={() => setDialogVisible(false)}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{dialogTitle}</DialogTitle>
          </DialogHeader>
          <DialogDescription>{dialogDescription}</DialogDescription>
          <DialogFooter>
            {dialogMode === 'confirm' ? (
              <>
                <DialogClose>
                  <Button title="إلغاء" onPress={() => {}} variant="outline" size="sm" />
                </DialogClose>
                <Button title="إنهاء الجلسة" onPress={confirmRevokeDevice} variant="destructive" size="sm" />
              </>
            ) : (
              <DialogClose>
                <Button title="تم" onPress={() => {}} variant="default" size="sm" />
              </DialogClose>
            )}
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
