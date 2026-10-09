import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useCurrentOrgQuery,
  useFeesQuery,
  useLinkedStudentId,
} from "@/hooks/use-api-queries";
import { formatCurrency } from "@/lib/i18n";
import { useI18n } from "@/lib/i18n";
import type { TranslationKey } from "@/lib/i18n/types";
import { useLocalSearchParams } from "expo-router";
import { Text, View } from "react-native";

const feeStatusKeys: Record<string, TranslationKey> = {
  pending: "student_fee_pending",
  partially_paid: "student_fee_partially_paid",
  paid: "student_fee_paid",
  overdue: "student_fee_overdue",
  cancelled: "student_fee_cancelled",
  refunded: "student_fee_refunded",
};

export default function StudentFeesScreen() {
  const { id } = useLocalSearchParams<{ id: string }>();
  const requestedStudentId = typeof id === "string" ? id : undefined;
  const {
    studentId,
    isLinked,
    isResolving,
    retryIdentity,
  } = useLinkedStudentId(requestedStudentId);
  const feesQuery = useFeesQuery(studentId);
  const orgQuery = useCurrentOrgQuery();
  const { t, formatDate } = useI18n();

  const fees = [...(feesQuery.data ?? [])].sort((a, b) =>
    a.due_date.localeCompare(b.due_date),
  );
  const balanceDue = fees.reduce((total, fee) => {
    const amount = Number(fee.balance_due);
    return Number.isFinite(amount) ? total + amount : total;
  }, 0);
  const currency = orgQuery.data?.base_currency_code ?? "YER";

  return (
    <StudentRecordLayout
      title={t("student_fees")}
      isPending={isResolving || (isLinked && feesQuery.isPending)}
      isError={!isResolving && (!isLinked || feesQuery.isError)}
      errorDescription={isLinked ? undefined : t("student_access_denied")}
      onRetry={() => {
        if (isLinked) {
          void feesQuery.refetch();
          void orgQuery.refetch();
        } else {
          void retryIdentity();
        }
      }}
    >
      {fees.length === 0 ? (
        <EmptyState title={t("student_no_fees")} />
      ) : (
        <View className="gap-three">
          <Card variant="outline" padding="four">
            <View className="gap-one">
              <Text className="text-sm text-muted-foreground">
                {t("student_total_balance_due")}
              </Text>
              <Text className="text-2xl font-bold text-foreground">
                {formatCurrency(balanceDue, currency)}
              </Text>
            </View>
          </Card>
          {fees.map((fee) => {
            const statusKey =
              feeStatusKeys[fee.status.toLowerCase()] ?? "student_fee_pending";
            return (
              <Card key={fee.id} variant="outline" padding="four">
                <View className="gap-two">
                  <View className="flex-row items-center justify-between gap-three">
                    <Text className="text-base font-semibold text-foreground">
                      {t("student_fee")}
                    </Text>
                    <Text
                      className={`text-sm font-semibold ${
                        fee.status === "overdue"
                          ? "text-destructive"
                          : fee.status === "paid"
                            ? "text-primary"
                            : "text-muted-foreground"
                      }`}
                    >
                      {t(statusKey)}
                    </Text>
                  </View>
                  <FeeAmount
                    label={t("student_fee_amount")}
                    value={formatCurrency(Number(fee.net_amount), currency)}
                  />
                  <FeeAmount
                    label={t("student_fee_paid_amount")}
                    value={formatCurrency(Number(fee.paid_amount), currency)}
                  />
                  <FeeAmount
                    label={t("student_fee_balance")}
                    value={formatCurrency(Number(fee.balance_due), currency)}
                  />
                  <FeeAmount
                    label={t("student_fee_due_date")}
                    value={formatDate(fee.due_date)}
                  />
                </View>
              </Card>
            );
          })}
        </View>
      )}
    </StudentRecordLayout>
  );
}

function FeeAmount({ label, value }: { label: string; value: string }) {
  return (
    <View className="flex-row items-center justify-between gap-three">
      <Text className="text-sm text-muted-foreground">{label}</Text>
      <Text className="text-sm font-medium text-foreground">{value}</Text>
    </View>
  );
}
