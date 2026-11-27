const GITHUB_RELEASES_URL =
  "https://api.github.com/repos/MxHabob/Orbis/releases?per_page=30";
const MOBILE_RELEASE_TAG_PREFIX = "mobile-v";
const REQUEST_TIMEOUT_MS = 10_000;

type GitHubRelease = {
  draft?: boolean;
  prerelease?: boolean;
  tag_name?: string;
  html_url?: string;
};

export type MobileAppUpdateCheck =
  | {
      status: "up_to_date";
      currentVersion: string;
      latestVersion: string;
    }
  | {
      status: "update_available";
      currentVersion: string;
      latestVersion: string;
      releaseUrl: string;
    }
  | { status: "no_mobile_release" };

function parseSemver(version: string): [number, number, number] | null {
  const match = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.exec(
    version.trim(),
  );
  if (!match) return null;
  return [Number(match[1]), Number(match[2]), Number(match[3])];
}

function compareSemver(
  a: [number, number, number],
  b: [number, number, number],
) {
  for (let i = 0; i < 3; i += 1) {
    if (a[i] !== b[i]) return a[i] - b[i];
  }
  return 0;
}

function isValidReleaseUrl(url: string): boolean {
  try {
    const parsed = new URL(url);
    return (
      parsed.protocol === "https:" &&
      parsed.hostname === "github.com" &&
      parsed.pathname.startsWith("/MxHabob/Orbis/releases/tag/")
    );
  } catch {
    return false;
  }
}

export async function checkForMobileAppUpdate(
  currentVersion: string,
): Promise<MobileAppUpdateCheck> {
  const currentParts = parseSemver(currentVersion);
  if (!currentParts) {
    throw new Error(`Invalid installed app version: "${currentVersion}".`);
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);

  try {
    const response = await fetch(GITHUB_RELEASES_URL, {
      headers: {
        Accept: "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
      },
      signal: controller.signal,
    });

    if (!response.ok) {
      throw new Error(`GitHub release check failed (${response.status}).`);
    }

    const releases: unknown = await response.json();
    if (!Array.isArray(releases)) {
      throw new Error("Invalid GitHub releases response.");
    }

    let latest: {
      version: string;
      parts: [number, number, number];
      url: string;
    } | null = null;

    for (const item of releases as GitHubRelease[]) {
      if (
        item.draft ||
        item.prerelease ||
        typeof item.tag_name !== "string" ||
        typeof item.html_url !== "string" ||
        !item.tag_name.startsWith(MOBILE_RELEASE_TAG_PREFIX) ||
        !isValidReleaseUrl(item.html_url)
      ) {
        continue;
      }

      const version = item.tag_name.slice(MOBILE_RELEASE_TAG_PREFIX.length);
      const parts = parseSemver(version);
      if (!parts) continue;

      if (!latest || compareSemver(parts, latest.parts) > 0) {
        latest = { version, parts, url: item.html_url };
      }
    }

    if (!latest) return { status: "no_mobile_release" };

    return compareSemver(latest.parts, currentParts) > 0
      ? {
          status: "update_available",
          currentVersion,
          latestVersion: latest.version,
          releaseUrl: latest.url,
        }
      : {
          status: "up_to_date",
          currentVersion,
          latestVersion: latest.version,
        };
  } finally {
    clearTimeout(timeout);
  }
}
