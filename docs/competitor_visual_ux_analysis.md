# Competitor Visual / UX Reverse Engineering Report

> **ANALYSIS ONLY — DO NOT IMPLEMENT**
> This document extracts UX patterns, interaction models, and layout principles from the competitor's system.
> The goal is to LEARN from the competitor, NOT COPY it.

---

## 1. PDF VISUAL PAGE INVENTORY

| Page Range | Module | Screen Type | Key Visual Elements |
|---|---|---|---|
| 10–11 | Main Screen | Dashboard / Shell | Tab bar, tree nav, icon toolbar, status bar |
| 12 | Window Types | Reference | 4 window archetypes diagram |
| 13 | Service Windows + Search | Dialog | Search dialog, document assignment dialog |
| 13 (bottom) | Simple Windows | Form | Form + data grid, CRUD toolbar |
| 14 | Advanced Windows | Form + Grid | Header/detail, tab navigation, field list panel |
| 15 | Report Windows | Report Viewer | Filter panel, data table, view/print/exit toolbar |
| 16 | Controls Reference | Widget Catalog | Dropdown, text field, tree, checkbox list |
| 20 | Report Tools | Report Filters | Date range pickers, tree filters, report type selectors |
| 28 | User Groups | CRUD + Grid | Simple form + data grid for user group management |
| 34 | Permissions | Tree + Checkbox Matrix | Split panel: tree left, checkbox permissions right |
| 36 | Settings | Settings Tree | Hierarchical settings with collapsible sections, key-value form |
| 50–51 | Chart of Accounts | Tree + Form | Split: tree on right, form on left, CRUD toolbar |
| 63 | Payment Voucher | Transaction Form | Header fields, detail grid, journal entry display |
| 99 | Account Statement | Report + Filters | Tree filter panel, date filters, data table |
| 100 | Account Statement Detail | Text | Detailed report parameter documentation |
| 119 | Trial Balance | Financial Report | Filter bar + tabular financial data with debit/credit columns |
| 124 | School Information | Setup Form | Multi-field form with logo upload, data grid at bottom |
| 136 | Academic Calendar | Setup + Grid | Simple form (dropdown + text), audit grid below |
| 154 | Student Registration | Tabbed Form | Multi-tab form (personal, guardian, academic, financial), photo upload, Excel import |
| 159 | Student Data Tree | Entity Workspace | Master tree (left) + detail toolbar + action buttons (top), tree shows enrollment history |
| 160 | Student Directory | Search + Grid | Form on top, data grid on bottom, tab structure |
| 165 | Daily Attendance | Batch Entry Grid | Date grid with checkbox columns (present/absent/late/excused), action buttons |
| 174–176 | Grades & Control | Menu + Grid | Cascading menu navigation for grades entry, semester grades, report cards |
| 185 | Class Schedules | Menu | Cascading sub-menu for schedule types |
| 188 | Student Promotion | Menu | Single-action menu entry for year-end promotion |
| 196 | Transportation | Simple CRUD | Bus routes form + data grid |
| 200 | Fee Receipt | Transaction Form | Header (student, date, amount, method) + detail line items grid |
| 209 | SMS Messages | Config Menu | Settings menu + message templates + subscription |
| 213 | Reports Module | Report Launcher | Vertical list of ~16 report links as menu |
| 220 | Success Rate Report | Report Link | Same vertical report menu pattern |

---

## 2. NAVIGATION ARCHITECTURE ANALYSIS

### 2.1 Primary Navigation: Horizontal Icon Tab Bar

**Pattern**: A single-row horizontal tab bar spans the full width of the application window. Each tab uses an **icon + Arabic text label** combination.

**Observed Tabs** (Right-to-Left order):
1. إدارة النظام (System Management) — gear icon
2. الحسابات (Accounts) — chart/bar icon
3. تهيئة النظام (System Setup) — wrench icon
4. شؤون الطلاب (Student Affairs) — people icon
5. شؤون الموظفين (Employee Affairs) — person icon
6. الكنترول (Control/Grades) — clipboard icon
7. إدارة الأنشطة الطلابية (Student Activities) — activity icon
8. إدارة المواصلات (Transportation) — bus icon
9. الشؤون المالية (Financial Affairs) — money icon
10. مخازن المدرسة (School Store) — box icon
11. الرسائل القصيرة (SMS Messages) — message icon
12. التقارير (Reports) — chart icon

