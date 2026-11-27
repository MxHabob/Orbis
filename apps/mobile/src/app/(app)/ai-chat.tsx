import React, { useCallback, useMemo, useRef, useState } from "react";
import { FlatList, View } from "react-native";
import { useRouter } from "expo-router";
import { XMark } from "@orbis/icons/native";
import { ChatComposer } from "@/components/ai/chat-composer";
import {
  ChatMessageBubble,
  type ChatMessageItem,
} from "@/components/ai/chat-message-bubble";
import { ChatSuggestions } from "@/components/ai/chat-suggestions";
import { Header } from "@/components/header";
import { Screen } from "@/components/layout/screen";
import { EmptyState } from "@/components/ui/empty-state";
import { IconButton } from "@/components/ui/icon-button";
import { LoadingIndicator } from "@/components/ui/loading-indicator";
import { Text } from "@/components/ui/text";
import {
  useAiChatMutation,
  useSystemSettingsQuery,
} from "@/hooks/use-api-queries";
import { useTheme } from "@/hooks/use-theme";
import type { AiMessage } from "@/lib/api";
import { useI18n } from "@/lib/i18n";

export default function AiChatScreen() {
  const router = useRouter();
  const theme = useTheme();
  const { t } = useI18n();
  const listRef = useRef<FlatList<ChatMessageItem>>(null);

  const settingsQuery = useSystemSettingsQuery();
  const isAiEnabled = settingsQuery.data?.ai?.mobile_chat_enabled !== false;

  const [messages, setMessages] = useState<ChatMessageItem[]>([]);
  const [draft, setDraft] = useState("");
  const chatMutation = useAiChatMutation();

  const suggestions = useMemo(
    () => [
      t("ai_chat_suggestion_attendance"),
      t("ai_chat_suggestion_schedule"),
      t("ai_chat_suggestion_approvals"),
    ],
    [t],
  );

  const sendPrompt = useCallback(
    async (rawText: string) => {
      const content = rawText.trim();
      if (!content || chatMutation.isPending) return;

      const userMsg: ChatMessageItem = {
        id: `user-${Date.now()}`,
        role: "user",
        content,
        createdAt: new Date().toISOString(),
      };

      const nextHistory = [...messages, userMsg];
      setMessages(nextHistory);
      setDraft("");

      const apiMessages: AiMessage[] = nextHistory
        .filter((msg) => !msg.isError)
        .map((msg) => ({
          role: msg.role,
          content: msg.content,
          created_at: msg.createdAt,
        }));

      try {
        const res = await chatMutation.mutateAsync({
          messages: apiMessages,
          ui_context: {
            route: "/(app)/ai-chat",
            screen: "ai_chat",
          },
        });
        const assistantMsg: ChatMessageItem = {
          id: `assistant-${Date.now()}`,
          role: "assistant",
          content: res.response,
          createdAt: new Date().toISOString(),
          executedTools: res.executed_tools,
          proposedActions: res.proposed_actions,
        };
        setMessages((prev) => [...prev, assistantMsg]);
      } catch (error: unknown) {
        const errorMessage =
          error instanceof Error ? error.message : t("common_error");
        setMessages((prev) => [
          ...prev,
          {
            id: `error-${Date.now()}`,
            role: "assistant",
            content: errorMessage,
            createdAt: new Date().toISOString(),
            isError: true,
          },
        ]);
      }
    },
    [chatMutation, messages, t],
  );

  const handleSend = useCallback(() => {
    void sendPrompt(draft);
  }, [draft, sendPrompt]);

  const handleClear = useCallback(() => {
    setMessages([]);
  }, []);

  const renderMessage = useCallback(
    ({ item }: { item: ChatMessageItem }) => (
      <ChatMessageBubble
        message={item}
        toolsLabel={t("ai_chat_tools_used")}
      />
    ),
    [t],
  );

  const clearAction =
    messages.length > 0 ? (
      <IconButton
        icon={<XMark size={18} color={theme.foreground} />}
        accessibilityLabel={t("ai_chat_clear")}
        variant="outline"
        size="md"
        onPress={handleClear}
      />
    ) : undefined;

  return (
    <View className="flex-1 bg-background">
      <Header
        title={t("ai_chat_title")}
        showBack
        onBack={() => router.back()}
        rightAction={clearAction}
      />

      <Screen edges={["bottom", "left", "right"]} keyboardAvoiding>
        {!isAiEnabled ? (
          <View className="flex-1 items-center justify-center p-six">
            <EmptyState title={t("ai_chat_disabled_title")} />
          </View>
        ) : (
          <View className="flex-1">
            {messages.length === 0 ? (
              <ChatSuggestions
                title={t("ai_chat_empty_title")}
                subtitle={t("ai_chat_empty_subtitle")}
                suggestions={suggestions}
                onSelect={(prompt) => void sendPrompt(prompt)}
              />
            ) : (
              <FlatList
                ref={listRef}
                data={messages}
                keyExtractor={(item) => item.id}
                renderItem={renderMessage}
                contentContainerClassName="gap-three p-four"
                keyboardShouldPersistTaps="handled"
                initialNumToRender={12}
                maxToRenderPerBatch={8}
                windowSize={7}
                removeClippedSubviews
                onContentSizeChange={() =>
                  listRef.current?.scrollToEnd({ animated: true })
                }
                ListFooterComponent={
                  chatMutation.isPending ? (
                    <View className="flex-row items-center gap-two pt-one">
                      <LoadingIndicator size="sm" />
                      <Text className="text-xs text-muted-foreground">
                        {t("ai_chat_thinking")}
                      </Text>
                    </View>
                  ) : undefined
                }
              />
            )}

            <ChatComposer
              value={draft}
              onChangeText={setDraft}
              onSend={handleSend}
              placeholder={t("ai_chat_placeholder")}
              sendLabel={t("ai_chat_send")}
              isSending={chatMutation.isPending}
            />
          </View>
        )}
      </Screen>
    </View>
  );
}
