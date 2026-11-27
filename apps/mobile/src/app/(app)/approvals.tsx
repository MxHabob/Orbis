import React, { useEffect, useMemo, useState } from "react";
import {
  FlatList,
  Pressable,
  RefreshControl,
  View,
} from "react-native";
import { useRouter } from "expo-router";
import {
  CheckCircle,
  ChevronLeft,
  ChevronRight,
  Search,
  User,
  XMark,
} from "@orbis/icons/native";
import { Header } from "@/components/header";
import { Screen } from "@/components/layout/screen";
import { Badge, type BadgeVariant } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { EmptyState } from "@/components/ui/empty-state";
import { Input } from "@/components/ui/input";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { SegmentedControl } from "@/components/ui/segmented-control";
import { Skeleton } from "@/components/ui/skeleton";
import { Text } from "@/components/ui/text";
import { useToast } from "@/components/ui/toast";
import {
  useApproveOverrideMutation,
  useOverrideRequestsQuery,
  useRejectOverrideMutation,
} from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth, type OverrideRequestDto } from "@/lib/api";
import { useI18n } from "@/lib/i18n";
import { useEffectiveContext } from "@/providers/effective-context";

type ApprovalTab = "pending" | "mine" | "history";
type PendingFilter = "all" | "admission" | "override";

const UUID_RE =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

function formatDisplayValue(val?: string | null): string {
  if (!val) return "";
  return UUID_RE.test(val.trim()) ? `${val.trim().slice(0, 8)}…` : val;
}

function statusBadgeVariant(status: string): BadgeVariant {
  switch (status) {
    case "approved":
    case "executed":
      return "lime";
    case "rejected":
      return "destructive";
    case "pending":
      return "warning";
    case "expired":
    case "cancelled":
      return "secondary";
    default:
      return "outline";
  }
}

function getTypeBadgeVariant(type: string): BadgeVariant {
  switch (type) {
    case "admission_approval":
      return "lime";
    case "capacity_override":
      return "warning";
    case "grade_change":
      return "default";
    case "prerequisite_bypass":
      return "destructive";
    case "discount_override":
      return "secondary";
    default:
      return "outline";
  }
}

