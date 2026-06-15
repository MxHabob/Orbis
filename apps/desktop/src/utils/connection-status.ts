import type { RouterOutputs } from "@/ipc/query-adapter";
import { differenceInDays } from "date-fns";

const DISPLAY_DAYS = 30;
const WARNING_DAYS = 14;
const ERROR_DAYS = 7;

// Type from unified connectionStatus endpoint
type ConnectionStatusData = RouterOutputs["academic"]["connectionStatus"];

// Legacy types for backward compatibility
type BankConnection = any;
type InboxAccount = any;

/**
 * Unified connection issue type for school system integrations
 */
export type ConnectionIssue = {
  type: "noor" | "whatsapp" | "sms" | "bank" | "inbox";
  severity: "error" | "warning";
  title: string;
  message: string;
  path: string;
  linkText: string;
  logoUrl?: string | null;
  provider?: "gmail" | "outlook" | "whatsapp" | "sms" | "noor";
};

/**
 * Get all connection issues from bank connections (Legacy support)
 */
export function getBankIssues(
  connections: BankConnection[] | undefined,
): ConnectionIssue[] {
  return [];
}

/**
 * Get all connection issues from inbox accounts (Legacy support)
 */
export function getInboxIssues(
  accounts: InboxAccount[] | undefined,
): ConnectionIssue[] {
  return [];
}

/**
 * Get the highest severity from a list of issues
 */
export function getHighestSeverity(
  issues: ConnectionIssue[],
): "error" | "warning" | null {
  if (issues.length === 0) return null;
  if (issues.some((issue) => issue.severity === "error")) return "error";
  if (issues.some((issue) => issue.severity === "warning")) return "warning";
  return null;
}

/**
 * Build connection issues from unified connectionStatus endpoint data
 */
export function buildConnectionIssues(
  data: ConnectionStatusData | undefined,
): ConnectionIssue[] {
  if (!data) return [];

  const issues: ConnectionIssue[] = [];

  // Process system integrations (Noor, WhatsApp, SMS)
  if (data.integrations) {
    for (const integration of data.integrations) {
      if (integration.status === "disconnected") {
        issues.push({
          type: integration.type as any,
          severity: "error",
          title: integration.name,
          message: "There is a connection problem",
          path: "/settings/integrations",
          linkText: "Re-link",
          provider: integration.type as any,
        });
      } else if (integration.status === "warning") {
        issues.push({
          type: integration.type as any,
          severity: "warning",
          title: integration.name,
          message: "System alert",
          path: "/settings/integrations",
          linkText: "Details",
          provider: integration.type as any,
        });
      }
    }
  }

  return issues;
}

// Legacy functions for backward compatibility
type Connection = BankConnection;

export function getConnectionsStatus(connections: Connection[]) {
  const warning = connections?.some(
    (connection) =>
      connection.expiresAt &&
      differenceInDays(new Date(connection.expiresAt), new Date()) <=
        WARNING_DAYS,
  );

  const error = connections?.some(
    (connection) =>
      connection.expiresAt &&
      differenceInDays(new Date(connection.expiresAt), new Date()) <=
        ERROR_DAYS,
  );

  const expired = connections?.some(
    (connection) =>
      connection.expiresAt &&
      differenceInDays(new Date(connection.expiresAt), new Date()) <= 0,
  );

  const show = connections?.some(
    (connection) =>
      connection.expiresAt &&
      differenceInDays(new Date(connection.expiresAt), new Date()) <=
        DISPLAY_DAYS,
  );

  return {
    warning,
    expired,
    error,
    show,
  };
}

export function connectionStatus(connection: Connection) {
  const warning =
    connection.expiresAt &&
    differenceInDays(new Date(connection.expiresAt), new Date()) <=
      WARNING_DAYS;

  const error =
    connection.expiresAt &&
    differenceInDays(new Date(connection.expiresAt), new Date()) <= ERROR_DAYS;

  const expired =
    connection.expiresAt &&
    differenceInDays(new Date(connection.expiresAt), new Date()) <= 0;

  const show =
    connection.expiresAt &&
    differenceInDays(new Date(connection.expiresAt), new Date()) <=
      DISPLAY_DAYS;

  return {
    warning,
    error,
    expired,
    show,
  };
}
