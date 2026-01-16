use crate::prelude::*;

pub const CURRENT_PROTOCOL_VERSION: VarInt = VarInt(774);
pub const CURRENT_VERSION_NAME: &str = "1.21.11";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolVersion(VarInt);

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self(CURRENT_PROTOCOL_VERSION)
    }
}

impl<const N: usize> From<ProtocolVersion> for String<N> {
    fn from(value: ProtocolVersion) -> Self {
        String::try_from(value.version_name().expect("Version isn't part of the k nown version numbers"))
            .expect("Version names are never longer than 8 characters")
    }
}

impl core::ops::Deref for ProtocolVersion {
    type Target = VarInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ProtocolVersion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ProtocolVersion {
    pub fn version_name(&self) -> Option<&'static str> {
        Some(match *self.0 {
            774 => "1.21.11",
            773 => "1.21.10",
            772 => "1.21.7",
            771 => "1.21.6",
            770 => "1.21.5",
            769 => "1.21.4",
            768 => "1.21.2",
            767 => "1.21",

            766 => "1.20.5",
            765 => "1.20.3",
            764 => "1.20.2",
            763 => "1.20",

            762 => "1.19.4",
            761 => "1.19.3",
            760 => "1.19.1",
            759 => "1.19",

            758 => "1.18.2",
            757 => "1.18",

            756 => "1.17.1",
            755 => "1.17",

            754 => "1.16.4",
            753 => "1.16.3",
            751 => "1.16.2",
            736 => "1.16.1",
            735 => "1.16",

            578 => "1.15.2",
            575 => "1.15.1",
            573 => "1.15",

            498 => "1.14.4",
            490 => "1.14.3",
            485 => "1.14.2",
            480 => "1.14.1",
            477 => "1.14",

            404 => "1.13.2",
            401 => "1.13.1",
            393 => "1.13",

            340 => "1.12.2",
            338 => "1.12.1",
            335 => "1.12",

            316 => "1.11.1",
            315 => "1.11",

            210 => "1.10",

            110 => "1.9.3",
            109 => "1.9.2",
            108 => "1.9.1",
            107 => "1.9",

            47 => "1.8",

            5 => "1.7.6",
            4 => "1.7.2",

            _ => return None,
        })
    }
}
