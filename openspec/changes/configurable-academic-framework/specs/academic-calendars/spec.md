# Spec Delta

## Purpose

تمكّن هذه القدرة المؤسسة من تعريف تقاويم أكاديمية وهياكل فترات تناسب دوراتها الفعلية، من دون فرض تقويم مدرسي أو دلالات تقييم على الفترات.

## ADDED Requirements

### Requirement: المؤسسة تعرّف تقاويمها الأكاديمية
The system SHALL allow each institution to create, configure, activate, close, and archive one or more academic calendars with institution-defined date ranges and labels.

#### Scenario: مؤسسة تعمل بدورات غير سنوية
- **WHEN** تنشئ المؤسسة دورة أكاديمية لا تطابق سنة تقويمية أو سنة دراسية تقليدية
- **THEN** يقبل النظام تواريخها وتسميتها دون فرض كيان سنة ثابت أو حدود سنوية

### Requirement: هيكل الفترات قابل للتكوين
The system SHALL allow institutions to define zero or more period levels, their parent-child relationships, order, dates, and optional operational windows without requiring month, term, or semester nodes.

#### Scenario: تقويم بلا فترات تقييم شهرية
- **WHEN** تفعّل مدرسة تقويمًا بفصلين فقط دون فترات شهرية
- **THEN** يقبل النظام الهيكل ولا ينشئ فترات شهرية ضمنيًا

#### Scenario: فترات وحدات تدريبية متداخلة
- **WHEN** يعرّف معهد دورة تحتوي وحدات تدريب متعددة ومستويات فرعية
- **THEN** يحفظ النظام العلاقات والترتيب والتواريخ المعرّفة دون تحويلها إلى فصول مدرسية

### Requirement: الفترة لا تحدد وظيفة أكاديمية ضمنية
The system SHALL keep period structure separate from assessment, enrollment, attendance, and reporting policies; each such policy MAY reference applicable periods explicitly.

#### Scenario: فترة تسجيل بلا درجات
- **WHEN** تكون الفترة مخصصة للتسجيل فقط ولا ترتبط بها قاعدة تقييم
- **THEN** لا يتوقع النظام إدخال درجات أو توليد نتائج لهذه الفترة

### Requirement: تواريخ الفترة وعلاقاتها صحيحة
The system SHALL reject periods whose end precedes their start or whose parent belongs to another institution or calendar.

#### Scenario: إنشاء فترة بعلاقة أب غير صالحة
- **WHEN** يرسل العميل فترة بتاريخ نهاية أسبق من البداية أو أب خارج تقويم المؤسسة
- **THEN** يرفض النظام الطلب برسالة تحقق واضحة دون حفظ الفترة
