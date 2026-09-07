/**
 * @module @orbis/mobile/app/(auth)/register
 *
 * New Student Admission & Registration Application Screen.
 * Allows parents to submit student admission details for school verification and activation.
 */

import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Header } from '@/components/header';
import { KeyboardAwareView } from '@/components/ui/keyboard-aware-view';
import {
  StudentRegistrationForm,
  StudentRegistrationSuccess,
  type StudentRegistrationData,
} from '@/components/auth';
import { Spacing } from '@/constants/theme';

export default function RegisterScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { t } = useI18n();

  const [submittedData, setSubmittedData] = useState<{
    referenceNumber: string;
    studentName: string;
  } | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmitApplication = async (data: StudentRegistrationData) => {
    setLoading(true);
    try {
      // Simulate network submission / API mutation to student admission endpoint
      await new Promise((resolve) => setTimeout(resolve, 800));

      // Generate unique admission reference number
      const randomSuffix = Math.floor(1000 + Math.random() * 9000);
      const year = new Date().getFullYear();
      const refNumber = `REG-${year}-${randomSuffix}`;

      setSubmittedData({
        referenceNumber: refNumber,
        studentName: data.studentName,
      });
    } finally {
      setLoading(false);
    }
  };

  const handleReset = () => {
    setSubmittedData(null);
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title={t('admission_title')}
        subtitle={t('admission_subtitle')}
        showBack={!submittedData}
      />

      <KeyboardAwareView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.six },
        ]}>
        {submittedData ? (
          <StudentRegistrationSuccess
            referenceNumber={submittedData.referenceNumber}
            studentName={submittedData.studentName}
            onReset={handleReset}
          />
        ) : (
          <StudentRegistrationForm
            onSubmit={handleSubmitApplication}
            loading={loading}
          />
        )}
      </KeyboardAwareView>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.two,
  },
});
