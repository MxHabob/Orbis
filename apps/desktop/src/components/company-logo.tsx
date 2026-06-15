"use client";

import { authClient } from "@/lib/auth-client";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui/avatar";
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { Spinner } from "@orbis/ui/spinner";
import { useRef } from "react";
import { useI18n } from "@/locales/client";

export function CompanyLogo() {
  const inputRef = useRef<HTMLInputElement>(null);
  const { data, isPending } = authClient.useSession();
  const t = useI18n();

  return (
    <Card>
      <div className="flex justify-between items-center pr-6 rtl:ps-6 rtl:pr-0">
        <CardHeader>
          <CardTitle>{t("settings.orbis_name_title")}</CardTitle>
          <CardDescription>
            {t("settings.orbis_name_description")}
          </CardDescription>
        </CardHeader>

        <Avatar
          className="rounded-none w-16 h-16 flex items-center justify-center bg-accent cursor-pointer"
          onClick={() => inputRef?.current?.click()}
        >
          {isPending ? (
            <Spinner className="h-4 w-4" />
          ) : (
            <>
              <AvatarImage
                src={data?.user?.image ?? undefined}
                alt={data?.user?.name ?? undefined}
                width={64}
                height={64}
              />
              <AvatarFallback>
                <span className="text-md">{data?.user?.name?.charAt(0)}</span>
              </AvatarFallback>
            </>
          )}

          <input
            ref={inputRef}
            type="file"
            style={{ display: "none" }}
            multiple={false}
            onChange={() => {}}
          />
        </Avatar>
      </div>
      <CardFooter>{t("account.avatar_optional")}</CardFooter>
    </Card>
  );
}
