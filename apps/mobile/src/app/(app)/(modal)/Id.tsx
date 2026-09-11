import { Ionicons } from "@expo/vector-icons";
import { useMemo } from "react";
import { StyleSheet, Text, View } from "react-native";
import { useSafeAreaInsets } from "react-native-safe-area-context";

import { Spacing } from "@/constants/theme";
import { useCurrentOrgQuery } from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import { useAuth } from "@/lib/api";
import { useEffectiveContext } from "@/providers/effective-context";
import Header from "@/components/header";

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

function Barcode({ value }: { value?: string }) {
  const theme = useTheme();

  const bars = useMemo(() => {
    const source = value || "ORBIS";

    return Array.from({ length: 48 }, (_, index) => {
      const charCode = source.charCodeAt(index % source.length);
      return 2 + ((charCode + index * 7) % 5);
    });
  }, [value]);

  return (
    <View
      style={[
        styles.barcodeCard,
        {
          backgroundColor: theme.background,
          borderColor: theme.border,
        },
      ]}
    >
      <View style={styles.barcode}>
        {bars.map((height, index) => (
          <View
            key={index}
            style={[
              styles.bar,
              {
                height: height * 5,
                backgroundColor: theme.text,
                opacity: index % 4 === 0 ? 0.65 : 1,
              },
            ]}
          />
        ))}
      </View>

      <Text
        style={[
          styles.barcodeLabel,
          {
            color: theme.textMuted,
          },
        ]}
      >
        DIGITAL ID
      </Text>
    </View>
  );
}

