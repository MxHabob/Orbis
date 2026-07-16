# Orbis — Architecture & Domain Audit Plan

> **الإصدار:** 1.0  
> **التاريخ:** 2026-07-16  
> **النطاق:** `apps/api-rust` + Desktop (Tauri) + قاعدة البيانات الموحّدة  
> **الهدف:** الوصول إلى نسخة إنتاجية مستقرة — **Keep It Simple, Highly Extensible**

---

## 1. الوضع الحالي (ملخص)

| المحور | التقييم | الملاحظة |
|--------|---------|----------|
| البنية العامة | ✅ جيد | Modular Monolith (18 crate) — مناسب لـ Desktop/SaaS المبكر |
| قاعدة البيانات | ⚠️ متوسط | Single SQLite موحّد (~72 جدول) — ازدواجية تقييم + FK ناقصة |
| حدود الدومين | 🔴 ضعيف | `svc-organization` و`ai-gateway` God services |
| Autopilot | ⚠️ جزئي | v2 أكاديمي فقط — ليس orchestrator مركزي |
| الأحداث | ⚠️ جزئي | EventBus publish-heavy — الاستهلاك كان Desktop فقط |
| الاتساق | 🔴 حرج | غياب transactions في مسارات مالية حرجة |
| الأداء | ⚠️ متوسط | SQLite + string dates + N+1 في autopilot |

**التوجّه الصحيح الموجود:**
- `ServiceContext` + Platform Traits
- `PeriodGate` مركزي
- `EventBus` + `ReactivePipeline`
- محرك تقييم Enterprise (`assessment_schemes`, `calculation_policies`, `snapshots`)
- محاسبة مزدوجة القيد عبر `journal_entries`

---

## 2. المشكلات الحرجة المكتشفة

### 2.1 قاعدة البيانات

1. **ازدواجية التقييم:** `assessments`/`grades` (legacy) مقابل `assessment_schemes`/`student_assessment_snapshots` (enterprise)
2. **`academic_policies`:** كان `org_id UNIQUE` يمنع branch overrides — يُصلح بفهارس جزئية
3. **`organizations.current_period_id`:** يشير لسنة أكاديمية وليس فترة — تسمية مضللة
4. **`promotion_jobs`:** FK على `academic_years` بتسمية `period_id`
5. **Denormalization بدون صيانة:** `classes.current_students`, `attendance_summary`, `treasuries.current_balance`
6. **جداول ميتة:** `permissions`, `user_permissions` — الكود يستخدم `role_permissions` فقط
7. **4 سجلات audit:** `audit_log`, `ai_action_log`, `system_log`, `process_log`
8. **FK مفقودة:** `classes.unit_id`, `class_subjects`, `scheduled_lessons`, وغيرها

### 2.2 Business Logic

1. **PeriodGate:** تبنّي جزئي + عدم invalidation بعد Autopilot
2. **Enrollment + Fees:** كان non-blocking — يجب transaction
3. **Payment:** 6+ writes بدون transaction
4. **Policy loading مكرر:** AutopilotEngine, OrgPolicyEngine, PeriodGate
5. **Permission enum مكرر:** shared-kernel ↔ svc-iam (serde bridge)

### 2.3 Domain Boundaries

1. **`svc-organization` God Domain:** org + periods + policies + promotion + registration + import/export
2. **`ai-gateway`:** يعتمد على كل svc-* crates
3. **Cyclic deps:** organization ↔ academic ↔ assessment ↔ finance

### 2.4 Orbis Autopilot

1. **v1 legacy:** resumption/planner/executor (calendar_events) — يُحذف
2. **Tick يدوي فقط** — لا scheduler
3. **لا تكامل:** finance, exam, promotion execution
4. **Actor::Autopilot bypass** للصلاحيات
5. **لا event-driven** — polling على التواريخ

### 2.5 Data Consistency

- Partial writes في payment, enrollment
- EventBus at-most-once (capacity 1024)
- ProcessManager in-memory رغم `process_log` table

---

## 3. خطة التنفيذ المرحلية

### المرحلة 0 — P0 استقرار الأساس (أسبوع 1–2)

**الهدف:** إصلاحات لا تغيّر البنية — تمنع فقدان بيانات وعدم اتساق.

| # | المهمة | الحالة |
|---|--------|--------|
| P0-1 | إصلاح `academic_policies` — branch overrides (schema + migration) | 🔄 |
| P0-2 | PeriodGate invalidation بعد Autopilot mutations | 🔄 |
| P0-3 | DB Transaction لـ `payment.record` | ⏳ |
| P0-4 | Atomic enrollment + mandatory fees (transaction) | ✅ |
| P0-5 | Domain event loop موحّد (Desktop + api-gateway) | 🔄 |
| P0-6 | حذف Autopilot v1 (planner, executor legacy) | ✅ |
| P0-7 | Resumption v2 (tick على startup) | 🔄 |
| P0-8 | Autopilot background scheduler | 🔄 |
| P0-9 | `FeeAssigner` + `period_id` في التسجيل | ✅ |

