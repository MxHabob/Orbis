import * as fs from "fs";
import * as path from "path";

interface Column {
  name: string;
  type: string;
  isNullable: boolean;
  isPrimaryKey?: boolean;
}

interface Table {
  name: string;
  columns: Column[];
}

function mapSql(col: Column): string {
  switch (col.type) {
    case "uuid":
    case "varchar":
    case "text":
    case "date":
    case "timestamp":
    case "time":
    case "jsonb":
      return "TEXT";
    case "integer":
      return "INTEGER";
    case "boolean":
      return "INTEGER";
    case "double":
    case "numeric":
      return "REAL";
    default:
      return "TEXT";
  }
}

const src = fs.readFileSync(
  path.resolve(__dirname, "generate-sea-entities.ts"),
  "utf8",
);

// Extract table definitions via regex
const tables: Table[] = [];
const tableRegex = /name:\s*"([^"]+)"[\s\S]*?columns:\s*\[([\s\S]*?)\]\s*\}/g;
let m: RegExpExecArray | null;
while ((m = tableRegex.exec(src)) !== null) {
  const name = m[1];
  const colsBlock = m[2];
  const colRegex =
    /\{\s*name:\s*"([^"]+)",\s*type:\s*"([^"]+)",\s*isNullable:\s*(true|false)(?:,\s*isPrimaryKey:\s*true)?\s*\}/g;
  const columns: Column[] = [];
  let cm: RegExpExecArray | null;
  while ((cm = colRegex.exec(colsBlock)) !== null) {
    const snippet = colsBlock.substring(cm.index, cm.index + 120);
    columns.push({
      name: cm[1],
      type: cm[2],
      isNullable: cm[3] === "true",
      isPrimaryKey: snippet.includes("isPrimaryKey"),
    });
  }
  if (columns.length > 0) {
    tables.push({ name, columns });
  }
}

let sql =
  "-- Orbis SQLite Schema (auto-generated)\nPRAGMA foreign_keys=ON;\n\n";

for (const table of tables) {
  const pkCols = table.columns.filter((c) => c.isPrimaryKey);
  const colDefs = table.columns
    .map((c) => {
      const colName = c.name === "type" ? '"type"' : c.name;
      let def = `${colName} ${mapSql(c)}`;
      if (c.isPrimaryKey && pkCols.length === 1) def += " PRIMARY KEY";
      else if (!c.isNullable) def += " NOT NULL";
      return `  ${def}`;
    })
    .join(",\n");

  let pkConstraint = "";
  if (pkCols.length > 1) {
    const pkNames = pkCols
      .map((c) => (c.name === "type" ? '"type"' : c.name))
      .join(", ");
    pkConstraint = `,\n  PRIMARY KEY (${pkNames})`;
  }

  sql += `CREATE TABLE IF NOT EXISTS ${table.name} (\n${colDefs}${pkConstraint}\n);\n\n`;
}

const out = path.resolve(
  __dirname,
  "../apps/api-rust/crates/core-domain/src/database/schema.sql",
);
fs.writeFileSync(out, sql, "utf8");
console.log(`✅ Wrote ${out} (${tables.length} tables)`);
