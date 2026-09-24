import fs from "node:fs";

const ar = JSON.parse(fs.readFileSync("messages/ar.json", "utf-8"));
const en = JSON.parse(fs.readFileSync("messages/en.json", "utf-8"));

ar["academics_reports_desc"] = "حدد قالب التقرير للمعاينة أو التصدير أو الطباعة.";
en["academics_reports_desc"] = "Select a report template to preview, export, or print.";

ar["academics_reports_empty"] = "لا توجد تقارير متاحة.";
en["academics_reports_empty"] = "No report definitions found.";

fs.writeFileSync("messages/ar.json", JSON.stringify(ar, null, 2) + "\n", "utf-8");
fs.writeFileSync("messages/en.json", JSON.stringify(en, null, 2) + "\n", "utf-8");

console.log("Added academics_reports_desc and academics_reports_empty");
