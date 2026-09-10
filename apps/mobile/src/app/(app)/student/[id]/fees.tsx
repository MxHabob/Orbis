/**
 * @module @orbis/mobile/app/(app)/student/[id]/fees
 *
 * Student Fees, Invoices, and Payment Processing Screen.
 */

import React, { useState } from "react";
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  Pressable,
  Modal,
  Alert,
} from "react-native";
import { useLocalSearchParams } from "expo-router";
import { Ionicons } from "@expo/vector-icons";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { useTheme } from "@/hooks/use-theme";
import { Header } from "@/components/header";
import { Card } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Radii, Spacing, Typography } from "@/constants/theme";

interface InvoiceItem {
  id: string;
  title: string;
  dueDate: string;
  amount: number;
  discount: number;
  netAmount: number;
  status: "Paid" | "Pending" | "Overdue";
}

export default function StudentFeesScreen() {
  const insets = useSafeAreaInsets();
  const theme = useTheme();
  const { id } = useLocalSearchParams<{ id: string }>();

  const [invoices, setInvoices] = useState<InvoiceItem[]>([
    {
      id: "inv-101",
      title: "الرسوم الدراسية - القسط الثاني 2026",
      dueDate: "30 سبتمبر 2026",
      amount: 4000,
      discount: 500,
      netAmount: 3500,
      status: "Pending",
    },
    {
      id: "inv-102",
      title: "رسوم الأنشطة والمعامل العلمية",
      dueDate: "15 سبتمبر 2026",
      amount: 600,
      discount: 0,
      netAmount: 600,
      status: "Pending",
    },
    {
      id: "inv-099",
      title: "الرسوم الدراسية - القسط الأول 2026",
      dueDate: "1 سبتمبر 2026",
      amount: 4000,
      discount: 500,
      netAmount: 3500,
      status: "Paid",
    },
  ]);

  const [selectedInvoice, setSelectedInvoice] = useState<InvoiceItem | null>(
    null,
  );
  const [paying, setPaying] = useState(false);

  const totalPending = invoices
    .filter((i) => i.status === "Pending" || i.status === "Overdue")
    .reduce((acc, i) => acc + i.netAmount, 0);

  const handlePay = async () => {
    if (!selectedInvoice) return;
    setPaying(true);
    setTimeout(() => {
      setInvoices(
        invoices.map((inv) =>
          inv.id === selectedInvoice.id ? { ...inv, status: "Paid" } : inv,
        ),
      );
      setPaying(false);
      setSelectedInvoice(null);
      Alert.alert(
        "تمت العملية بنجاح",
        "تم سداد الفاتورة وإصدار سند القبض الإلكتروني فوراً.",
      );
    }, 1000);
  };

  return (
    <View style={[styles.container, { backgroundColor: theme.background }]}>
      <Header title="الرسوم والمستحقات المالية" showBack />

      <ScrollView
        contentContainerStyle={[
          styles.scrollContent,
          { paddingBottom: insets.bottom + Spacing.eight },
        ]}
        showsVerticalScrollIndicator={false}
      >
        {/* Balance Card */}
        <Card variant="elevated" style={styles.balanceCard}>
          <Text style={[styles.balanceLabel, { color: theme.textSecondary }]}>
            إجمالي المستحقات المتبقية
          </Text>
          <Text
            style={[
              styles.balanceValue,
              { color: totalPending > 0 ? theme.primary : theme.success },
            ]}
          >
            {totalPending.toLocaleString()} ر.س
          </Text>
          <Text style={[styles.balanceSub, { color: theme.textSecondary }]}>
            {totalPending === 0
              ? "جميع الرسوم مسددة بالكامل • لا توجد مستحقات"
              : "يرجى السداد قبل انتهاء فترة الاستحقاق"}
          </Text>
        </Card>

        {/* Invoices List */}
        <View style={styles.sectionHeader}>
          <Text style={[styles.sectionTitle, { color: theme.text }]}>
            الفواتير والمطالبات
          </Text>
        </View>

        <View style={styles.invoicesList}>
          {invoices.map((inv) => {
            const isPaid = inv.status === "Paid";
            return (
              <Card key={inv.id} variant="elevated" style={styles.invoiceCard}>
                <View style={styles.invHeader}>
                  <View style={{ flex: 1 }}>
                    <Text style={[styles.invTitle, { color: theme.text }]}>
                      {inv.title}
                    </Text>
                    <Text
                      style={[
                        styles.invDueDate,
                        { color: theme.textSecondary },
                      ]}
                    >
                      تاريخ الاستحقاق: {inv.dueDate}
                    </Text>
                  </View>
                  <Badge
                    label={isPaid ? "مسدد" : "مستحق"}
                    variant={isPaid ? "success" : "warning"}
                    size="md"
                  />
                </View>

                <View
                  style={[
                    styles.invDetailsRow,
                    { backgroundColor: theme.backgroundElement },
                  ]}
                >
                  <View style={styles.detailCol}>
                    <Text
                      style={[styles.detailLbl, { color: theme.textSecondary }]}
                    >
                      المبلغ
                    </Text>
                    <Text style={[styles.detailVal, { color: theme.text }]}>
                      {inv.amount.toLocaleString()} ر.س
                    </Text>
                  </View>

                  {inv.discount > 0 && (
                    <View style={styles.detailCol}>
                      <Text
                        style={[
                          styles.detailLbl,
                          { color: theme.textSecondary },
                        ]}
                      >
                        الخصم
                      </Text>
                      <Text
                        style={[styles.detailVal, { color: theme.success }]}
                      >
                        -{inv.discount.toLocaleString()} ر.س
                      </Text>
                    </View>
                  )}

                  <View style={styles.detailCol}>
                    <Text
                      style={[styles.detailLbl, { color: theme.textSecondary }]}
                    >
                      الصافي
                    </Text>
                    <Text
                      style={[
                        styles.detailVal,
                        { color: theme.primary, fontWeight: "700" },
                      ]}
                    >
                      {inv.netAmount.toLocaleString()} ر.س
                    </Text>
                  </View>
                </View>

                {!isPaid && (
                  <Button
                    title="سداد الفاتورة الآن"
                    onPress={() => setSelectedInvoice(inv)}
                    size="sm"
                    icon={
                      <Ionicons name="card-outline" size={16} color="#FFFFFF" />
                    }
                    fullWidth
                    style={{ marginTop: 4 }}
                  />
                )}
              </Card>
            );
          })}
        </View>
      </ScrollView>

      {/* Payment Gateway Sheet */}
      <Modal visible={!!selectedInvoice} transparent animationType="slide">
        <View style={styles.modalOverlay}>
          <View
            style={[styles.modalContainer, { backgroundColor: theme.card }]}
          >
            <View style={styles.modalHeader}>
              <Text style={[styles.modalTitle, { color: theme.text }]}>
                بوابة الدفع الإلكتروني
              </Text>
              <Pressable onPress={() => setSelectedInvoice(null)}>
                <Ionicons name="close" size={24} color={theme.textSecondary} />
              </Pressable>
            </View>

            {selectedInvoice && (
              <View style={styles.modalBody}>
                <View
                  style={[
                    styles.paySummaryBox,
                    { backgroundColor: theme.backgroundElement },
                  ]}
                >
                  <Text style={[styles.payTitle, { color: theme.text }]}>
                    {selectedInvoice.title}
                  </Text>
                  <Text style={[styles.payAmount, { color: theme.primary }]}>
                    {selectedInvoice.netAmount.toLocaleString()} ر.س
                  </Text>
                </View>

                <Text
                  style={[
                    Typography.captionBold,
                    { color: theme.text, marginTop: 8 },
                  ]}
                >
                  اختر طريقة الدفع:
                </Text>

                <View style={styles.paymentMethods}>
                  {[
                    "مدى (Mada)",
                    "Apple Pay",
                    "بطاقة ائتمانية (Visa/MC)",
                    "تحويل بنكي",
                  ].map((m, idx) => (
                    <Pressable
                      key={idx}
                      style={[
                        styles.methodOption,
                        {
                          backgroundColor:
                            idx === 0
                              ? theme.primaryLight
                              : theme.backgroundElement,
                          borderColor: idx === 0 ? theme.primary : theme.border,
                        },
                      ]}
                    >
                      <Ionicons
                        name={idx === 1 ? "logo-apple" : "card"}
                        size={20}
                        color={idx === 0 ? theme.primary : theme.textSecondary}
                      />
                      <Text
                        style={[
                          styles.methodLabel,
                          { color: idx === 0 ? theme.primary : theme.text },
                        ]}
                      >
                        {m}
                      </Text>
                      {idx === 0 && (
                        <Ionicons
                          name="checkmark-circle"
                          size={18}
                          color={theme.primary}
                        />
                      )}
                    </Pressable>
                  ))}
                </View>

                <Button
                  title={`تأكيد وسداد ${selectedInvoice.netAmount.toLocaleString()} ر.س`}
                  onPress={handlePay}
                  loading={paying}
                  fullWidth
                  size="lg"
                  style={{ marginTop: Spacing.four }}
                />
              </View>
            )}
          </View>
        </View>
      </Modal>
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
    gap: Spacing.four,
  },
  balanceCard: {
    padding: Spacing.four,
    gap: 4,
  },
  balanceLabel: {
    ...Typography.caption,
  },
  balanceValue: {
    ...Typography.h1,
    marginTop: 2,
  },
  balanceSub: {
    ...Typography.small,
    marginTop: 4,
  },
  sectionHeader: {
    marginTop: Spacing.two,
  },
  sectionTitle: {
    ...Typography.h3,
  },
  invoicesList: {
    gap: Spacing.three,
  },
  invoiceCard: {
    padding: Spacing.four,
    gap: Spacing.three,
  },
  invHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "flex-start",
  },
  invTitle: {
    ...Typography.bodyBold,
  },
  invDueDate: {
    ...Typography.small,
    marginTop: 2,
  },
  invDetailsRow: {
    flexDirection: "row",
    justifyContent: "space-around",
    padding: Spacing.three,
    borderRadius: Radii.md,
  },
  detailCol: {
    alignItems: "center",
    gap: 2,
  },
  detailLbl: {
    ...Typography.small,
  },
  detailVal: {
    ...Typography.captionBold,
  },
  modalOverlay: {
    flex: 1,
    backgroundColor: "#00000088",
    justifyContent: "flex-end",
  },
  modalContainer: {
    borderTopLeftRadius: Radii.xl,
    borderTopRightRadius: Radii.xl,
    padding: Spacing.five,
  },
  modalHeader: {
    flexDirection: "row",
    justifyContent: "space-between",
    alignItems: "center",
    marginBottom: Spacing.four,
  },
  modalTitle: {
    ...Typography.h2,
  },
  modalBody: {
    gap: Spacing.three,
  },
  paySummaryBox: {
    padding: Spacing.four,
    borderRadius: Radii.md,
    alignItems: "center",
    gap: 4,
  },
  payTitle: {
    ...Typography.bodyBold,
  },
  payAmount: {
    ...Typography.h1,
  },
  paymentMethods: {
    gap: Spacing.two,
    marginTop: 4,
  },
  methodOption: {
    flexDirection: "row",
    alignItems: "center",
    padding: Spacing.three,
    borderRadius: Radii.md,
    borderWidth: 1.5,
    gap: Spacing.three,
  },
  methodLabel: {
    ...Typography.bodyBold,
    flex: 1,
  },
});
