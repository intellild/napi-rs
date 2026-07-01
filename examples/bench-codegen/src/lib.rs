#![allow(clippy::too_many_arguments, unused_macros)]

use napi::bindgen_prelude::{Buffer, Either, Either3};
use napi_derive::napi;

macro_rules! small_baseline_object {
  ($name:ident) => {
    #[cfg_attr(feature = "object-codegen-modes", napi(object, codegen = "inline"))]
    #[cfg_attr(not(feature = "object-codegen-modes"), napi(object))]
    pub struct $name {
      pub a: String,
      pub b: u32,
    }
  };
}

macro_rules! medium_baseline_object {
  ($name:ident) => {
    #[cfg_attr(feature = "object-codegen-modes", napi(object, codegen = "inline"))]
    #[cfg_attr(not(feature = "object-codegen-modes"), napi(object))]
    pub struct $name {
      pub name: String,
      pub count: u32,
      pub enabled: bool,
      pub tags: Option<Vec<String>>,
      pub threshold: Option<f64>,
      pub mode: Option<String>,
    }
  };
}

macro_rules! large_baseline_object {
  ($name:ident) => {
    #[cfg_attr(feature = "object-codegen-modes", napi(object, codegen = "inline"))]
    #[cfg_attr(not(feature = "object-codegen-modes"), napi(object))]
    pub struct $name {
      pub f01: String,
      pub f02: String,
      pub f03: String,
      pub f04: String,
      pub f05: String,
      pub n01: u32,
      pub n02: u32,
      pub n03: u32,
      pub n04: u32,
      pub n05: u32,
      pub b01: bool,
      pub b02: bool,
      pub b03: bool,
      pub o01: Option<String>,
      pub o02: Option<String>,
      pub o03: Option<u32>,
      pub o04: Option<bool>,
      pub o05: Option<Vec<String>>,
      pub p01: Option<f64>,
      pub p02: Option<f64>,
    }
  };
}

macro_rules! union_baseline_object {
  ($name:ident) => {
    #[cfg_attr(feature = "object-codegen-modes", napi(object, codegen = "inline"))]
    #[cfg_attr(not(feature = "object-codegen-modes"), napi(object))]
    pub struct $name {
      pub names: Either<String, Vec<String>>,
      pub payload: Either3<Buffer, String, ()>,
      pub maybe_flag: Option<Either<u32, bool>>,
    }
  };
}

macro_rules! structured_baseline_enum {
  ($name:ident) => {
    #[cfg_attr(feature = "object-codegen-modes", napi(codegen = "inline"))]
    #[cfg_attr(not(feature = "object-codegen-modes"), napi)]
    pub enum $name {
      Alpha {
        id: String,
        count: u32,
      },
      Beta {
        tags: Vec<String>,
        enabled: bool,
      },
      Gamma {
        value: Either<String, u32>,
        extra: Option<String>,
      },
      Delta(String, u32),
      Epsilon {
        left: String,
        right: String,
        flag: Option<bool>,
      },
    }
  };
}

macro_rules! small_object {
  ($name:ident, $($napi_args:tt)*) => {
    #[napi($($napi_args)*)]
    pub struct $name {
      pub a: String,
      pub b: u32,
    }
  };
}

macro_rules! medium_object {
  ($name:ident, $($napi_args:tt)*) => {
    #[napi($($napi_args)*)]
    pub struct $name {
      pub name: String,
      pub count: u32,
      pub enabled: bool,
      pub tags: Option<Vec<String>>,
      pub threshold: Option<f64>,
      pub mode: Option<String>,
    }
  };
}

macro_rules! large_object {
  ($name:ident, $($napi_args:tt)*) => {
    #[napi($($napi_args)*)]
    pub struct $name {
      pub f01: String,
      pub f02: String,
      pub f03: String,
      pub f04: String,
      pub f05: String,
      pub n01: u32,
      pub n02: u32,
      pub n03: u32,
      pub n04: u32,
      pub n05: u32,
      pub b01: bool,
      pub b02: bool,
      pub b03: bool,
      pub o01: Option<String>,
      pub o02: Option<String>,
      pub o03: Option<u32>,
      pub o04: Option<bool>,
      pub o05: Option<Vec<String>>,
      pub p01: Option<f64>,
      pub p02: Option<f64>,
    }
  };
}

macro_rules! union_object {
  ($name:ident, $($napi_args:tt)*) => {
    #[napi($($napi_args)*)]
    pub struct $name {
      pub names: Either<String, Vec<String>>,
      pub payload: Either3<Buffer, String, ()>,
      pub maybe_flag: Option<Either<u32, bool>>,
    }
  };
}

