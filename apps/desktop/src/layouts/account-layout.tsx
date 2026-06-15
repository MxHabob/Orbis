import { Outlet } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { SecondaryMenu } from "@/components/secondary-menu";

/**
 * Account section layout — replaces Next.js account/layout.tsx
 * Preserves exact same DOM structure and CSS classes.
 */
export function AccountLayout() {
  const { t } = useTranslation();

  return (
    <div className="max-w-[800px]">
      <SecondaryMenu
        items={[
          { path: "/account", label: t("settings.general") },
          { path: "/account/support", label: t("settings.support") },
        ]}
      />

      <main className="mt-8">
        <Outlet />
      </main>
    </div>
  );
}
