/**
 * @module @orbis/mobile/components/auth/student-registration-form
 *
 * Multi-step Student Admission & Registration Form.
 * Allows parents to submit student admission applications to the educational institution.
 */

import React, { useState } from 'react';
import { View, Text, StyleSheet, Pressable, Keyboard } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/hooks/use-theme';
import { useI18n } from '@/lib/i18n';
import { Radii, Spacing, Typography } from '@/constants/theme';

export interface StudentRegistrationData {
  studentName: string;
  studentId: string;
  dob: string;
  grade: string;
  gender: 'male' | 'female';
  guardianName: string;
  guardianRelation: 'father' | 'mother' | 'other';
  guardianPhone: string;
  guardianEmail: string;
  guardianId: string;
  notes?: string;
}

interface StudentRegistrationFormProps {
  onSubmit: (data: StudentRegistrationData) => Promise<void>;
  loading?: boolean;
}

export const StudentRegistrationForm = React.memo<StudentRegistrationFormProps>(
  ({ onSubmit, loading = false }) => {
    const theme = useTheme();
    const { t } = useI18n();

    const [step, setStep] = useState<1 | 2>(1);
    const [error, setError] = useState<string | null>(null);

    // Student Information
    const [studentName, setStudentName] = useState('');
    const [studentId, setStudentId] = useState('');
    const [dob, setDob] = useState('');
    const [grade, setGrade] = useState('');
    const [gender, setGender] = useState<'male' | 'female'>('male');

    // Guardian Information
    const [guardianName, setGuardianName] = useState('');
    const [guardianRelation, setGuardianRelation] = useState<'father' | 'mother' | 'other'>('father');
    const [guardianPhone, setGuardianPhone] = useState('');
    const [guardianEmail, setGuardianEmail] = useState('');
    const [guardianId, setGuardianId] = useState('');
    const [notes, setNotes] = useState('');

    const handleNextStep = () => {
      Keyboard.dismiss();
      setError(null);

      if (!studentName.trim()) {
        setError(t('admission_err_student_name'));
        return;
      }
      if (!studentId.trim()) {
        setError(t('admission_err_student_id'));
        return;
      }
      if (!grade.trim()) {
        setError(t('admission_err_grade'));
        return;
      }

      setStep(2);
    };

    const handlePrevStep = () => {
      Keyboard.dismiss();
      setError(null);
      setStep(1);
    };

    const handleSubmit = async () => {
      Keyboard.dismiss();
      setError(null);

      if (!guardianName.trim()) {
        setError(t('admission_err_guardian_name'));
        return;
      }
      if (!guardianPhone.trim()) {
        setError(t('admission_err_guardian_phone'));
        return;
      }
      if (!guardianEmail.trim() || !guardianEmail.includes('@')) {
        setError(t('admission_err_guardian_email'));
        return;
      }

      await onSubmit({
        studentName: studentName.trim(),
        studentId: studentId.trim(),
        dob: dob.trim(),
        grade: grade.trim(),
        gender,
        guardianName: guardianName.trim(),
        guardianRelation,
        guardianPhone: guardianPhone.trim(),
        guardianEmail: guardianEmail.trim(),
        guardianId: guardianId.trim(),
        notes: notes.trim(),
      });
    };

    return (
      <View style={styles.container}>
        {/* Step Indicator Header */}
        <View style={styles.stepHeader}>
          <View style={styles.stepTrack}>
            {/* Step 1 */}
            <View style={styles.stepItem}>
              <View
                style={[
                  styles.stepBadge,
                  {
                    backgroundColor:
                      step === 1 ? theme.primary : theme.success,
                  },
                ]}>
                {step > 1 ? (
                  <Ionicons name="checkmark" size={14} color="#FFFFFF" />
                ) : (
                  <Text style={styles.stepNumber}>1</Text>
                )}
              </View>
              <Text
                style={[
                  styles.stepLabel,
                  { color: step === 1 ? theme.primary : theme.textSecondary },
                ]}>
                {t('admission_step_student')}
              </Text>
            </View>

            {/* Connecting Line */}
            <View
              style={[
                styles.stepLine,
                { backgroundColor: step > 1 ? theme.success : theme.border },
              ]}
            />

            {/* Step 2 */}
            <View style={styles.stepItem}>
              <View
                style={[
                  styles.stepBadge,
                  {
                    backgroundColor:
                      step === 2
                        ? theme.primary
                        : theme.backgroundElement,
                    borderColor: step === 2 ? theme.primary : theme.border,
                    borderWidth: step === 2 ? 0 : 1.5,
                  },
                ]}>
                <Text
                  style={[
                    styles.stepNumber,
                    { color: step === 2 ? '#FFFFFF' : theme.textMuted },
                  ]}>
                  2
                </Text>
              </View>
              <Text
                style={[
                  styles.stepLabel,
                  { color: step === 2 ? theme.primary : theme.textMuted },
                ]}>
                {t('admission_step_guardian')}
              </Text>
            </View>
          </View>
        </View>

        {error && (
          <View style={[styles.errorBox, { backgroundColor: theme.errorLight }]}>
            <Ionicons name="alert-circle" size={18} color={theme.error} />
            <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
          </View>
        )}

        {/* Step 1: Student Information */}
        {step === 1 && (
          <Card variant="elevated" style={styles.card}>
            <View style={styles.cardHeader}>
              <Text style={[styles.title, { color: theme.text }]}>
                {t('admission_student_info')}
              </Text>
              <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
                {t('admission_student_sub')}
              </Text>
            </View>

            <View style={styles.fields}>
              <Input
                label={t('admission_student_name')}
                placeholder={t('admission_student_name_ph')}
                value={studentName}
                onChangeText={(text) => {
                  setStudentName(text);
                  setError(null);
                }}
                returnKeyType="next"
                icon={<Ionicons name="person-outline" size={20} color={theme.textMuted} />}
              />

              <Input
                label={t('admission_student_id')}
                placeholder={t('admission_student_id_ph')}
                value={studentId}
                onChangeText={(text) => {
                  setStudentId(text);
                  setError(null);
                }}
                keyboardType="number-pad"
                returnKeyType="next"
                icon={<Ionicons name="card-outline" size={20} color={theme.textMuted} />}
              />

              {/* Gender Selector */}
              <View style={styles.selectorGroup}>
                <Text style={[styles.inputLabel, { color: theme.text }]}>
                  {t('admission_gender_label')}
                </Text>
                <View style={styles.genderRow}>
                  <Pressable
                    onPress={() => setGender('male')}
                    style={[
                      styles.choicePill,
                      {
                        backgroundColor:
                          gender === 'male' ? theme.primaryLight : theme.backgroundElement,
                        borderColor: gender === 'male' ? theme.primary : theme.border,
                      },
                    ]}>
                    <Ionicons
                      name="male"
                      size={18}
                      color={gender === 'male' ? theme.primary : theme.textSecondary}
                    />
                    <Text
                      style={[
                        styles.choiceText,
                        { color: gender === 'male' ? theme.primary : theme.text },
                      ]}>
                      {t('admission_gender_male')}
                    </Text>
                  </Pressable>

                  <Pressable
                    onPress={() => setGender('female')}
                    style={[
                      styles.choicePill,
                      {
                        backgroundColor:
                          gender === 'female' ? theme.primaryLight : theme.backgroundElement,
                        borderColor: gender === 'female' ? theme.primary : theme.border,
                      },
                    ]}>
                    <Ionicons
                      name="female"
                      size={18}
                      color={gender === 'female' ? theme.primary : theme.textSecondary}
                    />
                    <Text
                      style={[
                        styles.choiceText,
                        { color: gender === 'female' ? theme.primary : theme.text },
                      ]}>
                      {t('admission_gender_female')}
                    </Text>
                  </Pressable>
                </View>
              </View>

              <Input
                label={t('admission_grade_label')}
                placeholder={t('admission_grade_ph')}
                value={grade}
                onChangeText={(text) => {
                  setGrade(text);
                  setError(null);
                }}
                returnKeyType="next"
                icon={<Ionicons name="school-outline" size={20} color={theme.textMuted} />}
              />

              <Input
                label={t('admission_student_dob')}
                placeholder={t('admission_student_dob_ph')}
                value={dob}
                onChangeText={setDob}
                keyboardType="numbers-and-punctuation"
                returnKeyType="done"
                onSubmitEditing={handleNextStep}
                icon={<Ionicons name="calendar-outline" size={20} color={theme.textMuted} />}
              />

              <Button
                title={t('admission_next_step')}
                onPress={handleNextStep}
                fullWidth
                size="lg"
              />
            </View>
          </Card>
        )}

        {/* Step 2: Guardian & Contact Information */}
        {step === 2 && (
          <Card variant="elevated" style={styles.card}>
            <View style={styles.cardHeader}>
              <Text style={[styles.title, { color: theme.text }]}>
                {t('admission_guardian_info')}
              </Text>
              <Text style={[styles.subtitle, { color: theme.textSecondary }]}>
                {t('admission_guardian_sub')}
              </Text>
            </View>

            <View style={styles.fields}>
              <Input
                label={t('admission_guardian_name')}
                placeholder={t('admission_guardian_name_ph')}
                value={guardianName}
                onChangeText={(text) => {
                  setGuardianName(text);
                  setError(null);
                }}
                returnKeyType="next"
                icon={<Ionicons name="people-outline" size={20} color={theme.textMuted} />}
              />

              {/* Relationship Selector */}
              <View style={styles.selectorGroup}>
                <Text style={[styles.inputLabel, { color: theme.text }]}>
                  {t('admission_guardian_relation')}
                </Text>
                <View style={styles.genderRow}>
                  {[
                    { key: 'father', label: t('admission_guardian_relation_father') },
                    { key: 'mother', label: t('admission_guardian_relation_mother') },
                    { key: 'other', label: t('admission_guardian_relation_other') },
                  ].map((rel) => {
                    const isSelected = guardianRelation === rel.key;
                    return (
                      <Pressable
                        key={rel.key}
                        onPress={() => setGuardianRelation(rel.key as any)}
                        style={[
                          styles.choicePill,
                          {
                            backgroundColor: isSelected
                              ? theme.primaryLight
                              : theme.backgroundElement,
                            borderColor: isSelected ? theme.primary : theme.border,
                          },
                        ]}>
                        <Text
                          style={[
                            styles.choiceText,
                            { color: isSelected ? theme.primary : theme.text },
                          ]}>
                          {rel.label}
                        </Text>
                      </Pressable>
                    );
                  })}
                </View>
              </View>

              <Input
                label={t('admission_guardian_phone')}
                placeholder={t('admission_guardian_phone_ph')}
                value={guardianPhone}
                onChangeText={(text) => {
                  setGuardianPhone(text);
                  setError(null);
                }}
                keyboardType="phone-pad"
                returnKeyType="next"
                icon={<Ionicons name="call-outline" size={20} color={theme.textMuted} />}
              />

              <Input
                label={t('admission_guardian_email')}
                placeholder={t('admission_guardian_email_ph')}
                value={guardianEmail}
                onChangeText={(text) => {
                  setGuardianEmail(text);
                  setError(null);
                }}
                keyboardType="email-address"
                autoCapitalize="none"
                returnKeyType="next"
                icon={<Ionicons name="mail-outline" size={20} color={theme.textMuted} />}
              />

              <Input
                label={t('admission_guardian_id')}
                placeholder={t('admission_guardian_id_ph')}
                value={guardianId}
                onChangeText={setGuardianId}
                keyboardType="number-pad"
                returnKeyType="next"
                icon={<Ionicons name="card-outline" size={20} color={theme.textMuted} />}
              />

              <Input
                label={t('admission_notes_label')}
                placeholder={t('admission_notes_ph')}
                value={notes}
                onChangeText={setNotes}
                multiline
                numberOfLines={3}
                style={{ height: 72, textAlignVertical: 'top' }}
              />

              <View style={styles.actionRow}>
                <Button
                  title={t('admission_prev_step')}
                  onPress={handlePrevStep}
                  variant="outline"
                  style={{ flex: 1 }}
                />
                <Button
                  title={t('admission_submit_btn')}
                  onPress={handleSubmit}
                  loading={loading}
                  style={{ flex: 2 }}
                />
              </View>
            </View>
          </Card>
        )}
      </View>
    );
  },
);

