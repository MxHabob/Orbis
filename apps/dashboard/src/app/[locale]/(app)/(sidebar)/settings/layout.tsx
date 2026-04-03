import { SecondaryMenu } from "@/components/secondary-menu";
import { getI18n } from "@/locales/server";

export default async function Layout({ children }: { children: React.ReactNode }) {
  const t = await getI18n();

  return (
    <div className="max-w-[800px]">
      <SecondaryMenu
        items={[
          { path: "/settings", label: t("settings.general") },
          { path: "/settings/classes", label: t("settings.classes") },
          { path: "/settings/subjects", label: t("settings.subjects") },
          { path: "/settings/staff", label: t("settings.staff") },
          { path: "/settings/promotions", label: t("menu.promotions") },
          { path: "/settings/notifications", label: t("settings.notifications") },
        ]}
      />

      <main className="mt-8">{children}</main>
    </div>
  );
}
