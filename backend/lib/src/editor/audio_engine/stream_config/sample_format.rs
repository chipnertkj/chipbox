use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid sample format: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleFormat(pub cpal::SampleFormat);

impl From<cpal::SampleFormat> for SampleFormat {
    fn from(value: cpal::SampleFormat) -> Self {
        Self(value)
    }
}

impl FromStr for SampleFormat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const I8: &str = "i8";
        const I16: &str = "i16";
        const I32: &str = "i32";
        const I64: &str = "i64";
        const U8: &str = "u8";
        const U16: &str = "u16";
        const U32: &str = "u32";
        const U64: &str = "u64";
        const F32: &str = "f32";
        const F64: &str = "f64";

        match s.to_lowercase().as_str() {
            I8 => Ok(Self(cpal::SampleFormat::I8)),
            I16 => Ok(Self(cpal::SampleFormat::I16)),
            I32 => Ok(Self(cpal::SampleFormat::I32)),
            I64 => Ok(Self(cpal::SampleFormat::I64)),
            U8 => Ok(Self(cpal::SampleFormat::U8)),
            U16 => Ok(Self(cpal::SampleFormat::U16)),
            U32 => Ok(Self(cpal::SampleFormat::U32)),
            U64 => Ok(Self(cpal::SampleFormat::U64)),
            F32 => Ok(Self(cpal::SampleFormat::F32)),
            F64 => Ok(Self(cpal::SampleFormat::F64)),
            _ => Err(ParseError(s.into())),
        }
    }
}

impl std::fmt::Display for SampleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
