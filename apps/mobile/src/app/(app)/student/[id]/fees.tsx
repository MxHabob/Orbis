/**
 * @module @orbis/mobile/app/(app)/student/[id]/fees
 *
 * Student Financial Ledger & Tuition Fee Statement for Orbis Mobile.
 * Shows balance due, payment progress, invoice statuses, and due dates.
 */

import React from "react";
import { View, Text } from "react-native";
import { useLocalSearchParams } from "expo-router";
import { StudentRecordLayout } from "@/components/student-record-layout";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { EmptyState } from "@/components/ui/empty-state";
import {
  useCurrentOrgQuery,
  useFeesQuery,
  useLinkedStudentId,
} from "@/hooks/use-api-queries";
import { formatCurrency, useI18n } from "@/lib/i18n";
import { useTheme } from "@/hooks/use-theme";
import type { TranslationKey } from "@/lib/i18n/types";
import { CreditCard, Banknotes } from "@orbis/icons/native";

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
  const theme = useTheme();
  const { t, formatDate } = useI18n();

  const requestedStudentId = typeof id === "string" ? id : undefined;
  const { studentId, isLinked, isResolving, identityMissing, retryIdentity } =
    useLinkedStudentId(requestedStudentId);

  const feesQuery = useFeesQuery(studentId);
  const orgQuery = useCurrentOrgQuery();

  const rawFees = feesQuery.data ?? [];
  const fees = [...rawFees].sort((a, b) =>
    a.due_date.localeCompare(b.due_date),
  );

  const balanceDue = fees.reduce((total, fee) => {
    const amount = Number(fee.balance_due);
    return Number.isFinite(amount) ? total + amount : total;
  }, 0);

  const currency = orgQuery.data?.base_currency_code ?? "USD";

  return (
    <StudentRecordLayout
      title={t("student_fees") || "Fees & Tuition"}
      showBack={true}
      isPending={isResolving || (isLinked && feesQuery.isPending)}
      isError={!isResolving && (!isLinked || feesQuery.isError)}
      canRetry={isLinked}
      errorTitle={identityMissing ? t("student_profile_missing") : undefined}
      errorDescription={
        isLinked || isResolving || identityMissing
          ? undefined
          : t("student_access_denied")
      }
      onRetry={() => {
        if (isLinked) {
          void feesQuery.refetch();
          void orgQuery.refetch();
        } else {
          void retryIdentity();
        }
      }}
    >
      {/* 1. Total Balance Due Banner */}
      <Card variant="default" padding="four">
        <View className="flex-row items-center justify-between">
          <View className="gap-half">
            <Text className="text-xs font-semibold text-muted-foreground">
              {t("student_total_balance_due") || "Total Balance Due"}
            </Text>
            <Text
              className={`text-3xl font-extrabold ${
                balanceDue > 0 ? "text-destructive" : "text-success"
              }`}
            >
              {formatCurrency(balanceDue, currency)}
            </Text>
          </View>

          <View className="h-12 w-12 items-center justify-center rounded-md bg-primary/10">
            <CreditCard size={28} color={theme.primary} />
          </View>
        </View>
      </Card>

      {/* 2. Fee Invoices Breakdown */}
      {fees.length === 0 ? (
        <View className="py-ten">
          <EmptyState
            title={t("student_no_fees") || "No Fees Found"}
            icon={<Banknotes size={32} color={theme.mutedForeground} />}
          />
        </View>
      ) : (
        <View className="gap-three">
          {fees.map((fee) => {
            const status = fee.status.toLowerCase();
            const statusKey = feeStatusKeys[status] ?? "student_fee_pending";
            const isOverdue = status === "overdue";
            const isPaid = status === "paid";

            const net = Number(fee.net_amount) || 0;
            const paid = Number(fee.paid_amount) || 0;
            const percentPaid = net > 0 ? Math.round((paid / net) * 100) : 0;

            return (
              <Card
                key={fee.id}
                variant="outline"
                padding="four"
                className="gap-two"
              >
                <View className="flex-row items-center justify-between pb-one">
                  <Text className="text-base font-bold text-foreground">
                    {t("student_fee") || "Tuition Invoice"}
                  </Text>

                  <Badge
                    variant={
                      isOverdue
                        ? "destructive"
                        : isPaid
                          ? "default"
                          : "secondary"
                    }
                    className={`px-two py-half ${isPaid ? "bg-success/20" : ""}`}
                  >
                    <Text
                      className={`text-[10px] font-bold ${
                        isOverdue
                          ? "text-destructive-foreground"
                          : isPaid
                            ? "text-success"
                            : "text-foreground"
                      }`}
                    >
                      {t(statusKey)}
                    </Text>
                  </Badge>
                </View>

                {/* Progress ratio */}
                <View className="h-1.5 w-full overflow-hidden rounded-full bg-secondary my-one">
                  <View
                    style={{ width: `${Math.min(percentPaid, 100)}%` }}
                    className={`h-full rounded-full ${
                      isPaid ? "bg-success" : "bg-primary"
                    }`}
                  />
                </View>

                {/* Detailed financial attributes */}
                <View className="gap-one border-t border-border/40 pt-two">
                  <FeeAmount
                    label={t("student_fee_amount") || "Total Amount"}
                    value={formatCurrency(net, currency)}
                  />
                  <FeeAmount
                    label={t("student_fee_paid_amount") || "Paid Amount"}
                    value={formatCurrency(paid, currency)}
                  />
                  <FeeAmount
                    label={t("student_fee_balance") || "Remaining Balance"}
                    value={formatCurrency(
                      Number(fee.balance_due) || 0,
                      currency,
                    )}
                    highlight={Number(fee.balance_due) > 0}
                  />
                  <FeeAmount
                    label={t("student_fee_due_date") || "Due Date"}
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

function FeeAmount({
  label,
  value,
  highlight = false,
}: {
  label: string;
  value: string;
  highlight?: boolean;
}) {
  return (
    <View className="flex-row items-center justify-between">
      <Text className="text-xs text-muted-foreground">{label}</Text>
      <Text
        className={`text-xs font-semibold ${
          highlight ? "text-destructive font-bold" : "text-foreground"
        }`}
      >
        {value}
      </Text>
    </View>
  );
}
