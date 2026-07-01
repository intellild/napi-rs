use std::ffi::{c_void, CStr};
use std::sync::Arc;

pub use callback_info::*;
pub use env::*;
pub use iterator::Generator;
pub use js_values::*;
pub use module_register::*;

use super::sys;
use crate::{JsError, Result, Status, ValueType};

#[cfg(feature = "tokio_rt")]
pub mod async_iterator;
#[cfg(feature = "tokio_rt")]
pub use async_iterator::AsyncGenerator;
mod callback_info;
mod env;
mod error;
pub mod iterator;
mod js_values;
mod module_register;

pub trait ObjectFinalize: Sized {
  #[allow(unused)]
  fn finalize(self, env: Env) -> Result<()> {
    Ok(())
  }
}

/// # Safety
///
/// called when node wrapper objects destroyed
#[doc(hidden)]
pub(crate) unsafe extern "C" fn raw_finalize_unchecked<T: ObjectFinalize>(
  env: sys::napi_env,
  finalize_data: *mut c_void,
  _finalize_hint: *mut c_void,
) {
  let data: Box<T> = unsafe { Box::from_raw(finalize_data.cast()) };
  if let Err(err) = data.finalize(Env::from_raw(env)) {
    let e: JsError = err.into();
    unsafe { e.throw_into(env) };
    return;
  }
  if let Some((_, ref_val, finalize_callbacks_ptr)) =
    REFERENCE_MAP.with(|cell| cell.borrow_mut(|reference_map| reference_map.remove(&finalize_data)))
  {
    let finalize_callbacks_rc = unsafe { Arc::from_raw(finalize_callbacks_ptr) };

    #[cfg(all(debug_assertions, not(target_family = "wasm")))]
    {
      let rc_strong_count = Arc::strong_count(&finalize_callbacks_rc);
      // If `Arc` strong count is 2, it means the finalize of referenced `Object` is called before the `fn drop` of the `Reference`
      // It always happened on exiting process
      // In general, the `fn drop` would happen first
      if rc_strong_count != 1 && rc_strong_count != 2 {
        eprintln!("Arc strong count is: {rc_strong_count}, it should be 1 or 2");
      }
    }
    let finalize = unsafe { Box::from_raw(finalize_callbacks_rc.get()) };
    finalize();
    let delete_reference_status = unsafe { sys::napi_delete_reference(env, ref_val) };
    debug_assert!(
      delete_reference_status == sys::Status::napi_ok,
      "Delete reference in finalize callback failed {}",
      Status::from(delete_reference_status)
    );
  }
}

/// # Safety
///
/// called when node buffer is ready for gc
#[doc(hidden)]
pub unsafe extern "C" fn drop_buffer(
  _env: sys::napi_env,
  #[allow(unused)] finalize_data: *mut c_void,
  finalize_hint: *mut c_void,
) {
  #[cfg(all(debug_assertions, not(windows), not(target_family = "wasm")))]
  {
    js_values::BUFFER_DATA.with(|buffer_data| {
      let mut buffer = buffer_data.lock().expect("Unlock Buffer data failed");
      buffer.remove(&(finalize_data as *mut u8));
    });
  }
  unsafe {
    drop(Box::from_raw(finalize_hint as *mut Buffer));
  }
}

/// # Safety
///
/// called when node buffer slice is ready for gc
#[doc(hidden)]
pub unsafe extern "C" fn drop_buffer_slice(
  _env: sys::napi_env,
  finalize_data: *mut c_void,
  finalize_hint: *mut c_void,
) {
  let (len, cap): (usize, usize) = *unsafe { Box::from_raw(finalize_hint.cast()) };
  #[cfg(all(debug_assertions, not(windows), not(target_family = "wasm")))]
  {
    js_values::BUFFER_DATA.with(|buffer_data| {
      let mut buffer = buffer_data.lock().expect("Unlock Buffer data failed");
      buffer.remove(&(finalize_data as *mut u8));
    });
  }
  if finalize_data.is_null() {
    return;
  }
  unsafe {
    drop(Vec::from_raw_parts(finalize_data, len, cap));
  }
}

