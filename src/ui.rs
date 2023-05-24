#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
struct Size<T> {
    width: Distance<T>,
    height: Distance<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
#[serde(tag = "type")]
enum Distance<T> {
    /// Value is in logical pixels.
    Logical(T),
    /// Value is a fraction of some physical size.
    Fraction(f64),
}

impl<T> Distance<T>
where
    T: std::ops::Mul<f64, Output = T> + Copy,
{
    pub fn as_physical(
        &self,
        logical_scale: f64,
        percentage_parent_size: T,
    ) -> T {
        match self {
            Distance::Logical(logical_size) => {
                logical_size.to_owned() * logical_scale
            }
            Distance::Fraction(size_percentage) => {
                percentage_parent_size * size_percentage.to_owned()
            }
        }
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Layout {
    pub primary_window: Window,
    pub secondary_windows: Vec<Window>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Window {
    pub top_bar: TopBar,
    pub dock_area: DockArea,
    pub left_shelf: Shelf,
    pub right_shelf: Shelf,
    pub top_shelf: Shelf,
    pub bottom_shelf: Shelf,
    pub inner_size: nalgebra::Vector2<u32>,
}

impl Default for Window {
    fn default() -> Self {
        let top_bar = Default::default();

        let timeline_panel = EditorPanel {
            editor_tabs: vec![EditorTab::Timeline, EditorTab::Mixer],
            selected_tab_idx: 0,
            size: Size {
                width: Distance::Fraction(1.),
                height: Distance::Fraction(1.),
            },
        };
        let pattern_panel = EditorPanel {
            editor_tabs: vec![
                EditorTab::Pattern,
                EditorTab::Nodes,
                EditorTab::Audio,
            ],
            selected_tab_idx: 0,
            size: Size {
                width: Distance::Fraction(1.),
                height: Distance::Fraction(1.),
            },
        };

        let editor_panels = vec![];
        let dock_area = DockArea { editor_panels };

        let inner_size = nalgebra::Vector2::new(1280, 720);
        Self {
            top_bar,
            dock_area,
            inner_size,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TopBar {
    /// Relative to window size.
    pub height: Distance<u32>,
}

impl Default for TopBar {
    fn default() -> Self {
        let height = Distance::Logical(20);
        Self { height }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DockArea {
    pub editor_panels: Vec<EditorPanel>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EditorPanel {
    pub editor_tabs: Vec<EditorTab>,
    /// Must be within `editor_tabs::len`.
    pub selected_tab_idx: usize,
    // Relative to owning `DockArea`.
    pub size: Size<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum EditorTab {
    Timeline,
    Mixer,
    Pattern,
    Nodes,
    Audio,
    Library,
    Explorer,
    Properties,
    Project,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum ShelfType {
    Over,
    NextTo,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Shelf {
    // Should not be higher than 0.4% of the window size - top bar height.
    pub length: Distance<u32>,
    pub dock_area: DockArea,
    pub shelf_type: ShelfType,
}
