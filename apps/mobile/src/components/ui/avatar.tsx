import React, { useState } from "react";
import { Image, Text, View } from "react-native";

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
    const [imageError, setImageError] = useState(false);
    const initials = getInitials(name);
    const showImage = Boolean(imageUrl) && !imageError;

    return (
      <View
        className="items-center justify-center overflow-hidden rounded-full border border-border bg-brand-pink"
        style={[
          {
            width: size,
            height: size,
            borderRadius: size / 2,
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
            className="text-center font-semibold text-primary-foreground"
            style={{ fontSize: size * 0.3 }}
          >
            {initials}
          </Text>
        )}
      </View>
    );
  },
);

Avatar.displayName = "Avatar";
