//! Window state and type enumerations.

/// The state a window can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowState {
    /// State is unknown or unset.
    #[default]
    Unknown,
    /// Normal restored state.
    Restored,
    /// Window is minimized (hidden from view).
    Minimized,
    /// Window is maximized (fills the application zone).
    Maximized,
    /// Window is vertically maximized.
    VertMaximized,
    /// Window is horizontally maximized.
    HorizMaximized,
    /// Window is fullscreen (covers the entire output).
    Fullscreen,
    /// Window is hidden.
    Hidden,
    /// Window is attached to an output edge (e.g., panels).
    Attached,
}

impl WindowState {
    /// Convert from a raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Restored,
            2 => Self::Minimized,
            3 => Self::Maximized,
            4 => Self::VertMaximized,
            5 => Self::Fullscreen,
            6 => Self::HorizMaximized,
            7 => Self::Hidden,
            8 => Self::Attached,
            _ => Self::Unknown,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Unknown => 0,
            Self::Restored => 1,
            Self::Minimized => 2,
            Self::Maximized => 3,
            Self::VertMaximized => 4,
            Self::Fullscreen => 5,
            Self::HorizMaximized => 6,
            Self::Hidden => 7,
            Self::Attached => 8,
        }
    }
}

/// The type of a window, describing its role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WindowType {
    /// A regular application window.
    #[default]
    Normal,
    /// A utility/floating window.
    Utility,
    /// A dialog window.
    Dialog,
    /// A gloss window.
    Gloss,
    /// A freestyle window (no constraints on shape/decoration).
    Freestyle,
    /// A menu window.
    Menu,
    /// An input method window (e.g., on-screen keyboard).
    InputMethod,
    /// A satellite/toolbar window.
    Satellite,
    /// A tooltip window.
    Tip,
    /// A decoration window.
    Decoration,
}

impl WindowType {
    /// Convert from a raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Utility,
            2 => Self::Dialog,
            3 => Self::Gloss,
            4 => Self::Freestyle,
            5 => Self::Menu,
            6 => Self::InputMethod,
            7 => Self::Satellite,
            8 => Self::Tip,
            9 => Self::Decoration,
            _ => Self::Normal,
        }
    }

    /// Convert to a raw C enum value.
    #[allow(dead_code)]
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Utility => 1,
            Self::Dialog => 2,
            Self::Gloss => 3,
            Self::Freestyle => 4,
            Self::Menu => 5,
            Self::InputMethod => 6,
            Self::Satellite => 7,
            Self::Tip => 8,
            Self::Decoration => 9,
        }
    }
}

/// Resize edge flags for interactive resize requests.
///
/// The values mirror `MirResizeEdge`, which is a bitmask: the corner variants are the
/// bitwise-or of their two adjacent edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResizeEdge {
    /// No specific edge.
    #[default]
    None,
    /// Left edge.
    Left,
    /// Right edge.
    Right,
    /// Top edge.
    Top,
    /// Bottom edge.
    Bottom,
    /// Top-left corner.
    TopLeft,
    /// Top-right corner.
    TopRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom-right corner.
    BottomRight,
}

impl ResizeEdge {
    /// Convert from a raw `MirResizeEdge` value.
    ///
    /// Unrecognized combinations fall back to [`ResizeEdge::None`].
    pub fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Left,
            2 => Self::Right,
            4 => Self::Top,
            8 => Self::Bottom,
            5 => Self::TopLeft,
            6 => Self::TopRight,
            9 => Self::BottomLeft,
            10 => Self::BottomRight,
            _ => Self::None,
        }
    }

    /// Convert to the raw `MirResizeEdge` value.
    pub fn to_raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Left => 1,
            Self::Right => 2,
            Self::Top => 4,
            Self::Bottom => 8,
            Self::TopLeft => 5,
            Self::TopRight => 6,
            Self::BottomLeft => 9,
            Self::BottomRight => 10,
        }
    }
}

