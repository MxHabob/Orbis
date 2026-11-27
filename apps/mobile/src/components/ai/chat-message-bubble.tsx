import React from "react";
import { View } from "react-native";
import { Badge } from "@/components/ui/badge";
import { Text } from "@/components/ui/text";
import type { AiProposedAction, AiToolResult } from "@/lib/api";

export interface ChatMessageItem {
  id: string;
  role: "user" | "assistant";
  content: string;
  createdAt: string;
  executedTools?: AiToolResult[];
  proposedActions?: AiProposedAction[];
  isError?: boolean;
}

interface ChatMessageBubbleProps {
  message: ChatMessageItem;
  toolsLabel: string;
}

export const ChatMessageBubble = React.memo<ChatMessageBubbleProps>(
  ({ message, toolsLabel }) => {
    const isUser = message.role === "user";

    return (
      <View className={`w-full ${isUser ? "items-end" : "items-start"}`}>
        <View
          className={`max-w-[85%] gap-two rounded-md border px-3.5 py-2.5 ${
            isUser
              ? "border-primary bg-primary"
              : message.isError
                ? "border-destructive/40 bg-destructive/10"
                : "border-border bg-card"
          }`}
        >
          <Text
            selectable
            className={`text-start text-sm leading-5 ${
              isUser
                ? "text-primary-foreground"
                : message.isError
                  ? "text-destructive"
                  : "text-foreground"
            }`}
          >
            {message.content}
          </Text>

          {!isUser &&
          message.executedTools &&
          message.executedTools.length > 0 ? (
            <View className="flex-row flex-wrap items-center gap-one pt-half">
              <Text className="text-[11px] font-medium text-muted-foreground">
                {toolsLabel}:
              </Text>
              {message.executedTools.map((tool) => (
                <Badge
                  key={tool.tool_call_id}
                  variant={tool.is_error ? "destructive" : "secondary"}
                  label={tool.tool_name.replace(/_/g, " ")}
                  size="sm"
                />
              ))}
            </View>
          ) : null}

          {!isUser &&
          message.proposedActions &&
          message.proposedActions.length > 0 ? (
            <View className="gap-one border-t border-border pt-two">
              {message.proposedActions.map((action, index) => (
                <View
                  key={`${message.id}-action-${index}`}
                  className="rounded-sm bg-secondary/50 px-2.5 py-1.5"
                >
                  <Text className="text-start text-xs font-semibold text-foreground">
                    {action.title}
                  </Text>
                  <Text className="text-start text-[11px] text-muted-foreground">
                    {action.description}
                  </Text>
                </View>
              ))}
            </View>
          ) : null}
        </View>
      </View>
    );
  },
);

ChatMessageBubble.displayName = "ChatMessageBubble";
