import React from "react";
import { Pressable, View } from "react-native";
import { Sparkles } from "@orbis/icons/native";
import { Text } from "@/components/ui/text";
import { useTheme } from "@/hooks/use-theme";

interface ChatSuggestionsProps {
  title: string;
  subtitle: string;
  suggestions: readonly string[];
  onSelect: (prompt: string) => void;
}

export const ChatSuggestions = React.memo<ChatSuggestionsProps>(
  ({ title, subtitle, suggestions, onSelect }) => {
    const theme = useTheme();

    return (
      <View className="flex-1 items-center justify-center gap-five px-five py-eight">
        <View className="items-center gap-two">
          <View className="h-12 w-12 items-center justify-center rounded-full border border-border bg-card">
            <Sparkles size={22} color={theme.primary} />
          </View>
          <Text className="text-center text-lg font-semibold text-foreground">
            {title}
          </Text>
          <Text className="max-w-[300px] text-center text-xs leading-5 text-muted-foreground">
            {subtitle}
          </Text>
        </View>

        <View className="w-full max-w-[360px] gap-two">
          {suggestions.map((prompt) => (
            <Pressable
              key={prompt}
              accessibilityRole="button"
              onPress={() => onSelect(prompt)}
              className="min-h-11 justify-center rounded-md border border-border bg-card px-four py-2.5 active:bg-secondary/50"
            >
              <Text className="text-start text-sm font-medium text-foreground">
                {prompt}
              </Text>
            </Pressable>
          ))}
        </View>
      </View>
    );
  },
);

ChatSuggestions.displayName = "ChatSuggestions";