/// Depth layer for window stacking order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DepthLayer {
    /// Application layer (default for normal windows).
    #[default]
    Application,
    /// Always below other windows.
    AlwaysBelow,
    /// Below normal application windows.
    Below,
    /// Above normal application windows.
    Above,
    /// Always above other windows.
    AlwaysAbove,
    /// Overlay layer (used by panels, OSDs).
    Overlay,
}

impl DepthLayer {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Application,
            1 => Self::AlwaysBelow,
            2 => Self::Below,
            3 => Self::Above,
            4 => Self::AlwaysAbove,
            5 => Self::Overlay,
            _ => Self::Application,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Application => 0,
            Self::AlwaysBelow => 1,
            Self::Below => 2,
            Self::Above => 3,
            Self::AlwaysAbove => 4,
            Self::Overlay => 5,
        }
    }
}

/// Focus mode determining how a window gains/loses focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FocusMode {
    /// Normal focus behavior.
    #[default]
    Focusable,
    /// Window is not focusable.
    Disabled,
    /// Grab focus immediately.
    Grabbing,
}

impl FocusMode {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Focusable,
            1 => Self::Disabled,
            2 => Self::Grabbing,
            _ => Self::Focusable,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Focusable => 0,
            Self::Disabled => 1,
            Self::Grabbing => 2,
        }
    }
}

/// Orientation mode for the window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OrientationMode {
    /// Portrait orientation.
    Portrait,
    /// Landscape orientation.
    Landscape,
    /// Inverted portrait.
    InvertedPortrait,
    /// Inverted landscape.
    InvertedLandscape,
    /// Any orientation is acceptable.
    #[default]
    Any,
}

impl OrientationMode {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            1 => Self::Portrait,
            2 => Self::Landscape,
            4 => Self::InvertedPortrait,
            8 => Self::InvertedLandscape,
            _ => Self::Any,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Portrait => 1,
            Self::Landscape => 2,
            Self::InvertedPortrait => 4,
            Self::InvertedLandscape => 8,
            Self::Any => 15,
        }
    }
}

/// Pointer confinement state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PointerConfinementState {
    /// Pointer is unconfined (default).
    #[default]
    Unconfined,
    /// Pointer is confined to the window area.
    ConfinedToWindow,
    /// Pointer is locked in place.
    LockedToWindow,
}

impl PointerConfinementState {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Unconfined,
            1 => Self::ConfinedToWindow,
            2 => Self::LockedToWindow,
            _ => Self::Unconfined,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Unconfined => 0,
            Self::ConfinedToWindow => 1,
            Self::LockedToWindow => 2,
        }
    }
}

/// Shell chrome type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShellChrome {
    /// Normal shell chrome.
    #[default]
    Normal,
    /// Low chrome (minimal decorations).
    Low,
}

impl ShellChrome {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Low,
            _ => Self::Normal,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Low => 1,
        }
    }
}

/// Placement gravity flags for window positioning.
///
/// These can be combined with bitwise OR for edges/corners.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PlacementGravity(
    /// Raw placement gravity flags.
    pub u32,
);

impl PlacementGravity {
    /// Center (no specific edge).
    pub const CENTER: Self = Self(0);
    /// Left edge.
    pub const LEFT: Self = Self(1);
    /// Right edge.
    pub const RIGHT: Self = Self(2);
    /// Top edge.
    pub const TOP: Self = Self(4);
    /// Bottom edge.
    pub const BOTTOM: Self = Self(8);

    /// Check if a specific edge is set.
    pub fn contains(&self, other: PlacementGravity) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Combine with another gravity.
    pub fn union(self, other: PlacementGravity) -> Self {
        Self(self.0 | other.0)
    }
}

/// Placement hints for popup positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PlacementHints(
    /// Raw placement hint flags.
    pub u32,
);

