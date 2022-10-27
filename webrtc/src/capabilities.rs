use core::fmt::{Debug, Display, Formatter};

use crate::errors::MediaStreamError;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum DeviceSupportedCapabilities {
    DeviceID,
    GroupID,
    AspectRatio,
    FacingMode,
    FrameRate,
    Height,
    Width,
    ResizeMode,
}

impl Display for DeviceSupportedCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cap = match self {
            DeviceSupportedCapabilities::DeviceID => "deviceId",
            DeviceSupportedCapabilities::GroupID => "groupId",
            DeviceSupportedCapabilities::AspectRatio => "aspectRatio",
            DeviceSupportedCapabilities::FacingMode => "facingMode",
            DeviceSupportedCapabilities::FrameRate => "frameRate",
            DeviceSupportedCapabilities::Height => "height",
            DeviceSupportedCapabilities::Width => "width",
            DeviceSupportedCapabilities::ResizeMode => "resizeMode",
        };

        write!(f, "{}", cap)
    }
}

impl Debug for DeviceSupportedCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.to_string();
        write!(f, "{}", str)
    }
}

impl TryFrom<String> for DeviceSupportedCapabilities {
    type Error = MediaStreamError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        let result = match value {
            "deviceId" => DeviceSupportedCapabilities::DeviceID,
            "groupId" => DeviceSupportedCapabilities::GroupID,
            "aspectRatio" => DeviceSupportedCapabilities::AspectRatio,
            "facingMode" => DeviceSupportedCapabilities::FacingMode,
            "frameRate" => DeviceSupportedCapabilities::FrameRate,
            "height" => DeviceSupportedCapabilities::Height,
            "width" => DeviceSupportedCapabilities::Width,
            "resizeMode" => DeviceSupportedCapabilities::ResizeMode,
            _ => {
                return Err(MediaStreamError::StructureError {
                    structure: "DeviceSupportedCapabilities".to_string(),
                    error: "No Match Str".to_string(),
                })
            }
        };
        Ok(result)
    }
}
