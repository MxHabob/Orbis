/**
 * @module @orbis/mobile/app/(app)/(tabs)/index
 *
 * Primary Workspace & Hub Screen for Orbis Mobile.
 * Faithful implementation of the designed UI across 3 roles:
 * 1. Parent (ولي أمر) - "Your children" + Child shortcuts + School Update banner
 * 2. Student (طالب) - "My Learning" + Subject shortcuts + Continue Learning banner
 * 3. Teacher (أستاذ) - "My Classes" + Class shortcuts + Today's Teaching banner
 *
 * Note: Only the main content is rendered here. Top Header and Bottom TabBar are
 * handled by parent layout / navigation chrome.
 */

import React, { useState } from "react";
import {
  View,
  Text,
  ScrollView,
  Pressable,
  useWindowDimensions,
} from "react-native";
import { useRouter } from "expo-router";
import Svg, { Path, Circle as SvgCircle, G } from "react-native-svg";
import { useI18n } from "@/lib/i18n";
import { useAuth } from "@/lib/api";
import { useTheme } from "@/hooks/use-theme";
import {
  ChatBubbles,
  Info,
  Sun as SunSolid,
} from "@orbis/icons/native";
import {
  AssignmentChecklist,
  Assignments,
  Attendance,
  English,
  Grade7Math,
  Grade8Science,
  Grade10Math,
  Grades,
  Mathematics,
  Schedule,
  Science,
  StudentBoy,
  StudentGirl,
} from "@orbis/icons/native/Illustrations";

type RoleMode = "parent" | "student" | "teacher";