impl PlacementHints {
    /// No adjustment.
    pub const NONE: Self = Self(0);
    /// Flip horizontally if needed.
    pub const FLIP_X: Self = Self(1);
    /// Flip vertically if needed.
    pub const FLIP_Y: Self = Self(2);
    /// Slide horizontally if needed.
    pub const SLIDE_X: Self = Self(4);
    /// Slide vertically if needed.
    pub const SLIDE_Y: Self = Self(8);
    /// Resize horizontally if needed.
    pub const RESIZE_X: Self = Self(16);
    /// Resize vertically if needed.
    pub const RESIZE_Y: Self = Self(32);
    /// Allow antipodes (opposite anchor points).
    pub const ANTIPODES: Self = Self(64);
}

/// Tiled edge flags indicating which edges touch the tiling grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TiledEdges(
    /// Raw tiled edge flags.
    pub u32,
);

impl TiledEdges {
    /// No edges tiled.
    pub const NONE: Self = Self(0);
    /// Left edge tiled.
    pub const LEFT: Self = Self(1);
    /// Right edge tiled.
    pub const RIGHT: Self = Self(2);
    /// Top edge tiled.
    pub const TOP: Self = Self(4);
    /// Bottom edge tiled.
    pub const BOTTOM: Self = Self(8);

    /// Check if a specific edge is set.
    pub fn contains(&self, other: TiledEdges) -> bool {
        (self.0 & other.0) == other.0
    }
}

/// Input reception mode for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InputReceptionMode {
    /// Normal input routing.
    #[default]
    Normal,
    /// Receives all input events regardless of focus.
    ReceivesAllInput,
}

impl InputReceptionMode {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::ReceivesAllInput,
            _ => Self::Normal,
        }
    }

    /// Convert to a raw C enum value.
    pub(crate) fn to_raw(self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::ReceivesAllInput => 1,
        }
    }
}

/// Aspect ratio as a width:height pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AspectRatio {
    /// Width component of the ratio.
    pub width: u32,
    /// Height component of the ratio.
    pub height: u32,
}

impl AspectRatio {
    /// Create a new aspect ratio.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Policy for handling focus stealing attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusStealing {
    /// Prevent new windows from stealing focus.
    Prevent,
    /// Allow new windows to take focus.
    Allow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_documented_variants() {
        assert_eq!(WindowState::default(), WindowState::Unknown);
        assert_eq!(WindowType::default(), WindowType::Normal);
        assert_eq!(ResizeEdge::default(), ResizeEdge::None);
        assert_eq!(DepthLayer::default(), DepthLayer::Application);
        assert_eq!(FocusMode::default(), FocusMode::Focusable);
        assert_eq!(OrientationMode::default(), OrientationMode::Any);
        assert_eq!(
            PointerConfinementState::default(),
            PointerConfinementState::Unconfined
        );
        assert_eq!(ShellChrome::default(), ShellChrome::Normal);
        assert_eq!(InputReceptionMode::default(), InputReceptionMode::Normal);
    }

    #[test]
    fn window_state_round_trips_through_raw() {
        let all = [
            WindowState::Unknown,
            WindowState::Restored,
            WindowState::Minimized,
            WindowState::Maximized,
            WindowState::VertMaximized,
            WindowState::Fullscreen,
            WindowState::HorizMaximized,
            WindowState::Hidden,
            WindowState::Attached,
        ];
        for state in all {
            assert_eq!(WindowState::from_raw(state.to_raw()), state);
        }
        assert_eq!(WindowState::from_raw(999), WindowState::Unknown);
    }

    #[test]
    fn window_type_round_trips_through_raw() {
        let all = [
            WindowType::Normal,
            WindowType::Utility,
            WindowType::Dialog,
            WindowType::Gloss,
            WindowType::Freestyle,
            WindowType::Menu,
            WindowType::InputMethod,
            WindowType::Satellite,
            WindowType::Tip,
            WindowType::Decoration,
        ];
        for window_type in all {
            assert_eq!(WindowType::from_raw(window_type.to_raw()), window_type);
        }
        assert_eq!(WindowType::from_raw(-1), WindowType::Normal);
    }

