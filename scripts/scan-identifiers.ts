import fs from 'fs';
import path from 'path';

function getFiles(dir: string, exts = ['.svelte', '.ts', '.tsx']): string[] {
  let results: string[] = [];
  if (!fs.existsSync(dir)) return results;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== 'node_modules' && entry.name !== '.svelte-kit') {
        results.push(...getFiles(full, exts));
      }
    } else if (exts.some((ext) => entry.name.endsWith(ext))) {
      results.push(full);
    }
  }
  return results;
}

const files = [...getFiles('apps/desktop/src'), ...getFiles('packages/ui/src')];
const oldNames = [
  'LayoutDashboard', 'Megaphone', 'GraduationCap', 'FileText', 'FileSpreadsheet',
  'MoreHorizontal', 'MoreVertical', 'ShieldAlert', 'BrainCircuit', 'RefreshCw',
  'RefreshCcw', 'CheckCircle2', 'AlertCircle', 'AlertTriangle', 'TriangleAlert',
  'Paperclip', 'Award', 'Coins', 'ClipboardList', 'CalendarDays', 'CalendarIcon',
  'DoorOpen', 'DollarSign', 'Percent', 'Receipt', 'SlidersHorizontal', 'Sliders',
  'Landmark', 'WandSparkles', 'Zap', 'History'
];

let totalHits = 0;
for (const file of files) {
  const content = fs.readFileSync(file, 'utf8');
  for (const name of oldNames) {
    const regex = new RegExp(`\\b${name}\\b`, 'g');
    if (regex.test(content)) {
      console.log(`Found "${name}" in ${file}`);
      totalHits++;
    }
  }
}
console.log(`Total occurrences found: ${totalHits}`);