**UX Insight for Orbis**: 12 top-level modules is very high. Modern apps typically flatten this to 5-7 major groups with sub-navigation. The competitor solves this with icon+label tabs, but this doesn't scale well on smaller screens.

### 2.2 Secondary Navigation: Cascading Menus

Each primary tab reveals a **vertical cascading menu** on the left side. These menus use:
- **Category headers** with expand/collapse arrows (◄)
- **Sub-items** as direct action links
- **Keyboard shortcuts** shown inline (e.g., `Ctrl+T`, `Ctrl+S`, `F7`, `F8`, `Ctrl+F8`)

**Example — Student Affairs**:
```
التسجيل والقبول ◄
  ├── شجرة بيانات الطلاب   Ctrl+T
  ├── دليل الطلاب           Ctrl+S
  ├── دليل أولياء الأمور
  ├── وثائق الطالب          Ctrl+D
  ├── توزيع الطلاب على الشعب
  └── بيانات الطلاب الغير مسجلين
الطلاب ◄
متابعة الطلاب ◄
  ├── التحضير اليومي للطلاب
  ├── سلوكيات الطلاب
  └── كشوفات حضور وغياب الطلاب
```

**UX Insight for Orbis**: The cascading menu is functional but visually dated. Modern pattern: collapsible sidebar with icon-only collapsed state, search-as-you-type for commands, breadcrumb trail.

### 2.3 Tertiary Navigation: Tab Panels within Forms

Complex forms (Student Registration, Payment Voucher, Account Statement) use **horizontal tab panels** within the content area:
- البيانات الشخصية | بيانات ولي الأمر | البيانات العلمية | البيانات المالية | الاشتراك بالمواصلات

**UX Insight for Orbis**: This tab-within-tab pattern creates nested navigation. Modern alternative: a vertical stepper for sequential data entry, or a single scrollable form with section anchors.

---

## 3. LAYOUT PATTERNS TAXONOMY

### 3.1 Window Type: Simple Form (نوافذ بسيطة)

**Structure**: 
```
┌─────────────────────────────────┐
│ Window Title                     │
├─────────────────────────────────┤
│  Form Fields (Label: Input)      │
│  [field1] [field2]               │
│  [field3] [dropdown4]            │
├─────────────────────────────────┤
│  CRUD Toolbar                    │
│  [Add][Edit][Delete][Refresh]    │
│  [Search][Print][Exit]           │
├─────────────────────────────────┤
│  Data Grid                       │
│  ┌───┬────┬────┬────┐           │
│  │ # │Col1│Col2│Col3│           │
│  ├───┼────┼────┼────┤           │
│  │ 1 │ .. │ .. │ .. │           │
│  └───┴────┴────┴────┘           │
├─────────────────────────────────┤
│  Status: User Log  │  Record Nav │
└─────────────────────────────────┘
```

**Used In**: Student Directory, User Groups, Bus Routes, Account Guides, School Days

**UX Insight**: This is the workhorse pattern — form above, grid below, CRUD toolbar in between. Orbis should have a reusable `FormGridView` component that implements this pattern with modern styling.

---

### 3.2 Window Type: Advanced Form (نوافذ متقدمة)

**Structure**:
```
┌──────────────────────────────────────┐
│ Window Title              │ Record # │
├──────────────────────────────────────┤
│ Date: [__/__/____]  Status: [____]   │
│                                      │
│ Header Fields:                       │
│ [Account] [Amount] [Currency]        │
│ [Method]  [Fund]   [Reference]       │
│                                      │
│ ┌─ Tabs ─────────────────────────┐   │
│ │ Tab1 │ Tab2 │ Tab3 │           │   │
│ ├────────────────────────────────┤   │
│ │ Tab Content Area               │   │
│ └────────────────────────────────┘   │
│                                      │
│ Detail Grid (line items):            │
│ ┌────┬─────┬──────┬─────┐           │
│ │ Del│ Save│ Name │ Amt │           │
│ └────┴─────┴──────┴─────┘           │
│                                      │
├──────────────────────────────────────┤
│ [Add][Edit][Delete][Refresh]         │
│ [Search][Print][Print Doc][Exit]     │
├──────────────────────────────────────┤
│ Tab Order Config: [field list panel] │
│ Record Nav: [◄][1 من 1][►]          │
└──────────────────────────────────────┘
```

**Used In**: Payment Voucher, Receipt Voucher, Fee Receipt, Journal Entries