macro_rules! structured_enum {
  ($name:ident, $($napi_args:tt)*) => {
    #[napi($($napi_args)*)]
    pub enum $name {
      Alpha {
        id: String,
        count: u32,
      },
      Beta {
        tags: Vec<String>,
        enabled: bool,
      },
      Gamma {
        value: Either<String, u32>,
        extra: Option<String>,
      },
      Delta(String, u32),
      Epsilon {
        left: String,
        right: String,
        flag: Option<bool>,
      },
    }
  };
}

small_baseline_object!(SmallBaseline);
medium_baseline_object!(MediumBaseline);
large_baseline_object!(LargeBaseline);
union_baseline_object!(UnionBaseline);
structured_baseline_enum!(StructuredBaseline);

#[cfg(feature = "object-codegen-modes")]
small_object!(SmallInline, object, codegen = "inline");
#[cfg(feature = "object-codegen-modes")]
small_object!(SmallCompact, object, codegen = "compact");
#[cfg(feature = "object-codegen-modes")]
small_object!(SmallAuto, object, codegen = "auto");

#[cfg(feature = "object-codegen-modes")]
medium_object!(MediumInline, object, codegen = "inline");
#[cfg(feature = "object-codegen-modes")]
medium_object!(MediumCompact, object, codegen = "compact");
#[cfg(feature = "object-codegen-modes")]
medium_object!(MediumAuto, object, codegen = "auto");

#[cfg(feature = "object-codegen-modes")]
large_object!(LargeInline, object, codegen = "inline");
#[cfg(feature = "object-codegen-modes")]
large_object!(LargeCompact, object, codegen = "compact");
#[cfg(feature = "object-codegen-modes")]
large_object!(LargeAuto, object, codegen = "auto");

#[cfg(feature = "object-codegen-modes")]
union_object!(UnionInline, object, codegen = "inline");
#[cfg(feature = "object-codegen-modes")]
union_object!(UnionCompact, object, codegen = "compact");
#[cfg(feature = "object-codegen-modes")]
union_object!(UnionAuto, object, codegen = "auto");

#[cfg(feature = "object-codegen-modes")]
structured_enum!(StructuredInline, codegen = "inline");
#[cfg(feature = "object-codegen-modes")]
structured_enum!(StructuredCompact, codegen = "compact");
#[cfg(feature = "object-codegen-modes")]
structured_enum!(StructuredAuto, codegen = "auto");

