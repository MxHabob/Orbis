/**
 * @module @orbis/mobile/app/(app)/profile/index
 *
 * User Profile & Settings Screen for Orbis Mobile.
 * Faithful implementation of the designed UI:
 * - Top Profile Header with circular sculpture avatar, name, student/grade, school badge, and gear icon.
 * - Divider.
 * - 8 Grouped Menu Items with circular icons, titles, subtitles, and chevron navigation.
 * - Bottom Card with Orbis branding, version, and "App Info" pill button.
 */

import React, { useState } from "react";
import {
  View,
  Text,
  ScrollView,
  Pressable,
  Alert,
} from "react-native";
import { Image } from "expo-image";
import { useRouter } from "expo-router";
import { useSafeAreaInsets } from "react-native-safe-area-context";
import { useI18n } from "@/lib/i18n";
import { useAuth } from "@/lib/api";
import { useTheme } from "@/hooks/use-theme";
import {
  saveThemePreference,
  loadThemePreference,
  type ThemePreference,
} from "@/lib/theme-preference";
import {
  User,
  ShieldCheck,
  Bell,
  GlobeAlt,
  Moon,
  QuestionMarkCircle,
  InfoCircle,
  BuildingLibrary,
  AcademicCap,
  Cog6,
  ChevronRight,
  ChevronLeft,
  Logo,
  SignOut,
} from "@orbis/icons/native";

const BUST_AVATAR_URI =
  "https://images.unsplash.com/photo-1544005313-94ddf0286df2?w=400&auto=format&fit=crop&q=80";

