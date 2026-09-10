/**
 * @module @orbis/mobile/app/(app)/settings/security
 *
 * Security, Password, MFA, and Active Devices Management Screen.
 */

import React, { useState } from 'react';
import {
  ScrollView,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { useToast } from '@/components/ui/toast';
import {
  Radii,
  Spacing,
  Typography,
} from '@/constants/theme';

interface DeviceItem {
  id: string;
  name: string;
  type: 'mobile' | 'desktop' | 'web';
  lastActive: string;
  isCurrent: boolean;
}

export default function SecuritySettingsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { user } = useAuth();
  const toast = useToast();

  const [currentPass, setCurrentPass] =
    useState('');

  const [newPass, setNewPass] =
    useState('');

  const [confirmPass, setConfirmPass] =
    useState('');

  const [mfaEnabled, setMfaEnabled] =
    useState(user?.mfa_enabled ?? true);

  const [googleLinked, setGoogleLinked] =
    useState(true);

  const [dialogVisible, setDialogVisible] =
    useState(false);

  const [pendingAction, setPendingAction] =
    useState<'google' | 'device' | null>(null);

  const [dialogMode, setDialogMode] =
    useState<'message' | 'confirm'>(
      'message',
    );

  const [dialogTitle, setDialogTitle] =
    useState('');

  const [dialogDescription, setDialogDescription] =
    useState('');

  const [pendingDeviceId, setPendingDeviceId] =
    useState<string | null>(null);

  const [devices, setDevices] =
    useState<DeviceItem[]>([
      {
        id: 'dev-1',
        name: 'iPhone 15 Pro',
        type: 'mobile',
        lastActive: 'نشط الآن',
        isCurrent: true,
      },
      {
        id: 'dev-2',
        name: 'Orbis Desktop App',
        type: 'desktop',
        lastActive: 'منذ ساعتين',
        isCurrent: false,
      },
      {
        id: 'dev-3',
        name: 'Chrome Web',
        type: 'web',
        lastActive: 'أمس، 04:30 م',
        isCurrent: false,
      },
    ]);

  const openDialog = (
    title: string,
    description: string,
    mode: 'message' | 'confirm' = 'message',
  ) => {
    setDialogTitle(title);
    setDialogDescription(description);
    setDialogMode(mode);
    setDialogVisible(true);
  };

  const handleRevokeDevice = (
    deviceId: string,
  ) => {
    setPendingDeviceId(deviceId);
    setPendingAction('device');

    openDialog(
      'إنهاء الجلسة',
      'هل أنت متأكد من رغبتك في تسجيل الخروج من هذا الجهاز؟',
      'confirm',
    );
  };

  const confirmRevokeDevice = () => {
    if (pendingDeviceId) {
      setDevices((current) =>
        current.filter(
          (device) =>
            device.id !== pendingDeviceId,
        ),
      );

      toast.success(
        'تم إنهاء الجلسة عن بعد بنجاح.',
      );
    }

    setDialogVisible(false);
    setPendingAction(null);
    setPendingDeviceId(null);
  };

  const handleGooglePress = () => {
    if (googleLinked) {
      setPendingAction('google');

      openDialog(
        'فصل حساب Google',
        'هل أنت متأكد من إلغاء ربط حساب Google؟',
        'confirm',
      );

      return;
    }

    setGoogleLinked(true);

    toast.success(
      'تم ربط حساب Google بنجاح.',
    );
  };

  const confirmGoogleDisconnect = () => {
    setGoogleLinked(false);
    setDialogVisible(false);
    setPendingAction(null);

    toast.success(
      'تم إلغاء ربط حساب Google.',
    );
  };

  const handleUpdatePassword = () => {
    if (!currentPass || !newPass) {
      openDialog(
        'تنبيه',
        'يرجى إدخال كلمة المرور الحالية والجديدة.',
      );

      return;
    }

    if (newPass !== confirmPass) {
      openDialog(
        'تنبيه',
        'كلمات المرور الجديدة غير متطابقة.',
      );

      return;
    }

    openDialog(
      'تم التحديث',
      'تم تغيير كلمة المرور بنجاح.',
    );

    toast.success(
      'تم تغيير كلمة المرور بنجاح.',
    );

    setCurrentPass('');
    setNewPass('');
    setConfirmPass('');
  };

  const handleMfaChange = (
    value: boolean,
  ) => {
    setMfaEnabled(value);

    openDialog(
      'حالة الأمان',
      value
        ? 'تم تفعيل التحقق بخطوتين.'
        : 'تم تعطيل التحقق بخطوتين.',
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
        keyboardShouldPersistTaps="handled"
        contentContainerStyle={[
          styles.content,
          {
            paddingBottom:
              insets.bottom + 120,
          },
        ]}
      >
        <Header
          title="Security"
          showBack
          showCrown={false}
        />

        <View style={styles.main}>
          {/* ============================================================
              MFA
              ============================================================ */}

          <SectionLabel
            title="التحقق بخطوتين"
            theme={theme}
          />

          <View
            style={[
              styles.securityRow,
              {
                borderTopColor: theme.border,
                borderBottomColor: theme.border,
              },
            ]}
          >
            <View
              style={[
                styles.securityIcon,
                {
                  backgroundColor:
                    theme.backgroundElement,
                },
              ]}
            >
              <Ionicons
                name="shield-checkmark-outline"
                size={20}
                color={theme.text}
              />
            </View>

            <View style={styles.securityInfo}>
              <Text
                style={[
                  styles.rowTitle,
                  {
                    color: theme.text,
                  },
                ]}
              >
                المصادقة متعددة العوامل
              </Text>

              <Text
                style={[
                  styles.rowSubtitle,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                حماية إضافية لحسابك
              </Text>
            </View>

            <Switch
              value={mfaEnabled}
              onValueChange={handleMfaChange}
              accessibilityLabel="التحقق بخطوتين"
            />
          </View>

          {/* ============================================================
              PASSWORD
              ============================================================ */}

          <SectionLabel
            title="الحسابات المرتبطة"
            theme={theme}
            style={styles.sectionSpacing}
          />

          <View
            style={[
              styles.providerRow,
              {
                borderBottomColor: theme.border,
              },
            ]}
          >
            <View
              style={[
                styles.googleIcon,
                {
                  backgroundColor:
                    theme.backgroundElement,
                },
              ]}
            >
              <Ionicons
                name="logo-google"
                size={22}
                color={theme.text}
              />
            </View>

            <View style={styles.providerInfo}>
              <Text
                style={[
                  styles.providerTitle,
                  {
                    color: theme.text,
                  },
                ]}
              >
                Google
              </Text>

              <Text
                numberOfLines={1}
                style={[
                  styles.providerEmail,
                  {
                    color: theme.textMuted,
                  },
                ]}
              >
                {googleLinked
                  ? 'user@gmail.com'
                  : 'غير متصل'}
              </Text>
            </View>

            <View style={styles.status}>
              <View
                style={[
                  styles.statusDot,
                  {
                    backgroundColor:
                      googleLinked
                        ? theme.primary
                        : theme.border,
                  },
                ]}
              />

              <Text
                style={[
                  styles.statusText,
                  {
                    color: googleLinked
                      ? theme.textSecondary
                      : theme.textMuted,
                  },
                ]}
              >
                {googleLinked
                  ? 'متصل'
                  : 'غير متصل'}
              </Text>
            </View>
          </View>

          <View style={styles.actionArea}>
            <Button
              title={
                googleLinked
                  ? 'فصل حساب Google'
                  : 'ربط حساب Google'
              }
              onPress={handleGooglePress}
              variant={
                googleLinked
                  ? 'secondary'
                  : 'primary'
              }
              size="md"
              fullWidth
            />
          </View>

          <SectionLabel
            title="كلمة المرور"
            theme={theme}
            style={styles.sectionSpacing}
          />

          <View
            style={[
              styles.passwordSection,
              {
                borderTopColor: theme.border,
              },
            ]}
          >
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
              label="تأكيد كلمة المرور"
              value={confirmPass}
              onChangeText={setConfirmPass}
              secureTextEntry
              placeholder="••••••••"
            />

            <Button
              title="تحديث كلمة المرور"
              onPress={handleUpdatePassword}
              size="md"
              fullWidth
            />
          </View>

          {/* ============================================================
              DEVICES
              ============================================================ */}

          <SectionLabel
            title="الجلسات النشطة"
            theme={theme}
            style={styles.sectionSpacing}
          />

          <View
            style={[
              styles.devicesList,
              {
                borderTopColor: theme.border,
                borderBottomColor: theme.border,
              },
            ]}
          >
            {devices.map((device, index) => (
              <DeviceRow
                key={device.id}
                device={device}
                theme={theme}
                divider={index > 0}
                onRevoke={() =>
                  handleRevokeDevice(
                    device.id,
                  )
                }
              />
            ))}
          </View>
        </View>
      </ScrollView>

      <Dialog
        visible={dialogVisible}
        onClose={() =>
          setDialogVisible(false)
        }
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {dialogTitle}
            </DialogTitle>
          </DialogHeader>

          <DialogDescription>
            {dialogDescription}
          </DialogDescription>

          <DialogFooter>
            {dialogMode === 'confirm' ? (
              <>
                <DialogClose>
                  <Button
                    title="إلغاء"
                    onPress={() => {
                      setPendingAction(null);
                    }}
                    variant="outline"
                    size="sm"
                  />
                </DialogClose>

                <Button
                  title={
                    pendingAction === 'google'
                      ? 'فصل الحساب'
                      : 'إنهاء الجلسة'
                  }
                  onPress={
                    pendingAction === 'google'
                      ? confirmGoogleDisconnect
                      : confirmRevokeDevice
                  }
                  variant="destructive"
                  size="sm"
                />
              </>
            ) : (
              <DialogClose>
                <Button
                  title="تم"
                  onPress={() => {}}
                  variant="default"
                  size="sm"
                />
              </DialogClose>
            )}
          </DialogFooter>
        </DialogContent>
      </Dialog>
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
/* Device Row                                                                 */
/* -------------------------------------------------------------------------- */

function DeviceRow({
  device,
  theme,
  divider,
  onRevoke,
}: {
  device: DeviceItem;
  theme: ReturnType<typeof useTheme>;
  divider: boolean;
  onRevoke: () => void;
}) {
  const icon =
    device.type === 'mobile'
      ? 'phone-portrait-outline'
      : device.type === 'desktop'
        ? 'laptop-outline'
        : 'globe-outline';

  return (
    <View
      style={[
        styles.deviceRow,
        divider && {
          borderTopWidth:
            StyleSheet.hairlineWidth,
          borderTopColor: theme.border,
        },
      ]}
    >
      <Ionicons
        name={icon}
        size={21}
        color={
          device.isCurrent
            ? theme.text
            : theme.textSecondary
        }
      />

      <View style={styles.deviceInfo}>
        <Text
          numberOfLines={1}
          style={[
            styles.rowTitle,
            {
              color: theme.text,
            },
          ]}
        >
          {device.name}
        </Text>

        <Text
          style={[
            styles.rowSubtitle,
            {
              color: theme.textMuted,
            },
          ]}
        >
          {device.lastActive}
        </Text>
      </View>

      {device.isCurrent ? (
        <View style={styles.currentDevice}>
          <View
            style={[
              styles.currentDot,
              {
                backgroundColor:
                  theme.primary,
              },
            ]}
          />

          <Text
            style={[
              styles.currentText,
              {
                color: theme.textSecondary,
              },
            ]}
          >
            الحالي
          </Text>
        </View>
      ) : (
        <Button
          title="إنهاء"
          onPress={onRevoke}
          variant="ghost"
          size="sm"
          textStyle={{
            color: theme.error,
          }}
        />
      )}
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

  /* MFA */

  securityRow: {
    minHeight: 76,

    flexDirection: 'row',
    alignItems: 'center',

    borderTopWidth:
      StyleSheet.hairlineWidth,

    borderBottomWidth:
      StyleSheet.hairlineWidth,
  },

  securityIcon: {
    width: 42,
    height: 42,

    borderRadius: Radii.lg,

    alignItems: 'center',
    justifyContent: 'center',
  },

  securityInfo: {
    flex: 1,
    minWidth: 0,
    marginLeft: Spacing.three,
  },

  /* Linked Accounts */

  providerRow: {
    minHeight: 72,

    flexDirection: 'row',
    alignItems: 'center',

    borderBottomWidth:
      StyleSheet.hairlineWidth,
    paddingBottom: Spacing.four,
  },

  googleIcon: {
    width: 48,
    height: 48,

    borderRadius: Radii.full,

    alignItems: 'center',
    justifyContent: 'center',
  },

  providerInfo: {
    flex: 1,
    minWidth: 0,
    marginLeft: Spacing.three,
  },

  providerTitle: {
    ...Typography.bodyBold,
    fontSize: 15,
  },

  providerEmail: {
    ...Typography.small,
    marginTop: 3,
  },

  status: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },

  statusDot: {
    width: 6,
    height: 6,
    borderRadius: Radii.full,
  },

  statusText: {
    ...Typography.small,
  },

  actionArea: {
    marginTop: Spacing.five,
  },

  /* Password */

  passwordSection: {
    borderTopWidth:
      StyleSheet.hairlineWidth,

    paddingTop: Spacing.four,

    gap: Spacing.three,
  },

  /* Devices */

  devicesList: {
    borderTopWidth:
      StyleSheet.hairlineWidth,

    borderBottomWidth:
      StyleSheet.hairlineWidth,
  },

  deviceRow: {
    minHeight: 72,

    flexDirection: 'row',
    alignItems: 'center',

    gap: Spacing.three,
  },

  deviceInfo: {
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

  currentDevice: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },

  currentDot: {
    width: 6,
    height: 6,
    borderRadius: Radii.full,
  },

  currentText: {
    ...Typography.small,
  },
});