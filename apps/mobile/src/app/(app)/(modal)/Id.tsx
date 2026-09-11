import { Ionicons } from "@expo/vector-icons";
import { router } from "expo-router";
import { useMemo } from "react";
import { Pressable, StyleSheet, Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { Spacing } from "@/constants/theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useEffectiveContext } from "@/providers/effective-context";

export type IdCardInfo = {
  fullName: string;
  email?: string | null;
  orgName?: string;
  orgLogo?: string;
  role?: string;
  status?: string;
  userId?: string;
  studentNumber?: string;
  nationalId?: string;
  passportNumber?: string;
  mfaEnabled?: boolean;
  isVerified?: boolean;
};

function getInitials(name: string) {
  const words = name.trim().split(/\s+/).filter(Boolean);
  if (!words.length) return "OR";
  if (words.length === 1) return words[0].slice(0, 2).toUpperCase();
  return `${words[0][0]}${words[words.length - 1][0]}`.toUpperCase();
}

export function IdentityCard({ data }: { data: IdCardInfo }) {
  const theme = useTheme();
  const initials = getInitials(data.fullName || "Orbis");
  const status = (data.status || "ACTIVE").toUpperCase();
  const barcodeBars = [
    8, 4, 2, 6, 3, 8, 4, 2, 5, 2, 7, 3, 4, 6, 2, 9, 5, 2, 6, 3, 7, 2,
    4, 5, 3, 7, 8, 5, 2, 6, 4, 6, 3, 10, 4, 2, 5, 7, 3, 4, 9, 2, 6, 2,
    5, 8, 3, 6, 4, 7, 2, 5, 3, 8, 6,
  ];

  return (
    <View style={styles.sheetCard}>
      <View style={styles.cardHeader}>
        <View style={styles.cardHeaderLeft}>
          <Text style={[styles.brandText, { color: theme.text }]}>ORBIS</Text>
          <Text style={[styles.cardLabel, { color: theme.textMuted }]}>DIGITAL ID</Text>
        </View>

        <View
          style={[
            styles.statusBadge,
            {
              backgroundColor: "rgba(255,255,255,0.04)",
              borderColor: "rgba(255,255,255,0.1)",
            },
          ]}
        >
          <View style={[styles.statusDot, { backgroundColor: theme.primary }]} />
          <Text style={[styles.statusText, { color: theme.primary }]}>{status}</Text>
        </View>
      </View>

      <View style={styles.routeSection}>
        <View style={styles.routeBlock}>
          <Text style={[styles.routeLabel, { color: theme.textMuted }]}>FROM</Text>
          <Text style={[styles.routeValue, { color: theme.text }]}>{data.orgName || "ORBIS"}</Text>
        </View>

        <View style={styles.routeDivider}>
          <Ionicons name="arrow-forward" size={18} color={theme.textMuted} />
        </View>

        <View style={styles.routeBlock}>
          <Text style={[styles.routeLabel, { color: theme.textMuted }]}>TO</Text>
          <Text style={[styles.routeValue, { color: theme.text }]}>{data.role || "MEMBER"}</Text>
        </View>
      </View>

      <View style={styles.identityMain}>
        <View
          style={[
            styles.avatar,
            {
              backgroundColor: theme.background,
              borderColor: theme.border,
            },
          ]}
        >
          <Text style={[styles.avatarText, { color: theme.text }]}>{initials}</Text>
        </View>

        <View style={styles.userInfo}>
          <Text style={[styles.nameText, { color: theme.text }]}>{data.fullName}</Text>
          {!!data.email && (
            <Text style={[styles.emailText, { color: theme.textMuted }]}>{data.email}</Text>
          )}
        </View>
      </View>

      <View style={styles.dataGrid}>
        <View style={styles.dataCell}>
          <Text style={[styles.dataLabel, { color: theme.textMuted }]}>ID</Text>
          <Text style={[styles.dataValue, { color: theme.text }]}>{data.userId || "—"}</Text>
        </View>

        <View style={styles.dataCell}>
          <Text style={[styles.dataLabel, { color: theme.textMuted }]}>STATUS</Text>
          <Text style={[styles.dataValue, { color: theme.text }]}>{status}</Text>
        </View>

        <View style={styles.dataCell}>
          <Text style={[styles.dataLabel, { color: theme.textMuted }]}>NATIONAL ID</Text>
          <Text style={[styles.dataValue, { color: theme.text }]}>{data.nationalId || "—"}</Text>
        </View>

        <View style={styles.dataCell}>
          <Text style={[styles.dataLabel, { color: theme.textMuted }]}>PASSPORT</Text>
          <Text style={[styles.dataValue, { color: theme.text }]}>{data.passportNumber || "—"}</Text>
        </View>
      </View>

      <View style={styles.barcodeWrap}>
        <View style={styles.barcodeRow}>
          {barcodeBars.map((height, index) => (
            <View
              key={`${height}-${index}`}
              style={[
                styles.barcodeBar,
                { height, opacity: index % 2 === 0 ? 1 : 0.85 },
              ]}
            />
          ))}
        </View>
      </View>
    </View>
  );
}

export default function IdScreen() {
  const theme = useTheme();
  const insets = useSafeAreaInsets();
  const { user } = useAuth();
  const { data: orgData } = useCurrentOrgQuery();
  const { role } = useEffectiveContext();

  const userData = user as
    | {
        id?: string;
        full_name?: string;
        email?: string | null;
        status?: string;
        role?: string;
        org_id?: string;
        mfa_enabled?: boolean;
      }
    | undefined;

  const orgName =
    (orgData as { display_name?: string; name?: string } | undefined)?.display_name ||
    (orgData as { display_name?: string; name?: string } | undefined)?.name ||
    "Organization";

  const orgLogo =
    (orgData as { logo_url?: string; logo?: string } | undefined)?.logo_url ||
    (orgData as { logo_url?: string; logo?: string } | undefined)?.logo;

  const data = useMemo<IdCardInfo>(
    () => ({
      fullName:
        userData?.full_name?.trim() ||
        userData?.email?.split("@")[0] ||
        "Orbis User",
      email: userData?.email || undefined,
      orgName,
      orgLogo,
      role: role || userData?.role || "Member",
      status: userData?.status || "active",
      userId: userData?.id || "unknown",
      nationalId: userData?.id || "N/A",
      mfaEnabled: Boolean(userData?.mfa_enabled),
      isVerified: true,
    }),
    [orgName, orgLogo, role, userData],
  );

  return (
    <View
      style={[
        styles.overlay,
        {
          paddingTop: insets.top,
          paddingBottom: insets.bottom,
          backgroundColor: "rgba(0,0,0,0.5)",
        },
      ]}
    >
      <Pressable style={styles.backdrop} onPress={() => router.back()} />

      <View style={styles.sheetContainer}>
        <View style={styles.headerRow}>
          <Pressable onPress={() => router.back()} style={styles.closeButton}>
            <Ionicons name="chevron-back" size={22} color="#FFF" />
          </Pressable>

          <Text style={[styles.screenTitle, { color: "#FFF" }]}>Identity</Text>

          <View style={styles.spacer} />
        </View>

        <IdentityCard data={data} />
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  overlay: {
    flex: 1,
    justifyContent: "flex-end",
  },
  backdrop: {
    flex: 1,
  },
  sheetContainer: {
    paddingHorizontal: Spacing.five,
    paddingBottom: Spacing.five,
  },
  headerRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: Spacing.three,
    paddingHorizontal: Spacing.one,
  },
  closeButton: {
    width: 40,
    height: 40,
    borderRadius: 20,
    alignItems: "center",
    justifyContent: "center",
    backgroundColor: "rgba(255,255,255,0.08)",
  },
  spacer: {
    width: 40,
  },
  screenTitle: {
    fontSize: 18,
    fontWeight: "700",
    letterSpacing: 0.2,
  },
  sheetCard: {
    backgroundColor: "#111214",
    borderRadius: 28,
    borderWidth: 1,
    borderColor: "rgba(255,255,255,0.08)",
    overflow: "hidden",
    paddingHorizontal: Spacing.four,
    paddingTop: Spacing.three,
    paddingBottom: Spacing.four,
    minHeight: 420,
  },
  cardHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    marginBottom: Spacing.three,
  },
  cardHeaderLeft: {
    gap: 2,
  },
  brandText: {
    fontSize: 26,
    fontWeight: "800",
    letterSpacing: 1.5,
  },
  cardLabel: {
    fontSize: 10,
    letterSpacing: 1.8,
    fontWeight: "600",
  },
  statusBadge: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    paddingHorizontal: 10,
    paddingVertical: 6,
    borderRadius: 999,
    borderWidth: 1,
  },
  statusDot: {
    width: 8,
    height: 8,
    borderRadius: 999,
  },
  statusText: {
    fontSize: 10,
    fontWeight: "700",
    letterSpacing: 1,
    textTransform: "uppercase",
  },
  routeSection: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    borderRadius: 18,
    paddingVertical: Spacing.two,
    paddingHorizontal: Spacing.two,
    backgroundColor: "rgba(255,255,255,0.02)",
    marginBottom: Spacing.three,
  },
  routeBlock: {
    flex: 1,
  },
  routeDivider: {
    width: 32,
    alignItems: "center",
    justifyContent: "center",
  },
  routeLabel: {
    fontSize: 10,
    letterSpacing: 1,
    fontWeight: "600",
    marginBottom: 4,
  },
  routeValue: {
    fontSize: 18,
    fontWeight: "700",
  },
  identityMain: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.three,
    marginBottom: Spacing.three,
  },
  avatar: {
    width: 62,
    height: 62,
    borderRadius: 31,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  avatarText: {
    fontSize: 20,
    fontWeight: "700",
  },
  userInfo: {
    flex: 1,
  },
  nameText: {
    fontSize: 22,
    fontWeight: "700",
    marginBottom: 2,
  },
  emailText: {
    fontSize: 12,
    opacity: 0.8,
  },
  dataGrid: {
    flexDirection: "row",
    flexWrap: "wrap",
    gap: 10,
    marginBottom: Spacing.three,
  },
  dataCell: {
    width: "48%",
    paddingVertical: 12,
    paddingHorizontal: 10,
    borderRadius: 12,
    backgroundColor: "rgba(255,255,255,0.02)",
  },
  dataLabel: {
    fontSize: 10,
    letterSpacing: 1,
    fontWeight: "600",
    marginBottom: 4,
    textTransform: "uppercase",
  },
  dataValue: {
    fontSize: 13,
    fontWeight: "600",
  },
  barcodeWrap: {
    paddingTop: Spacing.two,
    borderTopWidth: 1,
    borderTopColor: "rgba(255,255,255,0.08)",
    overflow: "hidden",
  },
  barcodeRow: {
    flexDirection: "row",
    alignItems: "flex-end",
    justifyContent: "center",
    gap: 2,
    height: 52,
    paddingHorizontal: Spacing.two,
  },
  barcodeBar: {
    width: 3,
    borderRadius: 3,
    backgroundColor: "#F4F4F5",
  },
});
