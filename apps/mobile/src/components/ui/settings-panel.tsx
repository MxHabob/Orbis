import { Text, View } from "react-native";

export function SettingsPanelTitle({
  title,
  trailing,
}: {
  title: string;
  trailing?: string;
}) {
  return (
    <View className="min-h-11 flex-row items-center justify-between gap-three border-b border-border bg-secondary/50 px-four py-three">
      <Text className="text-sm font-medium text-foreground">{title}</Text>
      {trailing ? (
        <Text className="text-xs text-muted-foreground">{trailing}</Text>
      ) : null}
    </View>
  );
}

export function SettingsPanelDivider() {
  return <View className="h-px bg-border" />;
}
