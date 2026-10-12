import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useGradesQuery,
  useLinkedStudentId,
} from "@/hooks/use-api-queries";
import { useI18n } from "@/lib/i18n";
import { useLocalSearchParams } from "expo-router";
import { Text, View } from "react-native";

export default function StudentGradesScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    identityError,
    retryIdentity,
  } =
    useLinkedStudentId(requestedStudentId);
  const gradesQuery = useGradesQuery({ studentId });
  const { t, formatDate } = useI18n();
  const grades = [...(gradesQuery.data ?? [])].sort((a, b) =>
    b.created_at.localeCompare(a.created_at),
  );

  return (
    <StudentRecordLayout
      title={t("student_grades")}
      isPending={isResolving || (isLinked && gradesQuery.isPending)}
      isError={!isResolving && (!isLinked || gradesQuery.isError)}
      canRetry
      errorDescription={
        isLinked || isResolving || identityError
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() =>
        void (isLinked ? gradesQuery.refetch() : retryIdentity())
      }
    >
      {grades.length === 0 ? (
        <EmptyState title={t("student_no_grades")} />
      ) : (
        <View className="gap-three">
          {grades.map((grade) => {
            const score = Number(grade.score);
            const maxScore = Number(grade.max_score);
            const scoreText =
              Number.isFinite(score) && Number.isFinite(maxScore)
                ? `${score} / ${maxScore}`
                : grade.score;
            const percent =
              Number.isFinite(score) && Number.isFinite(maxScore) && maxScore > 0
                ? Math.round((score / maxScore) * 100)
                : undefined;

            return (
              <Card key={grade.id} variant="outline" padding="four">
                <View className="gap-two">
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="text-base font-semibold text-foreground">
                      {grade.letter_grade || t("student_grade_score")}
                    </Text>
                    {percent !== undefined ? (
                      <Text className="text-sm font-semibold text-primary">
                        {percent}%
                      </Text>
                    ) : null}
                  </View>
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="text-sm text-muted-foreground">
                      {t("student_grade_score")}
                    </Text>
                    <Text className="text-sm font-medium text-foreground">
                      {scoreText}
                    </Text>
                  </View>
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="text-sm text-muted-foreground">
                      {t("student_grade_date")}
                    </Text>
                    <Text className="text-sm text-foreground">
                      {formatDate(grade.created_at)}
                    </Text>
                  </View>
                </View>
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}
