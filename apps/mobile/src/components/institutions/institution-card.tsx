import { Image } from "expo-image";
import { memo, useCallback, useState } from "react";
import { Pressable, Text, View } from "react-native";
import {
  BuildingOffice,
  CheckCircle,
  ChevronLeft,
  ChevronRight,
} from "@orbis/icons/native";
import type { DirectoryInstitution } from "@/lib/api/institution-discovery";
import { useI18n } from "@/lib/i18n";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { useTheme } from "@/hooks/use-theme";

export interface InstitutionCardProps {
  institution: DirectoryInstitution;
  name: string;
  isCurrent: boolean;
  isBusy?: boolean;
  disabled?: boolean;
  localLabel: string;
  currentLabel?: string;
  onPress: (institution: DirectoryInstitution) => void;
  variant?: "featured" | "row" | "linked";
  width?: number;
}

export const InstitutionCard = memo(function InstitutionCard({
  institution,
  name,
  isCurrent,
  isBusy = false,
  disabled = false,
  localLabel,
  currentLabel,
  onPress,
  variant = "row",
  width,
}: InstitutionCardProps) {
  const [failedLogoUrl, setFailedLogoUrl] = useState<string | null>(null);
  const theme = useTheme();
  const { isArabic } = useI18n();
  const isFeatured = variant === "featured";
  const isLinked = variant === "linked";
  const DirectionChevron = isArabic ? ChevronLeft : ChevronRight;
  const logoUrl = institution.logo_url;
  const initials = Array.from(name.trim())[0]?.toLocaleUpperCase() ?? "?";
  const handlePress = useCallback(
    () => onPress(institution),
    [institution, onPress],
  );

  return (
    <Pressable
      accessibilityRole="button"
      accessibilityLabel={`${name}, ${institution.deployment_name}`}
      accessibilityState={{
        disabled: disabled || isCurrent,
        selected: isCurrent,
        busy: isBusy,
      }}
      disabled={disabled || isCurrent}
      onPress={handlePress}
      style={({ pressed }) => [
        width ? { width } : undefined,
        pressed && !disabled ? { opacity: 0.88 } : undefined,
      ]}
    >
      <Card
        variant="default"
        padding={isFeatured ? "four" : "three"}
        className={`relative flex-row items-center rounded-2xl ${
          isCurrent
            ? "border-primary/40 bg-primary/5"
            : "border-border bg-card"
        } ${isFeatured ? "min-h-[112px]" : "min-h-[84px]"}`}
      >
        <View
          className={`items-center justify-center overflow-hidden rounded-2xl bg-background-element ${
            isFeatured ? "me-4 h-16 w-16" : "me-3 h-14 w-14"
          }`}
        >
          {logoUrl && failedLogoUrl !== logoUrl ? (
            <Image
              source={{ uri: logoUrl }}
              contentFit="contain"
              cachePolicy="memory-disk"
              transition={120}
              onError={() => setFailedLogoUrl(logoUrl)}
              style={
                isFeatured
                  ? { width: 52, height: 52 }
                  : { width: 44, height: 44 }
              }
            />
          ) : institution.logo_url ? (
            <BuildingOffice size={24} color={theme.primary} />
          ) : (
            <Text className="text-xl font-bold text-primary">{initials}</Text>
          )}
        </View>

        <View className="min-w-0 flex-1">
          <Text
            numberOfLines={2}
            className={`text-start font-semibold text-foreground ${
              isFeatured ? "text-lg leading-6" : "text-sm leading-5"
            }`}
          >
            {name}
          </Text>
          <Text
            numberOfLines={1}
            className="mt-1 text-start text-xs leading-4 text-muted-foreground"
          >
            {institution.deployment_name}
          </Text>
          {isCurrent && currentLabel ? (
            <Text className="mt-1 text-start text-xs font-medium text-primary">
              {currentLabel}
            </Text>
          ) : institution.is_local ? (
            <Badge
              label={localLabel}
              variant="primary"
              size="sm"
              className="mt-1"
            />
          ) : null}
        </View>

        <View className="ms-2 h-9 w-9 items-center justify-center">
          {isBusy ? (
            <LoadingIndicator size="sm" />
          ) : isCurrent ? (
            <CheckCircle size={20} color={theme.primary} />
          ) : isLinked ? (
            <DirectionChevron size={18} color={theme.mutedForeground} />
          ) : null}
        </View>
      </Card>
    </Pressable>
  );
});

InstitutionCard.displayName = "InstitutionCard";
