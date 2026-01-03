// Re-export generated types
export * from "./generated/messages";

// Helper functions for working with messages

import type {
  Message,
  MessageContent,
  MessageRole,
  MessageStatus,
  ToolCall,
  ToolCallStatus,
} from "./generated/messages";

/**
 * Create a simple text message
 */
export function createTextMessage(
  role: MessageRole,
  text: string,
  status: MessageStatus = "complete",
): Message {
  return {
    id: crypto.randomUUID(),
    role,
    content: [{ type: "text", text }],
    timestamp: new Date().toISOString(),
    status,
  };
}

/**
 * Extract all text content from a message
 */
export function getMessageText(message: Message): string {
  return message.content
    .filter((c): c is { type: "text"; text: string } => c.type === "text")
    .map((c) => c.text)
    .join("");
}

/**
 * Extract all tool calls from a message
 */
export function getToolCalls(message: Message): ToolCall[] {
  return message.content
    .filter(
      (c): c is { type: "tool_use"; tool_call: ToolCall } =>
        c.type === "tool_use",
    )
    .map((c) => c.tool_call);
}

/**
 * Style configuration for each message role
 */
export const roleStyles: Record<
  MessageRole,
  {
    background: string;
    alignment: "left" | "right";
    textColor: string;
  }
> = {
  user: {
    background: "bg-surface-input",
    alignment: "right",
    textColor: "text-content",
  },
  assistant: {
    background: "",
    alignment: "left",
    textColor: "text-content",
  },
  system: {
    background: "bg-surface-hover",
    alignment: "left",
    textColor: "text-content-muted",
  },
  error: {
    background: "bg-red-900/50",
    alignment: "left",
    textColor: "text-red-300",
  },
};

/**
 * Style configuration for tool call statuses
 */
export const toolCallStatusStyles: Record<
  ToolCallStatus,
  {
    icon: string;
    color: string;
    bgColor: string;
  }
> = {
  pending: { icon: "○", color: "text-content-muted", bgColor: "bg-surface" },
  running: { icon: "◐", color: "text-yellow-400", bgColor: "bg-yellow-900/20" },
  success: { icon: "✓", color: "text-green-400", bgColor: "bg-green-900/20" },
  error: { icon: "✗", color: "text-red-400", bgColor: "bg-red-900/20" },
};
