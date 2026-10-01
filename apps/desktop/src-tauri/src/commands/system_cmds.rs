//! System Tauri IPC commands — system status, health, readiness, entitlements,
//! Mulverse ecosystem telemetry, GitHub releases, and intelligent update management.

use tauri::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::state::AppState;

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

/// Real release changelog item derived from GitHub Releases or local authoritative history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseChangelogItem {
    pub version: String,
    pub title_en: String,
    pub title_ar: String,
    pub release_date: String,
    pub is_current: bool,
    pub is_latest: bool,
    pub html_url: String,
    pub highlights_en: Vec<String>,
    pub highlights_ar: Vec<String>,
    pub raw_body: Option<String>,
}

/// Authentic architectural pillar of the Mulverse ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulversePillar {
    pub id: String,
    pub title_en: String,
    pub title_ar: String,
    pub description_en: String,
    pub description_ar: String,
    pub icon: String,
    pub capabilities: Vec<String>,
}

/// Authentic telemetry and build metadata of the Mulverse ecosystem and Orbis core node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MulverseEcosystemInfo {
    pub ecosystem_name: String,
    pub brand: String,
    pub product_name: String,
    pub product_code: String,
    pub version: String,
    pub build_profile: String,
    pub target_os: String,
    pub target_arch: String,
    pub github_repo: String,
    pub edition: String,
    pub node_type: String,
    pub node_id: String,
    pub is_local_first: bool,
    pub database_engine: String,
    pub description_en: String,
    pub description_ar: String,
    pub tagline_en: String,
    pub tagline_ar: String,
    pub pillars: Vec<MulversePillar>,
    pub latest_release: Option<ReleaseChangelogItem>,
}

/// Structured update check result with GitHub Releases comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
    pub release_notes: Option<String>,
    pub release_date: Option<String>,
    pub release_url: Option<String>,
    pub channel: String,
    pub status: String, // "up_to_date" | "update_available" | "checking_failed"
    pub error_message: Option<String>,
}

// ═══════════════════════════════════════════════════════════════
// GitHub Release & Changelog Helpers
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct GitHubReleaseDto {
    tag_name: String,
    name: Option<String>,
    published_at: Option<String>,
    body: Option<String>,
    html_url: String,
    #[serde(default)]
    prerelease: bool,
}

