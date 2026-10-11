# Spec Delta

## Purpose

تعرّف هذه القدرة قواعد حساب نتائج أكاديمية دقيقة وقابلة لإعادة الاستخدام والتكوين، وتفصل تنفيذ الحساب عن نوع المؤسسة وعن شكل التقرير.

## ADDED Requirements

### Requirement: قواعد الحساب محددة الأنواع وقابلة للتحقق
The system SHALL represent calculation inputs, operators, measures, and outputs with explicit types and SHALL validate rule references, units, and operand compatibility before activation.

#### Scenario: معادلة تشير إلى مكون غير موجود
- **WHEN** تحتوي قاعدة حساب على مرجع غير معرف في مدخلاتها
- **THEN** يرفض النظام تفعيل القاعدة ويحدد المرجع غير الصالح

### Requirement: الحسابات الرقمية دقيقة وحتمية
The system SHALL calculate numeric results using decimal arithmetic and SHALL apply the configured conversion and rounding rules in a deterministic order.

#### Scenario: التقريب وفق قاعدة المؤسسة
- **WHEN** تنتج المعادلة قيمة 86.665 وتحدد السياسة منزلتين وتقريبًا نصفه للأعلى
- **THEN** تنتج كل عملية حساب على المدخلات والإصدار نفسيهما القيمة 86.67

#### Scenario: قسمة على صفر
- **WHEN** تتطلب المعادلة قسمة على صفر
- **THEN** تفشل العملية بخطأ حساب واضح ولا تنتج قيمة نجاح افتراضية

### Requirement: الأوزان والتحويلات تعتمد على المقاييس المعلنة
The system SHALL support weighted and normalized calculations across compatible declared scales and SHALL reject incompatible scales unless an explicit conversion rule exists.

#### Scenario: تحويل مكون ذي حد مختلف
- **WHEN** تجمع السياسة مكونًا من 20 درجة مع مكون من 30 درجة ضمن مقياس نهائي من 100
- **THEN** تطبق التحويلات والأوزان المعرفة صراحة قبل التجميع

### Requirement: التجميعات محددة بسياسة المؤسسة
The system SHALL allow institutions to define one or more named aggregations from typed inputs and outputs without requiring annual, term, or other predefined summary levels.

#### Scenario: نتيجة دورة قصيرة
- **WHEN** تكتفي المؤسسة بمخرج إجمالي لدورة واحدة دون فصل أو سنة
- **THEN** يحسب النظام المخرج المعرف وحده دون إنشاء ملخصات زمنية إضافية

### Requirement: قواعد النجاح والتحويل إلى تقدير مستقلة عن الحساب
The system SHALL allow configurable threshold, categorical, or grade-scale rules to derive pass status and grade representations from calculated measures.

#### Scenario: تقدير حرفي بمقياس مؤسسي
- **WHEN** تطبق المؤسسة نطاقات التقدير A إلى F وحد نجاح خاصًا بها
- **THEN** يشتق النظام التقدير وحالة النجاح وفق الإصدار الفعال من القواعد لا وفق حدود افتراضية ثابتة

### Requirement: المعدلات والترتيب اختياريان ومضبوطان
The system SHALL calculate credit-weighted averages and rankings only when enabled by policy, using explicit inclusion, tie, and population rules.

#### Scenario: سياسة لا تستخدم الترتيب
- **WHEN** تعطل المؤسسة ترتيب المتعلمين في مخرجاتها
- **THEN** لا يحسب النظام ترتيبًا ولا يضيف حقلًا بترتيب مفترض

#### Scenario: معدّل جامعي موزون
- **WHEN** تحسب سياسة الجامعة المعدل من نقاط المقرر وساعاته المحتسبة
- **THEN** تقسم مجموع النقاط الموزونة على مجموع الساعات المؤهلة وفق قواعد الاستبعاد المحددة
