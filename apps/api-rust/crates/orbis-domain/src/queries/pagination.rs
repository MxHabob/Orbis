use serde::{Deserialize, Serialize};
use super::filter::UniversalFilter;

/// Client request for a cursor-paginated list.
#[derive(Debug, Clone, Deserialize)]
#[derive(utoipa::ToSchema)]
pub struct CursorRequest {
    /// The cursor value (typically the last item's ID from the previous page).
    /// `None` means "start from the beginning".
    pub cursor: Option<String>,

    /// Maximum number of items to return. Clamped to `MAX_LIMIT`.
    #[serde(default = "default_limit")]
    pub limit: u64,

    /// Optional generic query filter
    pub filter: Option<UniversalFilter>,
}

impl CursorRequest {
    /// The absolute maximum items per page (prevents abuse).
    pub const MAX_LIMIT: u64 = 200;

    /// Clamp limit to the allowed range.
    pub fn effective_limit(&self) -> u64 {
        self.limit.min(Self::MAX_LIMIT).max(1)
    }
}

impl Default for CursorRequest {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: default_limit(),
            filter: None,
        }
    }
}

fn default_limit() -> u64 {
    50
}

/// A page of results with cursor-based pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(utoipa::ToSchema)]
pub struct CursorPage<T: Serialize> {
    /// The items in this page.
    pub items: Vec<T>,

    /// Total items in this page.
    pub count: usize,

    /// Cursor for the next page. `None` means no more pages.
    pub next_cursor: Option<String>,

    /// Whether there are more items after this page.
    pub has_more: bool,
}

impl<T: Serialize> CursorPage<T> {
    /// Build a CursorPage from a list of items.
    ///
    /// The query should fetch `limit + 1` items. If we get more than `limit`,
    /// we know there are more pages and the extra item provides the next cursor.
    pub fn from_items_with<F>(mut items: Vec<T>, limit: u64, cursor_fn: F) -> Self
    where
        F: Fn(&T) -> String,
    {
        let effective_limit = limit.min(CursorRequest::MAX_LIMIT).max(1) as usize;
        let has_more = items.len() > effective_limit;

        if has_more {
            items.truncate(effective_limit);
        }

        let next_cursor = if has_more {
            items.last().map(&cursor_fn)
        } else {
            None
        };

        let count = items.len();

        Self {
            items,
            count,
            next_cursor,
            has_more,
        }
    }

    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            count: 0,
            next_cursor: None,
            has_more: false,
        }
    }
}

pub trait HasCursorId {
    fn cursor_id(&self) -> String;
}

impl<T: Serialize + HasCursorId> CursorPage<T> {
    pub fn from_items(items: Vec<T>, limit: u64) -> Self {
        Self::from_items_with(items, limit, |item| item.cursor_id())
    }
}
