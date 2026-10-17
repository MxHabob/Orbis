import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const projectRoot = process.cwd();
const env = { ...process.env, ORBIS_DEV_ALLOW_LOCAL_HTTP: "false" };

function run(command, args, cwd) {
  const result = spawnSync(command, args, { cwd, env, stdio: "inherit" });
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
}

run("bun", ["run", "expo", "prebuild", "--platform", "android"], projectRoot);

const androidRoot = resolve(projectRoot, "android");
if (process.platform === "win32") {
  run(process.env.ComSpec ?? "cmd.exe", ["/d", "/s", "/c", "gradlew.bat bundleRelease"], androidRoot);
} else {
  run("./gradlew", ["bundleRelease"], androidRoot);
}