export default function HomeTabScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t, isArabic } = useI18n();
  const { user, effectiveContext } = useAuth();
  const { width } = useWindowDimensions();

  // Role resolution
  const initialRole: RoleMode = (() => {
    const roles = new Set(
      [
        ...(user?.roles ?? []),
        effectiveContext?.effective_role ?? "",
        ...(effectiveContext?.roles ?? []),
      ].map((r) => r.toLowerCase().replace(/[^a-z]/g, "")),
    );
    if ([...roles].some((r) => r.includes("teacher"))) {
      return "teacher";
    }
    if ([...roles].some((r) => r.includes("student"))) {
      return "student";
    }
    return "parent";
  })();

  const [activeRole, setActiveRole] = useState<RoleMode>(initialRole);

  return (
    <ScrollView
      className="flex-1 bg-background"
      showsVerticalScrollIndicator={false}
      contentContainerStyle={{ paddingBottom: 110 }}
      bounces={true}
    >
      {/* 0. Elegant Role Preview Switcher Bar */}
      <View className="bg-[#6B57A5] px-4 pt-1.5 pb-2 items-center">
        <View className="flex-row items-center bg-white/20 p-1 rounded-full border border-white/25">
          <Pressable
            accessibilityRole="button"
            onPress={() => setActiveRole("parent")}
            className={`px-3.5 py-1 rounded-full ${
              activeRole === "parent"
                ? "bg-white shadow-xs"
                : "bg-transparent"
            }`}
          >
            <Text
              className={`text-xs font-bold ${
                activeRole === "parent" ? "text-[#6B57A5]" : "text-white/85"
              }`}
            >
              {isArabic ? "ولي أمر" : "Parent"}
            </Text>
          </Pressable>

          <Pressable
            accessibilityRole="button"
            onPress={() => setActiveRole("student")}
            className={`px-3.5 py-1 rounded-full ${
              activeRole === "student"
                ? "bg-white shadow-xs"
                : "bg-transparent"
            }`}
          >
            <Text
              className={`text-xs font-bold ${
                activeRole === "student" ? "text-[#6B57A5]" : "text-white/85"
              }`}
            >
              {isArabic ? "طالب" : "Student"}
            </Text>
          </Pressable>

          <Pressable
            accessibilityRole="button"
            onPress={() => setActiveRole("teacher")}
            className={`px-3.5 py-1 rounded-full ${
              activeRole === "teacher"
                ? "bg-white shadow-xs"
                : "bg-transparent"
            }`}
          >
            <Text
              className={`text-xs font-bold ${
                activeRole === "teacher" ? "text-[#6B57A5]" : "text-white/85"
              }`}
            >
              {isArabic ? "أستاذ" : "Teacher"}
            </Text>
          </Pressable>
        </View>
      </View>

      {/* 1. Purple Canvas Container with Playful Accents & Circle Hub */}
      <View className="relative bg-[#6B57A5] overflow-hidden pt-2">
        {/* Top-Left Organic Lime Brushstrokes */}
        <View className="absolute top-2 left-3 pointer-events-none opacity-85">
          <Svg width={46} height={46} viewBox="0 0 46 46" fill="none">
            <Path
              d="M7 34C11 22 21 12 36 7"
              stroke="#A8DC6A"
              strokeWidth={4.5}
              strokeLinecap="round"
            />
            <Path
              d="M5 24C8 15 15 9 26 6"
              stroke="#A8DC6A"
              strokeWidth={3.5}
              strokeLinecap="round"
            />
            <Path
              d="M12 40C15 32 21 26 31 22"
              stroke="#A8DC6A"
              strokeWidth={3}
              strokeLinecap="round"
            />
          </Svg>
        </View>

        {/* Top-Right Hand-Drawn Sun Sketch */}
        <View className="absolute top-3 right-4 pointer-events-none opacity-85">
          <SunSolid size={34} color="#A8DC6A" strokeWidth={2} />
        </View>

        {/* Bottom-Right Above Wave Dash Strokes */}
        <View className="absolute bottom-12 right-7 pointer-events-none opacity-85">
          <Svg width={32} height={20} viewBox="0 0 32 20" fill="none">
            <Path
              d="M4 15L15 4"
              stroke="#A8DC6A"
              strokeWidth={3.5}
              strokeLinecap="round"
            />
            <Path
              d="M16 17L27 6"
              stroke="#A8DC6A"
              strokeWidth={3.5}
              strokeLinecap="round"
            />
          </Svg>
        </View>

        {/* Interactive Circular Items Area */}
        <View className="px-4 pt-1 pb-4 items-center">
          {activeRole === "parent" && <ParentHeroHub />}
          {activeRole === "student" && <StudentHeroHub />}
          {activeRole === "teacher" && <TeacherHeroHub />}
        </View>

        {/* 2. Organic Double Layered Wave Transition */}
        <View className="w-full">
          {/* Subtle translucent upper wave band */}
          <Svg
            width="100%"
            height={26}
            viewBox="0 0 375 26"
            preserveAspectRatio="none"
          >
            <Path
              d="M0,8 C110,24 250,2 375,18 L375,26 L0,26 Z"
              fill="rgba(255, 255, 255, 0.24)"
            />
          </Svg>

          {/* Solid transition wave matching bottom background */}
          <Svg
            width="100%"
            height={38}
            viewBox="0 0 375 38"
            preserveAspectRatio="none"
            style={{ marginTop: -10 }}
          >
            <Path
              d="M0,14 C120,40 240,4 375,22 L375,38 L0,38 Z"
              fill={theme.background}
            />
          </Svg>
        </View>
      </View>

      {/* 3. Main Content Section Below Wave */}
      <View className="px-4.5 pt-0.5 gap-4.5">
        {/* Section Header with Info Icon Button */}
        <View className="flex-row items-center justify-between px-0.5">
          <Text className="text-[22px] font-black tracking-tight text-foreground">
            {activeRole === "parent" && (isArabic ? "أبناؤك" : "Your children")}
            {activeRole === "student" && (isArabic ? "تعلّمي" : "My Learning")}
            {activeRole === "teacher" && (isArabic ? "فصولي الدراسية" : "My Classes")}
          </Text>

          <Pressable
            accessibilityRole="button"
            accessibilityLabel="Info"
            className="w-7 h-7 rounded-full items-center justify-center border border-border/80 bg-card active:bg-secondary"
          >
            <Info size={15} color={theme.mutedForeground} />
          </Pressable>
        </View>

        {/* Horizontal Mini Cards Carousel */}
        <View>
          <ScrollView
            horizontal
            showsHorizontalScrollIndicator={false}
            contentContainerClassName="flex-row gap-2.5 py-0.5 px-0.5"
          >
            {activeRole === "parent" && <ParentHorizontalCards />}
            {activeRole === "student" && <StudentHorizontalCards />}
            {activeRole === "teacher" && <TeacherHorizontalCards />}
          </ScrollView>

          {/* Pagination Dots (2 dots matching images) */}
          <View className="flex-row justify-center items-center gap-1.5 pt-3">
            <View className="h-2 w-4 rounded-full bg-[#6B57A5]" />
            <View className="h-2 w-2 rounded-full bg-muted-foreground/30" />
          </View>
        </View>

        {/* Bottom Feature Banner Card */}
        {activeRole === "parent" && <ParentBannerCard />}
        {activeRole === "student" && <StudentBannerCard />}
        {activeRole === "teacher" && <TeacherBannerCard />}
      </View>
    </ScrollView>
  );
}

