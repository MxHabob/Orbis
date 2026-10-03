import fs from "node:fs";

const ar = JSON.parse(fs.readFileSync("messages/ar.json", "utf-8"));
const en = JSON.parse(fs.readFileSync("messages/en.json", "utf-8"));

ar["common_promo_mapping"] = "تعيين الفصول والشعب";
en["common_promo_mapping"] = "Class Mapping";

fs.writeFileSync("messages/ar.json", JSON.stringify(ar, null, 2) + "\n", "utf-8");
fs.writeFileSync("messages/en.json", JSON.stringify(en, null, 2) + "\n", "utf-8");

console.log("Added common_promo_mapping");
