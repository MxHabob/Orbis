import { useRouter } from "expo-router";
import { ActivityIndicator, ScrollView, Text, View } from "react-native";

import { FeatureCard } from "@/components/feature-card";
import { useI18n } from "@/lib/i18n";
import { useAuth } from "@/lib/api";
import { useClassesQuery } from "@/hooks/use-api-queries";

export default function HomeTabScreen() {
  const router = useRouter();
  const { t, isArabic } = useI18n();
  const { user, effectiveContext } = useAuth();

  const roles = new Set(
    [
      ...(user?.roles ?? []),
      effectiveContext?.effective_role ?? "",
      ...(effectiveContext?.roles ?? []),
    ].map((role) => role.toLowerCase().replace(/[^a-z]/g, "")),
  );
  const isGuardian = [...roles].some(
    (role) => role.includes("parent") || role.includes("guardian"),
  );
  const isTeacher = [...roles].some((role) => role.includes("teacher"));
  const isStudent = [...roles].some((role) => role.includes("student"));
  const classesQuery = useClassesQuery(isTeacher);
  const dependents = effectiveContext?.dependents ?? [];

  return (
    <ScrollView
      className="flex-1 bg-background"
      contentContainerClassName="gap-six px-five pb-[120px] pt-four"
    >
      <View className="gap-1">
        <Text className="text-sm font-medium text-muted-foreground">
          {t("home_welcome")}
        </Text>
        <Text className="text-2xl font-bold text-foreground">
          {isArabic ? user?.full_name_ar || user?.full_name : user?.full_name}
        </Text>
      </View>

      {isGuardian ? (
        <View className="gap-three">
          <Text className="text-lg font-bold text-foreground">
            {t("home_dependents")}
          </Text>
          {dependents.length > 0 ? (
            dependents.map((dependent) => (
              <FeatureCard
                key={dependent.student_id}
                title={
                  isArabic
                    ? dependent.full_name_ar || dependent.full_name
                    : dependent.full_name
                }
                description={[
                  dependent.current_grade_level,
                  dependent.current_section_name,
                ]
                  .filter(Boolean)
                  .join(" · ")}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/student/[id]",
                    params: { id: dependent.student_id },
                  })
                }
              />
            ))
          ) : (
            <Text className="rounded-xl bg-card px-four py-three text-sm leading-5 text-muted-foreground">
              {t("home_no_dependents")}
            </Text>
          )}
        </View>
      ) : null}

      {isTeacher ? (
        <View className="gap-three">
          <Text className="text-lg font-bold text-foreground">
            {t("home_classes")}
          </Text>
          {classesQuery.isPending ? (
            <ActivityIndicator />
          ) : classesQuery.isError ? (
            <View className="gap-2 rounded-xl border border-destructive/30 bg-card px-four py-three">
              <Text className="text-sm leading-5 text-destructive">
                {t("home_classes_error")}
              </Text>
              <Text
                accessibilityRole="button"
                onPress={() => void classesQuery.refetch()}
                className="font-semibold text-primary"
              >
                {t("home_retry")}
              </Text>
            </View>
          ) : classesQuery.data.length > 0 ? (
            classesQuery.data.map((classItem) => (
              <FeatureCard
                key={classItem.id}
                title={
                  isArabic
                    ? classItem.name_ar || classItem.name
                    : classItem.name
                }
                description={`${t("home_class_level")} ${classItem.level}`}
                onPress={() =>
                  router.push({
                    pathname: "/(app)/class/[id]",
                    params: { id: classItem.id },
                  })
                }
              />
            ))
          ) : (
            <Text className="rounded-xl bg-card px-four py-three text-sm leading-5 text-muted-foreground">
              {t("home_no_classes")}
            </Text>
          )}
        </View>
      ) : null}

      {isStudent ? (
        <View className="gap-three">
          <Text className="text-lg font-bold text-foreground">
            {t("home_services")}
          </Text>
          <FeatureCard
            title={t("student_profile")}
            description={t("student_grades")}
            onPress={() =>
              router.push({
                pathname: "/(app)/student/[id]",
                params: { id: "me" },
              })
            }
          />
        </View>
      ) : null}

      <View className="gap-three">
        <Text className="text-lg font-bold text-foreground">
          {t("home_services")}
        </Text>
        <FeatureCard
          title={t("home_notifications")}
          description={t("home_notifications_description")}
          onPress={() => router.push("/(app)/notifications")}
        />
        <FeatureCard
          title={t("nav_profile")}
          description={t("home_profile_description")}
          onPress={() => router.push("/(app)/profile")}
        />
      </View>
    </ScrollView>
  );
}
