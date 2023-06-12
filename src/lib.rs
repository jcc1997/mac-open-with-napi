#![deny(clippy::all)]

use core_foundation::url::CFURL;
use launch_services::{application_urls_for_url, default_application_url_for_url, LSRolesMask};
use napi::{bindgen_prelude::Array, Env};
use napi_derive::napi;
use std::path::Path;

#[napi(object)]
struct AppInfo {
  pub url: String,
  pub is_default: bool,
}

#[napi]
pub fn urls_for_file(env: Env, url: String) -> Array {
  let file_path = Path::new(&url);
  let cf_url = CFURL::from_path(file_path, false).unwrap();
  let app_urls = application_urls_for_url(&cf_url, LSRolesMask::ALL).unwrap();
  let default_url = default_application_url_for_url(&cf_url, LSRolesMask::ALL).unwrap();
  let default_absolute_path = default_url.absolute().to_path().unwrap();
  let default_absolute_url = default_absolute_path.to_str().unwrap();
  let mut arr = env.create_array(0).unwrap();

  for url in &app_urls {
    let absolute_path = url.absolute().to_path().unwrap();
    let absolute_url = absolute_path.to_str().unwrap();
    arr
      .insert(AppInfo {
        url: absolute_url.to_string(),
        is_default: absolute_url == default_absolute_url,
      })
      .unwrap();
  }

  arr
}
