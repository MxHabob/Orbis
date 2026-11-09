<div align="center">
  <a href="https://orbis.mulverse.com">
    <img src="logo.png" alt="Orbis logo" width="120" />
  </a>

  <h1>Orbis Enterprise</h1>

  <p>
    <strong>A next-generation, on-premise educational management ecosystem.</strong><br/>
    Built for data sovereignty, local-first operations, and institutions with one or many campuses.
  </p>

  <p>
    <a href="https://orbis.mulverse.com">Website</a>
    ·
    <a href="mailto:mulverseco@gmail.com">Contact Mulverse</a>
    ·
    <a href="https://github.com/MxHabob/Orbis/releases">Releases</a>
    ·
    <a href="docs/releases.md">Release guide</a>
  </p>

  <p>
    <a href="#features"><strong>Features</strong></a>
    ·
    <a href="#deployment"><strong>Deployment</strong></a>
    ·
    <a href="#translations"><strong>Translations</strong></a>
    ·
    <a href="#contributors"><strong>Contributors</strong></a>
  </p>
</div>

> [!IMPORTANT]
> License terms are maintained in [`LICENSE`](LICENSE). Review them and contact Mulverse for licensing, deployment, or partnership inquiries.

---

## Overview

**Orbis** is an on-premise educational management ecosystem for universities, school networks, vocational institutes, and academies. It brings academic operations, examinations, attendance, timetabling, and financial management together for institutions operating one or multiple campuses.

Orbis is designed to run on an institution's own infrastructure, with a native desktop client and local-first workflows. It supports centralized administration alongside branch-level operations and policy overrides.

<div align="center">
  <img src="apps/desktop/src/lib/assets/orbis-ecosystem.png" alt="Orbis ecosystem brand artwork" width="100%" />
  <sub>Orbis ecosystem artwork. Application UI screenshots are not currently included in this repository.</sub>
</div>

## Features

<div align="center">
  <table>
    <tr>
      <td width="50%" valign="top">
        <h4>Organization &amp; Identity</h4>
        <ul>
          <li>Model headquarters, campuses, faculties, and departments in a hierarchy</li>
          <li>Separate person records from user accounts for students, staff, and guardians</li>
          <li>Manage branch-level operations with inherited institutional policies</li>
        </ul>
      </td>
      <td width="50%" valign="top">
        <h4>Academics &amp; Student Lifecycle</h4>
        <ul>
          <li>Configure semesters, trimesters, academic years, or modular periods</li>
          <li>Support cohort-based and credit-hour academic structures</li>
          <li>Promote students through staged transitions with rollback support</li>
        </ul>
      </td>
    </tr>
    <tr>
      <td width="50%" valign="top">
        <h4>Exams &amp; Assessment</h4>
        <ul>
          <li>Coordinate exam cycles, halls, and proctor schedules</li>
          <li>Generate seating numbers and protected secret numbers</li>
          <li>Manage academic assessment workflows</li>
        </ul>
      </td>
      <td width="50%" valign="top">
        <h4>Attendance &amp; Timetabling</h4>
        <ul>
          <li>Record lesson-level and daily attendance</li>
          <li>Route medical absence excuses through a review workflow</li>
          <li>Detect teacher and room conflicts with constraint-based scheduling</li>
        </ul>
      </td>
    </tr>
    <tr>
      <td width="50%" valign="top">
        <h4>Finance</h4>
        <ul>
          <li>Maintain a unified double-entry chart of accounts</li>
          <li>Organize branch sub-treasuries and point-of-sale cash registers</li>
          <li>Configure tuition, fees, discounts, installments, and multi-currency receipts</li>
        </ul>
      </td>
      <td width="50%" valign="top">
        <h4>Documents &amp; Operations</h4>
        <ul>
          <li>Design certificates, transcripts, reports, and ID cards with DocForge</li>
          <li>Store documents on local or private on-premise storage</li>
          <li>Support role-based document access, audit logs, and background routines</li>
        </ul>
      </td>
    </tr>
  </table>
</div>

## Architecture

Orbis is organized as a monorepo containing the applications and services that make up the ecosystem:

| Component | Role |
| --- | --- |
| Desktop | Tauri and Svelte desktop client |
| API | Rust server and application services |
| Mobile | Mobile application |
| CLI | Command-line utilities |
| Solver service | Scheduling and constraint-solving support |
| Shared packages | UI, schemas, SDK, document generation, and integrations |

## Deployment

Orbis is designed for private institutional infrastructure:

- **Dedicated server:** deploy services on institution-managed hardware, including PostgreSQL-backed deployments.
- **Desktop client:** native Tauri application; the current project description targets Windows and macOS.
- **Private network:** core institutional operations can be hosted on a local network without a mandatory third-party cloud dependency.
- **Multi-campus operations:** central administration can manage multiple branches with scoped operational policies.

For deployment and licensing inquiries, [contact Mulverse](mailto:mulverseco@gmail.com).

### Local performance diagnostics

The API logs slow request timings locally without recording request bodies,
query strings, or record identifiers. Administrators can optionally use
PostgreSQL's local `pg_stat_statements` extension for query-level aggregates.
See the [performance diagnostics guide](docs/performance-diagnostics.md) for
configuration and safe analysis steps.

## Translations

The desktop application's supported locales and base locale are maintained in the project configuration. The message files are the source of truth for translated text:

- [Locale configuration](apps/desktop/project.inlang/settings.json)
- [Desktop translation messages](apps/desktop/messages/)
- [Mobile translation dictionaries](apps/mobile/src/lib/i18n/dictionaries/)

To report a translation issue or ask about contributing a translation, [contact Mulverse](mailto:mulverseco@gmail.com).

## Contributors

Orbis is developed and maintained by [Mulverse](https://github.com/mulverseco). View the [contributors to Orbis](https://github.com/MxHabob/Orbis/graphs/contributors) on GitHub.

For licensing, collaboration, or contribution inquiries, please [contact the team](mailto:mulverseco@gmail.com).

## License

See [`LICENSE`](LICENSE) for the current license terms.

<div align="center">
  <sub>Orbis is a product of Mulverse Inc.</sub>
</div>
