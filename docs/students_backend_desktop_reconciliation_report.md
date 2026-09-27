# تقرير المطابقة الشامل والمعماري لقسم الطلاب (Students Reconciliation Report)

> **الإصدار:** 2.0.0-CANONICAL  
> **حالة المطابقة:** `STUDENTS_RECONCILIATION_COMPLETED`  
> **تاريخ التحقق:** 2026-09-27  
> **مسار التحقق:** Rust Clean Architecture (`apps/api-rust`) ↔ Tauri IPC (`apps/desktop/src-tauri`) ↔ SDK (`packages/sdk`) ↔ Desktop Frontend (`apps/desktop`)

---

## 1. ملخص تنفيذي للمطابقة (Executive Summary)

تم إجراء تدقيق معماري وتطهير جذري شامل لجميع عقود وأنماط قسم الطلاب (Students)، والتأكد من أن **Backend / Domain Clean Architecture** هو المصدر الوحيد والحصري للحقيقة، مع تطابق بنسبة 100% عبر كامل السلسلة:
`Database -> Domain -> Application Service -> DTO -> Transport -> SDK -> Desktop API -> TanStack Query -> FormController -> Svelte Components`.

---

## 2. جدول مطابقة العقود والأنماط (Reconciliation Matrix)

| العقد القديم (Legacy Desktop) | عقد الباك إند الحقيقي (Backend Domain) | النوع المعتمد (Canonical TypeScript Type) | نمط الاستعلام / الأمر المعتمد | الإجراء المتخذ (Action) |
| :--- | :--- | :--- | :--- | :--- |
| `GuardianForm` مع `$state` متعدد وحقول مكررة | `AssignGuardianRequest` + ربط هوية أصلية | `AssignGuardianRequest` + `GuardianDto` | `api.assignGuardian(req)` | **REBUILD** (تحويل كامل لـ `FormController`) |
| `registerUser(role: 'parent')` وتوليد إيميلات وهمية | `StudentService::assign_guardian` | `AssignGuardianRequest` (ربط هوية قائمة) | `api.assignGuardian` | **REMOVE** (إلغاء إنشاء حسابات وهمية من واجهة الطالب) |
| `STU-${Date.now()}` | Sequence Generator / Numbering Scheme | `student_number?: string` | توليد ترقيم تسلسلي رسمي من الباك إند | **REPLACE** |
| `period_id: this.orgId` (fallback خاطئ) | `effectiveContext.currentPeriodId` | `string \| undefined` | استهلاك الفترة الأكاديمية الحقيقية | **REPLACE** |
| `StudentFeeDto.amount` | `net_amount`, `original_amount`, `paid_amount` | `StudentFeeDto` | استهلاك المبالغ المالية المعتمدة | **REMAP** |
| `FeeTemplateDto.is_mandatory === 1` | `is_mandatory: boolean` | `FeeTemplateDto` | مطابقة القيم المنطقية | **REMAP** |
| `DiscountTemplateDto.calculation_method` | `discount_type: "percentage" \| "fixed"` | `DiscountTemplateDto` | `DiscountTemplateDto` | **REMAP** |
| استعلام كل المستخدمين وتصفيتهم محلياً | `createUsersQuery` / فلترة مخصصة | `UserDto[]` | استعلام موجه مع بحث لحظي | **REPLACE** |
| `documentsQuery.data as any[]` | `api.listDocumentsByEntity` | `DocumentDto[]` | `queryKeys.student.documents(id)` | **REBUILD** |
| `editData?: any` / `JsonRecord` | `GuardianDto` / `UserDto` | `GuardianDto \| null` | أنواع معتمدة بدون أي تحويلات قسرية | **REMOVE** |

---

## 3. تصحيح دورة حياة ولي الأمر (Guardian Lifecycle Correction)

1. **فصل الهوية عن علاقة الولاية**:
   - تم إلغاء أي منطق يقوم بإنشاء حساب مستخدم (`registerUser`) أو توليد كلمات سر عشوائية أو عناوين بريد إلكتروني وهمية (`parent.${Date.now()}@orbis.local`).
   - تم حصر وظيفة استمارة ربط ولي الأمر (`guardian-form.svelte`) في البحث عن ولي الأمر المسجل مسبقاً في النظام وربطه بالطالب عبر الأمر المعتمد `api.assignGuardian`.
2. **توحيد نموذج إدخال الاستمارة (`FormController`)**:
   - تم استخدام `FormController<GuardianFormInput>` مع مخطط التحقق `guardianFormInputSchema`، ليكون المصدر الوحيد للحقيقة (Single Source of Truth) لحالة النموذج وحقوله وأخطائه.

---

## 4. تدقيق الأنواع والمكتبات (`types.ts` & SDK)

1. **توحيد نمط كتابة الأنواع**:
   - اعتماد تسميات `snake_case` المتوافقة مباشرة مع الباك إند والـ SDK.
   - إزالة أي خلط بين أسماء المتغيرات (مثل `studentId` vs `student_id`).
   - تصحيح معاملات الفلترة المالية (`StudentFeeFilterParams`, `PaymentFilterParams`) لتطابق معايير الـ API.
2. **إزالة كافة التحويلات القسرية (`as any`, `JsonRecord`, `@ts-ignore`)**:
   - لا توجد أي تحويلات قسرية أو تجاوزات للـ Type Checker في كامل نطاق الطلاب.

---

## 5. التحقق والاختبار (Verification Results)

- **TypeScript Compilation (`bun x tsc --noEmit`)**: نجاح تام (0 أخطاء).
- **Tauri Native Backend (`cargo check` في `src-tauri`)**: نجاح تام (0 أخطاء).
- **Backend Clean Architecture Workspace (`cargo check --workspace` في `apps/api-rust`)**: نجاح تام (0 أخطاء).
- **Static Production Build (`bun run build` في `apps/desktop`)**: نجاح تام وكامل.