/// Create an object with properties
///
/// When the `experimental` feature is enabled, uses `node_api_create_object_with_properties`
/// which creates the object with all properties in a single optimized call.
/// Otherwise falls back to `napi_create_object` + `napi_define_properties`.
#[doc(hidden)]
#[cfg(not(feature = "noop"))]
#[inline]
pub unsafe fn create_object_with_properties(
  env: sys::napi_env,
  properties: &[sys::napi_property_descriptor],
) -> Result<sys::napi_value> {
  use crate::check_status;

  let mut obj_ptr = std::ptr::null_mut();

  #[cfg(all(
    feature = "experimental",
    feature = "node_version_detect",
    not(target_family = "wasm")
  ))]
  {
    let node_version = NODE_VERSION.get().unwrap();
    if !properties.is_empty()
      && ((node_version.major == 25 && node_version.minor >= 2) || node_version.major > 25)
    {
      // Convert property names from C strings to napi_value
      let mut names: Vec<sys::napi_value> = Vec::with_capacity(properties.len());
      let mut values: Vec<sys::napi_value> = Vec::with_capacity(properties.len());

      for prop in properties {
        let mut name_value = std::ptr::null_mut();
        // utf8name is a null-terminated C string, use -1 to auto-detect length
        check_status!(
          sys::napi_create_string_utf8(env, prop.utf8name, -1, &mut name_value),
          "Failed to create property name string",
        )?;
        names.push(name_value);
        values.push(prop.value);
      }

      let mut result_obj = std::ptr::null_mut();
      check_status!(
        sys::node_api_create_object_with_properties(
          env,
          std::ptr::null_mut(), // prototype_or_null
          names.as_ptr(),
          values.as_ptr(),
          properties.len(),
          &mut result_obj,
        ),
        "Failed to create object with properties",
      )?;
      return Ok(result_obj);
    }
  }

  // Fallback: create object then define properties
  check_status!(
    sys::napi_create_object(env, &mut obj_ptr),
    "Failed to create object",
  )?;

  if !properties.is_empty() {
    check_status!(
      sys::napi_define_properties(env, obj_ptr, properties.len(), properties.as_ptr()),
      "Failed to define properties",
    )?;
  }

  Ok(obj_ptr)
}

#[doc(hidden)]
#[inline]
pub unsafe fn create_property_descriptor(
  name: &CStr,
  value: sys::napi_value,
) -> sys::napi_property_descriptor {
  sys::napi_property_descriptor {
    utf8name: name.as_ptr(),
    name: std::ptr::null_mut(),
    method: None,
    getter: None,
    setter: None,
    value,
    attributes: sys::PropertyAttributes::writable
      | sys::PropertyAttributes::enumerable
      | sys::PropertyAttributes::configurable,
    data: std::ptr::null_mut(),
  }
}

#[doc(hidden)]
#[cfg(not(feature = "noop"))]
#[inline]
pub unsafe fn get_named_property_raw(
  env: sys::napi_env,
  object: sys::napi_value,
  name: &CStr,
) -> Result<Option<sys::napi_value>> {
  use crate::check_status;

  let mut value = std::ptr::null_mut();
  check_status!(
    sys::napi_get_named_property(env, object, name.as_ptr(), &mut value),
    "get_named_property_raw error"
  )?;

  let ty = crate::type_of!(env, value)?;
  Ok(if ty == ValueType::Undefined {
    None
  } else {
    Some(value)
  })
}

#[doc(hidden)]
#[cfg(feature = "noop")]
#[inline]
pub unsafe fn get_named_property_raw(
  _env: sys::napi_env,
  _object: sys::napi_value,
  _name: &CStr,
) -> Result<Option<sys::napi_value>> {
  Ok(None)
}

#[doc(hidden)]
#[cfg(not(feature = "noop"))]
#[inline]
pub unsafe fn set_named_property_raw<T>(
  env: sys::napi_env,
  object: sys::napi_value,
  name: &CStr,
  value: T,
) -> Result<()>
where
  T: ToNapiValue,
{
  use crate::check_status;

  check_status!(sys::napi_set_named_property(
    env,
    object,
    name.as_ptr(),
    T::to_napi_value(env, value)?,
  ))
}

#[doc(hidden)]
#[cfg(feature = "noop")]
#[inline]
pub unsafe fn set_named_property_raw<T>(
  _env: sys::napi_env,
  _object: sys::napi_value,
  _name: &CStr,
  _value: T,
) -> Result<()>
where
  T: ToNapiValue,
{
  Ok(())
}

#[doc(hidden)]
#[inline]
pub unsafe fn from_raw_required_field<T>(
  env: sys::napi_env,
  value: Option<sys::napi_value>,
  struct_name: &str,
  field: &str,
) -> Result<T>
where
  T: FromNapiValue,
{
  match value {
    Some(value) => T::from_napi_value(env, value)
      .map_err(|err| crate::error::decorate_field_error(err, struct_name, field)),
    None => Err(crate::error::missing_field_error(field)),
  }
}

#[doc(hidden)]
#[inline]
pub unsafe fn from_raw_optional_field<T>(
  env: sys::napi_env,
  value: Option<sys::napi_value>,
  struct_name: &str,
  field: &str,
) -> Result<Option<T>>
where
  T: FromNapiValue,
{
  value
    .map(|value| {
      T::from_napi_value(env, value)
        .map_err(|err| crate::error::decorate_field_error(err, struct_name, field))
    })
    .transpose()
}

#[doc(hidden)]
#[cfg(feature = "noop")]
pub unsafe fn create_object_with_properties(
  _env: sys::napi_env,
  _properties: &[sys::napi_property_descriptor],
) -> Result<sys::napi_value> {
  Ok(std::ptr::null_mut())
}