/* =========================================================================
 * 1. HERO CIRCLE HUB COMPONENTS (Top Purple Canvas)
 * ========================================================================= */

/** Parent Top: Row 1 (Omar, Lina) | Row 2 (Attendance, Grades, Messages) */
function ParentHeroHub() {
  const router = useRouter();

  return (
    <View className="w-full items-center gap-4">
      {/* Row 1: Two children avatars */}
      <View className="flex-row justify-center items-center gap-6">
        <HeroCircleItem
          size={94}
          label="Omar"
          onPress={() => router.push("/(app)/student/omar")}
        >
          <StudentBoy size={54} />
        </HeroCircleItem>

        <HeroCircleItem
          size={94}
          label="Lina"
          onPress={() => router.push("/(app)/student/lina")}
        >
          <StudentGirl size={54} />
        </HeroCircleItem>
      </View>

      {/* Row 2: Attendance, Grades, Messages */}
      <View className="flex-row justify-center items-center gap-3.5">
        <HeroCircleItem
          size={82}
          label="Attendance"
          onPress={() => router.push("/(app)/student/omar/attendance")}
        >
          <Attendance size={44} />
        </HeroCircleItem>

        <HeroCircleItem
          size={82}
          label="Grades"
          onPress={() => router.push("/(app)/student/omar/grades")}
        >
          <Grades size={44} />
        </HeroCircleItem>

        <HeroCircleItem
          size={82}
          label="Messages"
          onPress={() => router.push("/(app)/notifications")}
        >
          <ChatBubbles size={42} color="#35285D" strokeWidth={1.75} />
        </HeroCircleItem>
      </View>
    </View>
  );
}

/** Student Top: Row 1 (Mathematics, Science) | Row 2 (English, Schedule, Assignments) | Row 3 (Grades) */
function StudentHeroHub() {
  const router = useRouter();

  return (
    <View className="w-full items-center gap-3.5">
      {/* Row 1: 2 Subjects */}
      <View className="flex-row justify-center items-center gap-6">
        <HeroCircleItem
          size={92}
          label="Mathematics"
          onPress={() => router.push("/(app)/student/me/grades")}
        >
          <Mathematics size={48} />
        </HeroCircleItem>

        <HeroCircleItem
          size={92}
          label="Science"
          onPress={() => router.push("/(app)/student/me/grades")}
        >
          <Science size={48} />
        </HeroCircleItem>
      </View>

      {/* Row 2: 3 Subject/Activity Tools */}
      <View className="flex-row justify-center items-center gap-3.5">
        <HeroCircleItem
          size={80}
          label="English"
          onPress={() => router.push("/(app)/student/me/grades")}
        >
          <English size={42} />
        </HeroCircleItem>

        <HeroCircleItem
          size={80}
          label="Schedule"
          onPress={() => router.push("/(app)/student/me/schedule")}
        >
          <Schedule size={42} />
        </HeroCircleItem>

        <HeroCircleItem
          size={80}
          label="Assignments"
          onPress={() => router.push("/(app)/student/me/exams")}
        >
          <AssignmentChecklist size={42} />
        </HeroCircleItem>
      </View>

      {/* Row 3: 1 Centered Grades Item */}
      <View className="flex-row justify-center items-center">
        <HeroCircleItem
          size={80}
          label="Grades"
          onPress={() => router.push("/(app)/student/me/grades")}
        >
          <Grades size={42} />
        </HeroCircleItem>
      </View>
    </View>
  );
}

