use base64::{Engine as _, engine::general_purpose::STANDARD};
use docforge_engine::model::document::TemplateDocument;
use orbis_domain::{
    entities::report::{ColumnAlign, ColumnDataType, ReportDataset, ReportLocale},
    errors::DomainError,
};
use serde_json::{Map, Value, json};

const FONT: &str = "Noto Sans Arabic";
const MAX_COLUMNS: usize = 9;

fn state_emblem_data_uri() -> &'static str {
    static EMBLEM: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    EMBLEM.get_or_init(|| {
        let encoded = STANDARD.encode(include_bytes!(
            "../../../orbis-app/src/defaults/assest/The Public Slogan.png"
        ));
        format!("data:image/png;base64,{encoded}")
    })
}

pub(super) fn build(
    dataset: &ReportDataset,
    locale: &ReportLocale,
) -> Result<(TemplateDocument, Value), DomainError> {
    let rtl = locale.is_rtl();
    let direction = if rtl { "rtl" } else { "ltr" };
    let leading = "start";
    let trailing = "end";
    let title = localized(rtl, &dataset.report_name, &dataset.report_name_ar);
    let org = localized(rtl, &dataset.org.name, &dataset.org.name_ar);
    let columns: Vec<_> = dataset
        .columns
        .iter()
        .filter(|column| column.visible)
        .collect();
    let landscape = columns.len() > 6;
    let (width, height, rows_per_page) = if landscape {
        (297.0, 210.0, 13)
    } else {
        (210.0, 297.0, 21)
    };
    let content_width = width - 28.0;
    let (first, other_columns) = columns
        .split_first()
        .map_or((None, &[][..]), |(first, rest)| (Some(*first), rest));
    // Repeat the first column on wide reports so records remain identifiable.
    let empty = dataset.rows.is_empty() || columns.is_empty();
    let panels = if empty {
        1
    } else {
        other_columns.len().div_ceil(MAX_COLUMNS - 1).max(1)
    };
    let row_pages = if empty {
        1
    } else {
        dataset.rows.len().div_ceil(rows_per_page)
    };
    let page_count = row_pages * panels;
    let mut context = Map::new();
    let emblem_uri = state_emblem_data_uri();
    let org_lines: Vec<String> = [
        dataset.org.country_name.as_deref(),
        dataset.org.ministry_name.as_deref(),
        dataset.org.region_name.as_deref(),
        Some(org),
    ]
    .into_iter()
    .flatten()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .map(str::to_string)
    .collect();
    let org_identity = org_lines.join("\n");

    context.insert(
        "report".into(),
        json!({ "title": title, "organization": org, "identity": org_identity }),
    );
    context.insert(
        "org".into(),
        json!({ "primary_color": dataset.org.primary_color, "emblem": emblem_uri }),
    );
    let mut children = Vec::new();

    let side_w: f64 = (content_width * 0.36_f64).min(78.0_f64);
    let center_w = 46.0;
    let center_x = (width - center_w) / 2.0;
    let right_x = width - 14.0 - side_w;
    let left_x = 14.0;
    let identity_x = if rtl { right_x } else { left_x };
    let meta_x = if rtl { left_x } else { right_x };

    for page in 0..row_pages {
        let start = page * rows_per_page;
        let end = (start + rows_per_page).min(dataset.rows.len());
        let source = format!("reportRows{page}");
        context.insert(source.clone(), json!(&dataset.rows[start..end]));

        for panel in 0..panels {
            let index = page * panels + panel + 1;
            if index > 1 {
                children.push(json!({ "type": "PageBreak", "id": format!("break-{index}") }));
            }
            let text = |id: &str,
                        content: &str,
                        x: f64,
                        y: f64,
                        w: f64,
                        h: f64,
                        size: f64,
                        bold: bool,
                        align: &str| {
                json!({
                    "type": "Text", "id": format!("{id}-{index}"),
                    "props": {
                        "content": content, "fontFamily": FONT, "fontSize": size,
                        "direction": if id == "page" { "ltr" } else { direction },
                        "fontWeight": if bold { "bold" } else { "normal" }, "textAlign": align,
                        "lineHeight": 1.35,
                        "color": if bold { "#0f172a" } else { "#475569" }
                    },
                    "position": { "x": x, "y": y, "width": w, "height": h }
                })
            };
            // Official 3-Column Letterhead Header
            children.push(text(
                "organization",
                "{{report.identity}}",
                identity_x,
                10.5,
                side_w,
                25.0,
                8.5,
                true,
                "center",
            ));
            if rtl {
                children.push(text(
                    "bismillah",
                    "بسم الله الرحمن الرحيم",
                    center_x,
                    9.5,
                    center_w,
                    5.0,
                    8.0,
                    true,
                    "center",
                ));
            }
            children.push(json!({
                "type": "Image",
                "id": format!("state-emblem-{index}"),
                "props": { "src": "{{org.emblem}}", "fit": "contain" },
                "position": { "x": (width - 18.0) / 2.0, "y": if rtl { 15.5 } else { 11.0 }, "width": 18.0, "height": 17.0 }
            }));
            children.push(text(
                "title",
                "{{report.title}}",
                meta_x,
                11.0,
                side_w,
                13.0,
                10.5,
                true,
                "center",
            ));
            let count = if rtl {
                format!(
                    "التاريخ: {}  |  السجلات: {}",
                    dataset.generated_at.format("%Y-%m-%d"),
                    dataset.total_count
                )
            } else {
                format!(
                    "Date: {}  |  Records: {}",
                    dataset.generated_at.format("%Y-%m-%d"),
                    dataset.total_count
                )
            };
            children.push(text(
                "meta", &count, meta_x, 25.0, side_w, 8.0, 8.0, false, "center",
            ));
            children.push(text(
                "page",
                &format!("{index} / {page_count}"),
                14.0,
                height - 19.0,
                content_width,
                8.0,
                9.0,
                false,
                if rtl { leading } else { trailing },
            ));
            children.push(json!({
                "type": "Separator", "id": format!("header-rule-{index}"),
                "props": { "direction": "horizontal", "color": "#0f172a", "thickness": 1.1 },
                "position": { "x": 14, "y": 37.5, "width": content_width, "height": 1 }
            }));
            children.push(json!({
                "type": "Separator", "id": format!("header-rule-sub-{index}"),
                "props": { "direction": "horizontal", "color": "#64748b", "thickness": 0.4 },
                "position": { "x": 14, "y": 39.2, "width": content_width, "height": 0.5 }
            }));
            if empty {
                children.push(text(
                    "empty",
                    if rtl {
                        "لا توجد بيانات لعرضها"
                    } else {
                        "No data to display"
                    },
                    14.0,
                    50.0,
                    content_width,
                    11.0,
                    11.0,
                    false,
                    "center",
                ));
                continue;
            }

            let panel_start = panel * (MAX_COLUMNS - 1);
            let panel_end = (panel_start + MAX_COLUMNS - 1).min(other_columns.len());
            let visible: Vec<_> = first
                .into_iter()
                .chain(other_columns[panel_start..panel_end].iter().copied())
                .collect();
            let weights: Vec<_> = visible
                .iter()
                .map(|column| {
                    column
                        .width
                        .as_deref()
                        .and_then(|width| width.strip_suffix('%'))
                        .and_then(|width| width.parse::<f64>().ok())
                        .filter(|width| width.is_finite() && *width > 0.0)
                        .unwrap_or(if column.data_type == ColumnDataType::Text {
                            18.0
                        } else {
                            11.0
                        })
                        .clamp(8.0, 35.0)
                })
                .collect();
            let total = weights.iter().sum::<f64>();
            let table_columns: Vec<_> = visible
                .iter()
                .zip(weights)
                .map(|(column, weight)| {
                    let align = match column.align {
                        Some(ColumnAlign::Center) => "center",
                        Some(ColumnAlign::End) => trailing,
                        Some(ColumnAlign::Start) | None => leading,
                    };
                    json!({
                        "id": column.key, "field": column.key,
                        "header": localized(rtl, &column.header, &column.header_ar),
                        "width": format!("{:.5}%", weight / total * 100.0), "alignment": align,
                        "defaultValue": "—",
                    })
                })
                .collect();
            children.push(json!({
                "type": "Table", "id": format!("table-{index}"),
                "props": {
                    "columns": table_columns, "dataSource": source,
                    "direction": direction, "showGrid": false, "showBorder": false,
                    "style": {
                        "fontFamily": FONT, "fontSize": 9, "headerFontSize": 9,
                        "headerBgColor": "#eaf0f5", "headerTextColor": "#0f172a",
                        "altRowBgColor": "#f8fafc", "rowHeight": 28, "padding": 4
                    }
                },
                "position": { "x": 14, "y": 43, "width": content_width, "height": height - 65.0 }
            }));
        }
    }

    let doc = serde_json::from_value(json!({
        "version": "2.0",
        "meta": {
            "name": title, "documentType": "REPORT", "entityType": "report",
            "locale": locale.primary_lang(),
            "pageSetup": {
                "size": "A4", "orientation": if landscape { "landscape" } else { "portrait" },
                "margins": { "top": 12, "bottom": 12, "left": 14, "right": 14 }, "unit": "mm"
            }
        },
        "root": { "type": "Page", "id": "report", "children": children }
    }))
    .map_err(|error| DomainError::Internal(format!("Invalid built-in DocForge report: {error}")))?;
    Ok((doc, Value::Object(context)))
}

fn localized<'a>(rtl: bool, english: &'a str, arabic: &'a str) -> &'a str {
    if rtl && !arabic.trim().is_empty() || english.trim().is_empty() {
        arabic
    } else {
        english
    }
}
