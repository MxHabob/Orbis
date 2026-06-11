"use client";

import { authClient } from "@orbis/auth/auth-client";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { useI18n } from "@/locales/client";
import { AvatarUpload } from "./avatar-upload";

export function UserAvatar() {
  const { data: session } = authClient.useSession();
  const user = session?.user;
  const t = useI18n();

  return (
    <Card>
      <div className="flex justify-between items-center pr-6 rtl:ps-6 rtl:pr-0">
        <CardHeader>
          <CardTitle>{t("account.avatar_title")}</CardTitle>
          <CardDescription>{t("account.avatar_description")}</CardDescription>
        </CardHeader>

        <AvatarUpload userId={user?.id!} avatarUrl={user?.image} />
      </div>
      <CardFooter>{t("account.avatar_optional")}</CardFooter>
    </Card>
  );
}
