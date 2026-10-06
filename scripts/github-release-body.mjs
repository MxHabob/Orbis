#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";
import { fileURLToPath } from "node:url";
import { validateReleaseNotes } from "./release-notes-validation.mjs";

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const tag = process.argv[2] ?? process.env.GITHUB_REF_NAME;
if (!/^v\d+\.\d+\.\d+$/.test(tag ?? "")) throw new Error(`Invalid release tag: ${tag}`);

const notesPath = path.join(rootDir, "releases", `${tag}.md`);
const body = fs.readFileSync(notesPath, "utf8").trim();
validateReleaseNotes(body, `Release notes at ${path.relative(rootDir, notesPath)}`);

const delimiter = `RELEASE_NOTES_${randomUUID()}`;
fs.appendFileSync(process.env.GITHUB_OUTPUT, `body<<${delimiter}\n${body}\n${delimiter}\n`);