export default function ApprovalsScreen() {
  const router = useRouter();
  const theme = useTheme();
  const toast = useToast();
  const { t, isArabic, formatDate } = useI18n();
  const { userId } = useAuth();
  const { hasAnyPermission } = useEffectiveContext();

  const canReadAll = hasAnyPermission([
    "workflow:read",
    "workflow:approve",
    "workflow:manage",
    "*",
  ]);
  const canApprove = hasAnyPermission([
    "workflow:approve",
    "workflow:manage",
    "*",
  ]);

  const [selectedTab, setSelectedTab] = useState<ApprovalTab>(
    canReadAll ? "pending" : "mine",
  );
  const activeTab: ApprovalTab = canReadAll ? selectedTab : "mine";
  const [pendingFilter, setPendingFilter] = useState<PendingFilter>("all");
  const [searchQuery, setSearchQuery] = useState("");
  const [deferredSearch, setDeferredSearch] = useState("");
  const [expandedId, setExpandedId] = useState<string | null>(null);
  const [actionNotes, setActionNotes] = useState<Record<string, string>>({});

  useEffect(() => {
    const trimmed = searchQuery.trim();
    const timer = setTimeout(
      () => setDeferredSearch(trimmed),
      trimmed ? 250 : 0,
    );
    return () => clearTimeout(timer);
  }, [searchQuery]);

  const requestFilter = useMemo(
    () => ({
      status:
        activeTab === "pending"
          ? "pending"
          : activeTab === "history"
            ? "history"
            : undefined,
      mine: activeTab === "mine",
      search: deferredSearch || undefined,
    }),
    [activeTab, deferredSearch],
  );

  const requestsQuery = useOverrideRequestsQuery(requestFilter);
  const approveMutation = useApproveOverrideMutation();
  const rejectMutation = useRejectOverrideMutation();
  const [processingId, setProcessingId] = useState<string | null>(null);

  const requests = useMemo(
    () => requestsQuery.data?.pages.flatMap((page) => page.items) ?? [],
    [requestsQuery.data],
  );

  const handleLoadMore = () => {
    if (requestsQuery.hasNextPage && !requestsQuery.isFetchingNextPage) {
      void requestsQuery.fetchNextPage();
    }
  };

  const visibleRequests = useMemo(() => {
    let list =
      activeTab === "mine"
        ? requests
        : activeTab === "history"
          ? requests.filter((r) => r.status !== "pending")
          : requests.filter((r) => r.status === "pending");

    if (activeTab === "pending" && pendingFilter === "admission") {
      list = list.filter((r) => r.override_type === "admission_approval");
    } else if (activeTab === "pending" && pendingFilter === "override") {
      list = list.filter((r) => r.override_type !== "admission_approval");
    }

    return [...list].sort((a, b) => b.created_at.localeCompare(a.created_at));
  }, [activeTab, pendingFilter, requests]);

  const tabOptions = useMemo(
    () =>
      canReadAll
        ? [
            { value: "pending" as const, label: t("approvals_tab_pending") },
            { value: "mine" as const, label: t("approvals_tab_mine") },
            { value: "history" as const, label: t("approvals_tab_history") },
          ]
        : [{ value: "mine" as const, label: t("approvals_tab_mine") }],
    [canReadAll, t],
  );

  const getTypeLabel = (type: string) => {
    switch (type) {
      case "admission_approval":
        return t("approvals_type_admission");
      case "capacity_override":
        return t("approvals_type_capacity");
      case "grade_change":
        return t("approvals_type_grade_change");
      case "prerequisite_bypass":
        return t("approvals_type_prerequisite");
      case "late_enrollment":
        return t("approvals_type_late_enrollment");
      case "discount_override":
        return t("approvals_type_discount");
      default:
        return type.replace(/_/g, " ");
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case "pending":
        return t("approvals_status_pending");
      case "approved":
        return t("approvals_status_approved");
      case "rejected":
        return t("approvals_status_rejected");
      case "expired":
        return t("approvals_status_expired");
      case "executed":
        return t("approvals_status_executed");
      case "cancelled":
        return t("approvals_status_cancelled");
      default:
        return status;
    }
  };

  const canDecide = (request: OverrideRequestDto) =>
    canApprove &&
    request.status === "pending" &&
    request.requested_by !== userId;

  const handleDecision = async (
    request: OverrideRequestDto,
    action: "approve" | "reject",
  ) => {
    if (!canDecide(request) || processingId) return;
    setProcessingId(request.id);
    const notes = actionNotes[request.id]?.trim() || "";

    try {
      if (action === "approve") {
        await approveMutation.mutateAsync({
          id: request.id,
          notes: notes || undefined,
        });
        toast.success(t("approvals_approved_toast"));
      } else {
        await rejectMutation.mutateAsync({
          id: request.id,
          notes: notes || t("approvals_default_rejection_reason"),
        });
        toast.success(t("approvals_rejected_toast"));
      }
      setActionNotes((prev) => ({ ...prev, [request.id]: "" }));
      setExpandedId(null);
    } catch (error: unknown) {
      toast.error(
        error instanceof Error ? error.message : t("common_error"),
      );
    } finally {
      setProcessingId(null);
    }
  };

  const ChainChevron = isArabic ? ChevronLeft : ChevronRight;
  const hasFilters =
    Boolean(searchQuery.trim()) ||
    (activeTab === "pending" && pendingFilter !== "all");

  const emptyTitle = hasFilters
    ? t("approvals_empty_search")
    : activeTab === "mine"
      ? t("approvals_empty_mine")
      : activeTab === "history"
        ? t("approvals_empty_history")
        : t("approvals_empty_pending");

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("approvals_title")}
        showBack
        onBack={() => router.back()}
      />

      <Screen edges={["bottom", "left", "right"]}>
        <View className="gap-three border-b border-border bg-card px-four pb-three pt-two">
          {tabOptions.length > 1 ? (
            <SegmentedControl
              options={tabOptions}
              value={activeTab}
              onChange={(nextTab) => {
                setSelectedTab(nextTab);
                setExpandedId(null);
              }}
            />
          ) : null}

          {activeTab === "pending" && canReadAll ? (
            <View className="flex-row flex-wrap gap-two">
              {(
                [
                  { id: "all", label: t("approvals_filter_all") },
                  { id: "admission", label: t("approvals_filter_admission") },
                  { id: "override", label: t("approvals_filter_override") },
                ] as const
              ).map((filterItem) => {
                const selected = pendingFilter === filterItem.id;
                return (
                  <Pressable
                    key={filterItem.id}
                    accessibilityRole="button"
                    accessibilityState={{ selected }}
                    onPress={() => setPendingFilter(filterItem.id)}
                    className={`min-h-9 justify-center rounded-md border px-three ${
                      selected
                        ? "border-primary bg-primary/10"
                        : "border-border bg-background"
                    }`}
                  >
                    <Text
                      className={`text-xs font-medium ${
                        selected ? "text-primary" : "text-muted-foreground"
                      }`}
                    >
                      {filterItem.label}
                    </Text>
                  </Pressable>
                );
              })}
            </View>
          ) : null}

          <Input
            placeholder={t("approvals_search_placeholder")}
            value={searchQuery}
            onChangeText={setSearchQuery}
            icon={<Search size={18} color={theme.textMuted} />}
            returnKeyType="search"
            autoCapitalize="none"
          />
        </View>

        {requestsQuery.isPending ? (
          <View className="gap-three p-four">
            <Skeleton width="100%" height={108} borderRadius={6} />
            <Skeleton width="100%" height={108} borderRadius={6} />
            <Skeleton width="100%" height={108} borderRadius={6} />
          </View>
        ) : requestsQuery.isError && requests.length === 0 ? (
          <View className="flex-1 items-center justify-center p-six">
            <EmptyState
              title={t("approvals_error_title")}
              description={
                requestsQuery.error instanceof Error
                  ? requestsQuery.error.message
                  : undefined
              }
              actionTitle={t("common_retry")}
              onAction={() => void requestsQuery.refetch()}
            />
          </View>
        ) : visibleRequests.length === 0 ? (
          <View className="flex-1 items-center justify-center p-six">
            <EmptyState title={emptyTitle} />
          </View>
        ) : (
          <FlatList
            data={visibleRequests}
            keyExtractor={(item) => item.id}
            keyboardShouldPersistTaps="handled"
            contentContainerClassName="gap-three p-four pb-eight"
            initialNumToRender={10}
            maxToRenderPerBatch={10}
            windowSize={7}
            onEndReached={handleLoadMore}
            onEndReachedThreshold={0.4}
            ListFooterComponent={
              requestsQuery.isFetchingNextPage ? (
                <View className="items-center py-four">
                  <LoadingIndicator size="sm" />
                </View>
              ) : requestsQuery.isFetchNextPageError ? (
                <View className="items-center py-three">
                  <Button
                    title={t("common_retry")}
                    variant="outline"
                    size="sm"
                    onPress={handleLoadMore}
                  />
                </View>
              ) : undefined
            }
            refreshControl={
              <RefreshControl
                refreshing={requestsQuery.isRefetching}
                onRefresh={() => void requestsQuery.refetch()}
                tintColor={theme.primary}
                colors={[theme.primary]}
                progressBackgroundColor={theme.backgroundElement}
              />
            }
            renderItem={({ item: request }) => {
              const isExpanded =
                expandedId === request.id ||
                (visibleRequests.length === 1 && canDecide(request));
              const isProcessing = processingId === request.id;
              const canReview = canDecide(request);
              const typeLabel = getTypeLabel(request.override_type);
              const requesterDisplay =
                request.requested_by_name ||
                formatDisplayValue(request.requested_by);
              const approvalChain = request.approval_chain ?? [];

              return (
                <Card variant="default" padding="none">
                  <Pressable
                    accessibilityRole="button"
                    accessibilityState={{ expanded: isExpanded }}
                    onPress={() =>
                      setExpandedId((prev) =>
                        prev === request.id ? null : request.id,
                      )
                    }
                    className="gap-two p-four active:bg-secondary/40"
                  >
                    <View className="flex-row flex-wrap items-center justify-between gap-two">
                      <View className="flex-row flex-wrap items-center gap-one">
                        <Badge
                          variant={getTypeBadgeVariant(request.override_type)}
                          label={typeLabel}
                          size="sm"
                        />
                        <Badge
                          variant={statusBadgeVariant(request.status)}
                          label={getStatusLabel(request.status)}
                          size="sm"
                        />
                      </View>
                      <Text className="font-mono text-[11px] text-muted-foreground">
                        {formatDate(request.created_at)}
                      </Text>
                    </View>

                    <Text
                      numberOfLines={isExpanded ? undefined : 2}
                      className="text-start text-sm leading-5 font-medium text-foreground"
                    >
                      {request.reason || typeLabel}
                    </Text>

                    {activeTab !== "mine" && requesterDisplay ? (
                      <View className="flex-row items-center gap-one">
                        <User size={13} color={theme.mutedForeground} />
                        <Text
                          numberOfLines={1}
                          className="text-xs text-muted-foreground"
                        >
                          {requesterDisplay}
                        </Text>
                      </View>
                    ) : null}
                  </Pressable>

                  {isExpanded ? (
                    <View className="gap-three border-t border-border bg-secondary/20 p-four">
                      {request.field_name ||
                      request.old_value ||
                      request.new_value ? (
                        <View className="flex-row flex-wrap items-center gap-two rounded-md border border-border bg-background px-three py-two">
                          {request.field_name ? (
                            <Text className="text-xs font-semibold text-muted-foreground">
                              {request.field_name}
                            </Text>
                          ) : null}
                          {request.old_value ? (
                            <Text className="font-mono text-xs text-muted-foreground line-through">
                              {formatDisplayValue(request.old_value)}
                            </Text>
                          ) : null}
                          {request.old_value && request.new_value ? (
                            <Text className="text-xs text-muted-foreground">
                              →
                            </Text>
                          ) : null}
                          {request.new_value ? (
                            <Text className="font-mono text-xs font-semibold text-foreground">
                              {formatDisplayValue(request.new_value)}
                            </Text>
                          ) : null}
                        </View>
                      ) : null}

                      {approvalChain.length > 0 ? (
                        <View className="flex-row flex-wrap items-center gap-one">
                          {approvalChain.map((step, idx) => (
                            <React.Fragment key={`${request.id}-step-${step.step_order}`}>
                              <Badge
                                variant={statusBadgeVariant(step.status)}
                                label={step.approver_name || step.approver_role}
                                size="sm"
                              />
                              {idx < approvalChain.length - 1 ? (
                                <ChainChevron
                                  size={12}
                                  color={theme.mutedForeground}
                                />
                              ) : null}
                            </React.Fragment>
                          ))}
                        </View>
                      ) : null}

                      {request.resolution_notes ? (
                        <View className="rounded-md border border-border bg-background px-3 py-2">
                          <Text className="text-start text-xs text-muted-foreground">
                            <Text className="font-semibold text-foreground">
                              {t("approvals_resolution")}:{" "}
                            </Text>
                            {request.resolution_notes}
                          </Text>
                        </View>
                      ) : null}

                      {canReview ? (
                        <View className="gap-three pt-one">
                          <Input
                            placeholder={t("approvals_notes_placeholder")}
                            value={actionNotes[request.id] ?? ""}
                            onChangeText={(value) =>
                              setActionNotes((prev) => ({
                                ...prev,
                                [request.id]: value,
                              }))
                            }
                            editable={!isProcessing}
                            multiline
                          />
                          <View className="flex-row gap-two">
                            <View className="flex-1">
                              <Button
                                title={t("approvals_reject")}
                                variant="destructive"
                                size="md"
                                fullWidth
                                disabled={isProcessing}
                                icon={
                                  <XMark
                                    size={15}
                                    color={theme.destructiveForeground}
                                  />
                                }
                                onPress={() =>
                                  void handleDecision(request, "reject")
                                }
                              />
                            </View>
                            <View className="flex-1">
                              <Button
                                title={t("approvals_approve")}
                                variant="lime"
                                size="md"
                                fullWidth
                                loading={isProcessing}
                                disabled={isProcessing}
                                icon={
                                  <CheckCircle
                                    size={15}
                                    color={theme.brandLimeForeground}
                                  />
                                }
                                onPress={() =>
                                  void handleDecision(request, "approve")
                                }
                              />
                            </View>
                          </View>
                        </View>
                      ) : null}
                    </View>
                  ) : null}
                </Card>
              );
            }}
          />
        )}
      </Screen>
    </View>
  );
}
