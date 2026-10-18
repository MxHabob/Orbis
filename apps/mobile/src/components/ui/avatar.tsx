import React, { useState } from "react";
import { Image, Text, View } from "react-native";

export type AvatarSize = number | "sm" | "md" | "lg" | "xl";

export interface AvatarProps {
  name?: string | null;
  fallback?: string;
  imageUrl?: string | null;
  size?: AvatarSize;
  className?: string;
  style?: import("react-native").StyleProp<import("react-native").ViewStyle>;
  children?: React.ReactNode;
}

const sizeMap: Record<"sm" | "md" | "lg" | "xl", number> = {
  sm: 28,
  md: 36,
  lg: 48,
  xl: 80,
};

function getInitials(name?: string | null, fallback?: string) {
  if (fallback) return fallback.trim().slice(0, 2);
  if (!name || typeof name !== "string") return "?";
  const words = name.trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) return "?";
  if (words.length === 1) return Array.from(words[0]).slice(0, 2).join("");
  return `${Array.from(words[0])[0]}${Array.from(words[1])[0]}`;
}

export const Avatar = React.memo<AvatarProps>(
  ({ name, fallback, imageUrl, size = 32, className, style, children }) => {
    const [failedImageUrl, setFailedImageUrl] = useState<string | null>(null);
    const pixelSize = typeof size === "number" ? size : (sizeMap[size] ?? 36);
    const initials = getInitials(name, fallback);
    const showImage = Boolean(imageUrl) && imageUrl !== failedImageUrl;

    return (
      <View
        className={`items-center justify-center overflow-hidden rounded-full border border-border bg-brand-pink ${className ?? ""}`}
        style={[
          {
            width: pixelSize,
            height: pixelSize,
            borderRadius: pixelSize / 2,
          },
          style,
        ]}
      >
        {children !== undefined ? (
          children
        ) : showImage ? (
          <Image
            source={{ uri: imageUrl! }}
            className="rounded-full"
            style={{
              width: pixelSize,
              height: pixelSize,
              borderRadius: pixelSize / 2,
            }}
            resizeMode="cover"
            onError={() => setFailedImageUrl(imageUrl ?? null)}
          />
        ) : (
          <Text
            className="text-center font-semibold text-primary-foreground"
            style={{ fontSize: pixelSize * 0.3 }}
          >
            {initials}
          </Text>
        )}
      </View>
    );
  },
);

Avatar.displayName = "Avatar";
