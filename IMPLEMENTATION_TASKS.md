# Orbis — Implementation Tasks

> **ملف المهام الرسمي** — يُحدَّث مع كل جلسة عمل.  
> **المرجع:** `ARCHITECTURE_AUDIT_PLAN.md`

**مفتاح الحالة:** `✅` مكتمل · `🔄` قيد التنفيذ · `⏳` معلّق · `❌` ملغى

---

## المرحلة 0 — P0 استقرار الأساس

### قاعدة البيانات

| ID | المهمة | الحالة | الملفات | ملاحظات |
|----|--------|--------|---------|---------|
| P0-DB-01 | إزالة `org_id UNIQUE` من `academic_policies` في baseline | ✅ | `schema_baseline.sql` | فهارس جزئية مضافة |
| P0-DB-02 | Migration idempotent لقواعد موجودة | ✅ | `schema_migrations.rs` | `idx_ap_org_default` |
| P0-DB-03 | تشغيل migrations في api-gateway init | ✅ | `app_state.rs` | مكتمل |

### Business Logic & Transactions

| ID | المهمة | الحالة | الملفات | ملاحظات |
|----|--------|--------|---------|---------|
| P0-BL-01 | Transaction: enrollment + fees | ✅ | `enrollment.rs`, `fee_assigner.rs` | |
| P0-BL-02 | `FeeAssigner` يقبل `period_id` | ✅ | `traits.rs`, `fee_assigner.rs` | |
| P0-BL-03 | Transaction: `payment.record` | ✅ | `payment.rs`, `ledger.rs` | منجز مسبقًا باستخدام txn |
| P0-BL-04 | `update_account_balance_on` للـ transactions | ✅ | `repository/commands.rs` | |
| P0-BL-05 | PeriodGate invalidation بعد Autopilot | ✅ | `academic_executor.rs`, `engine.rs` | |
| P0-BL-06 | PeriodGate invalidation من handlers يدوية | ✅ | `academic_period.rs` ops | |

### الأحداث والمنصة

| ID | المهمة | الحالة | الملفات | ملاحظات |
|----|--------|--------|---------|---------|
| P0-EV-01 | `DomainEventLoop` في shared-kernel | ✅ | `event_loop.rs` | |
| P0-EV-02 | DocForge projection handler | ✅ | `docforge-orbis/projection_handler.rs` | |
| P0-EV-03 | ربط event loop في Desktop | ✅ | `state.rs` | |
| P0-EV-04 | ربط event loop في api-gateway | ✅ | `app_state.rs`, `Cargo.toml` | |

### Orbis Autopilot

| ID | المهمة | الحالة | الملفات | ملاحظات |
|----|--------|--------|---------|---------|
| P0-AP-01 | حذف v1: planner.rs, executor.rs | ✅ | — | محذوف |
| P0-AP-02 | Resumption v2 (tick على startup) | ✅ | `resumption.rs` | |
| P0-AP-03 | Scheduler خلفية (15 min) | ✅ | `scheduler.rs` | |
| P0-AP-04 | `AutopilotEngine.with_period_gate()` | ✅ | `engine.rs` | |
| P0-AP-05 | تحديث `autopilot_run_tick` command | ✅ | `thin.rs` | |
| P0-AP-06 | Catch-up على Desktop startup | ✅ | `state.rs` | `catch_up_all` مضاف للـ init |

### التحقق

| ID | المهمة | الحالة | ملاحظات |
|----|--------|--------|---------|
| P0-QA-01 | `cargo check --workspace` | ✅ | |
| P0-QA-02 | اختبار يدوي: enrollment + fees atomic | ⏳ | |
| P0-QA-03 | اختبار يدوي: payment rollback | ⏳ | |

---

## المرحلة 1 — توحيد المنصة

| ID | المهمة | الحالة |
|----|--------|--------|
| P1-01 | استخراج `platform-policy` crate | ✅ |
| P1-02 | Persist ProcessManager → `process_log` | ✅ |
| P1-03 | Override workflow ↔ PeriodGate | ✅ |
| P1-04 | Transaction: student_registration saga | ✅ |
| P1-05 | Notification service حقيقي (بدل LogNotificationSender) | ✅ |
| P1-06 | دمج Permission enums (إزالة serde bridge) | ✅ |

---

## المرحلة 2 — توحيد الدومينات

| ID | المهمة | الحالة |
|----|--------|--------|
| P2-01 | تقسيم `svc-organization` | ⏳ |
| P2-02 | Assessment unification plan + migration | ⏳ |
| P2-03 | Unified `audit_events` table | ⏳ |
| P2-04 | `current_students` maintenance trigger | ⏳ |
| P2-05 | FK constraints migration | ⏳ |
| P2-06 | Decouple `ai-gateway` | ⏳ |
| P2-07 | إصلاح `current_period_id` semantics | ⏳ |
| P2-08 | إصلاح `promotion_jobs` FK naming | ⏳ |

---

## المرحلة 3 — Autopilot مركزي

| ID | المهمة | الحالة |
|----|--------|--------|
| P3-01 | Autopilot يستمع لـ domain events | ⏳ |
| P3-02 | Finance executor (fee reminders) | ⏳ |
| P3-03 | Exam executor (session prep) | ⏳ |
| P3-04 | Batch snapshot على period.close | ⏳ |
| P3-05 | Promotion auto dry-run + HITL | ⏳ |
| P3-06 | Outbox pattern | ⏳ |
| P3-07 | Mid-year onboarding sync | ⏳ |

---

## المرحلة 4 — Scale & SaaS

| ID | المهمة | الحالة |
|----|--------|--------|
| P4-01 | PostgreSQL adapter | ⏳ |
| P4-02 | ABAC policy engine | ⏳ |
| P4-03 | Party/Person model | ⏳ |
| P4-04 | Blueprint bootstrap كامل | ⏳ |
| P4-05 | Plugin executors | ⏳ |

---

## سجل التقدم

| التاريخ | ما تم |
|---------|-------|
| 2026-07-16 | تقرير Architecture Audit كامل |
| 2026-07-16 | P0: schema fix, enrollment txn, fee period_id, event_loop, autopilot v2 cleanup |
| 2026-07-16 | إنشاء `ARCHITECTURE_AUDIT_PLAN.md` + `IMPLEMENTATION_TASKS.md` |

---

## المهمة التالية (الآن)

1. الانتقال إلى المرحلة 1 (توحيد المنصة)
2. P1-01: استخراج `platform-policy` crate
3. P1-02: Persist ProcessManager → `process_log`
4. P1-03: ربط `academic_override_requests` ↔ PeriodGate
5. تحديث حالة المهام في هذا الملف مع التقدم
