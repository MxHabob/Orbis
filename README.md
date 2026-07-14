<p align="center">
  <img src="logo.png" alt="Orbis Logo" width="128" />
</p>

<p align="center">
	<h1 align="center"><b>Orbis</b></h1>
<p align="center">
    The Open-Source School Management Platform
    <br />
    <br />
    <a href="https://orbis.mulverse.com">Website</a>
    ·
    <a href="https://github.com/mulverse/orbis/issues">Issues</a>
    ·
    <a href="docs/LOCAL-DEPLOYMENT.md">Self-Host Guide</a>
  </p>
</p>


## About Orbis

Orbis is a modern, self-hostable school management platform designed to run entirely on-premise — giving schools full ownership of their data with zero cloud dependency. Built as a monorepo with a focus on Arabic-first education workflows, Orbis provides everything a school needs to manage students, grades, attendance, schedules, and more from a single unified system.


## Features

**Student Management** — Register, enroll, and track students across academic years with support for class divisions, seating numbers, and custom metadata.<br/>
**Grading & Assessments** — Create exams, quizzes, assignments, and oral assessments with weighted scoring, grade publishing, and per-month tracking.<br/>
**Attendance Tracking** — Mark attendance per scheduled lesson with statuses like present, absent, late, excused, and sick — with teacher verification support.<br/>
**Class Scheduling** — Build weekly timetables with time slots, subject-teacher assignments, and automatic conflict detection.<br/>
**Academic Structure** — Manage academic years, terms, grades (1–9), and subjects with automatic seed data for quick setup.<br/>
**Certificate Designer** — A Figma-inspired visual editor for designing and printing student certificates and reports.<br/>
**Document Vault** — Secure S3-backed file storage with folders, pinning, and per-user document management.<br/>
**Parent Portal** — Link parents to students with relationship types and primary contact designation.<br/>
**Student Promotions** — Promote, retain, or transfer students between academic years with full audit trails.<br/>
**Notifications** — In-app notifications for grades, attendance alerts, announcements, behavior reports, and more.<br/>
**On-Premise Deployment** — Deploy to any school server with a single PowerShell command — no source code required on-site.<br/>
**Desktop App** — Native desktop application built with Tauri for offline-capable access.


## Get Started

Clone the repo and run the automated setup:

```bash
bun run setup:local
```

This will spin up PostgreSQL, Redis, and RustFS via Docker, run migrations, seed initial data (grades, subjects, terms), and print access URLs.

Then start the dev server:

```bash
bun run dev
```

For detailed setup instructions, see the [Local Deployment Guide](docs/LOCAL-DEPLOYMENT.md).

For deploying to a school server (production), see the [School Deployment Guide](docs/school_deployment_guide.md).

## Architecture

### Monorepo Structure

```
apps/
  api/           → Hono + tRPC API server (port 3002)
  dashboard/     → Next.js admin dashboard (port 3001)
  desktop/       → Tauri native desktop app
packages/
  auth/          → Better Auth integration
  db/            → Drizzle ORM schema & migrations
  cache/         → Redis caching layer
  jobs/          → Background job worker (BullMQ)
  storage/       → S3-compatible file storage (RustFS)
  ui/            → Shared UI components (Shadcn)
  email/         → Transactional emails (Resend)
  encryption/    → Data encryption utilities
  export/        → Data export (Excel/CSV)
  import/        → Data import utilities
  notifications/ → Notification system
  logger/        → Structured logging (Pino)
  location/      → Geolocation utilities
  utils/         → Shared utilities
cli/             → School server deployment CLI
```

### Tech Stack

| Layer | Technology |
|-------|-----------|
| **Runtime** | Bun |
| **Language** | TypeScript |
| **API** | Hono + tRPC |
| **Frontend** | Next.js 16 + React 19 |
| **Desktop** | Tauri |
| **Styling** | TailwindCSS + Shadcn UI |
| **Database** | PostgreSQL 17 (Drizzle ORM) |
| **Cache** | Redis 7 |
| **Storage** | RustFS (S3-compatible) |
| **Auth** | Better Auth |
| **Jobs** | BullMQ |
| **Build** | Turborepo |

### Hosting (Cloud Option)

- **Vercel** — Dashboard
- **Fly.io** — API
- **GitHub Actions** — CI/CD

## License

This software is licensed under the **Mulverse M.U.V. Proprietary Software License**.

Copyright © 2022–2026 **Mulverse Inc.**, Sana'a, Republic of Yemen.

See the full [LICENSE](LICENSE) file for details.

For commercial licensing inquiries, contact [mulverseco@gmail.com](mailto:mulverseco@gmail.com).
