const fs = require('fs');

const arPath = 'd:/projects/Orbis/apps/desktop-svelte/messages/ar.json';
const enPath = 'd:/projects/Orbis/apps/desktop-svelte/messages/en.json';

const newAr = {
  "common_filter_classes": "الفصول",
  "common_filter_all_classes": "جميع الفصول",
  "common_dt_empty": "لا توجد قوالب خصم",
  "common_ft_empty_desc": "لا توجد قوالب رسوم",
  "common_sidebar_help_tour": "جولة في الواجهة",
  "common_sidebar_help_community": "المجتمع",
  "common_sidebar_help_discord": "ديسكورد",
  "common_sidebar_help_twitter": "تويتر / X",
  "common_sidebar_help_github": "جيت هب",
  "common_sidebar_help_legal": "الشؤون القانونية",
  "common_search_add": "إضافة",
  "common_refresh": "تحديث"
};

const newEn = {
  "common_filter_classes": "Classes",
  "common_filter_all_classes": "All Classes",
  "common_dt_empty": "No discount templates",
  "common_ft_empty_desc": "No fee templates",
  "common_sidebar_help_tour": "Tour the new navigation",
  "common_sidebar_help_community": "Community",
  "common_sidebar_help_discord": "Discord",
  "common_sidebar_help_twitter": "Twitter / X",
  "common_sidebar_help_github": "GitHub",
  "common_sidebar_help_legal": "Legal",
  "common_search_add": "Add",
  "common_refresh": "Refresh"
};

function updateFile(path, newData) {
  let data = JSON.parse(fs.readFileSync(path, 'utf8'));
  data = { ...data, ...newData };
  fs.writeFileSync(path, JSON.stringify(data, null, 2));
}

updateFile(arPath, newAr);
updateFile(enPath, newEn);
console.log("Updated translation files");
