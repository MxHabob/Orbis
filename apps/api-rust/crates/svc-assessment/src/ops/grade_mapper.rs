use shared_kernel::errors::AppError;

#[derive(serde::Serialize)]
pub struct GradeScaleItemData {
    pub label: String,
    pub min_score: f64,
    pub max_score: f64,
    pub gpa_points: Option<f64>,
    pub is_passing: bool,
}

#[derive(serde::Serialize)]
pub struct GradeMapper {
    pub items: Vec<GradeScaleItemData>,
}

#[derive(serde::Serialize)]
pub struct GradeResult {
    pub label: String,
    pub gpa_points: Option<f64>,
    pub is_passing: bool,
}

impl GradeMapper {
    pub fn new(mut items: Vec<GradeScaleItemData>) -> Self {
        // Sort items by max_score descending to easily find the grade
        items.sort_by(|a, b| b.max_score.partial_cmp(&a.max_score).unwrap_or(std::cmp::Ordering::Equal));
        Self { items }
    }

    pub fn map_score(&self, score: f64) -> Result<GradeResult, AppError> {
        for item in &self.items {
            if score >= item.min_score && score <= item.max_score {
                return Ok(GradeResult {
                    label: item.label.clone(),
                    gpa_points: item.gpa_points,
                    is_passing: item.is_passing,
                });
            }
        }
        
        // If not found, perhaps it exceeds the scale or falls below.
        // As a fallback, check if it's below the lowest scale item
        if let Some(lowest) = self.items.last() {
            if score < lowest.min_score {
                return Ok(GradeResult {
                    label: lowest.label.clone(),
                    gpa_points: lowest.gpa_points,
                    is_passing: lowest.is_passing,
                });
            }
        }

        // If it's above the highest
        if let Some(highest) = self.items.first() {
            if score > highest.max_score {
                return Ok(GradeResult {
                    label: highest.label.clone(),
                    gpa_points: highest.gpa_points,
                    is_passing: highest.is_passing,
                });
            }
        }

        Err(AppError::validation(format!("Score {} does not map to any grade scale item", score)))
    }
}
