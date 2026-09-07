/**
 * @module @orbis/mobile/app/(auth)/register
 *
 * Multi-Step Student / Parent / Teacher Onboarding & Registration Wizard.
 */

import React, { useState } from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  KeyboardAvoidingView,
  Platform,
  Pressable,
} from 'react-native';
import { router, Link } from 'expo-router';
import { Ionicons } from '@expo/vector-icons';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import { useAuth, api } from '@/lib/api';
import { useTheme } from '@/hooks/use-theme';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Header } from '@/components/header';
import { Badge } from '@/components/ui/badge';
import { Radii, Spacing, Typography } from '@/constants/theme';

export default function RegisterScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { setSession } = useAuth();

  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [role, setRole] = useState<'Student' | 'Parent' | 'Teacher'>('Student');
  const [fullName, setFullName] = useState('');
  const [nationalId, setNationalId] = useState('');
  const [schoolCode, setSchoolCode] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleNext = () => {
    setError(null);
    if (step === 1) {
      if (!fullName.trim()) {
        setError('يرجى إدخال الاسم الكامل الثلاثي');
        return;
      }
      setStep(2);
    } else if (step === 2) {
      if (!schoolCode.trim()) {
        setError('يرجى إدخال رمز المدرسة أو المؤسسة التعليمية');
        return;
      }
      setStep(3);
    }
  };

  const handleRegister = async () => {
    setError(null);
    if (!email.trim() || !password) {
      setError('يرجى إدخال البريد الإلكتروني وكلمة المرور');
      return;
    }
    if (password !== confirmPassword) {
      setError('كلمات المرور غير متطابقة');
      return;
    }
    if (password.length < 6) {
      setError('كلمة المرور يجب أن لا تقل عن 6 أحرف');
      return;
    }

    setLoading(true);
    try {
      // Register with Orbis backend
      const user = await api.registerUser({
        email: email.trim(),
        full_name: fullName.trim(),
        password: password,
        role: role,
      });

      // Automatically authenticate session
      await setSession({
        token: `jwt-registered-${user.id}-${Date.now()}`,
        user: user,
      });

      router.replace('/(app)/(tabs)');
    } catch (err: any) {
      setError(err?.message || 'فشل إكمال التسجيل. يرجى المحاولة لاحقاً.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <KeyboardAvoidingView
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
      style={[styles.container, { backgroundColor: theme.background }]}>
      <Header
        title="إنشاء حساب جديد"
        subtitle={`الخطوة ${step} من 3`}
        showBack
        onBack={() => {
          if (step > 1) {
            setStep((step - 1) as any);
          } else {
            router.back();
          }
        }}
      />

      <ScrollView
        contentContainerStyle={[styles.scrollContent, { paddingBottom: insets.bottom + Spacing.six }]}
        showsVerticalScrollIndicator={false}>
        {/* Step Indicator */}
        <View style={styles.stepIndicator}>
          {[1, 2, 3].map((s) => {
            const isDone = s < step;
            const isCurrent = s === step;
            return (
              <React.Fragment key={s}>
                <View
                  style={[
                    styles.stepCircle,
                    {
                      backgroundColor: isDone
                        ? theme.success
                        : isCurrent
                          ? theme.primary
                          : theme.backgroundSelected,
                    },
                  ]}>
                  {isDone ? (
                    <Ionicons name="checkmark" size={14} color="#FFFFFF" />
                  ) : (
                    <Text
                      style={[
                        styles.stepNumber,
                        { color: isCurrent ? '#FFFFFF' : theme.textMuted },
                      ]}>
                      {s}
                    </Text>
                  )}
                </View>
                {s < 3 && (
                  <View
                    style={[
                      styles.stepLine,
                      { backgroundColor: isDone ? theme.success : theme.border },
                    ]}
                  />
                )}
              </React.Fragment>
            );
          })}
        </View>

        {error && (
          <View style={[styles.errorBox, { backgroundColor: theme.errorLight }]}>
            <Ionicons name="alert-circle" size={18} color={theme.error} />
            <Text style={[styles.errorText, { color: theme.error }]}>{error}</Text>
          </View>
        )}

        {/* Step 1: Role & Identity */}
        {step === 1 && (
          <Card variant="elevated" style={styles.card}>
            <Text style={[styles.sectionTitle, { color: theme.text }]}>نوع الحساب والاسم</Text>
            <Text style={[styles.sectionSubtitle, { color: theme.textSecondary }]}>
              حدد دورك الأساسي في المنصة التعليمية
            </Text>

            <View style={styles.rolePicker}>
              {[
                { key: 'Student', label: 'طالب (Student)', icon: 'school-outline' },
                { key: 'Parent', label: 'ولي أمر (Parent)', icon: 'people-outline' },
                { key: 'Teacher', label: 'معلم (Teacher)', icon: 'briefcase-outline' },
              ].map((r) => {
                const isSelected = role === r.key;
                return (
                  <Pressable
                    key={r.key}
                    onPress={() => setRole(r.key as any)}
                    style={[
                      styles.roleOption,
                      {
                        backgroundColor: isSelected
                          ? theme.primaryLight
                          : theme.backgroundElement,
                        borderColor: isSelected ? theme.primary : theme.border,
                      },
                    ]}>
                    <Ionicons
                      name={r.icon as any}
                      size={22}
                      color={isSelected ? theme.primary : theme.textSecondary}
                    />
                    <Text
                      style={[
                        styles.roleLabel,
                        { color: isSelected ? theme.primary : theme.text },
                      ]}>
                      {r.label}
                    </Text>
                    {isSelected && (
                      <Ionicons name="checkmark-circle" size={20} color={theme.primary} />
                    )}
                  </Pressable>
                );
              })}
            </View>

            <Input
              label="الاسم الكامل"
              placeholder="مثال: عمر خالد عبدالله المنصور"
              value={fullName}
              onChangeText={setFullName}
              icon={<Ionicons name="person-outline" size={20} color={theme.textMuted} />}
            />

            <Button title="المتابعة للخطوة التالية" onPress={handleNext} fullWidth size="lg" />
          </Card>
        )}

        {/* Step 2: Institution & Student Info */}
        {step === 2 && (
          <Card variant="elevated" style={styles.card}>
            <Text style={[styles.sectionTitle, { color: theme.text }]}>المؤسسة التعليمية</Text>
            <Text style={[styles.sectionSubtitle, { color: theme.textSecondary }]}>
              أدخل رمز المدرسة أو المعرف الأكاديمي لربط الحساب
            </Text>

            <Input
              label="رمز المدرسة / المؤسسة"
              placeholder="مثال: SCH-RUH-01"
              value={schoolCode}
              onChangeText={setSchoolCode}
              autoCapitalize="characters"
              hint="يمكن الحصول على الرمز من إدارة المدرسة"
              icon={<Ionicons name="business-outline" size={20} color={theme.textMuted} />}
            />

            <Input
              label="رقم الهوية الوطنية / الإقامة (اختياري)"
              placeholder="10XXXXXXXX"
              value={nationalId}
              onChangeText={setNationalId}
              keyboardType="number-pad"
              icon={<Ionicons name="card-outline" size={20} color={theme.textMuted} />}
            />

            <Button title="المتابعة إلى بيانات الدخول" onPress={handleNext} fullWidth size="lg" />
          </Card>
        )}

        {/* Step 3: Account Credentials */}
        {step === 3 && (
          <Card variant="elevated" style={styles.card}>
            <Text style={[styles.sectionTitle, { color: theme.text }]}>بيانات الحساب وكلمة المرور</Text>
            <Text style={[styles.sectionSubtitle, { color: theme.textSecondary }]}>
              أنشئ بيانات الاعتماد لتسجيل الدخول مستقبلاً
            </Text>

            <Input
              label="البريد الإلكتروني"
              placeholder="name@school.edu.sa"
              value={email}
              onChangeText={setEmail}
              keyboardType="email-address"
              autoCapitalize="none"
              icon={<Ionicons name="mail-outline" size={20} color={theme.textMuted} />}
            />

            <Input
              label="كلمة المرور"
              placeholder="••••••••"
              value={password}
              onChangeText={setPassword}
              secureTextEntry
              icon={<Ionicons name="lock-closed-outline" size={20} color={theme.textMuted} />}
            />

            <Input
              label="تأكيد كلمة المرور"
              placeholder="••••••••"
              value={confirmPassword}
              onChangeText={setConfirmPassword}
              secureTextEntry
              icon={<Ionicons name="shield-checkmark-outline" size={20} color={theme.textMuted} />}
            />

            <Button
              title="إتمام التسجيل والدخول"
              onPress={handleRegister}
              loading={loading}
              fullWidth
              size="lg"
            />
          </Card>
        )}

        <View style={styles.footerRow}>
          <Text style={[styles.footerText, { color: theme.textSecondary }]}>لديك حساب بالفعل؟</Text>
          <Link href="/(auth)/sign-in" asChild>
            <Pressable hitSlop={8}>
              <Text style={[styles.loginLink, { color: theme.primary }]}>تسجيل الدخول</Text>
            </Pressable>
          </Link>
        </View>
      </ScrollView>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollContent: {
    paddingHorizontal: Spacing.four,
    gap: Spacing.four,
  },
  stepIndicator: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    marginVertical: Spacing.two,
  },
  stepCircle: {
    width: 28,
    height: 28,
    borderRadius: 14,
    alignItems: 'center',
    justifyContent: 'center',
  },
  stepNumber: {
    ...Typography.small,
    fontWeight: '700',
  },
  stepLine: {
    flex: 1,
    height: 2,
    maxWidth: 60,
    marginHorizontal: 8,
  },
  errorBox: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: Spacing.three,
    borderRadius: Radii.md,
    gap: Spacing.two,
  },
  errorText: {
    ...Typography.captionBold,
    flex: 1,
  },
  card: {
    gap: Spacing.three,
  },
  sectionTitle: {
    ...Typography.h2,
  },
  sectionSubtitle: {
    ...Typography.caption,
    marginBottom: Spacing.two,
  },
  rolePicker: {
    gap: Spacing.two,
    marginBottom: Spacing.two,
  },
  roleOption: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: Spacing.three,
    borderRadius: Radii.md,
    borderWidth: 1.5,
    gap: Spacing.three,
  },
  roleLabel: {
    ...Typography.bodyBold,
    flex: 1,
  },
  footerRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: 6,
    marginTop: Spacing.two,
  },
  footerText: {
    ...Typography.body,
  },
  loginLink: {
    ...Typography.bodyBold,
  },
});
