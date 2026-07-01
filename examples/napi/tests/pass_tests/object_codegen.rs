use napi::bindgen_prelude::{Either, Either3};
use napi_derive::napi;

#[napi(object, codegen = "compact")]
pub struct CompactObject {
  pub required: String,
  pub optional: Option<u32>,
  pub union: Either<String, u32>,
  pub optional_union: Option<Either<u32, bool>>,
}

#[napi(object, raw_fields, object_to_js = false)]
pub struct RawFieldsInput {
  pub value: Option<String>,
}

#[napi(object, codegen = "inline", object_from_js = false)]
pub struct InlineOutput {
  pub value: bool,
}

#[napi(object, codegen = "auto")]
pub struct AutoObject {
  pub value: u32,
}

#[napi(object)]
pub struct DefaultAutoFromCompact {
  pub a: String,
  pub b: u32,
  pub c: Option<bool>,
}

#[napi(object, codegen = "auto")]
pub struct AutoBothCompact {
  pub a: String,
  pub b: u32,
  pub c: Option<bool>,
  pub d: Option<String>,
  pub union: Either3<String, u32, bool>,
}

#[napi(codegen = "compact")]
pub enum CompactStructuredEnum {
  Named {
    value: Either<String, u32>,
    optional: Option<bool>,
  },
  Tuple(String, Option<Either<u32, bool>>),
}

#[napi(codegen = "auto")]
pub enum AutoStructuredEnum {
  A { value: String },
  B { first: u32, second: Option<String> },
}

// Needed for the trybuild tests.
#[allow(unused)]
fn main() {}