/** Teacher Top: Row 1 (Grade 7 Math, Grade 8 Science) | Row 2 (Grade 9 English, Grade 10 Math, Assignments) */
function TeacherHeroHub() {
  const router = useRouter();

  return (
    <View className="w-full items-center gap-4">
      {/* Row 1: 2 Teaching Classes */}
      <View className="flex-row justify-center items-center gap-6">
        <HeroCircleItem
          size={92}
          label="Grade 7 Math"
          onPress={() => router.push("/(app)/class/7")}
        >
          <Grade7Math size={48} />
        </HeroCircleItem>

        <HeroCircleItem
          size={92}
          label="Grade 8 Science"
          onPress={() => router.push("/(app)/class/8")}
        >
          <Grade8Science size={48} />
        </HeroCircleItem>
      </View>

      {/* Row 2: 3 Items */}
      <View className="flex-row justify-center items-center gap-3.5">
        <HeroCircleItem
          size={80}
          label="Grade 9 English"
          onPress={() => router.push("/(app)/class/9")}
        >
          <English size={42} />
        </HeroCircleItem>

        <HeroCircleItem
          size={80}
          label="Grade 10 Math"
          onPress={() => router.push("/(app)/class/10")}
        >
          <Grade10Math size={42} />
        </HeroCircleItem>

        <HeroCircleItem
          size={80}
          label="Assignments"
          onPress={() => router.push("/(app)/class/assignments")}
        >
          <AssignmentChecklist size={42} />
        </HeroCircleItem>
      </View>
    </View>
  );
}

/** Circular Item with soft white surface, white halo border, and bottom pill badge */
function HeroCircleItem({
  size,
  label,
  children,
  onPress,
}: {
  size: number;
  label: string;
  children: React.ReactNode;
  onPress?: () => void;
}) {
  return (
    <Pressable
      onPress={onPress}
      className="items-center active:scale-95 transition-transform"
    >
      {/* Circle Container */}
      <View
        style={{ width: size, height: size, borderRadius: size / 2 }}
        className="items-center justify-center bg-white/95 border-[2.5px] border-white/80 shadow-md"
      >
        {children}
      </View>

      {/* Overlapping Pill Badge */}
      <View className="-mt-3 rounded-full bg-white px-3.5 py-0.5 shadow-xs border border-purple-200/50">
        <Text className="text-[12px] font-black text-[#2E2250]">{label}</Text>
      </View>
    </Pressable>
  );
}

/* =========================================================================
 * 2. HORIZONTAL MINI CARDS (Content Section)
 * ========================================================================= */

/** Parent Horizontal Cards: Omar, Lina, Attendance, Grades */
function ParentHorizontalCards() {
  const router = useRouter();

  return (
    <>
      <MiniCard
        label="Omar"
        onPress={() => router.push("/(app)/student/omar")}
      >
        <StudentBoy size={42} />
      </MiniCard>

      <MiniCard
        label="Lina"
        onPress={() => router.push("/(app)/student/lina")}
      >
        <StudentGirl size={42} />
      </MiniCard>

      <MiniCard
        label="Attendance"
        onPress={() => router.push("/(app)/student/omar/attendance")}
      >
        <Attendance size={40} />
      </MiniCard>

      <MiniCard
        label="Grades"
        onPress={() => router.push("/(app)/student/omar/grades")}
      >
        <Grades size={40} />
      </MiniCard>
    </>
  );
}

