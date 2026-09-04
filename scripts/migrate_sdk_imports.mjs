import fs from 'fs';
import path from 'path';

function walkDir(dir, callback) {
  fs.readdirSync(dir).forEach(f => {
    const dirPath = path.join(dir, f);
    const isDirectory = fs.statSync(dirPath).isDirectory();
    if (isDirectory) {
        walkDir(dirPath, callback);
    } else {
        callback(path.join(dir, f));
    }
  });
}

let updated = 0;

['apps/desktop/src', 'apps/mobile/src'].forEach(dir => {
    if (fs.existsSync(dir)) {
        walkDir(dir, (file) => {
            if (file.endsWith('.ts') || file.endsWith('.svelte')) {
                let content = fs.readFileSync(file, 'utf-8');
                const original = content;
                
                content = content.replace(/@orbis\/sdk-core/g, '@orbis/sdk');
                content = content.replace(/@orbis\/sdk-desktop/g, '@orbis/sdk');
                
                if (content !== original) {
                  fs.writeFileSync(file, content);
                  updated++;
                  console.log(`Updated ${file}`);
                }
            }
        });
    }
});

console.log(`Done. Updated ${updated} files.`);
