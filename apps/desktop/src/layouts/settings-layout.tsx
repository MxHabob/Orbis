import { Outlet } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { SecondaryMenu } from "@/components/secondary-menu";

/**
 * Settings section layout — replaces Next.js settings/layout.tsx
 * Preserves exact same DOM structure and CSS classes.
 */
export function SettingsLayout() {
  const { t } = useTranslation();

  return (
    <div className="max-w-[800px]">
      <SecondaryMenu
        items={[
          { path: "/settings", label: t("settings.general") },
          { path: "/settings/classes", label: t("settings.classes") },
          { path: "/settings/subjects", label: t("settings.subjects") },
          { path: "/settings/staff", label: t("settings.staff") },
          { path: "/settings/promotions", label: t("settings.promotions") },
          { path: "/settings/control", label: t("settings.control") },
          {
            path: "/settings/notifications",
            label: t("settings.notifications"),
          },
        ]}
      />

      <main className="mt-8">
        <Outlet />
      </main>
    </div>
  );
}
