import React from "react";
import { TextInput, View } from "react-native";
import { ArrowLeft, ArrowRight } from "@orbis/icons/native";
import { IconButton } from "@/components/ui/icon-button";
import { useColorScheme } from "@/hooks/use-color-scheme";
import { useTheme } from "@/hooks/use-theme";
import { useI18n } from "@/lib/i18n";

interface ChatComposerProps {
  value: string;
  onChangeText: (text: string) => void;
  onSend: () => void;
  placeholder: string;
  sendLabel: string;
  isSending: boolean;
}

export const ChatComposer = React.memo<ChatComposerProps>(
  ({ value, onChangeText, onSend, placeholder, sendLabel, isSending }) => {
    const theme = useTheme();
    const isDark = useColorScheme() === "dark";
    const { isArabic } = useI18n();
    const canSend = value.trim().length > 0 && !isSending;
    const SendIcon = isArabic ? ArrowLeft : ArrowRight;

    return (
      <View className="border-t border-border bg-card px-four py-2.5">
        <View
          className={`min-h-12 flex-row items-end gap-two rounded-md border border-border px-3 py-1.5 ${
            isDark ? "bg-background-element" : "bg-background"
          }`}
        >
          <TextInput
            className="max-h-28 flex-1 py-2 text-[15px] leading-5 text-foreground"
            style={{ textAlign: isArabic ? "right" : "left" }}
            placeholder={placeholder}
            placeholderTextColor={theme.textMuted}
            value={value}
            onChangeText={onChangeText}
            multiline
            editable={!isSending}
            returnKeyType="send"
            blurOnSubmit={false}
            onSubmitEditing={() => {
              if (canSend) onSend();
            }}
          />
          <IconButton
            icon={<SendIcon size={18} color={theme.primaryForeground} />}
            accessibilityLabel={sendLabel}
            onPress={onSend}
            disabled={!canSend}
            loading={isSending}
            size="md"
          />
        </View>
      </View>
    );
  },
);

ChatComposer.displayName = "ChatComposer";
