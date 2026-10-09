#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__HTTP_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__srv__HTTP_Request__init(msg: *mut HTTP_Request) -> bool;
    fn ros2web_interfaces__srv__HTTP_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HTTP_Request>, size: usize) -> bool;
    fn ros2web_interfaces__srv__HTTP_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HTTP_Request>);
    fn ros2web_interfaces__srv__HTTP_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HTTP_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<HTTP_Request>) -> bool;
}

// Corresponds to ros2web_interfaces__srv__HTTP_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTTP_Request {
    /// [<key>, <value>, <key>, <value>, ...]
    pub headers: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// HTTP method,
    pub method: rosidl_runtime_rs::String,

    /// The URL including PATH INFO without the host or scheme.
    pub path: rosidl_runtime_rs::String,

    /// The query string in the URL
    pub query: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub srv_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub text: rosidl_runtime_rs::String,

    /// ContentType
    pub content_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charset: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multipart: rosidl_runtime_rs::Sequence<super::super::msg::rmw::BodyPart>,

}



impl Default for HTTP_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__srv__HTTP_Request__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__srv__HTTP_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HTTP_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HTTP_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HTTP_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/srv/HTTP_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__HTTP_Request() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__HTTP_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__srv__HTTP_Response__init(msg: *mut HTTP_Response) -> bool;
    fn ros2web_interfaces__srv__HTTP_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HTTP_Response>, size: usize) -> bool;
    fn ros2web_interfaces__srv__HTTP_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HTTP_Response>);
    fn ros2web_interfaces__srv__HTTP_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HTTP_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<HTTP_Response>) -> bool;
}

// Corresponds to ros2web_interfaces__srv__HTTP_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTTP_Response {
    /// HTTPStatusCode
    pub status: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reason: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub text: rosidl_runtime_rs::String,

    /// ContentType
    pub content_type: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charset: rosidl_runtime_rs::String,

    /// int16 zlib_executor_size
    /// int16 zlib_executor
    pub file_path: rosidl_runtime_rs::String,

}



impl Default for HTTP_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__srv__HTTP_Response__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__srv__HTTP_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HTTP_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__HTTP_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HTTP_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HTTP_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/srv/HTTP_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__HTTP_Response() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__WS_Request() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__srv__WS_Request__init(msg: *mut WS_Request) -> bool;
    fn ros2web_interfaces__srv__WS_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WS_Request>, size: usize) -> bool;
    fn ros2web_interfaces__srv__WS_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WS_Request>);
    fn ros2web_interfaces__srv__WS_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WS_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<WS_Request>) -> bool;
}

// Corresponds to ros2web_interfaces__srv__WS_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WS_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub srv_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route: rosidl_runtime_rs::String,

    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::super::msg::rmw::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: rosidl_runtime_rs::String,

}



impl Default for WS_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__srv__WS_Request__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__srv__WS_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WS_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WS_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WS_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/srv/WS_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__WS_Request() }
  }
}


#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__WS_Response() -> *const std::ffi::c_void;
}

#[link(name = "ros2web_interfaces__rosidl_generator_c")]
extern "C" {
    fn ros2web_interfaces__srv__WS_Response__init(msg: *mut WS_Response) -> bool;
    fn ros2web_interfaces__srv__WS_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WS_Response>, size: usize) -> bool;
    fn ros2web_interfaces__srv__WS_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WS_Response>);
    fn ros2web_interfaces__srv__WS_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WS_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<WS_Response>) -> bool;
}

// Corresponds to ros2web_interfaces__srv__WS_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WS_Response {
    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::super::msg::rmw::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: rosidl_runtime_rs::String,

}



impl Default for WS_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ros2web_interfaces__srv__WS_Response__init(&mut msg as *mut _) {
        panic!("Call to ros2web_interfaces__srv__WS_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WS_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ros2web_interfaces__srv__WS_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WS_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WS_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ros2web_interfaces/srv/WS_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ros2web_interfaces__srv__WS_Response() }
  }
}






#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros2web_interfaces__srv__HTTP() -> *const std::ffi::c_void;
}

// Corresponds to ros2web_interfaces__srv__HTTP
#[allow(missing_docs, non_camel_case_types)]
pub struct HTTP;

impl rosidl_runtime_rs::Service for HTTP {
    type Request = HTTP_Request;
    type Response = HTTP_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros2web_interfaces__srv__HTTP() }
    }
}




#[link(name = "ros2web_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ros2web_interfaces__srv__WS() -> *const std::ffi::c_void;
}

// Corresponds to ros2web_interfaces__srv__WS
#[allow(missing_docs, non_camel_case_types)]
pub struct WS;

impl rosidl_runtime_rs::Service for WS {
    type Request = WS_Request;
    type Response = WS_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ros2web_interfaces__srv__WS() }
    }
}