export default function ProfileScreen() {
  const router = useRouter();
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { user, effectiveContext, logout } = useAuth();
  const { t, isArabic, locale, setLocale } = useI18n();

  const [themeMode, setThemeMode] = useState<ThemePreference>("system");
  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;

  // Real user data with graceful fallbacks matching design
  const displayName = user
    ? isArabic
      ? user.full_name_ar || user.full_name || "عمر الهادي"
      : user.full_name || "Omar Al-Hadi"
    : "Omar Al-Hadi";

  const gradeRole = isArabic
    ? "طالب  •  الصف العاشر"
    : "Student  •  Grade 10";

  const schoolName = isArabic ? "مدرسة النور" : "Al-Noor School";

  // Cycle language between English and Arabic
  const handleToggleLanguage = () => {
    const nextLocale = isArabic ? "en" : "ar";
    setLocale(nextLocale);
  };

  // Cycle theme between System, Light, and Dark
  const handleCycleTheme = async () => {
    const next: ThemePreference =
      themeMode === "system" ? "dark" : themeMode === "dark" ? "light" : "system";
    setThemeMode(next);
    await saveThemePreference(next);
  };

  // Sign out confirmation dialog
  const handleSignOut = () => {
    Alert.alert(
      isArabic ? "تسجيل الخروج" : "Sign Out",
      isArabic
        ? "هل أنت متأكد من رغبتك في تسجيل الخروج من أوربيس؟"
        : "Are you sure you want to sign out of Orbis?",
      [
        { text: isArabic ? "إلغاء" : "Cancel", style: "cancel" },
        {
          text: isArabic ? "خروج" : "Sign Out",
          style: "destructive",
          onPress: () => {
            void logout().then(() => router.replace("/(auth)/sign-in"));
          },
        },
      ],
    );
  };

  // Settings gear action modal/prompt
  const handleSettingsPress = () => {
    Alert.alert(
      isArabic ? "الإعدادات السريعة" : "Quick Settings",
      isArabic ? "خيارات الحساب" : "Account Options",
      [
        {
          text: isArabic ? "تعديل الملف الشخصي" : "Edit Profile",
          onPress: () => router.push("/(app)/profile/account"),
        },
        {
          text: isArabic ? "الأمان وكلمة المرور" : "Security & Password",
          onPress: () => router.push("/(app)/profile/security"),
        },
        {
          text: isArabic ? "تسجيل الخروج" : "Sign Out",
          style: "destructive",
          onPress: handleSignOut,
        },
        { text: isArabic ? "إغلاق" : "Close", style: "cancel" },
      ],
    );
  };

  return (
    <ScrollView
      className="flex-1 bg-background"
      showsVerticalScrollIndicator={false}
      contentContainerStyle={{
        paddingTop: Math.max(insets.top, 16) + 12,
        paddingBottom: Math.max(insets.bottom, 16) + 100,
        paddingHorizontal: 20,
      }}
      bounces={true}
    >
      {/* 1. Header Profile Identity Section */}
      <View className="flex-row items-center justify-between pb-4">
        {/* Left Avatar + User Details */}
        <View className="flex-1 flex-row items-center gap-3.5 pr-2">
          {/* Circular Sculpture Bust Avatar */}
          <View className="w-20 h-20 rounded-full overflow-hidden bg-muted border border-border/80 shadow-xs">
            <Image
              source={{ uri: BUST_AVATAR_URI }}
              style={{ width: "100%", height: "100%" }}
              contentFit="cover"
              transition={200}
            />
          </View>

          {/* User Info Column */}
          <View className="flex-1 justify-center gap-0.5">
            <Text
              numberOfLines={1}
              className="text-[23px] font-bold tracking-tight text-foreground font-serif"
            >
              {displayName}
            </Text>

            <Text className="text-[13px] font-normal text-muted-foreground">
              {gradeRole}
            </Text>

            {/* School Pill Badge */}
            <View className="self-start flex-row items-center gap-1.5 rounded-full bg-[#E8F7EC] dark:bg-[#1D3323] px-2.5 py-1 mt-1 border border-[#CCEED3] dark:border-[#284830]">
              <AcademicCap size={13} color="#2A7A38" strokeWidth={2} />
              <Text className="text-[11px] font-semibold text-[#2A7A38] dark:text-[#67D17A]">
                {schoolName}
              </Text>
            </View>
          </View>
        </View>

        {/* Right Settings Gear Button */}
        <Pressable
          accessibilityRole="button"
          accessibilityLabel="Settings"
          hitSlop={8}
          onPress={handleSettingsPress}
          className="w-10 h-10 items-center justify-center rounded-full active:bg-secondary"
        >
          <Cog6 size={24} color={theme.foreground} strokeWidth={1.75} />
        </Pressable>
      </View>

      {/* 2. Soft Horizontal Divider */}
      <View className="h-px w-full bg-border/50 my-1.5" />

      {/* 3. Grouped Menu Rows (8 Items) */}
      <View className="py-1 gap-1">
        {/* 1. Personal Information */}
        <ProfileMenuRow
          icon={<User size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "المعلومات الشخصية" : "Personal Information"}
          subtitle={
            isArabic
              ? "عرض وتعديل تفاصيل ملفك الشخصي"
              : "View and edit your profile details"
          }
          onPress={() => router.push("/(app)/profile/account")}
          chevron={DirectionChevron}
        />

        {/* 2. Account & Security */}
        <ProfileMenuRow
          icon={<ShieldCheck size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "الحساب والأمان" : "Account & Security"}
          subtitle={
            isArabic
              ? "كلمة المرور، التحقق بخطوتين"
              : "Password, two-step verification"
          }
          onPress={() => router.push("/(app)/profile/security")}
          chevron={DirectionChevron}
        />

        {/* 3. Notifications */}
        <ProfileMenuRow
          icon={<Bell size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "الإشعارات" : "Notifications"}
          subtitle={
            isArabic
              ? "إدارة تنبيهات وإشعارات التطبيق"
              : "Manage your notifications"
          }
          onPress={() => router.push("/(app)/notifications")}
          chevron={DirectionChevron}
        />

        {/* 4. Language */}
        <ProfileMenuRow
          icon={<GlobeAlt size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "اللغة" : "Language"}
          subtitle={
            isArabic
              ? "تغيير لغة التطبيق"
              : "Change the app language"
          }
          trailingText={isArabic ? "العربية" : "English"}
          onPress={handleToggleLanguage}
          chevron={DirectionChevron}
        />

        {/* 5. Appearance */}
        <ProfileMenuRow
          icon={<Moon size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "المظهر" : "Appearance"}
          subtitle={
            isArabic
              ? "الوضع الفاتح / الداكن"
              : "Light / Dark mode"
          }
          trailingText={
            themeMode === "system"
              ? isArabic ? "تلقائي" : "System"
              : themeMode === "dark"
              ? isArabic ? "داكن" : "Dark"
              : isArabic ? "فاتح" : "Light"
          }
          onPress={handleCycleTheme}
          chevron={DirectionChevron}
        />

        {/* 6. Help & Support */}
        <ProfileMenuRow
          icon={
            <QuestionMarkCircle
              size={20}
              color={theme.foreground}
              strokeWidth={1.75}
            />
          }
          title={isArabic ? "المساعدة والدعم" : "Help & Support"}
          subtitle={
            isArabic
              ? "احصل على المساعدة أو تواصل معنا"
              : "Get help or contact us"
          }
          onPress={() =>
            Alert.alert(
              isArabic ? "المساعدة والدعم" : "Help & Support",
              isArabic
                ? "فريق دعم أوربيس متاح دائماً لدعمكم: support@orbis.edu"
                : "Orbis support is available 24/7 at support@orbis.edu",
            )
          }
          chevron={DirectionChevron}
        />

        {/* 7. About Orbis */}
        <ProfileMenuRow
          icon={<InfoCircle size={20} color={theme.foreground} strokeWidth={1.75} />}
          title={isArabic ? "عن أوربيس" : "About Orbis"}
          subtitle={
            isArabic
              ? "تعرّف على منصة أوربيس ورسالتنا"
              : "Learn more about the app and our mission"
          }
          onPress={() =>
            Alert.alert(
              "Orbis Learning Platform",
              "Next-generation school management & AI-powered learning environment.",
            )
          }
          chevron={DirectionChevron}
        />

        {/* 8. About the Organization */}
        <ProfileMenuRow
          icon={
            <BuildingLibrary
              size={20}
              color={theme.foreground}
              strokeWidth={1.75}
            />
          }
          title={isArabic ? "عن المنشأة التعليمية" : "About the Organization"}
          subtitle={
            isArabic
              ? "الرؤية، القيم والمزيد"
              : "Vision, values and more"
          }
          onPress={() =>
            Alert.alert(
              schoolName,
              isArabic
                ? "مؤسسة النور التعليمية الرائدة في الابتكار والتميز الأكاديمي."
                : "Pioneering educational institution fostering excellence and holistic learning.",
            )
          }
          chevron={DirectionChevron}
        />
      </View>

      {/* 4. Bottom Orbis Info Card */}
      <View className="rounded-[20px] bg-[#F4F4F6] dark:bg-card-elevated border border-border/40 p-4 flex-row items-center justify-between mt-3 shadow-2xs">
        {/* Left Logo + Brand Info */}
        <View className="flex-row items-center gap-3">
          <View className="w-11 h-11 rounded-2xl bg-white dark:bg-card border border-border/60 items-center justify-center shadow-2xs">
            <Logo size={24} color={theme.foreground} />
          </View>

          <View className="gap-0.5">
            <Text className="text-[15px] font-black tracking-tight text-foreground font-serif">
              Orbis
            </Text>
            <Text className="text-xs font-medium text-muted-foreground">
              Version 1.0.0
            </Text>
          </View>
        </View>

        {/* Right "App Info" Pill Button */}
        <Pressable
          accessibilityRole="button"
          onPress={() =>
            Alert.alert(
              "Orbis Mobile",
              "Version 1.0.0 (Build 57)\nEngine: Expo 57 / React Native 0.86\nNativeWind 5.0",
            )
          }
          className="flex-row items-center gap-1.5 rounded-full bg-white dark:bg-card border border-border/70 px-3.5 py-1.5 shadow-2xs active:bg-secondary"
        >
          <InfoCircle size={14} color={theme.foreground} strokeWidth={2} />
          <Text className="text-xs font-bold text-foreground">
            {isArabic ? "معلومات التطبيق" : "App Info"}
          </Text>
        </Pressable>
      </View>
    </ScrollView>
  );
}

