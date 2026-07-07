# مهام إعادة بناء طبقة الاتصال (IPC Layer)

## المرحلة 1: إصلاح الاعتماد المكسور (Critical)
- [x] تحديث `src-tauri/Cargo.toml` — استبدال `core-domain` باعتمادات domain crates مباشرة
- [x] إضافة `DatabaseProvider` إلى `infra-persistence` (كان في `core-domain` المحذوف)
- [x] إعادة كتابة `state.rs` — `AppState` يوفر `DatabaseConnection` مباشرة بدون God Object
- [x] تحديث `lib.rs` — imports جديدة

## المرحلة 2: إعادة تنظيم أوامر Tauri
- [x] إنشاء بنية مجلدات domain-driven (`commands/iam/`, `commands/lifecycle/`, etc.)
- [x] تحديث `commands/error.rs` — ربط `AppError` بـ `CommandError` مع `error_code`
- [x] نقل أوامر `misc.rs` إلى مجالاتها الصحيحة
- [x] تحويل أوامر IAM (auth, users, permissions)
- [x] تحويل أوامر Lifecycle (cycles, calendar, years, promotion, overrides)
- [x] تحويل أوامر Academic (classes, subjects, enrollments, guardians, documents)
- [x] تحويل أوامر Assessment (grades, attendance, assessments, behavior)
- [x] تحويل أوامر Scheduling (time_slots, lessons, rooms, solver)
- [x] تحويل أوامر Finance (ledger, fees, payments, treasuries, discounts)
- [x] تحويل أوامر Exam (sessions, ordering, logistics, solver)
- [x] تحديث `commands/mod.rs` الجديد
- [x] تحديث `invoke_handler` في `lib.rs`

## المرحلة 3: نظام الأخطاء الموحد
- [x] تحديث `error.ts` في Frontend — إضافة `ErrorCode` و `StructuredError`
- [x] تحديث `types.ts` — `CommandError` مع `code` إلزامي

## المرحلة 4: Type Safety
- [x] تحويل 28 نوع `JsonRecord` إلى interfaces واضحة في `types.ts`
- [x] تحديث `commands.ts` لاستخدام الأنواع الجديدة

## المرحلة 5: نظام الأحداث
- [x] إضافة `events.rs` في Tauri backend
- [x] تحديث `events.ts` في Frontend

## المرحلة 6: أوامر AI جديدة
- [ ] أوامر Autopilot (status, tasks, diagnostics, configure)
- [ ] أوامر Agent (sessions, actions)

## المرحلة 7: تحسينات الأداء
- [ ] Event-driven cache invalidation
- [ ] تحديث prefetch strategy