export function IdentityCard({ data }: { data: IdCardInfo }) {
  const theme = useTheme();

  const initials = getInitials(data.fullName || "Orbis");
  const status = (data.status || "ACTIVE").toUpperCase();

  const isActive =
    status === "ACTIVE" ||
    status === "VERIFIED" ||
    status === "ENABLED";

  return (
    <View
      style={[
        styles.identityCard,
        {
          backgroundColor: theme.background,
          borderColor: theme.border,
        },
      ]}
    >
      {/* Header */}
      <View style={styles.cardHeader}>
        <View>
          <Text
            style={[
              styles.brand,
              {
                color: theme.text,
              },
            ]}
          >
            ORBIS
          </Text>

          <Text
            style={[
              styles.cardTitle,
              {
                color: theme.textMuted,
              },
            ]}
          >
            DIGITAL ID
          </Text>
        </View>

        <View
          style={[
            styles.status,
            {
              backgroundColor: theme.primary + "08",
              borderColor: theme.border,
            },
          ]}
        >
          <View
            style={[
              styles.statusDot,
              {
                backgroundColor: isActive
                  ? theme.primary
                  : theme.textMuted,
              },
            ]}
          />

          <Text
            style={[
              styles.statusText,
              {
                color: isActive
                  ? theme.primary
                  : theme.textMuted,
              },
            ]}
          >
            {status}
          </Text>
        </View>
      </View>

      {/* Identity */}
      <View style={styles.identity}>
        <View
          style={[
            styles.avatar,
            {
              backgroundColor: theme.primary + "08",
              borderColor: theme.border,
            },
          ]}
        >
          <Text
            style={[
              styles.avatarText,
              {
                color: theme.primary,
              },
            ]}
          >
            {initials}
          </Text>
        </View>

        <View style={styles.identityInfo}>
          <Text
            numberOfLines={1}
            style={[
              styles.name,
              {
                color: theme.text,
              },
            ]}
          >
            {data.fullName}
          </Text>

          <Text
            numberOfLines={1}
            style={[
              styles.role,
              {
                color: theme.textMuted,
              },
            ]}
          >
            {data.role || "Member"}
          </Text>

          <Text
            numberOfLines={1}
            style={[
              styles.organization,
              {
                color: theme.textMuted,
              },
            ]}
          >
            {data.orgName || "Organization"}
          </Text>
        </View>
      </View>

      {/* ID */}
      <View
        style={[
          styles.idRow,
          {
            borderTopColor: theme.border,
          },
        ]}
      >
        <Text
          style={[
            styles.idLabel,
            {
              color: theme.textMuted,
            },
          ]}
        >
          ID
        </Text>

        <Text
          numberOfLines={1}
          style={[
            styles.idValue,
            {
              color: theme.text,
            },
          ]}
        >
          {data.userId || "—"}
        </Text>
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
    (orgData as
      | {
          display_name?: string;
          name?: string;
        }
      | undefined)?.display_name ||
    (orgData as
      | {
          display_name?: string;
          name?: string;
        }
      | undefined)?.name ||
    "Organization";

  const orgLogo =
    (orgData as
      | {
          logo_url?: string;
          logo?: string;
        }
      | undefined)?.logo_url ||
    (orgData as
      | {
          logo_url?: string;
          logo?: string;
        }
      | undefined)?.logo;

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
        styles.screen,
        {
          backgroundColor: theme.background,
          paddingTop: insets.top,
          paddingBottom: insets.bottom,
        },
      ]}
    >
      <View style={styles.container}>
        <Header title="Digital ID" showCrown={false} />

        <View style={styles.content}>
          <IdentityCard data={data} />

          <Barcode value={data.userId} />
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  screen: {
    flex: 1,
  },

  container: {
    flex: 1,
    paddingHorizontal: Spacing.five,
  },

  content: {
    flex: 1,
    justifyContent: "center",
    paddingBottom: Spacing.five,
    gap: Spacing.three,
  },

  identityCard: {
    width: "100%",
    borderWidth: 1,
    borderRadius: 20,
    padding: Spacing.four,
  },

  cardHeader: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
  },

  brand: {
    fontSize: 14,
    fontWeight: "800",
    letterSpacing: 1.5,
  },

  cardTitle: {
    marginTop: 2,
    fontSize: 8,
    fontWeight: "600",
    letterSpacing: 1,
  },

  status: {
    flexDirection: "row",
    alignItems: "center",
    gap: 6,
    paddingHorizontal: 9,
    paddingVertical: 6,
    borderRadius: 999,
    borderWidth: 1,
  },

  statusDot: {
    width: 6,
    height: 6,
    borderRadius: 3,
  },

  statusText: {
    fontSize: 9,
    fontWeight: "700",
    letterSpacing: 0.5,
  },

  identity: {
    flexDirection: "row",
    alignItems: "center",
    gap: Spacing.three,
    marginTop: Spacing.four,
  },

  avatar: {
    width: 64,
    height: 64,
    borderRadius: 18,
    borderWidth: 1,
    alignItems: "center",
    justifyContent: "center",
  },

  avatarText: {
    fontSize: 21,
    fontWeight: "700",
  },

  identityInfo: {
    flex: 1,
    minWidth: 0,
  },

  name: {
    fontSize: 20,
    lineHeight: 25,
    fontWeight: "700",
    letterSpacing: -0.3,
  },

  role: {
    marginTop: 3,
    fontSize: 12,
    fontWeight: "500",
  },

  organization: {
    marginTop: 2,
    fontSize: 11,
  },

  idRow: {
    flexDirection: "row",
    alignItems: "center",
    justifyContent: "space-between",
    gap: Spacing.three,
    marginTop: Spacing.four,
    paddingTop: Spacing.three,
    borderTopWidth: StyleSheet.hairlineWidth,
  },

  idLabel: {
    fontSize: 10,
    fontWeight: "600",
  },

  idValue: {
    flex: 1,
    textAlign: "right",
    fontSize: 11,
    fontWeight: "600",
  },

  barcodeCard: {
    width: "100%",
    minHeight: 92,
    borderWidth: 1,
    borderRadius: 20,
    paddingHorizontal: Spacing.four,
    paddingVertical: Spacing.three,
    alignItems: "center",
    justifyContent: "center",
  },

  barcode: {
    height: 38,
    flexDirection: "row",
    alignItems: "flex-end",
    justifyContent: "center",
    gap: 2,
    width: "100%",
  },

  bar: {
    width: 2,
    borderRadius: 1,
  },

  barcodeLabel: {
    marginTop: 7,
    fontSize: 8,
    fontWeight: "600",
    letterSpacing: 1.2,
  },
});