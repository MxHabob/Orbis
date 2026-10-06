# Spec Delta

## Purpose

The Orbis icon system provides a recognizable, legible, and accessible visual vocabulary for product interfaces, with consistent rendering behavior across supported web and native platforms.

## ADDED Requirements

### Requirement: Consistent Orbis icon visual language
Icons in the library MUST follow a documented Orbis visual language for canvas, optical alignment, stroke weight, line endings, corner treatment, and detail density.

#### Scenario: Standard interface icon rendering
- **WHEN** an icon is rendered at a supported interface size
- **THEN** its geometry and visual weight are consistent with other icons in its semantic family

#### Scenario: Detailed symbol at small size
- **WHEN** an icon contains detail that is not legible at the standard small interface size
- **THEN** the detail is simplified or the icon is assigned a simpler approved variant

### Requirement: Clear outline and filled variant semantics
Icons MUST use outline or filled construction intentionally, and paired variants MUST remain recognizable as the same semantic symbol while each construction receives optical adjustment for its own visual weight.

#### Scenario: Filled variant is available
- **WHEN** an icon has both outline and filled variants
- **THEN** the variants preserve the same meaning and use the library's documented naming convention

#### Scenario: Existing icon has no paired variant
- **WHEN** an icon has no intentional alternate construction
- **THEN** consumers receive one canonical component rather than visually inconsistent duplicates

#### Scenario: Filled icon is reviewed at small size
- **WHEN** a filled icon is displayed at 16 CSS pixels or points
- **THEN** its silhouette and negative-space details remain open, balanced, and recognizable without relying on a stroke-only counterpart

### Requirement: Orbis glyphs are independently designed
Redesigned glyphs MUST use original Orbis-specific drawing decisions and MUST NOT be simple copies, fill conversions, or cosmetic restylings of another icon set.

#### Scenario: Filled variant is redesigned
- **WHEN** a filled variant is introduced or substantially redesigned
- **THEN** its silhouette, counters, and visual weight are deliberately drawn for Orbis and reviewed beside its paired outline form

#### Scenario: Reference products inform quality
- **WHEN** a reference product is used to evaluate icon quality
- **THEN** the review applies general qualities such as clarity and consistency without reproducing the reference's distinctive glyph artwork

### Requirement: Cross-platform icon equivalence
An icon with the same public name MUST convey the same symbol, silhouette, and default visual weight in Svelte and React Native.

#### Scenario: Same icon rendered on web and native
- **WHEN** a consumer renders the same named icon with equivalent size, color, and stroke properties on both platforms
- **THEN** the icon's shape and visual weight are materially equivalent

#### Scenario: Platform-specific rendering constraints
- **WHEN** a platform requires different rendering primitives
- **THEN** the difference does not change the icon's meaning, default proportions, or consumer-facing property semantics

### Requirement: Stable public icon API
Existing public icon names, package entry points, and supported component properties MUST remain usable during the visual redesign.

#### Scenario: Existing named import
- **WHEN** an existing consumer imports an icon by its current public name and platform entry point
- **THEN** the import continues to resolve to the redesigned icon without requiring a consumer migration

#### Scenario: Duplicate or deprecated symbol is consolidated
- **WHEN** an existing symbol is determined to be a duplicate or is replaced by a canonical variant
- **THEN** its old public name remains as a documented compatibility alias unless a separately approved breaking change authorizes removal

### Requirement: Accessible and theme-aware icon behavior
Icons MUST inherit consumer-selected color by default, remain decorative to assistive technology unless given an accessible name, and support explicit accessible properties.

#### Scenario: Decorative icon
- **WHEN** an icon supplements a visible label
- **THEN** it is hidden from the accessibility tree by default

#### Scenario: Icon conveys an action or status without visible text
- **WHEN** an icon is the only accessible representation of an action or status
- **THEN** the consumer can provide an accessible name using supported platform properties

#### Scenario: Icon is used in a themed interface
- **WHEN** a consumer does not set an explicit icon color
- **THEN** the icon follows the current text color rather than introducing a hard-coded theme color

#### Scenario: Icon communicates a control or state
- **WHEN** an icon's visible form is necessary to identify a control, state, or focus indicator
- **THEN** it has at least 3:1 contrast against adjacent colors where WCAG 2.2 AA non-text contrast applies, and state is not communicated by color alone

#### Scenario: Icon is placed inside an interactive target
- **WHEN** an icon is used in a web or native interactive control
- **THEN** the containing target meets the applicable WCAG 2.2 AA target-size criterion or its documented exception, and follows the host platform's touch-target guidance

### Requirement: Icons communicate familiar meanings without ambiguous affordances
Icons MUST use a recognizable symbol for their intended meaning and MUST NOT depend on decorative detail, color, or an undocumented convention to communicate an action or state.

#### Scenario: Common action uses an established metaphor
- **WHEN** an icon represents a common action or status
- **THEN** its meaning follows a familiar platform or product convention unless user evaluation shows that a distinct Orbis symbol is clearer

#### Scenario: Icon meaning is ambiguous without context
- **WHEN** a symbol can reasonably be interpreted as multiple actions or statuses
- **THEN** the consuming interface provides a visible label or a discoverable contextual label and an accessible name

#### Scenario: State distinction is required
- **WHEN** two icons or variants communicate different states
- **THEN** the distinction remains perceivable without color and is verified at the smallest supported size

### Requirement: Directional icons respect interface direction and meaning
Directional icons MUST distinguish between meaning-relative direction and absolute physical direction so interfaces can present them correctly in right-to-left and left-to-right contexts.

#### Scenario: Meaning-relative navigation in RTL
- **WHEN** a directional icon represents a logical backward, forward, previous, or next action in an RTL interface
- **THEN** the interface presents the direction appropriate to that locale without changing the icon's underlying meaning

#### Scenario: Absolute physical direction
- **WHEN** an icon represents an absolute physical direction, chart axis, or fixed spatial relationship
- **THEN** its direction is not automatically mirrored as if it were a navigation action

### Requirement: Icon designs are validated for user recognition
High-frequency and consequential icon meanings MUST be reviewed with representative users or a documented usability method before the visual language is applied across the catalog.

#### Scenario: Pilot symbol recognition
- **WHEN** the initial icon pilot is evaluated in realistic interface context
- **THEN** reviewers record recognition issues and revise any symbol that is repeatedly mistaken or misunderstood

#### Scenario: Icon-only consequential action
- **WHEN** an icon-only action can trigger a consequential or destructive outcome
- **THEN** its meaning is explicit before activation through an accessible name and a visible label or discoverable contextual explanation

### Requirement: Icon quality is reviewable before release
The library MUST provide a repeatable way to review the complete icon catalog and detect visual, API, or cross-platform regressions before changes are released.

#### Scenario: Catalog review
- **WHEN** a maintainer reviews a proposed icon change
- **THEN** the icon is visible with its name and relevant variants in a catalog at representative interface sizes

#### Scenario: Quality checks run
- **WHEN** icon source or public exports change
- **THEN** automated checks detect invalid source, missing platform counterparts, export drift, and violations of documented icon conventions
