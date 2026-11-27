import type { DirectoryInstitution } from "@/lib/api/institution-discovery";
import type { InstitutionCardProps } from "@/components/institutions/institution-card";

export const isLocalDiscoveryAvailable = false;

interface LocalInstitutionDiscoveryProps {
  title: string;
  connectingId: string | null;
  selectedInstitution: DirectoryInstitution | null;
  onConnect: (institution: DirectoryInstitution) => void;
  institutionCardVariant?: InstitutionCardProps["variant"];
}

export function LocalInstitutionDiscovery(
  _props: LocalInstitutionDiscoveryProps,
) {
  return null;
}
