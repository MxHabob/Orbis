# Orbis Desktop — Engineering Rules & Agent Execution Contract

> **Document Version:** 2.0.0-PROD  
> **Target System:** Orbis Enterprise Educational Operating System (Desktop Core)  
> **Master Execution Plan:** `docs/ORBIS_ERP_TRANSFORMATION_EXECUTION_PLAN.md`  

---

## 1. Executive Instructions for AI Coding Agents

Every agent starting a session on Orbis MUST:
1. **Read this AGENTS.md file first.**
2. **Read the current CONTINUATION STATE at the bottom of this file.**
3. **Inspect the existing implementation before writing any code.**
4. **Follow the Cardinal Law: CONSOLIDATION BEFORE ADDITION.**
5. **Never create mock data, placeholder buttons, empty handlers, or parallel duplicate components.**
6. **Never use `as any` casts to hide architectural mismatches.**
7. **Perform a Legacy Removal Pass after modifying any vertical section.**
8. **Verify the Section Completion Contract before declaring any section complete.**
9. **Update the CONTINUATION STATE section at the bottom of this file before ending the turn.**

---

## 2. Technology Stack & Core Commands

- **Frontend:** SvelteKit 2 with Svelte 5 Runes (`$state`, `$derived`, `$props`, `$effect`, snippets).
- **Backend (Embedded):** Rust Clean Architecture (`orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`, `docforge-engine`) via Tauri 2.0 IPC commands.
- **Dual-Transport Parity:** `OrbisApiClient` polymorphic interface implemented identically by `LocalClient` (Tauri IPC / SQLite) and `RemoteClient` (HTTP fetch / `@orbis/sdk` / PostgreSQL).
- **UI & Grid Library:** `@orbis/ui` + `@orbis/svelte-grid` (virtualized DOM/canvas hybrid `OrbisGrid`).
- **Styling:** TailwindCSS 4 with glassmorphic dark/light themes.
- **Internationalization:** Paraglide.js (compile-time type-safe RTL/LTR i18n).
