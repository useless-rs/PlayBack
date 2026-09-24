//! Library view composition.

use playback_core::PlaylistItem;

/// A media-library row model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryRow {
    /// Display title.
    pub title: String,
    /// File path or URL.
    pub path: String,
    /// Whether the row is selected.
    pub selected: bool,
}

/// The media-library view model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryViewModel {
    /// Rows in display order.
    pub rows: Vec<LibraryRow>,
    /// Currently selected row index.
    pub selected_index: Option<usize>,
}

impl LibraryViewModel {
    /// Creates a library model from playlist items.
    pub fn from_items(items: &[PlaylistItem]) -> Self {
        Self {
            rows: items
                .iter()
                .map(|item| LibraryRow {
                    title: item.title.clone(),
                    path: item.path.clone(),
                    selected: false,
                })
                .collect(),
            selected_index: None,
        }
    }

    /// Selects a row by index.
    pub fn select(&mut self, index: Option<usize>) -> bool {
        if index.is_some_and(|value| value >= self.rows.len()) {
            return false;
        }
        self.selected_index = index;
        for (row_index, row) in self.rows.iter_mut().enumerate() {
            row.selected = index == Some(row_index);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::LibraryViewModel;
    use playback_core::PlaylistItem;

    #[test]
    fn selects_library_rows() {
        let items = vec![PlaylistItem {
            path: "movie.mkv".to_owned(),
            title: "Movie".to_owned(),
            duration: None,
        }];
        let mut library = LibraryViewModel::from_items(&items);
        assert!(library.select(Some(0)));
        assert!(library.rows[0].selected);
        assert!(!library.select(Some(1)));
    }
}