**Key Features**:
- Record navigator at bottom (`سجل 1 من 1`)
- Tab-order customization panel (left sidebar showing field traversal order)
- Document status indicator (posted/unposted: `حالة السند غير مرحل`)
- Keyboard shortcut for print: `طباعة السند`
- Header/detail pattern (master record + line items grid)

**UX Insight for Orbis**: This is the most complex and data-intensive pattern. It needs a dedicated `TransactionForm` component with: 
- Clear header/detail separation
- Inline editing in the detail grid
- Document lifecycle state machine (draft → approved → posted → cancelled)
- Print preview without separate window

---

### 3.3 Window Type: Report Viewer (نوافذ التقارير)

**Structure**:
```
┌──────────────────────────────────────┐
│ Report Title                         │
├──────────────────────────────────────┤
│ Filter Bar:                          │
│ [Date From] [Date To] [Branch]       │
│ [Account Tree] [Currency] [Level]    │
│                                      │
│ [عرض] [طباعة] [طباعة المحدد] [خروج]│
├──────────────────────────────────────┤
│ ┌────────────────────────────────┐   │
│ │ Report Data Table              │   │
│ │ ┌────┬──────┬──────┬──────┐   │   │
│ │ │Name│Debit │Credit│Balance│   │   │
│ │ ├────┼──────┼──────┼──────┤   │   │
│ │ │... │...   │...   │...   │   │   │
│ │ └────┴──────┴──────┴──────┘   │   │
│ └────────────────────────────────┘   │
└──────────────────────────────────────┘
```

**Used In**: Trial Balance, Account Statement, Attendance Reports, Grade Reports, Financial Statements

**Filter Patterns**:
- Date range: `خلال يوم` / `خلال شهر` / `من...إلى` / `حتى يوم`
- Entity selector: Tree or dropdown
- Scope: `كافة الحسابات` / `حساب واحد` / `اختيار مخصص`
- Report type: `إجمالي` / `تفصيلي`

**UX Insight for Orbis**: Report filters need a collapsible panel. The competitor uses a static always-visible filter bar which consumes significant vertical space. Better pattern: a collapsible filter drawer with "active filter chips" showing what's applied.

---

### 3.4 Window Type: Entity Workspace (شجرة بيانات)

**The most sophisticated pattern** — demonstrated in the Student Data Tree (p.159):

**Structure**:
```
┌──────────────────────────────────────────┐
│ Quick Action Toolbar (Icon Buttons)       │
│ [تسجيل] [طالب] [رسوم] [أضافه ولي أمر]   │
│ [أضافه وثن أمر] [طالب] [توزيع] [اشتراك] │
│ [بالمواصلات] [إعلان]                      │
├──────────────────────────────────────────┤
│ Student Info Bar:                         │
│ [✎ تسجيل] [تسجيل طالب]                  │
│ Name: أيهم محمد خالد محمد الحباسي        │
│ الرقم: 1  الاسم: ___ الصف: ___          │
│                                           │
│ ┌──── Tabs ──────────────────┐            │
│ │حضور وغياب│الفترات│الجدول  │            │
│ │الدراسي│الحوزة│رسوم│       │            │
│ └────────────────────────────┘            │
│                                           │
│ ┌─────────┬──────────────────┐           │
│ │ Student │                  │           │
│ │ Tree    │  Detail Content  │           │
│ │ (Left)  │  (Center/Right)  │           │
│ │         │                  │           │
│ │ ☐ أيهم  │  [Grid/Form]     │           │
│ │  ├ الصف │                  │           │
│ │  ├ الشعبة│                 │           │
│ │  └ الفصل│                  │           │
│ └─────────┴──────────────────┘           │
│                                           │
│ عدد الطلاب:1  المسجلين:0  غير مسجلين:1  │
└──────────────────────────────────────────┘
```

**Key Features**:
- **Icon toolbar** at top with color-coded quick action buttons
- **Entity header** showing the selected entity's key info
- **Tab navigation** for different aspects of the entity
- **Tree panel** on one side showing the entity hierarchy
- **Detail panel** showing contextual content based on selection
- **Status bar** with aggregate counts

**UX Insight for Orbis**: This is the most powerful pattern. It provides a "360-degree view" of a student entity. Orbis should implement this as a `EntityWorkspace` component with:
- Command palette (instead of icon toolbar)
- Sidebar tree with search/filter
- Tabbed detail area
- Contextual actions per tab
- Summary statistics bar

---

## 4. FORM DESIGN PATTERNS

### 4.1 Field Layout

