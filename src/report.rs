use std::{collections::HashMap, convert::TryFrom, fmt, path::PathBuf};

use bitflags::bitflags;
use serde::{
    Deserialize,
    de::{self, Visitor},
};
use serde_json::Value;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Report {
    #[serde(rename = "REPORT_ID")]
    pub id: String,
    pub app_version_code: u32,
    pub app_version_name: String,
    pub package_name: String,
    pub file_path: PathBuf,
    pub phone_model: String,
    pub brand: String,
    pub product: String,
    pub android_version: String,
    pub build: Build,
    pub total_mem_size: u64,
    pub available_mem_size: u64,
    pub build_config: HashMap<String, Value>,
    pub custom_data: HashMap<String, Value>,
    pub is_silent: bool,
    pub stack_trace: String,
    pub initial_configuration: Configuration,
    pub crash_configuration: Configuration,
    pub display: HashMap<String, Display>,
    pub user_comment: Option<String>,
    pub user_email: String,
    pub user_app_start_date: String,
    pub user_crash_date: String,
    pub dumpsys_meminfo: Option<String>,
    pub logcat: String,
    pub installation_id: String,
    pub device_features: HashMap<String, Value>,
    pub environment: HashMap<String, Value>,
    pub shared_preferences: HashMap<String, Value>,
    // non-default
    pub application_log: Option<String>,
    pub dropbox: Option<Value>,
    pub eventslog: Option<String>,
    pub media_codec_list: Option<HashMap<String, MediaCodec>>,
    pub radiolog: Option<String>,
    pub settings_global: Option<HashMap<String, String>>,
    pub settings_secure: Option<HashMap<String, String>>,
    pub settings_system: Option<HashMap<String, String>>,
    pub stack_trace_hash: Option<String>,
    pub thread_details: Option<ThreadDetails>,
    pub user_ip: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Build {
    pub board: String,
    pub bootloader: String,
    pub brand: String,
    pub cpu_abi: String,
    pub cpu_abi2: String,
    pub device: String,
    pub display: String,
    pub fingerprint: String,
    pub hardware: String,
    pub host: String,
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub product: String,
    pub radio: String,
    pub serial: String,
    pub supported_32_bit_abis: Option<Vec<String>>,
    pub supported_64_bit_abis: Option<Vec<String>>,
    pub supported_abis: Option<Vec<String>>,
    pub tags: String,
    pub time: u64,
    #[serde(rename = "TYPE")]
    pub type_: String,
    pub user: String,
    pub version: Version,
    // PERMISSIONS_REVIEW_REQUIRED
    // IS_EMULATOR
    // IS_DEBUGGABLE
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Version {
    pub base_os: Option<String>,
    pub codename: String,
    pub incremental: String,
    pub preview_sdk_int: Option<i32>,
    pub release: String,
    pub release_or_codename: Option<String>,
    pub sdk: String,
    pub sdk_int: i32,
    pub security_patch: Option<String>,
    // ACTIVE_CODENAMES
    // PREVIEW_SDK_FINGERPRINT
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub color_mode: Option<ColorMode>,
    pub density_dpi: Option<u16>,
    pub font_scale: f32,
    pub hard_keyboard_hidden: HardKeyboardHidden,
    pub keyboard: Keyboard,
    pub keyboard_hidden: KeyboardHidden,
    pub locale: String,
    pub mcc: i32,
    pub mnc: i32,
    pub navigation: Navigation,
    pub navigation_hidden: NavigationHidden,
    pub orientation: Orientation,
    pub screen_height_dp: u32,
    pub screen_layout: ScreenLayout,
    pub screen_width_dp: u32,
    pub smallest_screen_width_dp: u32,
    pub touchscreen: Touchscreen,
    pub ui_mode: UiMode,
    // assetsSeq
    // seq
    // userSetLocale
}

bitflags! {
    #[derive(Debug)]
    pub struct ColorMode: u32 {
        const WIDE_COLOR_GAMUT_NO  = 1;
        const WIDE_COLOR_GAMUT_YES = 2;

        const HDR_NO  = 4;
        const HDR_YES = 8;
    }
}

impl<'de> Deserialize<'de> for ColorMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColorModeVisitor;

        impl Visitor<'_> for ColorModeVisitor {
            type Value = ColorMode;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("color mode encoded as bit flags")
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ColorMode::from_bits_truncate(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match u32::try_from(value) {
                    Ok(v) => self.visit_u32(v),
                    Err(_) => Err(E::custom("value is out of range")),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match u32::try_from(value) {
                    Ok(v) => self.visit_u32(v),
                    Err(_) => Err(E::custom("value is out of range")),
                }
            }
        }

        deserializer.deserialize_u32(ColorModeVisitor)
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum HardKeyboardHidden {
    #[serde(rename = "HARDKEYBOARDHIDDEN_NO")]
    No,
    #[serde(rename = "HARDKEYBOARDHIDDEN_YES")]
    Yes,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Keyboard {
    #[serde(rename = "KEYBOARD_NOKEYS")]
    NoKeys,
    #[serde(rename = "KEYBOARD_QWERTY")]
    Qwerty,
    #[serde(rename = "KEYBOARD_12KEY")]
    TwelveKey,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum KeyboardHidden {
    #[serde(rename = "KEYBOARDHIDDEN_NO")]
    No,
    #[serde(rename = "KEYBOARDHIDDEN_YES")]
    Yes,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Navigation {
    #[serde(rename = "NAVIGATION_NONAV")]
    NoNav,
    #[serde(rename = "NAVIGATION_DPAD")]
    Dpad,
    #[serde(rename = "NAVIGATION_TRACKBALL")]
    Trackball,
    #[serde(rename = "NAVIGATION_WHEEL")]
    Wheel,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum NavigationHidden {
    #[serde(rename = "NAVIGATIONHIDDEN_NO")]
    No,
    #[serde(rename = "NAVIGATIONHIDDEN_YES")]
    Yes,
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Orientation {
    #[serde(rename = "ORIENTATION_LANDSCAPE")]
    Landscape,
    #[serde(rename = "ORIENTATION_PORTRAIT")]
    Portrait,
}

bitflags! {
    #[derive(Debug)]
    pub struct ScreenLayout: i32 {
        const SIZE_SMALL  = 1;
        const SIZE_NORMAL = 2;
        const SIZE_LARGE  = 3;
        const SIZE_XLARGE = 4;

        const LONG_NO  = 16;
        const LONG_YES = 32;

        const LAYOUTDIR_LTR = 64;
        const LAYOUTDIR_RTL = 128;

        const ROUND_NO  = 256;
        const ROUND_YES = 512;
    }
}

impl<'de> Deserialize<'de> for ScreenLayout {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScreenLayoutVisitor;

        impl Visitor<'_> for ScreenLayoutVisitor {
            type Value = ScreenLayout;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("screen layout encoded as strings concatenated with '+'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value.split('+').fold(ScreenLayout::empty(), |sl, value| {
                    sl | match value {
                        "SCREENLAYOUT_SIZE_SMALL" => ScreenLayout::SIZE_SMALL,
                        "SCREENLAYOUT_SIZE_NORMAL" => ScreenLayout::SIZE_NORMAL,
                        "SCREENLAYOUT_SIZE_LARGE" => ScreenLayout::SIZE_LARGE,
                        "SCREENLAYOUT_SIZE_XLARGE" => ScreenLayout::SIZE_XLARGE,
                        "SCREENLAYOUT_LONG_NO" => ScreenLayout::LONG_NO,
                        "SCREENLAYOUT_LONG_YES" => ScreenLayout::LONG_YES,
                        "SCREENLAYOUT_LAYOUTDIR_LTR" => ScreenLayout::LAYOUTDIR_LTR,
                        "SCREENLAYOUT_LAYOUTDIR_RTL" => ScreenLayout::LAYOUTDIR_RTL,
                        "SCREENLAYOUT_ROUND_NO" => ScreenLayout::ROUND_NO,
                        "SCREENLAYOUT_ROUND_YES" => ScreenLayout::ROUND_YES,
                        _ => ScreenLayout::empty(),
                    }
                }))
            }
        }

        deserializer.deserialize_str(ScreenLayoutVisitor)
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum Touchscreen {
    #[serde(rename = "TOUCHSCREEN_NOTOUCH")]
    NoTouch,
    #[serde(rename = "TOUCHSCREEN_FINGER")]
    Finger,
}

bitflags! {
    #[derive(Debug)]
    pub struct UiMode: i32 {
        const TYPE_NORMAL     = 1;
        const TYPE_DESK       = 2;
        const TYPE_CAR        = 3;
        const TYPE_TELEVISION = 4;
        const TYPE_APPLIANCE  = 5;
        const TYPE_WATCH      = 6;
        const TYPE_VR_HEADSET = 7;

        const NIGHT_NO  = 16;
        const NIGHT_YES = 32;
    }
}

impl<'de> Deserialize<'de> for UiMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct UiModeVisitor;

        impl Visitor<'_> for UiModeVisitor {
            type Value = UiMode;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("ui mode encoded as strings concatenated with '+'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value.split('+').fold(UiMode::empty(), |sl, value| {
                    sl | match value {
                        "UI_MODE_TYPE_NORMAL" => UiMode::TYPE_NORMAL,
                        "UI_MODE_TYPE_DESK" => UiMode::TYPE_DESK,
                        "UI_MODE_TYPE_CAR" => UiMode::TYPE_CAR,
                        "UI_MODE_TYPE_TELEVISION" => UiMode::TYPE_TELEVISION,
                        "UI_MODE_TYPE_APPLIANCE" => UiMode::TYPE_APPLIANCE,
                        "UI_MODE_TYPE_WATCHL" => UiMode::TYPE_WATCH,
                        "UI_MODE_TYPE_VR_HEADSET" => UiMode::TYPE_VR_HEADSET,
                        "UI_MODE_NIGHT_NO" => UiMode::NIGHT_NO,
                        "UI_MODE_NIGHT_YES" => UiMode::NIGHT_YES,
                        _ => UiMode::empty(),
                    }
                }))
            }
        }

        deserializer.deserialize_str(UiModeVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub current_size_range: SizeRange,
    pub flags: DisplayFlags,
    pub height: u32,
    pub is_valid: bool,
    pub metrics: DisplayMetrics,
    pub name: String,
    pub orientation: Rotation,
    pub pixel_format: PixelFormat,
    pub real_metrics: DisplayMetrics,
    pub real_size: (u32, u32),
    pub rect_size: (u32, u32, u32, u32),
    pub refresh_rate: f32,
    pub rotation: Rotation,
    pub size: (u32, u32),
    pub width: u32,
}

