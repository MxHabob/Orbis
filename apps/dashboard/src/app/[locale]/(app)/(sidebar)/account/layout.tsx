import { SecondaryMenu } from "@/components/secondary-menu";
import { getI18n } from "@/locales/server";

export default async function Layout({ children }: { children: React.ReactNode }) {
  const t = await getI18n();

  return (
    <div className="max-w-[800px]">
      <SecondaryMenu
        items={[
          { path: "/account", label: t("settings.general") },
        ]}
      />

      <main className="mt-8">{children}</main>
    </div>
  );
}