| Pattern | Description | Usage |
|---|---|---|
| **Label-right** | Labels appear to the RIGHT of fields (RTL layout) | All forms |
| **2-column grid** | Fields arranged in 2-column layout | Student Registration, Payment Voucher |
| **Key-value pairs** | Label and field on same line, right-aligned | School Information, Settings |
| **Grouped sections** | Fields grouped under collapsible headers | Settings (Academic, Print, Students, Accounts) |

### 4.2 Input Controls Inventory

| Control | Visual Pattern | Usage Context |
|---|---|---|
| **Text Input** | Standard bordered input field | Names, amounts, reference numbers |
| **Dropdown** | Standard select with arrow button | Currencies, methods, statuses |
| **Date Picker** | Text input + calendar icon button | All date fields |
| **Autocomplete Dropdown** | Text input with type-ahead matching + Tab to accept | Account names, student names |
| **Tree Selector** | Expandable tree with checkboxes | Account chart, grade hierarchy, branch selection |
| **Checkbox** | Standard checkbox with label | Report options, permissions |
| **Checkbox List** | Vertical list of checkboxes | Branch permissions, report columns |
| **Photo Upload** | Image placeholder with upload action | Student photo |
| **Record Navigator** | `[◄] [1 من 1] [►]` controls | Bottom of all form windows |
| **Multi-line Text** | Standard textarea | Notes, descriptions |

### 4.3 Validation & Feedback

| Mechanism | Visual Treatment |
|---|---|
| **Required fields** | No visible asterisk — implied by form structure |
| **System prompts** | Modal dialog box with warning icon (⚠️) |
| **Missing data warning** | Yellow/orange dialog: "هل تريد القيام بذلك الآن؟" |
| **Success confirmation** | No visible toast/snackbar — action completes silently |
| **Error states** | Modal error dialog |

**UX Insight for Orbis**: The competitor's validation is entirely modal-dialog-based. Modern pattern: inline field validation with red borders + helper text, toast notifications for success, contextual error banners for form-level errors.

---

## 5. TOOLBAR & ACTION PLACEMENT MATRIX

### 5.1 CRUD Toolbar (Consistent across ALL windows)

**Standard Button Order** (Right-to-Left):
```
[إضافة] [تعديل] [حذف] [تحديث] [بحث] [طباعة] [خروج]
 (Add)   (Edit) (Delete)(Refresh)(Search)(Print) (Exit)
```

**Visual Style**: 
- Each button has an icon + text label
- Color-coded: Add (green+), Edit (blue pencil), Delete (red X), Refresh (blue arrows), Search (magnifier), Print (printer), Exit (door)
- Placed BETWEEN form and data grid
- Enabled/disabled based on mode (add mode, edit mode, browse mode)

### 5.2 Mode-Dependent States

| Mode | Active Buttons | Inactive Buttons |
|---|---|---|
| **Browse** | Add, Edit, Delete, Search, Print, Exit, Refresh | — |
| **Add** | Save (via Add), Cancel, Exit | Edit, Delete, Search |
| **Edit** | Save (via Edit), Cancel, Exit | Add, Delete, Search |

### 5.3 Additional Action Buttons (Context-Specific)

| Button | Location | Context |
|---|---|---|
| `ترحيل السند` (Post Document) | Left side of form | Financial vouchers |
| `إلغاء الترحيل` (Unpost) | Left side of form | Posted financial vouchers |
| `إعتماد` (Approve) | Left side of form | Documents requiring approval |
| `طباعة السند` (Print Document) | CRUD toolbar extension | Financial vouchers |
| `استيراد الطلاب` (Import Students) | Below form | Student Registration |
| `حفظ القالب` (Save Template) | Bottom-left | Report filters |
| `طباعة مصادقة رصيد` (Print Balance Confirmation) | CRUD toolbar extension | Account Statement |

**UX Insight for Orbis**: The consistent CRUD toolbar is a strong pattern. However, placing ALL actions in a single row creates visual clutter. Modern approach:
- Primary actions (Save, Cancel) as prominent buttons in the form header
- Secondary actions (Print, Delete) in a `...` overflow menu
- Destructive actions (Delete) with confirmation dialog
- State transitions (Post, Approve) as distinct workflow buttons with clear visual states

---

## 6. DATA GRID PATTERNS

### 6.1 Standard Data Grid

