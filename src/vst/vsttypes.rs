pub use crate::base::tchar as TChar;
pub type String128 = [TChar; 128];
pub type CString = *const crate::base::char8;
pub type MediaType = i32;
pub type BusDirection = i32;
pub type BusType = i32;
pub type IoMode = i32;
pub type UnitID = i32;
pub type ParamValue = f64;
pub type ParamID = u32;
pub type ProgramListID = i32;
pub type CtrlNumber = i16;
pub type TQuarterNote = f64;
pub type TSamples = i64;
pub type ColorSpec = u32;
pub type Sample32 = f32;
pub type Sample64 = f64;
pub type SampleRate = f64;
pub type SpeakerArrangement = u64;
pub type Speaker = u64;

pub const kNoParamId: ParamID = 0xFFFF_FFFF;
pub const kVstVersionMajor: i32 = 3;
pub const kVstVersionMinor: i32 = 6;
pub const kVstVersionSub: i32 = 13;
pub const VST_VERSION: i32 = (kVstVersionMajor << 16) | (kVstVersionMinor << 8) | kVstVersionSub;
pub const kVstVersionString: CString = b"VST 3.6.13".as_ptr() as *const _;