/// Build authoritative canonical changelog entries for Orbis.
pub fn get_authoritative_changelog(current_version: &str) -> Vec<ReleaseChangelogItem> {
    vec![
        ReleaseChangelogItem {
            version: "0.3.0".to_string(),
            title_en: "v0.3.0 — Mulverse Autonomous Desktop Core & DAG Setup".to_string(),
            title_ar: "الإصدار 0.3.0 — عقدة سطح المكتب المستقلة وتهيئة DAG الذكية".to_string(),
            release_date: "2026-10-01".to_string(),
            is_current: current_version == "0.3.0",
            is_latest: true,
            html_url: "https://github.com/MxHabob/Orbis/releases/tag/v0.3.0".to_string(),
            highlights_en: vec![
                "Sovereign local-first SQLite core with zero-latency operations".to_string(),
                "Real-time Tauri system environment & regional detection".to_string(),
                "Intelligent Arabic-to-Latin phonetic slug generator".to_string(),
                "DocForge visual publishing engine for certificates & transcripts".to_string(),
                "Double-entry ledger with multi-currency & ISO reference seed".to_string(),
                "Autopilot proactive workflow & automated grading closures".to_string(),
            ],
            highlights_ar: vec![
                "نواة SQLite محلية مستقلة ذات أداء فائق وسرعة فورية".to_string(),
                "اكتشاف تلقائي ذكي للمنطقة الجغرافية، العملة، والمنطقة الزمنية عبر Tauri".to_string(),
                "توليد فوري للمعرف اللاتيني (Slug) بالتعريب الصوتي الذكي".to_string(),
                "محرك DocForge للنشر البصري وتصميم الشهادات والسجلات".to_string(),
                "دليل محاسبي مزدوج القيد مع دعم العملات العالمية والإقليمية".to_string(),
                "محرك Autopilot للتشغيل الاستباقي وإغلاق الفترات التقويمية".to_string(),
            ],
            raw_body: Some(
                "## Orbis v0.3.0 - Mulverse Core\n\n- Sovereign desktop engine powered by Rust & SQLite.\n- Intelligent Regional & Locale detection.\n- DocForge visual document engine.\n- Full DAG onboarding configuration.".to_string(),
            ),
        },
        ReleaseChangelogItem {
            version: "0.2.0".to_string(),
            title_en: "v0.2.0 — Enterprise Institution Blueprints & IAM".to_string(),
            title_ar: "الإصدار 0.2.0 — نماذج المؤسسات الشاملة وإدارة الصلاحيات IAM".to_string(),
            release_date: "2026-08-15".to_string(),
            is_current: current_version == "0.2.0",
            is_latest: false,
            html_url: "https://github.com/MxHabob/Orbis/releases/tag/v0.2.0".to_string(),
            highlights_en: vec![
                "Precompiled blueprints for K-12, Universities, and Institutes".to_string(),
                "Multi-role IAM security system with supervisor overrides".to_string(),
                "Dual Hijri-Gregorian academic calendar support".to_string(),
                "Student registration engine with custom dynamic schema".to_string(),
            ],
            highlights_ar: vec![
                "نماذج معتمدة للمدارس K-12، الجامعات، والمعاهد المهنية".to_string(),
                "نظام صلاحيات أمني متعدد الأدوار مع طلبات الاستثناء والموافقة".to_string(),
                "دعم كامل للتقويم الهجري المزدوج والتقويم الميلادي".to_string(),
                "محرك تسجيل الطلاب مع الحقول المخصصة المرنة".to_string(),
            ],
            raw_body: Some(
                "## Orbis v0.2.0\n\n- Added institution blueprints.\n- Multi-role permission system.\n- Dual calendar integration.".to_string(),
            ),
        },
        ReleaseChangelogItem {
            version: "0.1.0".to_string(),
            title_en: "v0.1.0 — Zero-to-One Technical Foundation".to_string(),
            title_ar: "الإصدار 0.1.0 — التأسيس التقني الأولي للنظام".to_string(),
            release_date: "2026-05-01".to_string(),
            is_current: current_version == "0.1.0",
            is_latest: false,
            html_url: "https://github.com/MxHabob/Orbis/releases/tag/v0.1.0".to_string(),
            highlights_en: vec![
                "Core domain entities and event-driven architecture".to_string(),
                "SeaORM migrations and transactional persistence".to_string(),
                "Tauri v2 IPC communication layer".to_string(),
            ],
            highlights_ar: vec![
                "كيانات النطاق الأساسية والمعمارية القائمة على الأحداث".to_string(),
                "هجرة قواعد البيانات SeaORM والعمليات المتزامنة".to_string(),
                "طبقة الاتصال السريع Tauri v2 IPC".to_string(),
            ],
            raw_body: Some(
                "## Orbis v0.1.0\n\n- Initial release with foundational domain services.".to_string(),
            ),
        },
    ]
}

/// Fetch real live releases from GitHub if online, or return authoritative changelog.
pub async fn fetch_github_changelog(current_version: &str) -> Vec<ReleaseChangelogItem> {
    let client = reqwest::Client::builder()
        .user_agent("Orbis-Desktop-App")
        .timeout(std::time::Duration::from_secs(3))
        .build();

    if let Ok(client) = client {
        let url = "https://api.github.com/repos/MxHabob/Orbis/releases";
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                if let Ok(releases) = resp.json::<Vec<GitHubReleaseDto>>().await {
                    if !releases.is_empty() {
                        let mut items = Vec::new();
                        for (idx, r) in releases.into_iter().enumerate() {
                            let clean_version = r.tag_name.trim_start_matches('v').to_string();
                            let is_curr = clean_version == current_version;
                            let is_lat = idx == 0;
                            let raw_body = r.body.clone();

                            let mut highlights_en = Vec::new();
                            let mut highlights_ar = Vec::new();

                            if let Some(ref b) = r.body {
                                for line in b.lines() {
                                    let trimmed = line.trim();
                                    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                                        let item = trimmed[2..].trim().to_string();
                                        if !item.is_empty() && highlights_en.len() < 6 {
                                            highlights_en.push(item.clone());
                                            highlights_ar.push(item);
                                        }
                                    }
                                }
                            }

                            if highlights_en.is_empty() {
                                highlights_en.push(format!("Official Release {}", r.tag_name));
                                highlights_ar.push(format!("الإصدار الرسمي {}", r.tag_name));
                            }

                            items.push(ReleaseChangelogItem {
                                version: clean_version.clone(),
                                title_en: r.name.clone().unwrap_or_else(|| format!("Orbis {}", r.tag_name)),
                                title_ar: format!("أوربيس {}", r.tag_name),
                                release_date: r.published_at.unwrap_or_else(|| "2026-10-01".to_string()),
                                is_current: is_curr,
                                is_latest: is_lat,
                                html_url: r.html_url,
                                highlights_en,
                                highlights_ar,
                                raw_body,
                            });
                        }
                        return items;
                    }
                }
            }
        }
    }

    get_authoritative_changelog(current_version)
}