#[napi]
pub fn consume_small_baseline(input: SmallBaseline) -> u32 {
  input.b + input.a.len() as u32
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_small_inline(input: SmallInline) -> u32 {
  input.b + input.a.len() as u32
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_small_compact(input: SmallCompact) -> u32 {
  input.b + input.a.len() as u32
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_small_auto(input: SmallAuto) -> u32 {
  input.b + input.a.len() as u32
}

#[napi]
pub fn create_small_baseline() -> SmallBaseline {
  SmallBaseline {
    a: "alpha".to_owned(),
    b: 42,
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn create_small_inline() -> SmallInline {
  SmallInline {
    a: "alpha".to_owned(),
    b: 42,
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn create_small_compact() -> SmallCompact {
  SmallCompact {
    a: "alpha".to_owned(),
    b: 42,
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn create_small_auto() -> SmallAuto {
  SmallAuto {
    a: "alpha".to_owned(),
    b: 42,
  }
}

#[napi]
pub fn consume_medium_baseline(input: MediumBaseline) -> u32 {
  score_medium(input.count, input.tags)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_medium_inline(input: MediumInline) -> u32 {
  score_medium(input.count, input.tags)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_medium_compact(input: MediumCompact) -> u32 {
  score_medium(input.count, input.tags)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_medium_auto(input: MediumAuto) -> u32 {
  score_medium(input.count, input.tags)
}

fn score_medium(count: u32, tags: Option<Vec<String>>) -> u32 {
  count + tags.map(|tags| tags.len() as u32).unwrap_or_default()
}

macro_rules! consume_large {
  ($name:ident, $ty:ty) => {
    #[napi]
    pub fn $name(input: $ty) -> u32 {
      input.n01
        + input.n02
        + input.n03
        + input.n04
        + input.n05
        + input.f01.len() as u32
        + input
          .o05
          .map(|items| items.len() as u32)
          .unwrap_or_default()
    }
  };
}

consume_large!(consume_large_baseline, LargeBaseline);
#[cfg(feature = "object-codegen-modes")]
consume_large!(consume_large_inline, LargeInline);
#[cfg(feature = "object-codegen-modes")]
consume_large!(consume_large_compact, LargeCompact);
#[cfg(feature = "object-codegen-modes")]
consume_large!(consume_large_auto, LargeAuto);

#[napi]
pub fn consume_union_baseline(input: UnionBaseline) -> u32 {
  score_union(input.names, input.payload, input.maybe_flag)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_union_inline(input: UnionInline) -> u32 {
  score_union(input.names, input.payload, input.maybe_flag)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_union_compact(input: UnionCompact) -> u32 {
  score_union(input.names, input.payload, input.maybe_flag)
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_union_auto(input: UnionAuto) -> u32 {
  score_union(input.names, input.payload, input.maybe_flag)
}

fn score_union(
  names: Either<String, Vec<String>>,
  payload: Either3<Buffer, String, ()>,
  maybe_flag: Option<Either<u32, bool>>,
) -> u32 {
  let names_score = match names {
    Either::A(value) => value.len() as u32,
    Either::B(value) => value.len() as u32,
  };
  let payload_score = match payload {
    Either3::A(value) => value.len() as u32,
    Either3::B(value) => value.len() as u32,
    Either3::C(()) => 0,
  };
  let flag_score = match maybe_flag {
    Some(Either::A(value)) => value,
    Some(Either::B(value)) => u32::from(value),
    None => 0,
  };
  names_score + payload_score + flag_score
}

#[napi]
pub fn consume_structured_baseline(input: StructuredBaseline) -> u32 {
  match input {
    StructuredBaseline::Alpha { id, count } => id.len() as u32 + count,
    StructuredBaseline::Beta { tags, enabled } => tags.len() as u32 + u32::from(enabled),
    StructuredBaseline::Gamma { value, extra } => {
      score_either(value) + extra.map(|s| s.len() as u32).unwrap_or_default()
    }
    StructuredBaseline::Delta(name, count) => name.len() as u32 + count,
    StructuredBaseline::Epsilon { left, right, flag } => {
      left.len() as u32 + right.len() as u32 + flag.map(u32::from).unwrap_or_default()
    }
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_structured_inline(input: StructuredInline) -> u32 {
  match input {
    StructuredInline::Alpha { id, count } => id.len() as u32 + count,
    StructuredInline::Beta { tags, enabled } => tags.len() as u32 + u32::from(enabled),
    StructuredInline::Gamma { value, extra } => {
      score_either(value) + extra.map(|s| s.len() as u32).unwrap_or_default()
    }
    StructuredInline::Delta(name, count) => name.len() as u32 + count,
    StructuredInline::Epsilon { left, right, flag } => {
      left.len() as u32 + right.len() as u32 + flag.map(u32::from).unwrap_or_default()
    }
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_structured_compact(input: StructuredCompact) -> u32 {
  match input {
    StructuredCompact::Alpha { id, count } => id.len() as u32 + count,
    StructuredCompact::Beta { tags, enabled } => tags.len() as u32 + u32::from(enabled),
    StructuredCompact::Gamma { value, extra } => {
      score_either(value) + extra.map(|s| s.len() as u32).unwrap_or_default()
    }
    StructuredCompact::Delta(name, count) => name.len() as u32 + count,
    StructuredCompact::Epsilon { left, right, flag } => {
      left.len() as u32 + right.len() as u32 + flag.map(u32::from).unwrap_or_default()
    }
  }
}

#[cfg(feature = "object-codegen-modes")]
#[napi]
pub fn consume_structured_auto(input: StructuredAuto) -> u32 {
  match input {
    StructuredAuto::Alpha { id, count } => id.len() as u32 + count,
    StructuredAuto::Beta { tags, enabled } => tags.len() as u32 + u32::from(enabled),
    StructuredAuto::Gamma { value, extra } => {
      score_either(value) + extra.map(|s| s.len() as u32).unwrap_or_default()
    }
    StructuredAuto::Delta(name, count) => name.len() as u32 + count,
    StructuredAuto::Epsilon { left, right, flag } => {
      left.len() as u32 + right.len() as u32 + flag.map(u32::from).unwrap_or_default()
    }
  }
}

fn score_either(value: Either<String, u32>) -> u32 {
  match value {
    Either::A(value) => value.len() as u32,
    Either::B(value) => value,
  }
}
