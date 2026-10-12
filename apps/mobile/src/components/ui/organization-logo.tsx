/**
 * @module @orbis/mobile/components/ui/organization-logo
 *
 * Official vector & image logo component for Orbis institutions and organizations.
 * Supports vector SVG rendering with exact monorepo brand geometry, custom color overrides,
 * dynamic image URLs, and polished glow / container styles.
 */

import React, { useState } from "react";
import {
  View,
  Image,
  type ViewStyle,
  type ImageSourcePropType,
} from "react-native";
import Svg, { Path } from "react-native-svg";
import { useTheme } from "@/hooks/use-theme";
import { useColorScheme } from "@/hooks/use-color-scheme";

interface OrganizationLogoProps {
  size?: number;
  logoUrl?: string | null;
  uri?: string | null;
  imageSource?: ImageSourcePropType;
  color?: string;
  dotColor?: string;
  withContainer?: boolean;
  containerStyle?: ViewStyle;
}

export const OrganizationLogo = React.memo<OrganizationLogoProps>(
  ({
    size = 48,
    logoUrl,
    uri,
    imageSource,
    color,
    dotColor = "#A821D9",
    withContainer = false,
    containerStyle,
  }) => {
    const theme = useTheme();
    const colorScheme = useColorScheme();
    const isDark = colorScheme === "dark";
    const [imageError, setImageError] = useState(false);

    // Primary arc color: in dark theme, high-contrast text/white or custom brand color
    const primaryColor = color || (isDark ? "#FFFFFF" : theme.primary);
    const resolvedDotColor = dotColor || "#A821D9";

    const resolvedLogoUrl = logoUrl ?? uri ?? null;

    // If an explicit image source or valid logoUrl is provided without error:
    const hasImage =
      !imageError && (Boolean(imageSource) || Boolean(resolvedLogoUrl));

    const logoContent = (
      <View
        className="items-center justify-center"
        pointerEvents="none"
        style={{
          width: size,
          height: size,
        }}
      >
        {hasImage ? (
          <Image
            source={imageSource || { uri: resolvedLogoUrl! }}
            style={{ width: size, height: size, borderRadius: size * 0.25 }}
            resizeMode="contain"
            onError={() => setImageError(true)}
          />
        ) : (
          <Svg width={size} height={size} viewBox="0 0 642 660" fill="none">
            {/* Primary Orbit/Arc Path */}
            <Path
              fillRule="evenodd"
              clipRule="evenodd"
              d="M266.088 4.14842C13.2491 48.4308 -88.4636 368.722 89.8783 558.989C266.569 747.489 572.837 659.084 633.009 402.221C653.563 314.469 638.492 266.298 591.748 270.345C565.093 272.658 552.235 289.064 552.235 320.761C552.235 509.135 362.999 628.727 208.025 538.302C-1.18046 416.23 85.0444 90.7106 326.588 90.7106C377.228 90.7106 393.966 28.5226 349.691 4.89411C338.035 -1.32301 299.284 -1.66652 266.088 4.14842Z"
              fill={primaryColor}
            />
            {/* Accent Satellite Dot Path */}
            <Path
              d="M461.208 126.279C450.979 89.5042 473.161 48.4644 509.122 37.6473C570.32 19.2473 622.788 82.9853 594.489 141.361C565.366 201.438 479.405 191.71 461.208 126.279Z"
              fill={resolvedDotColor}
            />
          </Svg>
        )}
      </View>
    );

    if (withContainer) {
      const containerSize = size + 36;
      return (
        <View
          className="items-center justify-center border-[1.5px]"
          style={[
            {
              width: containerSize,
              height: containerSize,
              borderRadius: 24,
              backgroundColor: isDark ? "#1C1929" : "#F5F3FF",
              borderColor: isDark
                ? "rgba(139, 92, 246, 0.3)"
                : "rgba(139, 92, 246, 0.2)",
              shadowColor: "#8B5CF6",
              shadowOffset: { width: 0, height: 6 },
              shadowOpacity: 0.25,
              shadowRadius: 16,
              elevation: 6,
            },
            containerStyle,
          ]}
        >
          <View
            className="items-center justify-center"
            style={{
              width: size + 16,
              height: size + 16,
              borderRadius: 18,
              backgroundColor: isDark
                ? "rgba(139, 92, 246, 0.18)"
                : "rgba(139, 92, 246, 0.1)",
            }}
          >
            {logoContent}
          </View>
        </View>
      );
    }

    return logoContent;
  },
);

OrganizationLogo.displayName = "OrganizationLogo";
