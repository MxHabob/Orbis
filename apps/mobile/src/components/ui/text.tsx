import React from "react";
import { Text as NativeText } from "react-native";

type TextProps = React.ComponentProps<typeof NativeText>;

export const Text = React.forwardRef<
  React.ElementRef<typeof NativeText>,
  TextProps
>(({ className, ...props }, ref) => {
  const hasColorToken =
    typeof className === "string" &&
    /(?:^|\s)(?:[\w-]+:)*text-(?:foreground|overlay|background(?:-(?:element|selected))?|card(?:-foreground|-elevated|-border)?|popover(?:-foreground)?|primary(?:-foreground|-light|-dark)?|secondary(?:-foreground)?|muted(?:-foreground)?|accent(?:-foreground)?|destructive(?:-foreground)?|border|input|ring|chart-[1-5]|sidebar(?:-[\w-]+)?|brand-[\w-]+|success(?:-light)?|warning(?:-light)?|error(?:-light)?|info(?:-light)?|btn-shadow-[\w-]+|tab-bar-[\w-]+|white|black|transparent|current|[a-z]+-\d{2,3}|\[(?:color:|#|var\(--(?:color-)?)[^\]]+\])(?:\s|$)/.test(
      className,
    );
  const hasFontFamily =
    typeof className === "string" &&
    /(?:^|\s)(?:[\w-]+:)*font-(?:sans|mono|serif|\[[^\]]+\])(?:\s|$)/.test(
      className,
    );
  const defaultClasses = [
    hasFontFamily ? "" : "font-sans",
    hasColorToken ? "" : "text-foreground",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <NativeText
      {...props}
      ref={ref}
      className={`${defaultClasses} ${className ?? ""}`}
    />
  );
});

Text.displayName = "Text";