**Features Observed**:
- Fixed header row
- Row selection highlighting
- Column sorting (click header)
- Right-aligned numeric columns
- Auto-generated row number column (`الرقم`)
- Multi-column display with horizontal scrollbar
- User log columns: `المستخدم`, `وقت الإدخال` (user, entry time)

### 6.2 Editable Grid (Detail Lines)

**Used In**: Payment details, Fee line items, Attendance grid

**Features**:
- Inline editing within cells
- Row-level action buttons: Delete (❌), Save (✔), Undo (↩)
- Dropdown selectors within cells
- Auto-calculation (totals row)

### 6.3 Attendance Grid (Specialized)

**Unique Pattern** from p.165:
```
┌──────┬────────────────────────────┬───┬───┬───┬───┬───────┐
│ حذف  │ تراج │ حفظ │ اسم الطالب  │حاضر│غائب│غائب│بدون│ملاحظات│
│      │      │     │              │    │بعذر│    │عذر │       │
├──────┼──────┼─────┼──────────────┼────┼────┼────┼────┼───────┤
│  ❌  │  ✔   │  ✔  │ أيهم محمد    │ ☑  │ ☐  │ ☐  │ ☐  │       │
│  ❌  │  ✔   │  ✔  │ أيهم محمد    │ ☑  │ ☐  │ ☐  │ ☐  │       │
└──────┴──────┴─────┴──────────────┴────┴────┴────┴────┴───────┘
```

**UX Insight for Orbis**: The attendance grid is a batch-entry pattern. Each row represents a student-day, with radio-button-like behavior (only one status can be selected). Modern approach: use a calendar-view or card-based layout alongside the grid for batch entry.

---

## 7. TREE VIEW PATTERNS

### 7.1 Chart of Accounts Tree
```
☐ شجرة الحسابات
  ├─ ☐ (1) الأصول
  │   ├─ ☐ (11) الأصول الثابتة
  │   │   ├── (1101) الأراضي
  │   │   ├── (1102) المباني
  │   │   └── (1103) السيارات
  │   └─ ☐ (12) الأصول المتداولة
  ├─ ☐ (2) الخصوم
  ├─ ☐ (3) المصروفات
  └─ ☐ (4) الإيرادات
```

**Features**: Expand/collapse, checkbox selection, numeric code prefix, right-click context menu (add child, edit, delete)

### 7.2 Settings Tree
```
إدارة النظام ◄
  ├── إدارة قواعد البيانات
  ├── أدارة الأمن والصلاحيات ◄
  ├── إدارة إعدادات النظام ◄
  │     └── الإعدادات ◄
  ├── خدمات إضافية ◄
  ├── إدارة الترجمات ◄
  └── حول البرنامج
```

### 7.3 Student Tree
```
☐ أيهم محمد خالد محمد الحباسي
  ├─ الصف: أول تمهيدي
  │   └─ الشعبة: البراعم
  ├─ الصف: الأول أساسي
  │   └─ الشعبة: الفراشات
  ├─ الصف: الأول أساسي
  │   └─ الشعبة: أ
  └─ الصف: الثاني أساسي
```

**UX Insight for Orbis**: Trees are used extensively for hierarchical data. The competitor's tree implementation is functional but visually basic. Modern approach: 
- Virtual scrolling for large trees
- Search/filter within tree
- Drag-and-drop for reorganization
- Breadcrumb trail showing tree path
- Keyboard navigation (arrow keys)

---

## 8. VISUAL HIERARCHY & DESIGN SYSTEM ANALYSIS

### 8.1 Color Palette