### المرحلة 1 — توحيد المنصة (أسبوع 3–4)

| # | المهمة |
|---|--------|
| P1-1 | استخراج `platform-policy` — PolicyResolver موحّد |
| P1-2 | Persist ProcessManager → `process_log` |
| P1-3 | ربط `academic_override_requests` ↔ PeriodGate |
| P1-4 | Transaction لـ student_registration saga |
| P1-5 | invalidation PeriodGate من handlers اليدوية (period lock/open) |
| P1-6 | api-gateway: تشغيل P0 migrations عند init |

### المرحلة 2 — توحيد الدومينات (أسبوع 5–8)

| # | المهمة |
|---|--------|
| P2-1 | تقسيم `svc-organization` → calendar / enrollment-workflow / org-admin |
| P2-2 | Assessment unification — migration legacy → enterprise |
| P2-3 | Unified audit stream (`audit_events`) |
| P2-4 | إصلاح denormalization (`current_students` triggers) |
| P2-5 | FK constraints للجداول الحرجة |
| P2-6 | Decouple `ai-gateway` من كل domains |

### المرحلة 3 — Autopilot كمحرك مركزي (أسبوع 9–12)

| # | المهمة |
|---|--------|
| P3-1 | Event subscriptions لـ Autopilot |
| P3-2 | Executors: finance (fee reminders), exam, notifications |
| P3-3 | Mid-year onboarding sync |
| P3-4 | Batch snapshot عند `period.close` |
| P3-5 | Promotion: auto dry-run → HITL → execute |
| P3-6 | Outbox pattern للأحداث الحرجة |

### المرحلة 4 — Scale & SaaS (أسبوع 13+)

| # | المهمة |
|---|--------|
| P4-1 | PostgreSQL adapter |
| P4-2 | ABAC policy engine |
| P4-3 | Party/Person model (بدل users monolith) |
| P4-4 | Institution blueprints كاملة (school/university/training) |
| P4-5 | Plugin marketplace لـ Autopilot executors |

---

## 4. قرارات معمارية مستهدفة (ERP/SIS 2026)

1. **Modular Monolith** — لا microservices مبكرة
2. **Calendar Domain** مصدر حقيقة للنوافذ — Autopilot ينفّذ لا يقرر وحده
3. **Policy-as-Data** + **PolicyResolver** واحد
4. **Command/Saga** لكل cross-domain flow
5. **Outbox** للأحداث الحرجة — EventBus للـ reactive فقط
6. **Assessment Engine** كـ plugin architecture
7. **Autopilot = Orchestrator** — executors per bounded context
8. **HITL** عبر `autopilot_tasks` + `academic_override_requests`
9. **SQLite desktop / PostgreSQL cloud** — نفس schema abstraction

---

## 5. مخاطر مستقبلية

| المخاطرة | الاحتمال | الأثر |
|----------|----------|-------|
| Drift legacy grades vs snapshots | عالي | تقارير خاطئة |
| فقدان events (broadcast lag) | متوسط | projections ناقصة |
| Partial payment writes | متوسط | محاسبة غير متوازنة |
| svc-organization growth | عالي | صيانة صعبة |
| SQLite multi-branch | متوسط | locking/أداء |

---

## 6. معايير القبول (Definition of Done)

### P0 مكتمل عندما:
- [ ] كل payment/enrollment في transaction مع rollback
- [ ] PeriodGate cache يُبطل فور تغيير الفترة (Autopilot + handlers)
- [ ] Event loop يعمل في Desktop و api-gateway
- [ ] Autopilot scheduler يعمل كل 15 دقيقة
- [ ] لا ملفات v1 legacy في autopilot crate
- [ ] `cargo check --workspace` يمر بدون أخطاء

### النسخة النهائية عندما:
- [ ] تقييم موحّد
- [ ] Autopilot يغطي academic + finance + exam
- [ ] Override workflow مكتمل
- [ ] PostgreSQL جاهز
- [ ] Blueprint bootstrap لكل أنواع المؤسسات

---

## 7. مراجع الكود الرئيسية

| المكون | المسار |
|--------|--------|
| Schema | `apps/api-rust/crates/infra-persistence/src/schemas/schema_baseline.sql` |
| P0 Migrations | `apps/api-rust/crates/infra-persistence/src/schema_migrations.rs` |
| PeriodGate | `apps/api-rust/crates/shared-kernel/src/period_gate.rs` |
| Event Loop | `apps/api-rust/crates/shared-kernel/src/event_loop.rs` |
| Autopilot Engine | `apps/api-rust/crates/autopilot/src/engine.rs` |
| Platform Bootstrap | `apps/api-rust/crates/api-gateway/src/app_state.rs` |
| Desktop State | `apps/desktop-svelte/src-tauri/src/state.rs` |

---

*آخر تحديث: 2026-07-16 — يُحدَّث مع كل مرحلة في `IMPLEMENTATION_TASKS.md`*
