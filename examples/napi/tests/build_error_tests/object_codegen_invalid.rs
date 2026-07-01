use napi_derive::napi;

#[napi(object, codegen = "tiny")]
pub struct InvalidObjectCodegen {
  pub value: u32,
}

// Needed for the trybuild tests.
#[allow(unused)]
fn main() {}
