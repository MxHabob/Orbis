/**
 * @orbis/data-grid — S.C.R.A.P.S. Custom Cell Renderers
 *
 * Cell renderers implemented using hyperscript for maximum performance (60fps)
 * without mounting Svelte components inside cells.
 */

// Stylesheet
import "./renderers.css";

// Renderer Factories
export { avatarRenderer, type AvatarRendererConfig } from "./avatar";
export { badgeRenderer, type BadgeRendererConfig } from "./badge";
export { actionsRenderer, type ActionsRendererConfig } from "./actions";
export { progressRenderer, type ProgressRendererConfig } from "./progress";
export { linkRenderer, type LinkRendererConfig } from "./link";
export { tagListRenderer, type TagListRendererConfig } from "./tag-list";
export { iconTextRenderer, type IconTextRendererConfig } from "./icon-text";

// Overlays
export { default as ActionOverlay } from "./ActionOverlay.svelte";
