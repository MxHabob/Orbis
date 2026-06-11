import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@orbis/ui/card";
import { ThemeSwitch } from "@/components/theme-switch";
import { useI18n } from "@/locales/client";

export function ChangeTheme() {
  const t = useI18n();

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t("account.theme_title")}</CardTitle>
        <CardDescription>{t("account.theme_description")}</CardDescription>
      </CardHeader>

      <CardContent>
        <div className="w-[240px]">
          <ThemeSwitch />
        </div>
      </CardContent>
    </Card>
  );
}
