# دليل التشغيل المحلي لنظام Orbis
# Orbis Local Deployment Guide

## المتطلبات

| البرنامج | الإصدار | الرابط |
|----------|---------|--------|
| **Docker Desktop** | 4.x+ | [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/) |
| **Bun** | 1.2+ | [bun.sh](https://bun.sh) |
| **Git** | 2.x+ | [git-scm.com](https://git-scm.com) |

---

## التشغيل السريع (أمر واحد)

```powershell
bun run setup:local
```

هذا الأمر يقوم بالتالي تلقائياً:
1. ✅ يتحقق من Docker و Bun
2. ✅ ينسخ `.env.local` إلى `.env`
3. ✅ يشغّل PostgreSQL + Redis + RustFS في Docker
4. ✅ يشغّل migrations و seed لقاعدة البيانات
5. ✅ يطبع روابط الوصول

بعد الانتهاء، شغّل التطبيق:
```powershell
bun run dev
```

---

## التشغيل اليدوي (خطوة بخطوة)

### 1. تشغيل خدمات Docker

```powershell
bun run docker:up
```

هذا يشغّل 3 حاويات:
- **PostgreSQL 17** — قاعدة البيانات (بورت 5432)
- **Redis 7** — الكاش + قائمة المهام (بورت 6379)
- **RustFS** — تخزين الملفات S3 (بورت 9000 + لوحة التحكم 9001)

### 2. إعداد ملف البيئة

```powershell
copy .env.local .env
```

### 3. تثبيت الحزم

```powershell
bun install
```

### 4. إعداد قاعدة البيانات

```powershell
bun run db:push
bun run seed
```

### 5. تشغيل التطبيق

```powershell
bun run dev
```

---

## إعداد الشبكة المحلية (School LAN)

### البنية المطلوبة

```
┌─────────────────────────────────────────────────┐
│              الشبكة المحلية (LAN)                │
│                                                   │
│  ┌──────────────┐     ┌──────────────┐           │
│  │ جهاز السيرفر │     │ جهاز المعلم  │           │
│  │ (الرئيسي)    │     │ (متصفح فقط) │           │
│  │              │     │              │           │
│  │ • Docker     │◄────│ يفتح:        │           │
│  │ • API :3002  │     │ http://IP:3001│          │
│  │ • Dashboard  │     └──────────────┘           │
│  │   :3001      │                                 │
│  │ • PostgreSQL │     ┌──────────────┐           │
│  │ • Redis      │     │ جهاز الإدارة │           │
│  │ • RustFS     │◄────│ (متصفح فقط) │           │
│  └──────────────┘     │ http://IP:3001│          │
│                        └──────────────┘           │
└─────────────────────────────────────────────────┘
```

### خطوات الإعداد

#### 1. معرفة IP جهاز السيرفر

```powershell
ipconfig
```

ابحث عن `IPv4 Address` — مثال: `192.168.1.100`

#### 2. تعديل `.env`

أضف IP السيرفر في `.env`:

```env
SERVER_LAN_IP=192.168.1.100
ALLOWED_API_ORIGINS=http://localhost:3001,http://192.168.1.100:3001
NEXT_PUBLIC_API_URL=http://192.168.1.100:3002
NEXT_PUBLIC_URL=http://192.168.1.100:3001
BETTER_AUTH_URL=http://192.168.1.100:3002
```

#### 3. فتح جدار الحماية (Firewall)

```powershell
# تشغيل كمسؤول (Administrator)
netsh advfirewall firewall add rule name="Orbis Dashboard" dir=in action=allow protocol=TCP localport=3001
netsh advfirewall firewall add rule name="Orbis API" dir=in action=allow protocol=TCP localport=3002
```

#### 4. الوصول من الأجهزة الأخرى

افتح المتصفح في أي جهاز على الشبكة:
```
http://192.168.1.100:3001
```

---

## الأوامر المتاحة

| الأمر | الوصف |
|-------|-------|
| `bun run setup:local` | إعداد كامل (أول مرة) |
| `bun run dev` | تشغيل API + Dashboard |
| `bun run docker:up` | تشغيل خدمات Docker |
| `bun run docker:down` | إيقاف خدمات Docker |
| `bun run docker:reset` | إيقاف + حذف البيانات |
| `bun run docker:logs` | عرض logs الخدمات |
| `bun run db:push` | تطبيق schema على DB |
| `bun run seed` | إضافة بيانات أولية |
| `bun run worker` | تشغيل worker المهام |

---

## استكشاف الأخطاء

### Docker لا يعمل
```powershell
# تأكد أن Docker Desktop يعمل
docker info
```

### خطأ في قاعدة البيانات
```powershell
# أعد تشغيل PostgreSQL
docker restart orbis-postgres

# تحقق من الحالة
docker logs orbis-postgres --tail 20
```

### خطأ في الملفات/التخزين
```powershell
# افتح لوحة تحكم RustFS
# http://localhost:9001
# User: rustfsadmin
# Pass: rustfs_local_2026
```

### إعادة تشغيل كاملة
```powershell
bun run docker:reset
bun run docker:up
bun run db:push
bun run seed
bun run dev
```
