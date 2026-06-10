"use client";

import {
  type Activity,
  getMetadata,
  getMetadataProperty,
} from "@/hooks/use-notifications";
import { useI18n } from "@/locales/client";
import { Button } from "@orbis/ui/button";
import { cn } from "@orbis/ui/cn";
import { Icons } from "@orbis/ui/icons";
import { formatDistanceToNow } from "date-fns";
import { getNotificationDescription } from "./notification-descriptions";
import { NotificationLink } from "./notification-link";
import { authClient } from "@orbis/auth/auth-client";
import { 
  GraduationCap, 
  BookOpen, 
  Calendar, 
  Megaphone, 
  MessageSquare,
  Repeat,
  FileText,
  CreditCard,
  Inbox,
  Users,
  Wallet,
  User,
  AlertTriangle,
  Settings
} from "lucide-react";

interface NotificationItemProps {
  id: number;
  setOpen: (open: boolean) => void;
  activity: Activity;
  markMessageAsRead?: (id: number) => void;
}

export function NotificationItem({
  id,
  setOpen,
  activity,
  markMessageAsRead,
}: NotificationItemProps) {
  const t = useI18n();
  const { data: session } = authClient.useSession();
  const user = session?.user;

  const recordId = getMetadataProperty(activity, "recordId");
  const metadata = getMetadata(activity);
  const isUnread = activity.status === "unread";

  const getNotificationIcon = (activityType: string) => {
    const iconProps = { className: "size-4 text-muted-foreground" };

    switch (activityType) {
      case "grade_published":
        return <GraduationCap {...iconProps} />;
      case "new_assessment":
        return <BookOpen {...iconProps} />;
      case "attendance_alert":
        return <Calendar {...iconProps} />;
      case "announcement":
        return <Megaphone {...iconProps} />;
      case "message":
        return <MessageSquare {...iconProps} />;
      case "parent_meeting":
        return <Users {...iconProps} />;
      case "fee_reminder":
        return <Wallet {...iconProps} />;
      case "account_action":
        return <User {...iconProps} />;
      case "behavior_alert":
        return <AlertTriangle {...iconProps} />;
      case "system":
        return <Settings {...iconProps} />;
      case "transactions_created":
        return <CreditCard {...iconProps} />;
      case "inbox_new":
        return <Inbox {...iconProps} />;
      default:
        if (activityType.startsWith("invoice_")) return <FileText {...iconProps} />;
        if (activityType.startsWith("recurring_")) return <Repeat {...iconProps} />;
        return <Icons.Notifications size={16} className="text-muted-foreground" />;
    }
  };

  const description = getNotificationDescription(activity, user, t);

  const notificationContent = (
    <div className="flex gap-3 w-full">
      <div className="relative shrink-0 mt-0.5">
        <div className="h-8 w-8 flex items-center justify-center border rounded-full bg-secondary/30">
          {getNotificationIcon(activity.type)}
        </div>
        {isUnread && (
          <span className="absolute -top-0.5 -right-0.5 flex h-2 w-2">
            <span className="relative inline-flex rounded-full h-2 w-2 bg-[#FFD02B]" />
          </span>
        )}
      </div>

      <div className="flex-1 min-w-0">
        <div className="flex flex-col">
          <p className={cn(
            "text-xs leading-normal break-words transition-colors",
            isUnread ? "font-medium text-foreground" : "font-normal"
          )}>
            {description}
          </p>
          
          {activity.message && activity.message !== description && (
            <p className="text-[11px] text-[#606060] line-clamp-2 leading-relaxed mt-0.5">
              {activity.message}
            </p>
          )}

          <div className="flex items-center gap-2 mt-1 px-0">
             <span className="text-[10px] text-[#606060]/70 font-medium">
                {t("notifications.time_ago", {
                  time: formatDistanceToNow(new Date(activity.createdAt)),
                })}
             </span>
          </div>
        </div>
      </div>
    </div>
  );

  const actionButton = markMessageAsRead && (
    <div className="shrink-0 self-start mt-0.5">
      <Button
        size="icon"
        variant="ghost"
        className="rounded-full h-7 w-7 opacity-0 group-hover:opacity-100 transition-opacity hover:bg-accent"
        onClick={(e) => {
          e.stopPropagation();
          markMessageAsRead(id);
        }}
        title={t("notifications.archive_button")}
      >
        <Icons.Inventory2 size={14} />
      </Button>
    </div>
  );

  return (
    <div className="group">
      <NotificationLink
        activityType={activity.type}
        recordId={recordId}
        metadata={metadata}
        onNavigate={() => setOpen(false)}
        className="flex-1 text-left outline-none"
        actionButton={actionButton}
      >
        {notificationContent}
      </NotificationLink>
    </div>
  );
}
