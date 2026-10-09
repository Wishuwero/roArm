#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__BodyPart() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__BodyPart__init(msg: *mut BodyPart) -> bool;
    fn ros2web_interfaces__msg__BodyPart__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BodyPart>, size: usize) -> bool;
    fn ros2web_interfaces__msg__BodyPart__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BodyPart>);
    fn ros2web_interfaces__msg__BodyPart__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BodyPart>, out_seq: *mut rosidl_runtime_rs::Sequence<BodyPart>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__BodyPart
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyPart {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub content_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub headers: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for BodyPart {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__BodyPart__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__BodyPart__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BodyPart {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__BodyPart__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__BodyPart__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__BodyPart__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BodyPart {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BodyPart where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/BodyPart";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__BodyPart() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__ContentType() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__ContentType__init(msg: *mut ContentType) -> bool;
    fn ros2web_interfaces__msg__ContentType__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ContentType>, size: usize) -> bool;
    fn ros2web_interfaces__msg__ContentType__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ContentType>);
    fn ros2web_interfaces__msg__ContentType__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ContentType>, out_seq: *mut rosidl_runtime_rs::Sequence<ContentType>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__ContentType
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ContentType {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl ContentType {
    /// application
    pub const APPLICATION_JSON: &'static str = "application/json";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_MSWORD: &'static str = "application/msword";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_MSWORD_OPEN_XML: &'static str = "application/vnd.openxmlformats-officedocument.wordprocessingml.document";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_OCTET_STREAM: &'static str = "application/octet-stream";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_VND_AMAZON_EBOOK: &'static str = "application/vnd.amazon.ebook";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_ABIWORD: &'static str = "application/x-abiword";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_BZIP2: &'static str = "application/x-bzip2";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_BZIP: &'static str = "application/x-bzip";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_CSH: &'static str = "application/x-csh";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_FREEARC: &'static str = "application/x-freearc";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const APPLICATION_X_WWW_FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded";

    /// application/epub+zip
    /// application/gzip
    /// application/java-archive
    /// application/ld+json
    /// application/ogg
    /// application/pdf
    /// application/rtf
    /// application/vnd.apple.installer+xml
    /// application/vnd.mozilla.xul+xml
    /// application/vnd.ms-excel
    /// application/vnd.ms-fontobject
    /// application/vnd.ms-powerpoint
    /// application/vnd.oasis.opendocument.presentation
    /// application/vnd.oasis.opendocument.spreadsheet
    /// application/vnd.oasis.opendocument.text
    /// application/vnd.openxmlformats-officedocument.presentationml.presentation
    /// application/vnd.rar
    /// application/vnd.visio
    /// application/x-7z-compressed
    /// application/x-httpd-php
    /// application/x-sh
    /// application/x-shockwave-flash
    /// application/x-tar
    /// application/xhtml+xml
    /// application/xml
    /// application/zip
    /// audio
    pub const AUDIO_AAC: &'static str = "audio/aac";

    /// audio/3gpp
    /// audio/3gpp2
    /// audio/midi
    /// audio/mpeg
    /// audio/ogg
    /// audio/opus
    /// audio/wav
    /// audio/webm
    /// audio/x-midi
    /// font
    /// font/otf
    /// font/ttf
    /// font/woff
    /// font/woff2
    /// image
    pub const IMAGE_BMP: &'static str = "image/bmp";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IMAGE_JPEG: &'static str = "image/jpeg";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const IMAGE_PNG: &'static str = "image/png";

    /// image/gif
    /// image/svg+xml
    /// image/tiff
    /// image/vnd.microsoft.icon
    /// image/webp
    pub const MULTIPART_FORM_DATA: &'static str = "multipart/form-data";

    /// pplication/vnd.openxmlformats-officedocument.spreadsheetml.sheet
    /// text
    pub const TEXT_CSS: &'static str = "text/css";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT_CSV: &'static str = "text/csv";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT_HTML: &'static str = "text/html";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT_JAVASCRIPT: &'static str = "text/javascript";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT_PLAIN: &'static str = "text/plain";


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT_XML: &'static str = "text/xml";

    /// text/calendar
    /// video
    pub const VIDEO_X_MSVIDEO: &'static str = "video/x-msvideo";

}


impl Default for ContentType {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__ContentType__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__ContentType__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ContentType {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__ContentType__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__ContentType__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__ContentType__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ContentType {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ContentType where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/ContentType";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__ContentType() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__HTTPStatusCode() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__HTTPStatusCode__init(msg: *mut HTTPStatusCode) -> bool;
    fn ros2web_interfaces__msg__HTTPStatusCode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HTTPStatusCode>, size: usize) -> bool;
    fn ros2web_interfaces__msg__HTTPStatusCode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HTTPStatusCode>);
    fn ros2web_interfaces__msg__HTTPStatusCode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HTTPStatusCode>, out_seq: *mut rosidl_runtime_rs::Sequence<HTTPStatusCode>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__HTTPStatusCode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// HTTP Successful

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTTPStatusCode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl HTTPStatusCode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_OK: u16 = 200;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_ACCEPTED: u16 = 202;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NON_AUTHORITATIVE_INFORMATION: u16 = 203;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NO_CONTENT: u16 = 204;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_RESET_CONTENT: u16 = 205;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PARTIAL_CONTENT: u16 = 206;

    /// HTTP REDIRECTION
    pub const HTTP_MULTIPLE_CHOICES: u16 = 300;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_MOVED_PERMANENTLY: u16 = 301;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_FOUND: u16 = 302;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_SEE_OTHER: u16 = 303;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NOT_MODIFIED: u16 = 304;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_USE_PROXY: u16 = 305;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_TEMPORARY_REDIRECT: u16 = 307;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PERMANENT_REDIRECT: u16 = 308;

    /// HTTP ERROR
    /// HTTP CLIENT ERROR
    pub const HTTP_BAD_REQUEST: u16 = 400;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_UNAUTHORIZED: u16 = 401;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PAYMENT_REQUIRED: u16 = 402;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_FORBIDDEN: u16 = 403;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NOT_FOUND: u16 = 404;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_METHOD_NOT_ALLOWED: u16 = 405;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NOT_ACCEPTABLE: u16 = 406;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PROXY_AUTHENTICATION_REQUIRED: u16 = 407;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_REQUEST_TIMEOUT: u16 = 408;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_CONFLICT: u16 = 409;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_GONE: u16 = 410;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_LENGTH_REQUIRED: u16 = 411;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PRECONDITION_FAILED: u16 = 412;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_REQUEST_ENTITY_TOO_LARGE: u16 = 413;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_REQUEST_URI_TOOLONG: u16 = 414;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_UNSUPPORTED_MEDIA_TYPE: u16 = 415;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_REQUEST_RANGE_NOT_SATISFIABLE: u16 = 416;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_EXPECTATION_FAILED: u16 = 417;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_MISDIRECTED_REQUEST: u16 = 421;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_UNPROCESSABLE_ENTITY: u16 = 422;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_FAILED_DEPENDENCY: u16 = 424;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_UPGRADE_REQUIRED: u16 = 426;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_PRECONDITION_REQUIRED: u16 = 428;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_TOO_MANY_REQUESTS: u16 = 429;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE: u16 = 431;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: u16 = 451;

    /// HTTP SERVER ERROR
    pub const HTTP_INTERNAL_SERVER_ERROR: u16 = 500;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NOT_IMPLEMENTED: u16 = 501;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_BAD_GATEWAY: u16 = 502;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_SERVICE_UNAVAILABLE: u16 = 503;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_GATEWAY_TIMEOUT: u16 = 504;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_VERSION_NOT_SUPPORTED: u16 = 505;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_VARIANT_ALSO_NEGOTIATES: u16 = 506;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_INSUFFICIENT_STORAGE: u16 = 507;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NOT_EXTENDED: u16 = 510;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HTTP_NETWORK_AUTHENTICATION_REQUIRED: u16 = 511;

}


impl Default for HTTPStatusCode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__HTTPStatusCode__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__HTTPStatusCode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HTTPStatusCode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__HTTPStatusCode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__HTTPStatusCode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__HTTPStatusCode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HTTPStatusCode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HTTPStatusCode where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/HTTPStatusCode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__HTTPStatusCode() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsgData() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__WSMsgData__init(msg: *mut WSMsgData) -> bool;
    fn ros2web_interfaces__msg__WSMsgData__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WSMsgData>, size: usize) -> bool;
    fn ros2web_interfaces__msg__WSMsgData__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WSMsgData>);
    fn ros2web_interfaces__msg__WSMsgData__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WSMsgData>, out_seq: *mut rosidl_runtime_rs::Sequence<WSMsgData>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__WSMsgData
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WSMsgData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub str: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bytes: rosidl_runtime_rs::Sequence<u8>,

}



