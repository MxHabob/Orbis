/**
 * @module @orbis/mobile/app/(app)/settings/linked-accounts
 *
 * Linked Identity Providers & Google Account Management Screen.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { Header } from '@/components/header';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { useToast } from '@/components/ui/toast';
import { Spacing, Typography } from '@/constants/theme';

export default function LinkedAccountsScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const toast = useToast();

  const [googleLinked, setGoogleLinked] = useState(true);
  const [dialogVisible, setDialogVisible] = useState(false);
  const [dialogMode, setDialogMode] = useState<'confirm' | 'message'>('confirm');
  const [dialogTitle, setDialogTitle] = useState('');
  const [dialogDescription, setDialogDescription] = useState('');

  const openDialog = (title: string, description: string, mode: 'confirm' | 'message' = 'message') => {
    setDialogTitle(title);
    setDialogDescription(description);
    setDialogMode(mode);
    setDialogVisible(true);
  };

  const toggleGoogle = () => {
    if (googleLinked) {
      openDialog('فصل الحساب', 'هل أنت متأكد من إلغاء ربط حساب Google؟', 'confirm');
    } else {
      setGoogleLinked(true);
      toast.success('تم ربط حساب Google بنجاح.');
      openDialog('تم الربط', 'تم ربط حساب Google بنجاح.');
    }
  };

  const confirmDisconnect = () => {
    setGoogleLinked(false);
    setDialogVisible(false);
    toast.success('تم إلغاء ربط الحساب.');
    openDialog('تم', 'تم إلغاء ربط الحساب.');
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الحسابات المرتبطة" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}
        showsVerticalScrollIndicator={false}>
        <Card variant="elevated" style={styles.card}>
          <View style={styles.providerRow}>
            <View style={[styles.providerIcon, { backgroundColor: theme.backgroundElement }]}>
              <Ionicons name="logo-google" size={26} color={theme.text} />
            </View>

            <View style={{ flex: 1 }}>
              <Text style={[styles.providerTitle, { color: theme.text }]}>حساب Google</Text>
              <Text style={[styles.providerEmail, { color: theme.textSecondary }]}>
                {googleLinked ? 'user@gmail.com' : 'غير متصل'}
              </Text>
            </View>

            <Badge
              label={googleLinked ? 'متصل' : 'غير متصل'}
              variant={googleLinked ? 'success' : 'neutral'}
              size="sm"
            />
          </View>

          <Text style={[styles.desc, { color: theme.textSecondary }]}>
            يتيح لك ربط حساب Google تسجيل الدخول السريع بلمسة واحدة دون الحاجة لإدخال كلمة المرور في كل مرة.
          </Text>

          <Button
            title={googleLinked ? 'فصل حساب Google' : 'ربط حساب Google'}
            onPress={toggleGoogle}
            variant={googleLinked ? 'secondary' : 'primary'}
            size="md"
            fullWidth
          />
        </Card>
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
                <Button title="فصل" onPress={confirmDisconnect} variant="destructive" size="sm" />
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
  },
  card: {
    padding: Spacing.five,
    gap: Spacing.three,
  },
  providerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  providerIcon: {
    width: 48,
    height: 48,
    borderRadius: 24,
    alignItems: 'center',
    justifyContent: 'center',
  },
  providerTitle: {
    ...Typography.bodyBold,
  },
  providerEmail: {
    ...Typography.small,
    marginTop: 2,
  },
  desc: {
    ...Typography.caption,
    lineHeight: 20,
  },
});
