import type { AccountDto, FolderDto, ProvisionUserRequest } from "$lib/api";
import { api } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import { orgContext } from "$lib/stores/org-context.svelte";

interface RegistrationArtifactOptions {
  accounts?: AccountDto[];
  accountListError?: string;
  studentFolder?: FolderDto;
}

export function getOrCreateStudentArchiveFolder(
  studentId: string,
  studentName: string,
): Promise<FolderDto> {
  return api.getOrCreateStudentFolder(studentId, studentName);
}

export async function ensureStudentRegistrationArtifacts(
  studentId: string,
  studentName: string,
  options: RegistrationArtifactOptions = {},
): Promise<{ warnings: string[]; studentFolder?: FolderDto; folderError?: string }> {
  const warnings: string[] = [];
  let studentFolder: FolderDto | undefined;
  let folderError: string | undefined;

  const [folderResult, accountResult] = await Promise.allSettled([
    options.studentFolder ?? getOrCreateStudentArchiveFolder(studentId, studentName),
    (async () => {
      if (options.accountListError) throw new Error(options.accountListError);
      const accounts = options.accounts ?? (await api.listAccounts());
      const codeSuffix = studentId.replace(/-/g, "").toUpperCase();
      const existingCode = accounts.find((account) =>
        account.code.endsWith(`-STU-${codeSuffix}`),
      );
      if (existingCode) return existingCode;

      const receivables = accounts.find(
        (account) =>
          account.code === "1103" ||
          account.name.toLowerCase() === "accounts receivable" ||
          account.name_ar?.trim() === "الحسابات المدينة",
      );
      if (!receivables) {
        throw new Error(m.student_registration_receivables_missing());
      }

      return api.createSubAccount({
        parent_id: receivables.id,
        code: `${receivables.code}-STU-${codeSuffix}`,
        name: studentName,
        name_ar: studentName,
        entity_type: "student",
        entity_id: studentId,
        currency_code: orgContext.baseCurrency || receivables.currency,
      });
    })(),
  ] as const);

  if (folderResult.status === "fulfilled") {
    studentFolder = folderResult.value;
  } else {
    folderError =
      folderResult.reason instanceof Error
        ? folderResult.reason.message
        : String(folderResult.reason);
  }
  if (accountResult.status === "rejected") {
    warnings.push(
      accountResult.reason instanceof Error
        ? accountResult.reason.message
        : String(accountResult.reason),
    );
  }

  return { warnings, studentFolder, folderError };
}

export async function provisionStudentLoginAccount(input: {
  studentId: string;
  fullNameAr: string;
  fullNameEn?: string;
  email?: string;
  phone?: string;
  groupIds?: string[];
  preferredUsername?: string;
}) {
  const request: ProvisionUserRequest = {
    student_id: input.studentId,
    actor_type: "student",
    full_name: input.fullNameEn?.trim() || input.fullNameAr.trim(),
    full_name_ar: input.fullNameAr.trim(),
    email: input.email?.trim() || undefined,
    phone: input.phone?.trim() || undefined,
    preferred_username: input.preferredUsername?.trim() || undefined,
    roles: ["student"],
    group_ids: input.groupIds ?? [],
    delivery_channel: input.email?.trim() ? "email" : input.phone?.trim() ? "sms" : "manual",
  };
  return api.provisionUser(request);
}
