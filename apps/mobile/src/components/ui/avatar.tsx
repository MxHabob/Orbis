import React, { useState } from "react";
import { Image, Text, View } from "react-native";
import { useTheme } from "@/hooks/use-theme";

interface AvatarProps {
  name: string;
  imageUrl?: string | null;
  size?: number;
}

function getInitials(name: string) {
  const words = name.trim().split(/\s+/).filter(Boolean);
  if (words.length === 0) return "?";
  if (words.length === 1) return Array.from(words[0]).slice(0, 2).join("");
  return `${Array.from(words[0])[0]}${Array.from(words[1])[0]}`;
}

export const Avatar = React.memo<AvatarProps>(
  ({ name, imageUrl, size = 32 }) => {
    const theme = useTheme();
    const [imageError, setImageError] = useState(false);
    const initials = getInitials(name);
    const showImage = Boolean(imageUrl) && !imageError;

    return (
      <View
        className="items-center justify-center overflow-hidden border"
        style={[
          {
            width: size,
            height: size,
            borderRadius: size / 2,
            backgroundColor: theme.brandPink,
            borderColor: theme.border,
          },
        ]}
      >
        {showImage ? (
          <Image
            source={{ uri: imageUrl! }}
            className="rounded-full"
            style={{ width: size, height: size, borderRadius: size / 2 }}
            resizeMode="cover"
            onError={() => setImageError(true)}
          />
        ) : (
          <Text
            className="text-center font-semibold"
            style={[
              { color: theme.primaryForeground, fontSize: size * 0.3 },
            ]}
          >
            {initials}
          </Text>
        )}
      </View>
    );
  },
);

Avatar.displayName = "Avatar";
