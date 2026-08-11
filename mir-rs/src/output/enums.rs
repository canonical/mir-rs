/*
 * Copyright © Canonical Ltd.
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 or 3,
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Output-related enumerations.

/// Display connector type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum OutputType {
    /// Unknown connector type.
    #[default]
    Unknown,
    /// VGA connector.
    Vga,
    /// DVI-I connector.
    DviI,
    /// DVI-D connector.
    DviD,
    /// DVI-A connector.
    DviA,
    /// Composite video.
    Composite,
    /// S-Video.
    SVideo,
    /// LVDS (laptop panel).
    Lvds,
    /// Component video.
    Component,
    /// 9-pin DIN.
    NinePinDin,
    /// DisplayPort.
    DisplayPort,
    /// HDMI Type A.
    HdmiA,
    /// HDMI Type B.
    HdmiB,
    /// TV connector.
    Tv,
    /// Embedded DisplayPort (laptop panel).
    Edp,
    /// Virtual display.
    Virtual,
    /// DSI (mobile display).
    Dsi,
    /// DPI (parallel display interface).
    Dpi,
}

impl OutputType {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Vga,
            2 => Self::DviI,
            3 => Self::DviD,
            4 => Self::DviA,
            5 => Self::Composite,
            6 => Self::SVideo,
            7 => Self::Lvds,
            8 => Self::Component,
            9 => Self::NinePinDin,
            10 => Self::DisplayPort,
            11 => Self::HdmiA,
            12 => Self::HdmiB,
            13 => Self::Tv,
            14 => Self::Edp,
            15 => Self::Virtual,
            16 => Self::Dsi,
            17 => Self::Dpi,
            _ => Self::Unknown,
        }
    }
}

/// Physical dimensions of a display in millimeters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhysicalSizeMM {
    /// Width in millimeters.
    pub width: i32,
    /// Height in millimeters.
    pub height: i32,
}

/// Display power mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PowerMode {
    /// Display is on.
    #[default]
    On,
    /// Display is in standby.
    Standby,
    /// Display is suspended.
    Suspend,
    /// Display is off.
    Off,
}

impl PowerMode {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::On,
            1 => Self::Standby,
            2 => Self::Suspend,
            3 => Self::Off,
            _ => Self::On,
        }
    }
}

/// Display orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Orientation {
    /// Normal (0°).
    #[default]
    Normal,
    /// Rotated 90° clockwise (left).
    Left,
    /// Rotated 180° (inverted).
    Inverted,
    /// Rotated 90° counter-clockwise (right).
    Right,
}

impl Orientation {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Left,
            2 => Self::Inverted,
            3 => Self::Right,
            _ => Self::Normal,
        }
    }
}

/// Display form factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum FormFactor {
    /// Unknown form factor.
    #[default]
    Unknown,
    /// Standard monitor.
    Monitor,
    /// Television.
    Tv,
    /// Projector.
    Projector,
    /// Phone display.
    Phone,
    /// Tablet display.
    Tablet,
}

impl FormFactor {
    /// Convert from raw C enum value.
    pub(crate) fn from_raw(value: i32) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Monitor,
            2 => Self::Tv,
            3 => Self::Projector,
            4 => Self::Phone,
            5 => Self::Tablet,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_documented_variants() {
        assert_eq!(OutputType::default(), OutputType::Unknown);
        assert_eq!(PowerMode::default(), PowerMode::On);
        assert_eq!(Orientation::default(), Orientation::Normal);
        assert_eq!(FormFactor::default(), FormFactor::Unknown);
    }

    #[test]
    fn from_raw_maps_known_values() {
        assert_eq!(OutputType::from_raw(1), OutputType::Vga);
        assert_eq!(OutputType::from_raw(11), OutputType::HdmiA);
        assert_eq!(PowerMode::from_raw(3), PowerMode::Off);
        assert_eq!(Orientation::from_raw(1), Orientation::Left);
        assert_eq!(FormFactor::from_raw(1), FormFactor::Monitor);
    }

    #[test]
    fn from_raw_falls_back_to_default_for_unknown_values() {
        assert_eq!(OutputType::from_raw(999), OutputType::default());
        assert_eq!(PowerMode::from_raw(999), PowerMode::default());
        assert_eq!(Orientation::from_raw(999), Orientation::default());
        assert_eq!(FormFactor::from_raw(999), FormFactor::default());
    }
}
