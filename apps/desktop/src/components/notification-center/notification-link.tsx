import { useNavigate } from "react-router-dom";
import type { ReactNode } from "react";
import { useInboxParams } from "@/hooks/use-inbox-params";

const SUPPORTED_NOTIFICATION_TYPES = [
  "invoice_paid",
  "invoice_overdue",
  "invoice_created",
  "invoice_sent",
  "invoice_scheduled",
  "invoice_reminder_sent",
  "invoice_cancelled",
  "invoice_refunded",
  "transactions_created",
  "inbox_new",
  "inbox_needs_review",
  "inbox_auto_matched",
  "inbox_cross_currency_matched",
  "recurring_series_started",
  "recurring_series_completed",
  "recurring_series_paused",
  "recurring_invoice_upcoming",
  "grade_published",
  "new_assessment",
  "attendance_alert",
  "announcement",
  "message",
];

export function isNotificationClickable(activityType: string): boolean {
  return SUPPORTED_NOTIFICATION_TYPES.includes(activityType);
}

interface NotificationLinkProps {
  activityType: string;
  recordId: string | null | undefined;
  metadata?: Record<string, any>;
  onNavigate?: () => void;
  children: ReactNode;
  className?: string;
  actionButton?: ReactNode;
}

export function NotificationLink({
  activityType,
  recordId,
  metadata,
  onNavigate,
  children,
  className,
  actionButton,
}: NotificationLinkProps) {
  const { setParams: setInboxParams } = useInboxParams();
  const navigate = useNavigate();

  const isClickable = isNotificationClickable(activityType);

  const handleClick = () => {
    onNavigate?.();

    try {
      switch (activityType) {
        case "inbox_new":
          navigate("/inbox");
          break;

        case "inbox_needs_review":
        case "inbox_auto_matched":
        case "inbox_cross_currency_matched":
          if (metadata?.inboxId) {
            setInboxParams({ inboxId: metadata.inboxId, type: "details" });
          } else {
            navigate("/inbox");
          }
          break;

        case "grade_published":
          navigate("/grades");
          break;

        case "new_assessment":
          navigate("/assessments");
          break;

        case "attendance_alert":
          navigate("/attendance");
          break;

        case "announcement":
          navigate("/announcements");
          break;

        case "message":
          navigate("/messages");
          break;

        default:
          console.warn(`Unhandled notification type: ${activityType}`);
      }
    } catch (error) {
      console.error(`Error navigating for ${activityType}:`, error);
    }
  };

  if (isClickable) {
    return (
      <div className="flex items-center justify-between space-x-4 px-4 py-3 hover:bg-secondary/50 transition-colors">
        <button className={className} onClick={handleClick} type="button">
          {children}
        </button>
        {actionButton}
      </div>
    );
  }

  // Non-clickable notification
  return (
    <div className="flex items-center justify-between space-x-4 px-4 py-3">
      <div className="flex-1">{children}</div>
      {actionButton}
    </div>
  );
}
