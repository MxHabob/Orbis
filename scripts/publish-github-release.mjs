#!/usr/bin/env node
import fs from "node:fs";
import { spawnSync } from "node:child_process";

const repository = process.env.GITHUB_REPOSITORY;
const tag = process.env.RELEASE_TAG;
const version = process.env.RELEASE_VERSION ?? tag?.replace(/^v/, "");

if (!/^[^/]+\/[^/]+$/.test(repository ?? "")) {
  throw new Error(`Invalid GitHub repository name: ${repository}.`);
}
if (!/^v\d+\.\d+\.\d+$/.test(tag ?? "")) {
  throw new Error(`Invalid release tag: ${tag}.`);
}
if (version !== tag.slice(1)) {
  throw new Error(`Release version ${version} does not match tag ${tag}.`);
}

function runGh(args, options = {}) {
  const result = spawnSync("gh", args, {
    maxBuffer: 5 * 1024 * 1024,
    ...options,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    const stderr = result.stderr?.toString("utf8").trim();
    throw new Error(
      `gh ${args.slice(0, 2).join(" ")} failed${stderr ? `: ${stderr}` : "."}`,
    );
  }
  return result.stdout;
}

const releasePages = JSON.parse(
  runGh(
    [
      "api",
      "--paginate",
      "--slurp",
      `repos/${repository}/releases?per_page=100`,
    ],
    { encoding: "utf8" },
  ),
);
if (
  !Array.isArray(releasePages) ||
  releasePages.some((page) => !Array.isArray(page))
) {
  throw new Error(
    "GitHub Releases API returned an invalid paginated response.",
  );
}

const matches = releasePages
  .flat()
  .filter((release) => release.tag_name === tag);
if (matches.length !== 1) {
  throw new Error(
    `Expected one GitHub Release for ${tag}; found ${matches.length}.`,
  );
}

const [release] = matches;
if (!release.draft) {
  throw new Error(`Expected ${tag} to identify a draft GitHub Release.`);
}
if (!Number.isSafeInteger(release.id) || release.id < 1) {
  throw new Error(`GitHub Release ${tag} has an invalid ID.`);
}
if (!Array.isArray(release.assets)) {
  throw new Error(`GitHub Release ${tag} is missing its assets.`);
}
fs.mkdirSync("release-check", { recursive: true });

const findAsset = (pattern, label) => {
  const assets = release.assets.filter((asset) => pattern.test(asset.name));
  if (assets.length !== 1) {
    throw new Error(
      `Expected one ${label} release asset; found ${assets.length}.`,
    );
  }
  return assets[0];
};
const findAssetNamed = (name, label) => {
  const assets = release.assets.filter(
    (asset) => asset.name.toLowerCase() === name.toLowerCase(),
  );
  if (assets.length !== 1) {
    throw new Error(
      `Expected one ${label} release asset; found ${assets.length}.`,
    );
  }
  return assets[0];
};
const windowsInstaller = findAsset(/-setup\.exe$/i, "Windows NSIS installer");
const macosArchive = findAsset(/\.app\.tar\.gz$/i, "macOS updater archive");
const requiredAssets = [
  findAssetNamed("latest.json", "Tauri updater JSON"),
  findAssetNamed(`${windowsInstaller.name}.sig`, "Windows NSIS signature"),
  findAssetNamed(`${macosArchive.name}.sig`, "macOS updater signature"),
];

for (const asset of requiredAssets) {
  if (!Number.isSafeInteger(asset.id) || asset.id < 1) {
    throw new Error(`GitHub Release asset has an invalid ID: ${asset.name}.`);
  }
  if (/[\\/]/.test(asset.name)) {
    throw new Error(
      `GitHub Release asset has an invalid filename: ${asset.name}.`,
    );
  }
  const contents = runGh(
    [
      "api",
      `repos/${repository}/releases/assets/${asset.id}`,
      "-H",
      "Accept: application/octet-stream",
    ],
    { encoding: null },
  );
  if (!Buffer.isBuffer(contents) || contents.length === 0) {
    throw new Error(`GitHub Release asset ${asset.name} is empty.`);
  }
  fs.writeFileSync(`release-check/${asset.name}`, contents);
}
fs.writeFileSync(
  "release-check/release.json",
  JSON.stringify(release, null, 2),
);

const names = new Set(release.assets.map((asset) => asset.name.toLowerCase()));
const has = (pattern) => [...names].some((name) => pattern.test(name));
for (const [label, pattern] of [
  ["Windows NSIS installer", /-setup\.exe$/],
  ["Windows NSIS updater signature", /-setup\.exe\.sig$/],
  ["macOS disk image", /\.dmg$/],
  ["macOS updater archive", /\.app\.tar\.gz$/],
  ["macOS updater signature", /\.app\.tar\.gz\.sig$/],
  ["Tauri updater JSON", /^latest\.json$/],
  ["Android APK", new RegExp(`^orbis-${version}-android-arm64\\.apk$`, "i")],
  [
    "Android App Bundle",
    new RegExp(`^orbis-${version}-android-arm64\\.aab$`, "i"),
  ],
]) {
  if (!has(pattern)) throw new Error(`Release is missing ${label}.`);
}

const metadata = JSON.parse(
  fs.readFileSync("release-check/latest.json", "utf8"),
);
if (metadata.version?.replace(/^v/, "") !== version) {
  throw new Error(
    `Updater metadata version ${metadata.version} does not match ${version}.`,
  );
}

const [repositoryOwner, repositoryName] = repository.split("/");
for (const target of ["windows-x86_64", "darwin-aarch64"]) {
  const platform = metadata.platforms?.[target];
  if (!platform?.url || !platform?.signature) {
    throw new Error(`Updater metadata is missing a signed ${target} update.`);
  }

  const updateUrl = new URL(platform.url);
  if (
    updateUrl.protocol !== "https:" ||
    updateUrl.username ||
    updateUrl.password ||
    updateUrl.port ||
    updateUrl.search ||
    updateUrl.hash
  ) {
    throw new Error(
      `Updater URL for ${target} must be a clean HTTPS GitHub asset URL: ${platform.url}`,
    );
  }

  const pathParts = updateUrl.pathname.split("/");
  if (pathParts[0] !== "" || pathParts.slice(1).some((part) => !part)) {
    throw new Error(
      `Updater URL for ${target} has an invalid path: ${platform.url}`,
    );
  }

  const urlParts = pathParts.slice(1).map(decodeURIComponent);
  let fileName;
  if (updateUrl.hostname.toLowerCase() === "github.com") {
    const [
      urlOwner,
      urlRepository,
      releaseSegment,
      downloadSegment,
      releaseTag,
      urlFileName,
    ] = urlParts;
    if (
      urlParts.length !== 6 ||
      urlOwner?.toLowerCase() !== repositoryOwner.toLowerCase() ||
      urlRepository?.toLowerCase() !== repositoryName.toLowerCase() ||
      releaseSegment !== "releases" ||
      downloadSegment !== "download" ||
      releaseTag !== tag ||
      !urlFileName
    ) {
      throw new Error(
        `Updater URL for ${target} must point to this GitHub Release: ${platform.url}`,
      );
    }
    fileName = urlFileName;
  } else if (updateUrl.hostname.toLowerCase() === "api.github.com") {
    const [
      reposSegment,
      urlOwner,
      urlRepository,
      releasesSegment,
      assetsSegment,
      assetId,
    ] = urlParts;
    if (
      urlParts.length !== 6 ||
      reposSegment !== "repos" ||
      urlOwner?.toLowerCase() !== repositoryOwner.toLowerCase() ||
      urlRepository?.toLowerCase() !== repositoryName.toLowerCase() ||
      releasesSegment !== "releases" ||
      assetsSegment !== "assets" ||
      !/^\d+$/.test(assetId ?? "")
    ) {
      throw new Error(
        `Updater URL for ${target} must point to this GitHub Release: ${platform.url}`,
      );
    }
    const asset = release.assets.find(
      (candidate) => String(candidate.id) === assetId,
    );
    if (!asset) {
      throw new Error(
        `Updater URL for ${target} does not identify an asset in this GitHub Release: ${platform.url}`,
      );
    }
    fileName = asset.name;
  } else {
    throw new Error(
      `Updater URL for ${target} must point to this GitHub Release: ${platform.url}`,
    );
  }

  if (/[\\/]/.test(fileName)) {
    throw new Error(
      `Updater URL for ${target} does not identify a valid release asset filename.`,
    );
  }
  if (!fileName || !names.has(fileName.toLowerCase())) {
    throw new Error(
      `Updater URL for ${target} does not match a release asset.`,
    );
  }

  const signatureFileName = `${fileName}.sig`;
  if (!names.has(signatureFileName.toLowerCase())) {
    throw new Error(`Release is missing the updater signature for ${target}.`);
  }
  const signature = fs
    .readFileSync(`release-check/${signatureFileName}`, "utf8")
    .trim();
  if (signature !== platform.signature.trim()) {
    throw new Error(
      `Updater signature for ${target} does not match its release signature asset.`,
    );
  }
  if (target === "windows-x86_64" && !/-setup\.exe$/i.test(fileName)) {
    throw new Error(
      `Windows updater URL must point to the signed NSIS setup executable; received ${fileName}.`,
    );
  }
  if (target === "darwin-aarch64" && !/\.app\.tar\.gz$/i.test(fileName)) {
    throw new Error(
      `macOS updater URL must point to the signed app archive; received ${fileName}.`,
    );
  }
}

const publishedRelease = JSON.parse(
  runGh(
    [
      "api",
      "--method",
      "PATCH",
      `repos/${repository}/releases/${release.id}`,
      "-F",
      "draft=false",
    ],
    { encoding: "utf8" },
  ),
);
if (publishedRelease.tag_name !== tag || publishedRelease.draft !== false) {
  throw new Error(`GitHub did not confirm that release ${tag} was published.`);
}

console.log(`Validated and published GitHub Release ${tag}.`);
