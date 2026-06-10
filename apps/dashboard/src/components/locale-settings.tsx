"use client";

import { useUserMutation } from "@/hooks/use-user";
import { useI18n, useChangeLocale, languages } from "@/locales/client";
import { countries } from "@orbis/location/countries-intl";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { ComboboxDropdown } from "@orbis/ui/combobox-dropdown";
import { authClient } from "@orbis/auth/auth-client";

export function LocaleSettings() {
  const t = useI18n();
  const changeLocale = useChangeLocale();
  const { data: session } = authClient.useSession();
  const updateUserMutation = useUserMutation();

  const localeItems = Object.values(countries).map((c, index) => ({
    id: index.toString(),
    label: `${c.name} (${c.default_locale})`,
    value: c.default_locale,
  }));

  return (
    <Card className="flex justify-between items-center">
      <CardHeader>
        <CardTitle>{t("locale.title")}</CardTitle>
        <CardDescription>{t("locale.description")}</CardDescription>
      </CardHeader>

      <CardContent>
        <div className="w-[250px]">
          <ComboboxDropdown
            placeholder={t("locale.placeholder")}
            selectedItem={localeItems.find(
              (item) => item.value === session?.user.locale,
            )}
            searchPlaceholder={t("locale.searchPlaceholder")}
            items={localeItems}
            className="text-xs py-1"
            onSelect={(item) => {
              updateUserMutation.mutate({ locale: item.value });
              
              const langPart = item.value?.split('-')[0];
              if (langPart && languages.includes(langPart)) {
                changeLocale(langPart as any);
              }
            }}
          />
        </div>
      </CardContent>
    </Card>
  );
}
