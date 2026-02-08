
import { getI18n } from "@/locales/server";
import { CertificateTemplateList } from "@/components/certificates/template-list";
import { Button } from "@midday/ui/button";
import { Plus } from "lucide-react";
import Link from "next/link";
import { Suspense } from "react";
import { listCertificateTemplates } from "@midday/db/queries/certificate-templates";

export const metadata = {
  title: "Certificate Templates | Midday",
};

export default async function CertificateTemplatesPage() {
  const t = await getI18n();
  const templates = await listCertificateTemplates();

  return (
    <div className="flex flex-col gap-8 p-8 max-w-[1200px] mx-auto w-full">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-end gap-4">
        <div>
          <h1 className="text-3xl font-bold font-cal">{t("certificates.templates")}</h1>
          <p className="text-muted-foreground mt-2">
            {t("certificates.templates_description")}
          </p>
        </div>
        
        <Link href="/certificates/create">
          <Button>
            <Plus className="mr-2 w-4 h-4" />
            {t("certificates.create_template")}
          </Button>
        </Link>
      </div>

      <Suspense fallback={<div>Loading...</div>}>
        <CertificateTemplateList initialData={templates} />
      </Suspense>
    </div>
  );
}
