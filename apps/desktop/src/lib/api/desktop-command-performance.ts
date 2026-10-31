import { invoke as tauriInvoke } from "@tauri-apps/api/core";

const MAX_SAMPLES_PER_COMMAND = 100;
let collectionEnabled = false;

interface CommandSamples {
  totalCalls: number;
  failedCalls: number;
  durationsMs: number[];
  nextSampleIndex: number;
}

export interface DesktopCommandPerformance {
  command: string;
  totalCalls: number;
  failedCalls: number;
  sampleCount: number;
  averageMs: number;
  p95Ms: number;
  maxMs: number;
}

const commandSamples = new Map<string, CommandSamples>();

function recordCommand(command: string, durationMs: number, failed: boolean) {
  let samples = commandSamples.get(command);
  if (!samples) {
    samples = {
      totalCalls: 0,
      failedCalls: 0,
      durationsMs: [],
      nextSampleIndex: 0,
    };
    commandSamples.set(command, samples);
  }

  samples.totalCalls += 1;
  if (failed) samples.failedCalls += 1;
  if (samples.durationsMs.length < MAX_SAMPLES_PER_COMMAND) {
    samples.durationsMs.push(durationMs);
  } else {
    samples.durationsMs[samples.nextSampleIndex] = durationMs;
    samples.nextSampleIndex =
      (samples.nextSampleIndex + 1) % MAX_SAMPLES_PER_COMMAND;
  }
}

export async function invokeDesktopCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  if (!collectionEnabled) return tauriInvoke<T>(command, args);

  const startedAt = performance.now();
  try {
    const result = await tauriInvoke<T>(command, args);
    recordCommand(command, performance.now() - startedAt, false);
    return result;
  } catch (error) {
    recordCommand(command, performance.now() - startedAt, true);
    throw error;
  }
}

export function startDesktopCommandPerformance() {
  resetDesktopCommandPerformance();
  collectionEnabled = true;
}

export function stopDesktopCommandPerformance() {
  collectionEnabled = false;
}

export function isDesktopCommandPerformanceActive() {
  return collectionEnabled;
}

export function getDesktopCommandPerformance(): DesktopCommandPerformance[] {
  return Array.from(commandSamples, ([command, samples]) => {
    const sortedDurations = [...samples.durationsMs].sort((a, b) => a - b);
    const p95Index = Math.max(0, Math.ceil(sortedDurations.length * 0.95) - 1);
    const totalDuration = sortedDurations.reduce(
      (total, duration) => total + duration,
      0,
    );

    return {
      command,
      totalCalls: samples.totalCalls,
      failedCalls: samples.failedCalls,
      sampleCount: sortedDurations.length,
      averageMs:
        sortedDurations.length > 0 ? totalDuration / sortedDurations.length : 0,
      p95Ms: sortedDurations[p95Index] ?? 0,
      maxMs: sortedDurations.at(-1) ?? 0,
    };
  }).sort((a, b) => b.p95Ms - a.p95Ms);
}

export function resetDesktopCommandPerformance() {
  commandSamples.clear();
}
