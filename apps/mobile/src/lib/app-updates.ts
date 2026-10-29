const GITHUB_RELEASES_URL =
  "https://api.github.com/repos/MxHabob/Orbis/releases";
const MOBILE_RELEASE_TAG_PREFIX = "mobile-v";
const RELEASES_PER_PAGE = 100;
const REQUEST_TIMEOUT_MS = 15_000;

type GitHubRelease = {
  draft: boolean;
  prerelease: boolean;
  tag_name: string;
  html_url: string;
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

function parseVersion(version: string): [number, number, number] | null {
  const match = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.exec(version);
  if (!match) return null;

  const parts: [number, number, number] = [
    Number(match[1]),
    Number(match[2]),
    Number(match[3]),
  ];
  if (parts.some((part) => !Number.isSafeInteger(part))) return null;
  return parts;
}

function compareVersions(left: string, right: string): number {
  const leftParts = parseVersion(left);
  const rightParts = parseVersion(right);
  if (!leftParts || !rightParts) {
    throw new Error(
      "App versions must use the stable MAJOR.MINOR.PATCH format.",
    );
  }

  for (let index = 0; index < leftParts.length; index += 1) {
    const difference = leftParts[index] - rightParts[index];
    if (difference !== 0) return Math.sign(difference);
  }

  return 0;
}

function isGitHubRelease(value: unknown): value is GitHubRelease {
  if (typeof value !== "object" || value === null) return false;
  const release = value as Record<string, unknown>;
  return (
    typeof release.draft === "boolean" &&
    typeof release.prerelease === "boolean" &&
    typeof release.tag_name === "string" &&
    typeof release.html_url === "string"
  );
}

function getMobileVersion(tag: string): string | null {
  if (!tag.startsWith(MOBILE_RELEASE_TAG_PREFIX)) return null;
  const version = tag.slice(MOBILE_RELEASE_TAG_PREFIX.length);
  if (!parseVersion(version)) {
    throw new Error(`Mobile release tag "${tag}" has an invalid version.`);
  }
  return version;
}

function validateReleaseUrl(value: string): string {
  const releaseUrl = new URL(value);
  if (
    releaseUrl.protocol !== "https:" ||
    releaseUrl.hostname !== "github.com" ||
    !releaseUrl.pathname.startsWith("/MxHabob/Orbis/releases/tag/")
  ) {
    throw new Error(
      "The mobile release URL is not a valid Orbis GitHub release.",
    );
  }
  return releaseUrl.toString();
}

export async function checkForMobileAppUpdate(
  currentVersion: string,
): Promise<MobileAppUpdateCheck> {
  if (!parseVersion(currentVersion)) {
    throw new Error(
      `Installed app version "${currentVersion}" is not a valid MAJOR.MINOR.PATCH version.`,
    );
  }

  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);

  try {
    let latestMobileRelease: {
      version: string;
      url: string;
    } | null = null;

    for (let page = 1; ; page += 1) {
      const response = await fetch(
        `${GITHUB_RELEASES_URL}?per_page=${RELEASES_PER_PAGE}&page=${page}`,
        {
          headers: {
            Accept: "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
          },
          signal: controller.signal,
        },
      );

      if (!response.ok) {
        throw new Error(
          `GitHub release check failed with HTTP ${response.status}.`,
        );
      }

      const payload: unknown = await response.json();
      if (!Array.isArray(payload) || !payload.every(isGitHubRelease)) {
        throw new Error("GitHub returned an invalid release list.");
      }

      for (const release of payload) {
        if (release.draft || release.prerelease) continue;
        const latestVersion = getMobileVersion(release.tag_name);
        if (!latestVersion) continue;

        if (
          !latestMobileRelease ||
          compareVersions(latestVersion, latestMobileRelease.version) > 0
        ) {
          latestMobileRelease = {
            version: latestVersion,
            url: release.html_url,
          };
        }
      }

      if (payload.length < RELEASES_PER_PAGE) break;
    }

    if (!latestMobileRelease) {
      return { status: "no_mobile_release" };
    }

    return compareVersions(latestMobileRelease.version, currentVersion) > 0
      ? {
          status: "update_available",
          currentVersion,
          latestVersion: latestMobileRelease.version,
          releaseUrl: validateReleaseUrl(latestMobileRelease.url),
        }
      : {
          status: "up_to_date",
          currentVersion,
          latestVersion: latestMobileRelease.version,
        };
  } finally {
    clearTimeout(timeout);
  }
}
