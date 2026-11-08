import { Marked } from "marked";

function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function safeExternalUrl(value: string): string | null {
  try {
    const url = new URL(value);
    return url.protocol === "https:" ? url.href : null;
  } catch {
    return null;
  }
}

function safeImageUrl(value: string): string | null {
  const safeUrl = safeExternalUrl(value);
  if (!safeUrl) return null;
  const url = new URL(safeUrl);
  const isGitHubImageHost =
    url.hostname === "github.com" ||
    url.hostname === "githubusercontent.com" ||
    url.hostname.endsWith(".githubusercontent.com");
  return isGitHubImageHost ? safeUrl : null;
}

const parser = new Marked({
  gfm: true,
  breaks: false,
  renderer: {
    html({ text }) {
      return escapeHtml(text);
    },
    link({ href, title, tokens }) {
      const safeHref = safeExternalUrl(href);
      const content = this.parser.parseInline(tokens);
      if (!safeHref) return content;
      const titleAttribute = title ? ` title="${escapeHtml(title)}"` : "";
      return `<a href="${escapeHtml(safeHref)}"${titleAttribute} target="_blank" rel="noopener noreferrer">${content}</a>`;
    },
    image({ href, title, text }) {
      const safeSrc = safeImageUrl(href);
      if (!safeSrc) return escapeHtml(text);
      const titleAttribute = title ? ` title="${escapeHtml(title)}"` : "";
      return `<img src="${escapeHtml(safeSrc)}" alt="${escapeHtml(text)}"${titleAttribute} loading="lazy" referrerpolicy="no-referrer">`;
    },
  },
});

export function renderReleaseMarkdown(markdown?: string | null): string {
  if (!markdown?.trim()) return "";
  const rendered = parser.parse(markdown);
  if (typeof rendered !== "string") {
    throw new Error(
      "Release markdown parser unexpectedly returned an asynchronous result.",
    );
  }
  return rendered;
}
