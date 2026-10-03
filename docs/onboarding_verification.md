# Onboarding System Verification & Test Plan

**Project:** Orbis ERP / SIS  
**Document:** Onboarding Verification Plan (`docs/onboarding_verification.md`)  
**Status:** Canonical Reference Specification  
**Date:** 2026-10-01  

---

## 1. Automated & End-to-End Verification Matrix

| # | Test Scenario | Expected Outcome | Verification Method |
| :- | :--- | :--- | :--- |
| **T01** | Fresh Database Bootstrap | Org, root admin, ISO currencies, standard COA (1000, 1110, 1120, 2000, 3000, 4000, 4100, 5000), 8 system roles with complete permissions created. | Rust unit/integration test + DB inspect |
| **T02** | Bootstrap Idempotence | Calling bootstrap again returns 409 / `AlreadyInitialized`. | Integration test |
| **T03** | Startup State Transition | System resolves `READY_WITH_SETUP` upon successful login. | Startup orchestrator test |
| **T04** | Onboarding Initial DAG Generation | Evaluates 15+ tasks with `org_profile` unblocked, dependent tasks (`academic_year`, `classes`, `fees`, `registration`) blocked with localized reasons. | Backend `OnboardingService::get_state` test |
| **T05** | Blueprint Adaptation (School) | `School` blueprint generates K-12 terms & monthly periods task graph and 100-pt monthly assessment scheme requirements. | Service test with `InstitutionType::School` |
| **T06** | Blueprint Adaptation (University) | `University` blueprint generates semester/credit-hour task graph and GPA grading requirements. | Service test with `InstitutionType::University` |
| **T07** | Step-by-Step Task Completion | Creating an Org Unit marks `org_units` COMPLETED and unlocks `academic_year`, `chart_of_accounts`, and `academic_subjects`. | Sequential execution test |
| **T08** | Multi-Dependency Unlocking | Creating `academic_periods` and `academic_subjects` unlocks `academic_classes`. | Dependency graph test |
| **T09** | Fee Template Scoping | Fee template targeted to specific grades persists without conversion to "all grades". | Finance service test |
| **T10** | Crash & Resume Recovery | Simulated process kill during configuration resumes with accurate progress percentage on relaunch. | Idempotence test |
| **T11** | Local / Remote Parity | Tauri IPC command and HTTP API route return identical JSON structures. | Parity test |
| **T12** | Organization Scope Isolation | Org A cannot query or mutate Org B onboarding state or configuration. | Multitenancy test |
| **T13** | Final Operational Readiness | Completing all required setup items transitions `is_institution_ready` to true with 100% required progress. | End-to-end readiness test |
