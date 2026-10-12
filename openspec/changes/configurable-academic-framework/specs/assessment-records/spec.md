# Spec Delta

## Purpose

تحدد هذه القدرة كيفية تعريف أدوات التقييم وعناصرها وتسجيل نتائج المتعلمين بأنواع قياس متعددة، دون افتراض امتحان أو درجة رقمية واحدة.

## ADDED Requirements

### Requirement: أدوات التقييم معرفة بمقاييس صريحة
The system SHALL define assessment instruments and components with explicit measurement types, valid scales or value ranges, labels, and requiredness.

#### Scenario: تقييم كفاءة نوعي
- **WHEN** يعرّف المعهد مكونًا يقيس كفاءة بحالات إتقان محددة
- **THEN** يحفظ النظام قيم المكون وفق مقياس الكفاءة دون تحويلها قسرًا إلى نسبة مئوية

### Requirement: التقييم يربط بموضعه الأكاديمي صراحة
The system SHALL allow an assessment instance to reference its applicable offering, learner enrollment, and period independently, with each reference optional only when the assessment policy permits it.

#### Scenario: تقييم عملي خلال وحدة
- **WHEN** ينشئ مستخدم تقييمًا عمليًا لوحدة مرتبطة بعرض مقرر وفترة
- **THEN** يحفظ النظام المراجع الثلاثة ويعرض التقييم في سياقها

#### Scenario: تقييم بلا تقسيم فتراتي
- **WHEN** تسمح سياسة المؤسسة بتقييم دورة مستمرة دون فترة أكاديمية
- **THEN** يقبل النظام التقييم دون period identifier

### Requirement: حالة التقييم تميّز النقص عن الإعفاء
The system SHALL distinguish an unrecorded result from an absent, excused, exempt, incomplete, or submitted result.

#### Scenario: مكون لم تسجل درجته بعد
- **WHEN** لم يسجل المقيم أي نتيجة لمكون مطلوب
- **THEN** يبقى المكون غير مكتمل ولا يعامل كدرجة صفر أو كإعفاء

### Requirement: حفظ نتيجة التقييم يتحقق من المقياس
The system SHALL validate each submitted result against the assessment component's declared type and scale before persistence.

#### Scenario: قيمة خارج النطاق
- **WHEN** يرسل العميل قيمة رقمية خارج حدود المقياس المحدد
- **THEN** يرفض النظام النتيجة مع تحديد المكون والمدى المقبول

#### Scenario: قيمة من نوع غير متوافق
- **WHEN** يرسل العميل نصًا لمكون معرف بمقياس رقمي
- **THEN** يرفض النظام القيمة دون تخزينها كدرجة صالحة

### Requirement: تغييرات الدرجات قابلة للتدقيق
The system SHALL retain who changed a recorded assessment result, when it changed, and the reason when a prior value is corrected.

#### Scenario: تعديل درجة مسجلة
- **WHEN** يصحح مستخدم نتيجة سابقة
- **THEN** يحفظ النظام القيمة الجديدة وسبب التعديل وهوية المنفذ وتوقيته في سجل قابل للمراجعة