/* =========================================================================
 * Reusable Menu Item Component
 * ========================================================================= */

function ProfileMenuRow({
  icon,
  title,
  subtitle,
  trailingText,
  onPress,
  chevron: Chevron,
}: {
  icon: React.ReactNode;
  title: string;
  subtitle: string;
  trailingText?: string;
  onPress: () => void;
  chevron: any;
}) {
  return (
    <Pressable
      accessibilityRole="button"
      onPress={onPress}
      className="flex-row items-center justify-between py-2.5 px-1 rounded-2xl active:bg-secondary/50 transition-colors"
    >
      {/* Left Icon Container + Text Info */}
      <View className="flex-1 flex-row items-center gap-3.5 pr-2">
        {/* Circular Light Icon Container */}
        <View className="w-11 h-11 rounded-full bg-[#F4F4F6] dark:bg-secondary items-center justify-center">
          {icon}
        </View>

        {/* Title & Subtitle */}
        <View className="flex-1 justify-center gap-0.5">
          <Text
            numberOfLines={1}
            className="text-[15px] font-semibold text-foreground tracking-tight"
          >
            {title}
          </Text>
          <Text
            numberOfLines={1}
            className="text-[12px] font-normal text-muted-foreground"
          >
            {subtitle}
          </Text>
        </View>
      </View>

      {/* Right Trailing Text & Chevron */}
      <View className="flex-row items-center gap-2 pl-1">
        {trailingText ? (
          <Text className="text-xs font-medium text-muted-foreground">
            {trailingText}
          </Text>
        ) : null}
        <Chevron size={16} color="#A0A0AB" strokeWidth={2} />
      </View>
    </Pressable>
  );
}