| Element | Color | Usage |
|---|---|---|
| **Primary tabs** | Blue gradient (#4477BB → #336699) | Main navigation bar |
| **Active tab** | Bright blue with lighter background | Selected module |
| **Toolbar buttons** | Multi-color icon-based | Each action has distinct color |
| **Add button** | Green (#44AA44) | Create new record |
| **Edit button** | Blue (#4477BB) | Modify selected |
| **Delete button** | Red (#CC4444) | Remove selected |
| **Grid headers** | Light gray (#E8E8E8) | Table headers |
| **Grid rows** | White / Light blue alternating | Zebra striping |
| **Menu headers** | Teal/cyan (#008899) | Category labels in menus |
| **Warning dialogs** | Yellow background with ⚠️ icon | System prompts |

### 8.2 Typography

- **Primary font**: Arabic system font (Tahoma-like)
- **Font sizes**: ~11px for body text, ~13px for headers, ~9px for grid data
- **Weight**: Normal for data, Bold for labels and headers
- **Alignment**: Right-aligned for Arabic text, Left-aligned for numbers

### 8.3 Spacing & Density

- **Information density**: HIGH — screens pack many fields and controls
- **Padding**: Minimal (2-4px around controls)
- **Margins**: Tight (4-8px between sections)
- **Grid row height**: ~22px (compact)

**UX Insight for Orbis**: The competitor optimizes for information density over visual comfort. For Orbis, balance density with breathing room:
- Use 8px base spacing unit
- Minimum 40px touch targets for clickable elements
- 32px minimum grid row height
- Clear visual separation between sections

---

## 9. INFORMATION ARCHITECTURE PATTERNS

### 9.1 Multi-Level Settings Hierarchy

The settings system demonstrates a sophisticated **3-level hierarchy**:

```
Level 1: Global Settings (عام)
  ├── Administration Settings (إعدادات الإدارة)
  ├── Branch Settings (إعدادات الفرع)
  └── User Settings (إعدادات المستخدم)

Level 2: Role-Scoped
  ├── General Teacher (الأستاذ العام)
  │    ├── Admin Settings
  │    ├── Branch Settings
  │    └── User Settings
  └── School Teacher (المدارس)
       ├── Admin Settings
       ├── Branch Settings
       └── User Settings

Level 3: Branch-Scoped
  └── Branch Settings (إعدادات الفرع)
       ├── Admin Settings
       ├── Branch Settings
       └── User Settings
```

**Settings Content** (from p.36):
- Academic: min students, study days, absence period, grid color
- Print: print full certificate (yes/no)
- Students: default birth date, top N count
- Accounts: parent account ID

**UX Insight for Orbis**: This 3-level cascading settings model (Global → Role → Branch → User) is a powerful pattern for multi-tenant school systems. Orbis should implement this with:
- Visual inheritance indicator (showing where each setting value comes from)
- Override highlighting (when a lower level overrides a higher one)
- Reset-to-default action per setting

### 9.2 Permission Model

The permission system (p.34) uses a **screen-by-action matrix**:

```
Screen (Row)     × Action (Column)
─────────────────────────────────────
                 │ عرض │إضافة│تعديل│حذف│بحث│طباعة│
                 │(View)│(Add)│(Edit)│(Del)│(Search)│(Print)│
─────────────────┼──────┼─────┼──────┼────┼──────┼───────┤
الصلاحيات        │  ☑   │  ☑  │  ☑   │ ☑  │  ☑   │  ☑    │
معلومات الصف     │  ☑   │  ☑  │  ☑   │ ☐  │  ☐   │  ☐    │
سعلومية إضافة مبرد│ ☑   │  ☐  │  ☑   │ ☐  │  ☐   │  ☐    │
```

**Applied at**: User level OR User Group level, scoped by Branch

### 9.3 Document Lifecycle

Financial documents follow a clear lifecycle:
```
New → Draft (غير مرحل) → Posted (مرحل) → [Unpost possible]
```

Visual indicators:
- `حالة السند: غير مرحل` (Document status: Unposted) — shown as text label
- `ترحيل السند` (Post Document) — action button appears when in draft
- `إلغاء الترحيل` (Unpost) — action button appears when posted

---

## 10. WORKFLOW & INTERACTION PATTERNS

### 10.1 Master-Detail Pattern

**Observed in**: Fee Receipt (p.200), Payment Voucher (p.63)

Flow:
1. User selects/creates master record (header)
2. System displays empty detail grid
3. User adds line items to detail grid
4. Each line item has its own category selector
5. Total auto-calculates
6. User posts document

### 10.2 Batch Entry Pattern

**Observed in**: Daily Attendance (p.165)

Flow:
1. User selects scope filters (Class, Section, Month)
2. System generates grid with one row per student × date
3. User marks checkboxes across columns
4. User clicks Save to persist all changes
5. System supports "undo" per row

### 10.3 Tree-Based Navigation Pattern

**Observed in**: Chart of Accounts (p.51), Student Tree (p.159)

Flow:
1. Tree displays hierarchical data on one side
2. Selecting a tree node loads detail in adjacent panel
3. Right-click on tree node shows context menu (Add Child, Edit, Delete)
4. Tree supports expand/collapse with keyboard
5. Search within tree via Ctrl+Shift+Enter

### 10.4 Report Generation Pattern

**Observed in**: Account Statement (p.99), Trial Balance (p.119)

Flow:
1. User opens report window
2. Filter panel presented with defaults
3. User adjusts filters (date range, scope, type)
4. User clicks "عرض" (View) to generate
5. Report renders in data grid
6. User can "طباعة" (Print) or "طباعة المحدد" (Print Selected)
7. User can "حفظ القالب" (Save Template) for frequently used filter sets

---

## 11. KEYBOARD SHORTCUTS MAP

| Shortcut | Action | Context |
|---|---|---|
| `F3` | Open Search | Any window |
| `F7` | Open Payment Voucher | Accounts module |
| `F6` | Open Receipt Voucher | Accounts module |
| `F8` | Open Simple Journal Entry | Accounts module |
| `Ctrl+F8` | Open Daily Journal | Accounts module |
| `F9` | Account Statement | Reports |
| `F4` | Cash Box Balance | Accounts |
| `Ctrl+T` | Student Data Tree | Student Affairs |
| `Ctrl+S` | Student Directory | Student Affairs |
| `Ctrl+D` | Student Documents | Student Affairs |
| `Ctrl+H` | School Information | System Setup |
| `Ctrl+F3` | General Search | All modules |
| `Tab` | Next field / Accept autocomplete | Forms |
| `Space Bar` | Toggle checkbox | Checkboxes |

**UX Insight for Orbis**: The competitor has a comprehensive keyboard shortcut system. Orbis should implement a command palette (Ctrl+K) pattern instead of memorizable shortcuts, plus maintain common shortcuts for power users.

---

## 12. ERROR & FEEDBACK PATTERNS

### 12.1 Confirmation Dialogs

**Pattern**: Modal dialog with warning icon
```
┌──────────────────────────────────┐
│  ⚠️  تنبيه                       │
│                                   │
│  عزيزي المستخدم إن واجهات نظام   │
│  المدارس سهلة الاستخدام وبها     │
│  مرونة كبيرة...                   │
│                                   │
│        [نعم]    [لا]              │
└──────────────────────────────────┘
```

### 12.2 First-Run Setup Prompt

**Pattern**: System detects incomplete setup and prompts
```
┌──────────────────────────────────┐
│  ⚠️  ملاحظة                      │
│                                   │
│  في حال لم يقم مدير النظام بتهيئة│
│  هذه النافذة بالبيانات فسيقوم     │
│  النظام تلقائياً بسؤال المستخدم   │
│                                   │
│  هل تريد القيام بذلك الآن؟        │
│                                   │
│        [نعم]    [لا]              │
└──────────────────────────────────┘
```

### 12.3 User Action Log

Every form includes a bottom status bar showing:
- `عرض سجلات المستخدم` (View User Logs) checkbox
- When enabled: grid showing User, Branch, Timestamp for each modification

**UX Insight for Orbis**: The audit log in every form is a strong feature. Orbis should implement this as:
- Activity timeline panel (slide-out)
- Show who changed what, when
- Diff view for changes

---

## 13. MULTI-BRANCH & MULTI-CURRENCY SUPPORT

### 13.1 Branch System

- Login screen allows branch selection (checkbox list)
- Every transaction is branch-scoped
- Reports can filter by branch or show all branches
- Settings cascade: Global → Branch → User

### 13.2 Currency System

- Multiple currencies supported (Yemeni Riyal, Saudi Riyal, USD, AED)
- Each transaction specifies currency
- Exchange rates managed in system
- Reports can display in any configured currency

**UX Insight for Orbis**: Multi-tenancy (branch) and multi-currency are core requirements. Orbis should implement:
- Branch context shown in app header (always visible)
- Currency selector per transaction with auto-conversion
- Branch-scoped data isolation with cross-branch reporting capability

---

## 14. PRINT & EXPORT PATTERNS

### 14.1 Print System

| Action | Description |
|---|---|
| `طباعة` (Print) | Print full report/document |
| `طباعة المحدد` (Print Selected) | Print only selected rows |
| `طباعة السند` (Print Document) | Print financial voucher |
| `طباعة مصادقة رصيد` (Print Balance Confirmation) | Print official balance letter |

### 14.2 Import/Export

| Feature | Description |
|---|---|
| `استيراد الطلاب` (Import Students) | Import from Excel template with column mapping guidance |
| `تصدير ملف إكسل كنموذج` (Export Excel Template) | Generate blank Excel for data entry |

**UX Insight for Orbis**: The competitor's print system is direct-to-printer. Modern approach:
- Print preview in-app (PDF viewer)
- Export to PDF, Excel, CSV
- Share via email
- Template-based report formatting

---

## 15. COMPARATIVE GAP ANALYSIS SUMMARY

### Patterns Orbis MUST Implement (Strong Competitor Patterns)

| # | Pattern | Competitor Strength | Orbis Recommendation |
|---|---|---|---|
| 1 | **Entity Workspace** (Student Tree) | 360° view with tree + tabs + actions | Implement as core component |
| 2 | **CRUD Toolbar Consistency** | Same toolbar on every form | Standardize action bar component |
| 3 | **Cascading Settings** | 3-level override hierarchy | Implement with visual inheritance |
| 4 | **Document Lifecycle** | Draft → Posted with undo | State machine for financial docs |
| 5 | **Audit Trail per Screen** | User logs on every form | Activity timeline panel |
| 6 | **Keyboard Shortcuts** | Extensive system-wide shortcuts | Command palette + shortcuts |
| 7 | **Batch Entry Grid** | Attendance with row-level save | Editable data grid component |
| 8 | **Report Filter + Template** | Save/load filter presets | Filter builder with saved views |
| 9 | **Multi-Branch Scoping** | Branch context everywhere | Branch context in header + data isolation |
| 10 | **Permission Matrix** | Screen × Action × Branch | RBAC with fine-grained permissions |

### Patterns Orbis MUST Modernize (Competitor Weaknesses)

| # | Competitor Weakness | Modern Alternative for Orbis |
|---|---|---|
| 1 | Modal dialogs for ALL confirmations | Toast notifications, inline validation |
| 2 | 12+ top-level tabs | 5-7 grouped modules with sub-nav |
| 3 | No search/filter within navigation | Command palette (Ctrl+K), sidebar search |
| 4 | Print-only output (no PDF preview) | In-app PDF viewer with export options |
| 5 | Dense, cramped layouts | 8px spacing system, clear visual hierarchy |
| 6 | No loading states visible | Skeleton screens, progress indicators |
| 7 | No empty states | Illustrated empty states with call-to-action |
| 8 | Windows-native styling | Modern, theme-aware design system |
| 9 | No responsive design | Fluid layouts that adapt to window size |
| 10 | No dark mode | Theme-aware color system |
| 11 | Tab-within-tab nesting | Section anchors or vertical stepper |
| 12 | No drag-and-drop | Drag-and-drop for reordering, file upload |

---

## 16. RECOMMENDED ORBIS COMPONENT ARCHITECTURE

Based on this analysis, Orbis needs these core layout components:

```
1. SimpleFormView     — Form fields + CRUD toolbar + Data grid
2. AdvancedFormView   — Header/detail form with tabs + line items grid
3. EntityWorkspace    — Tree panel + detail tabs + action toolbar
4. ReportViewer       — Filter drawer + data table + export toolbar
5. BatchEntryGrid     — Multi-row editable grid with row-level actions
6. SettingsPage       — Cascading tree + key-value form
7. PermissionMatrix   — Screen × Action checkbox grid
8. TransactionForm    — Financial document with lifecycle state machine
```

---

## 17. KEY VISUAL EVIDENCE REFERENCES

| Evidence | Page | Key Observation |
|---|---|---|
| Main shell structure | p.10-11 | Tab bar + cascading menu + content area |
| Window type taxonomy | p.12-15 | 4 archetypes: Service, Simple, Advanced, Report |
| Controls catalog | p.16 | Dropdown, text, tree, checkbox patterns |
| User permission UI | p.34 | Split-panel: entity list + checkbox matrix |
| Settings hierarchy | p.36 | Tree settings with 3-level cascade |
| Chart of Accounts | p.51 | Tree + form split-panel pattern |
| Payment Voucher | p.63 | Header/detail transaction form |
| Account Statement | p.99 | Filter panel + report data table |
| Trial Balance | p.119 | Financial report with multi-column data |
| School Setup | p.124 | Multi-field setup form with data grid |
| Student Registration | p.154 | Multi-tab form with photo + Excel import |
| Student Tree | p.159 | Entity workspace: tree + tabs + actions |
| Attendance Entry | p.165 | Batch checkbox grid pattern |
| Fee Receipt | p.200 | Transaction form with line items |
| Reports Menu | p.213 | Vertical report launcher list |

---

> **End of Visual/UX Reverse Engineering Report**
> 
> This analysis provides the UX foundation for Orbis's modernization plan.
> Next steps: Compare against current Orbis UI components and build the modernization roadmap.
