/**
 * @module @orbis/mobile/app/(app)/settings/linked-accounts
 *
 * Linked Identity Providers & Google Account Management Screen.
 */

import React, { useState } from 'react';
import {
  Pressable,
  ScrollView,
  StyleSheet,
  Text,
  View,
} from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Button } from '@/components/ui/button';
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

export default function LinkedAccountsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();

  const [googleLinked, setGoogleLinked] =
    useState(true);

  const [dialogVisible, setDialogVisible] =
    useState(false);

  const [dialogMode, setDialogMode] =
    useState<'confirm' | 'message'>('confirm');

  const [dialogTitle, setDialogTitle] =
    useState('');

  const [dialogDescription, setDialogDescription] =
    useState('');

  const openDialog = (
    title: string,
    description: string,
    mode: 'confirm' | 'message' = 'message',
  ) => {
    setDialogTitle(title);
    setDialogDescription(description);
    setDialogMode(mode);
    setDialogVisible(true);
  };

  const handleGooglePress = () => {
    if (googleLinked) {
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

  const confirmDisconnect = () => {
    setGoogleLinked(false);
    setDialogVisible(false);

    toast.success(
      'تم إلغاء ربط حساب Google.',
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
          title="Linked Accounts"
          showBack
          showCrown={false}
        />

        <View style={styles.main}>
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
                    onPress={() => {}}
                    variant="outline"
                    size="sm"
                  />
                </DialogClose>

                <Button
                  title="فصل الحساب"
                  onPress={confirmDisconnect}
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
});