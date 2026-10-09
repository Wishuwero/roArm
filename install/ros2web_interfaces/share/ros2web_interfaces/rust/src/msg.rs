#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to ros2web_interfaces__msg__BodyPart

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BodyPart {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub content_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub headers: Vec<std::string::String>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,

}



impl Default for BodyPart {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BodyPart::default())
  }
}

impl rosidl_runtime_rs::Message for BodyPart {
  type RmwMsg = super::msg::rmw::BodyPart;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        filename: msg.filename.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        headers: msg.headers
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        data: msg.data.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        filename: msg.filename.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        headers: msg.headers
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        data: msg.data.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      filename: msg.filename.to_string(),
      content_type: msg.content_type.to_string(),
      headers: msg.headers
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      data: msg.data.into(),
    }
  }
}


// Corresponds to ros2web_interfaces__msg__ContentType
/// https://developer.mozilla.org/ja/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ContentType::default())
  }
}

impl rosidl_runtime_rs::Message for ContentType {
  type RmwMsg = super::msg::rmw::ContentType;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ros2web_interfaces__msg__HTTPStatusCode
/// HTTP Successful

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HTTPStatusCode::default())
  }
}

impl rosidl_runtime_rs::Message for HTTPStatusCode {
  type RmwMsg = super::msg::rmw::HTTPStatusCode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ros2web_interfaces__msg__WSMsgData

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WSMsgData {

    // This member is not documented.
    #[allow(missing_docs)]
    pub str: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub bytes: Vec<u8>,

}



impl Default for WSMsgData {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WSMsgData::default())
  }
}

impl rosidl_runtime_rs::Message for WSMsgData {
  type RmwMsg = super::msg::rmw::WSMsgData;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        str: msg.str.as_str().into(),
        bytes: msg.bytes.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        str: msg.str.as_str().into(),
        bytes: msg.bytes.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      str: msg.str.to_string(),
      bytes: msg.bytes.into(),
    }
  }
}


// Corresponds to ros2web_interfaces__msg__WSMsgType

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WSMsgType::default())
  }
}

impl rosidl_runtime_rs::Message for WSMsgType {
  type RmwMsg = super::msg::rmw::WSMsgType;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to ros2web_interfaces__msg__WSMsg

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WSMsg {

    // This member is not documented.
    #[allow(missing_docs)]
    pub route: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: std::string::String,

    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::msg::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: std::string::String,

}



impl Default for WSMsg {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WSMsg::default())
  }
}

impl rosidl_runtime_rs::Message for WSMsg {
  type RmwMsg = super::msg::rmw::WSMsg;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        route: msg.route.as_str().into(),
        ws_id: msg.ws_id.as_str().into(),
        type_: msg.type_,
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Owned(msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        route: msg.route.as_str().into(),
        ws_id: msg.ws_id.as_str().into(),
      type_: msg.type_,
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Borrowed(&msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      route: msg.route.to_string(),
      ws_id: msg.ws_id.to_string(),
      type_: msg.type_,
      data: super::msg::WSMsgData::from_rmw_message(msg.data),
      extra: msg.extra.to_string(),
    }
  }
}