/** Student Horizontal Cards: Next Class, Homework, Latest Grade, Attendance */
function StudentHorizontalCards() {
  const router = useRouter();

  return (
    <>
      <MiniCard
        label="Next Class"
        onPress={() => router.push("/(app)/student/me/schedule")}
      >
        <Schedule size={42} />
      </MiniCard>

      <MiniCard
        label="Homework"
        onPress={() => router.push("/(app)/student/me/exams")}
      >
        <AssignmentChecklist size={40} />
      </MiniCard>

      <MiniCard
        label="Latest Grade"
        onPress={() => router.push("/(app)/student/me/grades")}
      >
        <Grades size={40} />
      </MiniCard>

      <MiniCard
        label="Attendance"
        onPress={() => router.push("/(app)/student/me/attendance")}
      >
        <Attendance size={40} />
      </MiniCard>
    </>
  );
}

/** Teacher Horizontal Cards: Attendance, Gradebook, Messages, Homework */
function TeacherHorizontalCards() {
  const router = useRouter();

  return (
    <>
      <MiniCard
        label="Attendance"
        onPress={() => router.push("/(app)/class/attendance")}
      >
        <Attendance size={42} />
      </MiniCard>

      <MiniCard
        label="Gradebook"
        onPress={() => router.push("/(app)/class/grades")}
      >
        <Grades size={40} />
      </MiniCard>

      <MiniCard
        label="Messages"
        onPress={() => router.push("/(app)/notifications")}
      >
        <ChatBubbles size={40} color="#35285D" strokeWidth={1.75} />
      </MiniCard>

      <MiniCard
        label="Homework"
        onPress={() => router.push("/(app)/class/homework")}
      >
        <Assignments size={40} />
      </MiniCard>
    </>
  );
}

function MiniCard({
  label,
  children,
  onPress,
}: {
  label: string;
  children: React.ReactNode;
  onPress?: () => void;
}) {
  return (
    <Pressable
      onPress={onPress}
      className="w-20 h-24 rounded-2xl bg-card border border-border/70 shadow-xs items-center justify-center p-2 active:bg-secondary/60"
    >
      <View className="flex-1 items-center justify-center">{children}</View>
      <Text
        numberOfLines={1}
        className="text-[11px] font-extrabold text-foreground text-center mt-1"
      >
        {label}
      </Text>
    </Pressable>
  );
}

/* =========================================================================
 * 3. BOTTOM FEATURE BANNER CARDS
 * ========================================================================= */

/** Parent Banner: "Omar is doing great!" + Backpack illustration with green accents */
function ParentBannerCard() {
  return (
    <View className="rounded-[24px] bg-[#F2EEFB] dark:bg-card-elevated border border-purple-200/50 p-4.5 flex-row items-center justify-between shadow-xs">
      <View className="flex-1 pr-3 gap-1">
        {/* Pill Badge */}
        <View className="flex-row items-center gap-1.5 self-start rounded-full bg-[#DDF4D8] px-2.5 py-1">
          <Text className="text-[10px]">📢</Text>
          <Text className="text-[10px] font-black tracking-wider text-[#2A7524] uppercase">
            SCHOOL UPDATE
          </Text>
        </View>

        {/* Title */}
        <Text className="text-lg font-black text-foreground pt-1">
          Omar is doing great!
        </Text>

        {/* Subtitle */}
        <Text className="text-xs font-medium text-muted-foreground leading-4">
          Attendance and learning progress this week
        </Text>
      </View>

      {/* Right Backpack Art with soft lavender circular container and sparkle doodles */}
      <View className="relative items-center justify-center">
        <View className="w-20 h-20 rounded-full bg-white/80 dark:bg-card items-center justify-center shadow-xs">
          <StudentBoy size={48} />
        </View>

        {/* Decorative sparkles */}
        <View className="absolute -left-1 top-2 pointer-events-none">
          <Svg width={12} height={12} viewBox="0 0 12 12" fill="none">
            <Path
              d="M6 1V11M1 6H11"
              stroke="#A8DC6A"
              strokeWidth={2}
              strokeLinecap="round"
            />
          </Svg>
        </View>
        <View className="absolute -right-1 bottom-3 pointer-events-none">
          <Svg width={10} height={10} viewBox="0 0 10 10" fill="none">
            <Path
              d="M5 1V9M1 5H9"
              stroke="#A8DC6A"
              strokeWidth={1.8}
              strokeLinecap="round"
            />
          </Svg>
        </View>
      </View>
    </View>
  );
}

