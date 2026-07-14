const fs = require('fs');

const files = [
  'iam/thin.rs',
  'organization/thin.rs',
  'academic/thin.rs',
  'scheduling/thin.rs',
  'finance/thin.rs',
  'assessment/thin.rs',
  'exam/thin.rs'
].map(f => 'apps/desktop-svelte/src-tauri/src/commands/' + f);

const regex1 = /^\s*use sea_orm::EntityTrait;\n\s*let db = state\.db\(\);\n\s*let \w+ = [a-zA-Z0-9_:]+::Entity::find_by_id\([^\)]+\)\n\s*\.one\(&db\)\.await\.map_err\(CommandError::from\)\?\n\s*\.ok_or_else\(\|\| CommandError::not_found\("[^"]+"\)\)\?;\n/gm;

const regex2 = /^\s*let db = state\.db\(\);\n\s*let \w+ = [a-zA-Z0-9_:]+::Entity::find_by_id\([^\)]+\)\n\s*\.one\(&db\)\.await\.map_err\(CommandError::from\)\?\n\s*\.ok_or_else\(\|\| CommandError::not_found\("[^"]+"\)\)\?;\n/gm;

for (const file of files) {
  if (!fs.existsSync(file)) continue;
  let content = fs.readFileSync(file, 'utf8');
  
  let newContent = content.replace(regex1, '');
  newContent = newContent.replace(regex2, '');
  
  if (content !== newContent) {
    fs.writeFileSync(file, newContent, 'utf8');
    console.log(`Cleaned ${file}`);
  }
}