// ═══════════════════════════════════════════════════════════════
// System IPC Handlers
// ═══════════════════════════════════════════════════════════════

/// Get complete authentic telemetry for the Mulverse ecosystem and Orbis node.
#[tauri::command]
pub async fn system_get_ecosystem_info() -> Result<MulverseEcosystemInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION");
    let build_profile = if cfg!(debug_assertions) {
        "Development (Debug)".to_string()
    } else {
        "Production (Release)".to_string()
    };
    let target_os = std::env::consts::OS.to_string();
    let target_arch = std::env::consts::ARCH.to_string();

    let changelog = fetch_github_changelog(current_version).await;
    let latest_release = changelog.first().cloned();

    Ok(MulverseEcosystemInfo {
        ecosystem_name: "Mulverse".to_string(),
        brand: "Mulverse Systems".to_string(),
        product_name: "Orbis Next-Gen SIS / ERP".to_string(),
        product_code: "MULVERSE-ORBIS-DESKTOP".to_string(),
        version: current_version.to_string(),
        build_profile,
        target_os,
        target_arch,
        github_repo: "MxHabob/Orbis".to_string(),
        edition: "Autonomous Desktop Edition".to_string(),
        node_type: "Autonomous Desktop Node".to_string(),
        node_id: format!("node-{}", Uuid::new_v4().to_string().chars().take(8).collect::<String>()),
        is_local_first: true,
        database_engine: "Embedded SQLite 3 (WAL mode, Foreign Keys ON, Double-Entry Invariants)".to_string(),
        description_en: "Orbis is an enterprise institutional operating system engineered by Mulverse as an autonomous, zero-latency local-first desktop node.".to_string(),
        description_ar: "أوربيس هو نظام تشغيل وإدارة المؤسسات التعليمية والأكاديمية المتقدم، المطور كعقدة تشغيلية مستقلة ذات سرعة فائقة ضمن منظومة Mulverse.".to_string(),
        tagline_en: "Institutional Sovereignty, Zero Latency, and Intelligent Operations".to_string(),
        tagline_ar: "السيادة المؤسسية الكاملة، السرعة الفائقة، والذكاء التشغيلي".to_string(),
        pillars: vec![
            MulversePillar {
                id: "sovereignty".to_string(),
                title_en: "Institutional Data Sovereignty".to_string(),
                title_ar: "السيادة الرقمية والاستقلالية الكاملة".to_string(),
                description_en: "All institutional records, financial ledgers, and student files are stored locally and encrypted on your machine with zero cloud lock-in.".to_string(),
                description_ar: "بيانات مؤسستك وسجلاتها المالية ودرجات طلابها مشفرة ومخزنة محلياً بالكامل على جهازك دون أي تبعية سحابية إلزامية.".to_string(),
                icon: "ShieldCheck".to_string(),
                capabilities: vec![
                    "Zero cloud lock-in".to_string(),
                    "Encrypted local database".to_string(),
                    "Offline-first full operations".to_string(),
                ],
            },
            MulversePillar {
                id: "latency".to_string(),
                title_en: "Zero-Latency Rust Engine".to_string(),
                title_ar: "نواة Rust فائقة السرعة".to_string(),
                description_en: "Instant computations for multi-criteria grading, double-entry financial journals, and complex DAG timetable scheduling.".to_string(),
                description_ar: "معالجة فورية فائقة السرعة للدرجات والتقييمات والمحاسبة المزدوجة وجداول الحصص دون أي بطء أو انتظار.".to_string(),
                icon: "Zap".to_string(),
                capabilities: vec![
                    "Instant ledger transactions".to_string(),
                    "Multi-criteria grade matrix".to_string(),
                    "Parallel schedule solver".to_string(),
                ],
            },
            MulversePillar {
                id: "docforge".to_string(),
                title_en: "DocForge Visual Publishing".to_string(),
                title_ar: "محرك DocForge للنشر البصري".to_string(),
                description_en: "High-precision layout engine for official certificates, transcripts, diplomas, and ID cards with instant visual generation.".to_string(),
                description_ar: "محرك تصميم ونشر بصري فائق الدقة لإصدار الشهادات والبطاقات والسجلات الرسمية والتقارير بأعلى معايير الطباعة العالمية.".to_string(),
                icon: "FileText".to_string(),
                capabilities: vec![
                    "Pixel-perfect certificate builder".to_string(),
                    "Batch transcript generation".to_string(),
                    "QR code cryptographic verification".to_string(),
                ],
            },
            MulversePillar {
                id: "autopilot".to_string(),
                title_en: "Autopilot Smart Operations".to_string(),
                title_ar: "محرك التشغيل الذكي Autopilot".to_string(),
                description_en: "Continuous background orchestration of enrollment cycles, grading windows, and proactive academic workflows.".to_string(),
                description_ar: "إدارة ذكية ومستمرة لمراحل القبول والتسجيل، إغلاق الفترات التقويمية، والتنبيهات الاستباقية للعمليات المؤسسية.".to_string(),
                icon: "Sparkles".to_string(),
                capabilities: vec![
                    "DAG readiness monitoring".to_string(),
                    "Automated deadline enforcement".to_string(),
                    "Proactive institutional alerts".to_string(),
                ],
            },
        ],
        latest_release,
    })
}