StudentRegistrationForm.displayName = 'StudentRegistrationForm';

const styles = StyleSheet.create({
  container: {
    width: '100%',
    gap: Spacing.four,
  },
  stepHeader: {
    alignItems: 'center',
    marginVertical: Spacing.two,
  },
  stepTrack: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: Spacing.two,
    width: '100%',
    paddingHorizontal: Spacing.two,
  },
  stepItem: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
  },
  stepBadge: {
    width: 28,
    height: 28,
    borderRadius: 14,
    alignItems: 'center',
    justifyContent: 'center',
  },
  stepNumber: {
    ...Typography.captionBold,
    color: '#FFFFFF',
  },
  stepLabel: {
    ...Typography.captionBold,
    fontSize: 13,
  },
  stepLine: {
    flex: 1,
    height: 2,
    maxWidth: 60,
    marginHorizontal: Spacing.two,
  },
  card: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  cardHeader: {
    gap: 4,
    marginBottom: Spacing.one,
  },
  title: {
    ...Typography.h2,
    fontSize: 20,
    fontWeight: '700',
  },
  subtitle: {
    ...Typography.caption,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: Spacing.two,
    padding: Spacing.three,
    borderRadius: Radii.md,
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  fields: {
    gap: Spacing.three,
  },
  selectorGroup: {
    gap: Spacing.one,
  },
  inputLabel: {
    ...Typography.captionBold,
    marginBottom: 2,
  },
  genderRow: {
    flexDirection: 'row',
    gap: Spacing.two,
  },
  choicePill: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 6,
    paddingVertical: Spacing.two + 2,
    paddingHorizontal: Spacing.two,
    borderRadius: Radii.md,
    borderWidth: 1.5,
  },
  choiceText: {
    ...Typography.captionBold,
    fontSize: 13,
  },
  actionRow: {
    flexDirection: 'row',
    gap: Spacing.two,
    marginTop: Spacing.two,
  },
});
