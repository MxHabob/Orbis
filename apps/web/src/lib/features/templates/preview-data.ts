import { getPages, type TemplateDocument, type TemplateNode } from "@docforge/core";
import { templateOrganization } from "./template-brand";

type Data = Record<string, unknown>;

export function templatePreviewData(document: TemplateDocument): Data {
  const identity = templateOrganization();
  const data: Data = { org: identity, $org: identity };
  const rtl = document.meta.locale.startsWith("ar");

  function insert(path: string, value: unknown) {
    const parts = path.split(".");
    if (parts.some((key) => !/^[\w$-]+$/.test(key) || key in Object.prototype)) return;
    if (parts[0] === "org" || parts[0] === "$org") return;
    let target = data;
    for (const key of parts.slice(0, -1)) {
      target[key] ??= {};
      const child = target[key];
      if (typeof child !== "object" || child === null || Array.isArray(child)) return;
      target = child as Data;
    }
    target[parts[parts.length - 1]] = value;
  }

  for (const [key, variable] of Object.entries(document.variables ?? {})) {
    if (variable.type === "image" || variable.source === "$computed") continue;
    const label = rtl ? variable.labelAr || variable.label : variable.label || variable.labelAr;
    const sample = {
      string: variable.defaultValue ?? label ?? "—",
      number: 825.50,
      date: "2026-01-15",
      boolean: true,
      array: [],
    }[variable.type];
    insert(variable.source, sample);
    if (key !== variable.source) insert(key, sample);
  }

  function addTables(node: TemplateNode) {
    if (node.type === "Table" && node.props.dataSource) {
      const columns = node.props.columns.flatMap((column) =>
        column.field && column.isVisible !== false
          ? [{ field: column.field, label: rtl ? column.headerAr || column.header : column.header }]
          : [],
      );
      insert(node.props.dataSource, [1, 2, 3].map((row) =>
        Object.fromEntries(columns.map((column) => [
          column.field, `${row} · ${column.label}`,
        ])),
      ));
    }
    if ("children" in node) node.children?.forEach(addTables);
  }

  getPages(document).forEach(addTables);
  return data;
}