/** Student Banner: "Continue Learning" with study illustration and sparkles */
function StudentBannerCard() {
  return (
    <View className="rounded-[24px] bg-[#F2EEFB] dark:bg-card-elevated border border-purple-200/50 p-5 flex-row items-center justify-between shadow-xs">
      <View className="flex-1 pr-3 gap-1">
        {/* Title */}
        <Text className="text-lg font-black text-foreground">
          Continue Learning
        </Text>

        {/* Subtitle */}
        <Text className="text-xs font-medium text-muted-foreground leading-4 pt-0.5">
          Pick up where you left off and keep making progress.
        </Text>
      </View>

      {/* Right Books Art */}
      <View className="relative items-center justify-center">
        <View className="w-20 h-20 rounded-full bg-white/80 dark:bg-card items-center justify-center shadow-xs">
          <Assignments size={48} />
        </View>

        {/* Decorative sparkles */}
        <View className="absolute -left-1 top-2 pointer-events-none">
          <Svg width={12} height={12} viewBox="0 0 12 12" fill="none">
            <Path
              d="M6 1V11M1 6H11"
              stroke="#A8DC6A"
              strokeWidth={2}
              strokeLinecap="round"
            />
          </Svg>
        </View>
        <View className="absolute -right-1 bottom-3 pointer-events-none">
          <Svg width={10} height={10} viewBox="0 0 10 10" fill="none">
            <Path
              d="M5 1V9M1 5H9"
              stroke="#A8DC6A"
              strokeWidth={1.8}
              strokeLinecap="round"
            />
          </Svg>
        </View>
      </View>
    </View>
  );
}

/** Teacher Banner: "Today's Teaching" with schedule illustration */
function TeacherBannerCard() {
  return (
    <View className="rounded-[24px] bg-[#F2EEFB] dark:bg-card-elevated border border-purple-200/50 p-5 flex-row items-center justify-between shadow-xs">
      <View className="flex-1 pr-3 gap-1">
        {/* Title */}
        <Text className="text-lg font-black text-foreground">
          Today's Teaching
        </Text>

        {/* Subtitle */}
        <Text className="text-xs font-bold text-foreground/80 pt-0.5">
          Grade 8 - Science
        </Text>

        {/* Time */}
        <Text className="text-[11px] font-medium text-muted-foreground">
          10:00 AM - 11:00 AM
        </Text>
      </View>

      {/* Right Classroom Art */}
      <View className="relative items-center justify-center">
        <View className="w-20 h-20 rounded-full bg-white/80 dark:bg-card items-center justify-center shadow-xs">
          <Schedule size={48} />
        </View>

        {/* Decorative sparkles */}
        <View className="absolute -left-1 top-2 pointer-events-none">
          <Svg width={12} height={12} viewBox="0 0 12 12" fill="none">
            <Path
              d="M6 1V11M1 6H11"
              stroke="#A8DC6A"
              strokeWidth={2}
              strokeLinecap="round"
            />
          </Svg>
        </View>
        <View className="absolute -right-1 bottom-3 pointer-events-none">
          <Svg width={10} height={10} viewBox="0 0 10 10" fill="none">
            <Path
              d="M5 1V9M1 5H9"
              stroke="#A8DC6A"
              strokeWidth={1.8}
              strokeLinecap="round"
            />
          </Svg>
        </View>
      </View>
    </View>
  );
}