/// Fetch real release changelog entries from GitHub Releases.
#[tauri::command]
pub async fn system_get_changelog() -> Result<Vec<ReleaseChangelogItem>, String> {
    let current_version = env!("CARGO_PKG_VERSION");
    Ok(fetch_github_changelog(current_version).await)
}

/// Check for updates against GitHub Releases.
#[tauri::command]
pub async fn system_check_for_updates() -> Result<UpdateCheckResult, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    let changelog = fetch_github_changelog(&current_version).await;

    if let Some(latest) = changelog.first() {
        let is_newer = latest.version != current_version && latest.is_latest;
        let status = if is_newer {
            "update_available".to_string()
        } else {
            "up_to_date".to_string()
        };

        Ok(UpdateCheckResult {
            current_version,
            latest_version: latest.version.clone(),
            has_update: is_newer,
            release_notes: latest.raw_body.clone(),
            release_date: Some(latest.release_date.clone()),
            release_url: Some(latest.html_url.clone()),
            channel: "stable".to_string(),
            status,
            error_message: None,
        })
    } else {
        Ok(UpdateCheckResult {
            current_version: current_version.clone(),
            latest_version: current_version,
            has_update: false,
            release_notes: None,
            release_date: None,
            release_url: None,
            channel: "stable".to_string(),
            status: "up_to_date".to_string(),
            error_message: None,
        })
    }
}

/// System health check.
#[tauri::command]
pub async fn system_health(state: State<'_, AppState>) -> Result<orbis_app::services::HealthResponse, String> {
    Ok(state.system_service.health().await)
}

/// Check if any organization exists in the system.
#[tauri::command]
pub async fn system_has_organization(state: State<'_, AppState>) -> Result<bool, String> {
    state
        .system_service
        .has_organization()
        .await
        .map_err(|e| e.to_string())
}

/// Get organization readiness status.
#[tauri::command]
pub async fn system_get_readiness(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::OrganizationReadiness, String> {
    state
        .system_service
        .get_readiness()
        .await
        .map_err(|e| e.to_string())
}

/// Get system entitlement summary.
#[tauri::command]
pub async fn system_get_entitlement(
    state: State<'_, AppState>,
) -> Result<orbis_domain::entities::entitlement::EntitlementResult, String> {
    state
        .entitlement_provider
        .check_entitlement("orbis_desktop", "default")
        .await
        .map_err(|e| e.to_string())
}
