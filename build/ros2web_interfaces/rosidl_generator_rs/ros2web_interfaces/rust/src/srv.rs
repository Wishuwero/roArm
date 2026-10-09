#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to ros2web_interfaces__srv__HTTP_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTTP_Request {
    /// [<key>, <value>, <key>, <value>, ...]
    pub headers: Vec<std::string::String>,

    /// HTTP method,
    pub method: std::string::String,

    /// The URL including PATH INFO without the host or scheme.
    pub path: std::string::String,

    /// The query string in the URL
    pub query: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub srv_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub text: std::string::String,

    /// ContentType
    pub content_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charset: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub multipart: Vec<super::msg::BodyPart>,

}



impl Default for HTTP_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::HTTP_Request::default())
  }
}

impl rosidl_runtime_rs::Message for HTTP_Request {
  type RmwMsg = super::srv::rmw::HTTP_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        headers: msg.headers
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        method: msg.method.as_str().into(),
        path: msg.path.as_str().into(),
        query: msg.query.as_str().into(),
        srv_name: msg.srv_name.as_str().into(),
        route: msg.route.as_str().into(),
        body: msg.body.as_slice().into(),
        text: msg.text.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        charset: msg.charset.as_str().into(),
        multipart: msg.multipart
          .into_iter()
          .map(|elem| super::msg::BodyPart::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        headers: msg.headers
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
        method: msg.method.as_str().into(),
        path: msg.path.as_str().into(),
        query: msg.query.as_str().into(),
        srv_name: msg.srv_name.as_str().into(),
        route: msg.route.as_str().into(),
        body: msg.body.as_slice().into(),
        text: msg.text.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        charset: msg.charset.as_str().into(),
        multipart: msg.multipart
          .iter()
          .map(|elem| super::msg::BodyPart::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      headers: msg.headers
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
      method: msg.method.to_string(),
      path: msg.path.to_string(),
      query: msg.query.to_string(),
      srv_name: msg.srv_name.to_string(),
      route: msg.route.to_string(),
      body: msg.body.into(),
      text: msg.text.to_string(),
      content_type: msg.content_type.to_string(),
      charset: msg.charset.to_string(),
      multipart: msg.multipart
          .into_iter()
          .map(super::msg::BodyPart::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to ros2web_interfaces__srv__HTTP_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HTTP_Response {
    /// HTTPStatusCode
    pub status: u16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub reason: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub body: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub text: std::string::String,

    /// ContentType
    pub content_type: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub charset: std::string::String,

    /// int16 zlib_executor_size
    /// int16 zlib_executor
    pub file_path: std::string::String,

}



impl Default for HTTP_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::HTTP_Response::default())
  }
}

impl rosidl_runtime_rs::Message for HTTP_Response {
  type RmwMsg = super::srv::rmw::HTTP_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        reason: msg.reason.as_str().into(),
        body: msg.body.as_slice().into(),
        text: msg.text.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        charset: msg.charset.as_str().into(),
        file_path: msg.file_path.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        reason: msg.reason.as_str().into(),
        body: msg.body.as_slice().into(),
        text: msg.text.as_str().into(),
        content_type: msg.content_type.as_str().into(),
        charset: msg.charset.as_str().into(),
        file_path: msg.file_path.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      reason: msg.reason.to_string(),
      body: msg.body.into(),
      text: msg.text.to_string(),
      content_type: msg.content_type.to_string(),
      charset: msg.charset.to_string(),
      file_path: msg.file_path.to_string(),
    }
  }
}


// Corresponds to ros2web_interfaces__srv__WS_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WS_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub srv_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub route: std::string::String,

    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::msg::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: std::string::String,

}



impl Default for WS_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WS_Request::default())
  }
}

impl rosidl_runtime_rs::Message for WS_Request {
  type RmwMsg = super::srv::rmw::WS_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        srv_name: msg.srv_name.as_str().into(),
        route: msg.route.as_str().into(),
        type_: msg.type_,
        ws_id: msg.ws_id.as_str().into(),
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Owned(msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        srv_name: msg.srv_name.as_str().into(),
        route: msg.route.as_str().into(),
      type_: msg.type_,
        ws_id: msg.ws_id.as_str().into(),
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Borrowed(&msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      srv_name: msg.srv_name.to_string(),
      route: msg.route.to_string(),
      type_: msg.type_,
      ws_id: msg.ws_id.to_string(),
      data: super::msg::WSMsgData::from_rmw_message(msg.data),
      extra: msg.extra.to_string(),
    }
  }
}


// Corresponds to ros2web_interfaces__srv__WS_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WS_Response {
    /// WSMsgType
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ws_id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: super::msg::WSMsgData,


    // This member is not documented.
    #[allow(missing_docs)]
    pub extra: std::string::String,

}



impl Default for WS_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WS_Response::default())
  }
}

impl rosidl_runtime_rs::Message for WS_Response {
  type RmwMsg = super::srv::rmw::WS_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_,
        ws_id: msg.ws_id.as_str().into(),
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Owned(msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      type_: msg.type_,
        ws_id: msg.ws_id.as_str().into(),
        data: super::msg::WSMsgData::into_rmw_message(std::borrow::Cow::Borrowed(&msg.data)).into_owned(),
        extra: msg.extra.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_,
      ws_id: msg.ws_id.to_string(),
      data: super::msg::WSMsgData::from_rmw_message(msg.data),
      extra: msg.extra.to_string(),
    }
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


