# Spec Delta

## Purpose

تمكّن هذه القدرة المؤسسات من إدارة كتالوج المقررات وعروضها وتسجيلات المتعلمين ككيانات مترابطة لكنها مستقلة عن صفوف K-12 أو نوع المؤسسة.

## ADDED Requirements

### Requirement: تعريف المقرر مستقل عن عرضه
The system SHALL represent a course definition separately from each scheduled or otherwise delivered course offering.

#### Scenario: للمقرر أكثر من عرض
- **WHEN** يطرح المقرر نفسه لشعب أو فترات مختلفة
- **THEN** ترتبط العروض بتعريف المقرر دون دمج تسجيلاتها أو درجاتها

### Requirement: العرض الأكاديمي يحدد سياق التقديم
The system SHALL allow an offering to reference its course, organizational unit, instructional staff, optional period, section, capacity, and applicable credit or workload units.

#### Scenario: عرض مقرر بلا فترة تقويمية
- **WHEN** تنشئ مؤسسة عرضًا تدريبيًا مستمرًا لا يتبع فترة أكاديمية محددة
- **THEN** يقبل النظام العرض مع غياب مرجع الفترة

### Requirement: تسجيل المتعلم مرتبط بالعرض
The system SHALL record learner enrollment against a specific offering and preserve enrollment status and attempt identity independently from the learner's institution-level admission record.

#### Scenario: إعادة دراسة مقرر
- **WHEN** يسجل المتعلم مرة أخرى في عرض مقرر سبق أن درسه
- **THEN** ينشئ النظام تسجيل محاولة مستقلًا ويحافظ على السجل السابق

### Requirement: سياق التقييم يحترم تسجيل المتعلم
The system SHALL associate each course-level assessment result with the relevant learner enrollment and offering, and SHALL reject mismatched institution, learner, or offering references.

#### Scenario: درجة لتسجيل غير مطابق
- **WHEN** يسجل العميل درجة باستخدام تسجيل لا يخص المتعلم أو العرض المحدد
- **THEN** يرفض النظام العملية دون ربط الدرجة بتسجيل آخر