#[derive(Debug, Deserialize)]
pub struct SizeRange {
    pub largest: (u32, u32),
    pub smallest: (u32, u32),
}

bitflags! {
    #[derive(Debug)]
    pub struct DisplayFlags: u32 {
        const SUPPORTS_PROTECTED_BUFFERS = 1;
        const SECURE                     = 2;
        const PRIVATE                    = 4;
        const PRESENTATION               = 8;
        const ROUND                      = 16;
    }
}

impl<'de> Deserialize<'de> for DisplayFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct DisplayFlagsVisitor;

        impl Visitor<'_> for DisplayFlagsVisitor {
            type Value = DisplayFlags;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("display flags encoded as strings concatenated with '+'")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value.split('+').fold(DisplayFlags::empty(), |sl, value| {
                    sl | match value {
                        "FLAG_SUPPORTS_PROTECTED_BUFFERS" => {
                            DisplayFlags::SUPPORTS_PROTECTED_BUFFERS
                        }
                        "FLAG_SECURE" => DisplayFlags::SECURE,
                        "FLAG_PRIVATE" => DisplayFlags::PRIVATE,
                        "FLAG_PRESENTATION" => DisplayFlags::PRESENTATION,
                        "FLAG_ROUND" => DisplayFlags::ROUND,
                        _ => DisplayFlags::empty(),
                    }
                }))
            }
        }

        deserializer.deserialize_str(DisplayFlagsVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayMetrics {
    pub density: f32,
    pub density_dpi: u16,
    pub height_pixels: u32,
    pub scaled_density: String,
    pub width_pixels: u32,
    pub xdpi: f32,
    pub ydpi: f32,
}

#[derive(Debug)]
pub enum Rotation {
    Zero,
    Ninety,
    OneHundredEighty,
    TwoHundredSeventy,
}

impl<'de> Deserialize<'de> for Rotation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RotationVisitor;

        impl Visitor<'_> for RotationVisitor {
            type Value = Rotation;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("rotation either as integer or string")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match value {
                    0 => Rotation::Zero,
                    1 => Rotation::Ninety,
                    2 => Rotation::OneHundredEighty,
                    3 => Rotation::TwoHundredSeventy,
                    _ => return Err(E::custom(format!("unknown orientation {value}"))),
                })
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match u64::try_from(value) {
                    Ok(v) => self.visit_u64(v),
                    Err(_) => Err(E::custom("value is out of range")),
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match value {
                    "ROTATION_0" => Rotation::Zero,
                    "ROTATION_90" => Rotation::Ninety,
                    "ROTATION_180" => Rotation::OneHundredEighty,
                    "ROTATION_270" => Rotation::TwoHundredSeventy,
                    _ => return Err(E::custom(format!("unknown orientation '{value}'"))),
                })
            }
        }

        deserializer.deserialize_any(RotationVisitor)
    }
}

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
pub enum PixelFormat {
    Translucent = -3,
    Transparent = -2,
    Opaque = -1,
    Unknown = 0,
    Rgba8888 = 1,
    Rgbx8888 = 2,
    Rgb888 = 3,
    Rgb565 = 4,
    Rgba5551 = 6,
    Rgba4444 = 7,
    A8 = 8,
    L8 = 9,
    La88 = 10,
    Rgb332 = 11,
    YCbCr422Sp = 16,
    YCbCr420Sp = 17,
    YCbCr442l = 20,
    RgbaF16 = 22,
    Rgba1010102 = 43,
    Jpeg = 256,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCodec {
    pub is_encoder: bool,
    pub name: String,
    pub supported_types: HashMap<String, MediaType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub color_formats: Option<Vec<String>>,
    pub profile_levels: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadDetails {
    pub group_name: String,
    pub id: u32,
    pub name: String,
    pub priority: i8,
}