    #[test]
    fn resize_edge_matches_mir_resize_edge_bitmask() {
        // MirResizeEdge is a bitmask: west = 1, east = 2, north = 4, south = 8, and the
        // corners are the bitwise-or of their two adjacent edges.
        assert_eq!(ResizeEdge::from_raw(0), ResizeEdge::None);
        assert_eq!(ResizeEdge::from_raw(1), ResizeEdge::Left);
        assert_eq!(ResizeEdge::from_raw(2), ResizeEdge::Right);
        assert_eq!(ResizeEdge::from_raw(4), ResizeEdge::Top);
        assert_eq!(ResizeEdge::from_raw(8), ResizeEdge::Bottom);
        assert_eq!(ResizeEdge::from_raw(4 | 1), ResizeEdge::TopLeft);
        assert_eq!(ResizeEdge::from_raw(4 | 2), ResizeEdge::TopRight);
        assert_eq!(ResizeEdge::from_raw(8 | 1), ResizeEdge::BottomLeft);
        assert_eq!(ResizeEdge::from_raw(8 | 2), ResizeEdge::BottomRight);
        assert_eq!(ResizeEdge::from_raw(3), ResizeEdge::None);
    }

    #[test]
    fn resize_edge_round_trips_through_raw() {
        let all = [
            ResizeEdge::None,
            ResizeEdge::Left,
            ResizeEdge::Right,
            ResizeEdge::Top,
            ResizeEdge::Bottom,
            ResizeEdge::TopLeft,
            ResizeEdge::TopRight,
            ResizeEdge::BottomLeft,
            ResizeEdge::BottomRight,
        ];
        for edge in all {
            assert_eq!(ResizeEdge::from_raw(edge.to_raw()), edge);
        }
    }

    #[test]
    fn remaining_enums_round_trip_through_raw() {
        for layer in [
            DepthLayer::Application,
            DepthLayer::AlwaysBelow,
            DepthLayer::Below,
            DepthLayer::Above,
            DepthLayer::AlwaysAbove,
            DepthLayer::Overlay,
        ] {
            assert_eq!(DepthLayer::from_raw(layer.to_raw()), layer);
        }
        for mode in [
            FocusMode::Focusable,
            FocusMode::Disabled,
            FocusMode::Grabbing,
        ] {
            assert_eq!(FocusMode::from_raw(mode.to_raw()), mode);
        }
        for mode in [
            OrientationMode::Portrait,
            OrientationMode::Landscape,
            OrientationMode::InvertedPortrait,
            OrientationMode::InvertedLandscape,
            OrientationMode::Any,
        ] {
            assert_eq!(OrientationMode::from_raw(mode.to_raw()), mode);
        }
        for state in [
            PointerConfinementState::Unconfined,
            PointerConfinementState::ConfinedToWindow,
            PointerConfinementState::LockedToWindow,
        ] {
            assert_eq!(PointerConfinementState::from_raw(state.to_raw()), state);
        }
        for chrome in [ShellChrome::Normal, ShellChrome::Low] {
            assert_eq!(ShellChrome::from_raw(chrome.to_raw()), chrome);
        }
        for mode in [
            InputReceptionMode::Normal,
            InputReceptionMode::ReceivesAllInput,
        ] {
            assert_eq!(InputReceptionMode::from_raw(mode.to_raw()), mode);
        }
    }

    #[test]
    fn unknown_raw_values_fall_back_to_defaults() {
        assert_eq!(DepthLayer::from_raw(99), DepthLayer::default());
        assert_eq!(FocusMode::from_raw(99), FocusMode::default());
        assert_eq!(OrientationMode::from_raw(99), OrientationMode::default());
        assert_eq!(
            PointerConfinementState::from_raw(99),
            PointerConfinementState::default()
        );
        assert_eq!(ShellChrome::from_raw(99), ShellChrome::default());
        assert_eq!(
            InputReceptionMode::from_raw(99),
            InputReceptionMode::default()
        );
    }
}
