# Docforge — Phase 1: Foundation Tasks

## Package Setup
- [x] Create `packages/docforge/` — `@docforge/core` TypeScript package
- [x] Create `apps/api-rust/crates/docforge-engine/` — Standalone Rust crate
- [x] Create `apps/api-rust/crates/docforge-orbis/` — Orbis adapter crate
- [x] Wire packages into monorepo (turbo, Cargo workspace)

## @docforge/core — Document Model
- [x] Node types (18 types): Page, Frame, Grid, Text, Image, Shape, Table, Barcode, QRCode, Separator, Spacer, Repeater, Conditional, ComponentRef, PageBreak, Header, Footer, Watermark
- [x] PageSetup types (PageSize, Orientation, Margins)
- [x] Variable definition types
- [x] Asset reference types
- [x] TemplateDocument root type

## @docforge/core — Operations
- [x] Node CRUD: createNode, cloneNode, findNode, updateNodeProps, addNode, removeNode, moveNode
- [x] Tree traversal: walkNodes, findNodePath
- [x] Serialization: serializeDocument, deserializeDocument
- [x] Document creation: createDocument factory

## @docforge/core — Expression Engine
- [x] Expression parser (tokenizer for `{{expressions}}`)
- [x] Expression evaluator
- [x] Condition evaluator
- [x] 15 built-in formatters
- [x] Formatter registry (custom formatters)
- [x] extractVariableRefs utility

## @docforge/core — Editor State
- [x] EditorState class (framework-agnostic)
- [x] HistoryManager (undo/redo with command pattern)
- [x] Selection manager
- [ ] Clipboard manager
- [x] onChange/onSelectionChange observable

## @docforge/core — Validation
- [x] Schema validation rules
- [x] Expression validation rules
- [x] Layout validation rules
- [x] ValidationResult types

## @docforge/core — Utilities
- [x] mm ↔ px ↔ pt conversion
- [x] Page size definitions
- [x] Arabic numeral conversion
- [x] Color utilities
- [x] ID generator

## docforge-engine (Rust) — Model
- [x] Rust Document Model (serde mirror of TS types)
- [x] All 18 node types
- [x] DocforgeError enum

## docforge-engine (Rust) — Traits
- [x] AssetStore trait
- [x] DataSource trait
- [x] ProgressReporter trait
- [x] BatchStatusStore trait
- [x] ExpressionFormatter trait
- [x] OutputFormat trait

## docforge-orbis (Rust) — Database
- [x] Schema migration (10 tables)
- [x] SeaORM entities for all template tables (via FromQueryResult & SQL maps)
- [x] Template CRUD service
- [x] Versioning logic (create version on save, publish, revert)
- [x] Lifecycle management (draft → published → archived)

## docforge-orbis (Rust) — Integration
- [x] Permission variants (Template*, Generate*, Asset*, Component*)
- [x] Asset manager (upload, store, retrieve)
- [x] Audit log integration
- [x] Tauri commands (CRUD, lifecycle, versioning, assets)