impl Default for WSMsgData {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__WSMsgData__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__WSMsgData__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WSMsgData {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgData__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgData__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgData__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WSMsgData {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WSMsgData where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/WSMsgData";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsgData() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsgType() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__WSMsgType__init(msg: *mut WSMsgType) -> bool;
    fn ros2web_interfaces__msg__WSMsgType__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WSMsgType>, size: usize) -> bool;
    fn ros2web_interfaces__msg__WSMsgType__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WSMsgType>);
    fn ros2web_interfaces__msg__WSMsgType__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WSMsgType>, out_seq: *mut rosidl_runtime_rs::Sequence<WSMsgType>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__WSMsgType
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WSMsgType {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl WSMsgType {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CONTINUATION: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TEXT: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BINARY: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PING: u8 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PONG: u8 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CLOSE: u8 = 21;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CLOSING: u8 = 22;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CLOSED: u8 = 23;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ERROR: u8 = 31;

}


impl Default for WSMsgType {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__WSMsgType__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__WSMsgType__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WSMsgType {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgType__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgType__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsgType__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WSMsgType {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WSMsgType where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/WSMsgType";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsgType() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsg() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__msg__WSMsg__init(msg: *mut WSMsg) -> bool;
    fn ros2web_interfaces__msg__WSMsg__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WSMsg>, size: usize) -> bool;
    fn ros2web_interfaces__msg__WSMsg__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WSMsg>);
    fn ros2web_interfaces__msg__WSMsg__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WSMsg>, out_seq: *mut rosidl_runtime_rs::Sequence<WSMsg>) -> bool;
}

// Corresponds to ros2web_interfaces__msg__WSMsg
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WSMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub route: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: rosidl_runtime_rs::String,

    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::super::msg::rmw::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: rosidl_runtime_rs::String,

}



impl Default for WSMsg {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__msg__WSMsg__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__msg__WSMsg__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WSMsg {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsg__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsg__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__msg__WSMsg__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WSMsg {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WSMsg where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/msg/WSMsg";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__msg__WSMsg() }
  }
}


