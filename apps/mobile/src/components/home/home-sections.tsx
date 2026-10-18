import { memo, type ReactNode } from "react";
import { ScrollView, Text, View } from "react-native";
import { Card } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";
import type { HomeTileData } from "./types";

export const HomeSection = memo(function HomeSection({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  return (
    <View className="gap-three">
      <Text className="text-base font-semibold tracking-tight text-foreground">
        {title}
      </Text>
      {children}
    </View>
  );
});

export const QuickActionRow = memo(function QuickActionRow({
  items,
}: {
  items: HomeTileData[];
}) {
  return (
    <ScrollView
      horizontal
      nestedScrollEnabled
      showsHorizontalScrollIndicator={false}
      contentContainerClassName="gap-three px-one"
    >
      {items.map((item) => (
        <Card
          key={item.id}
          variant="interactive"
          padding="three"
          onPress={item.onPress}
          accessibilityRole="button"
          accessibilityLabel={item.title}
          style={{ width: 168, minHeight: 76 }}
        >
          <View className="flex-1 flex-row items-center gap-three">
            <View className="h-10 w-10 items-center justify-center rounded-md bg-primary/10">
              {item.icon}
            </View>
            <Text
              numberOfLines={2}
              className="flex-1 text-sm font-semibold leading-5 text-foreground"
            >
              {item.title}
            </Text>
          </View>
        </Card>
      ))}
    </ScrollView>
  );
});

export const HomeTileGrid = memo(function HomeTileGrid({
  items,
}: {
  items: HomeTileData[];
}) {
  return (
    <View className="flex-row flex-wrap justify-between gap-y-three">
      {items.map((item) => (
        <Card
          key={item.id}
          variant="interactive"
          padding="four"
          onPress={item.onPress}
          accessibilityRole="button"
          accessibilityLabel={item.title}
          style={{ width: "48%", minHeight: 108 }}
        >
          <View className="flex-1 justify-between gap-three">
            <View className="h-11 w-11 items-center justify-center rounded-md bg-primary/10">
              {item.icon}
            </View>
            <Text
              numberOfLines={1}
              className="text-sm font-semibold leading-5 text-foreground"
            >
              {item.title}
            </Text>
            {item.subtitle ? (
              <Text
                numberOfLines={1}
                className="text-xs leading-4 text-muted-foreground"
              >
                {item.subtitle}
              </Text>
            ) : null}
          </View>
        </Card>
      ))}
    </View>
  );
});

export const HomeTileSkeletonGrid = memo(function HomeTileSkeletonGrid() {
  return (
    <View className="flex-row flex-wrap justify-between gap-y-three">
      {["first", "second"].map((key) => (
        <Skeleton key={key} width="48%" height={108} borderRadius={8} />
      ))}
    </View>
  );
});
