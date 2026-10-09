#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to roarm_moveit__srv__GetPoseCmd_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPoseCmd_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPoseCmd_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPoseCmd_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetPoseCmd_Request {
  type RmwMsg = super::srv::rmw::GetPoseCmd_Request;

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


// Corresponds to roarm_moveit__srv__GetPoseCmd_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPoseCmd_Response {
    /// 目标位置的x坐标
    pub x: f64,

    /// 目标位置的y坐标
    pub y: f64,

    /// 目标位置的z坐标
    pub z: f64,

}



impl Default for GetPoseCmd_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetPoseCmd_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetPoseCmd_Response {
  type RmwMsg = super::srv::rmw::GetPoseCmd_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
    }
  }
}


// Corresponds to roarm_moveit__srv__MovePointCmd_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovePointCmd_Request {
    /// 目标位置的x坐标
    pub x: f64,

    /// 目标位置的y坐标
    pub y: f64,

    /// 目标位置的z坐标
    pub z: f64,

}



impl Default for MovePointCmd_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovePointCmd_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MovePointCmd_Request {
  type RmwMsg = super::srv::rmw::MovePointCmd_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
    }
  }
}


// Corresponds to roarm_moveit__srv__MovePointCmd_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovePointCmd_Response {
    /// 执行是否成功的标志
    pub success: bool,

    /// 执行结果的消息
    pub message: std::string::String,

}



impl Default for MovePointCmd_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MovePointCmd_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MovePointCmd_Response {
  type RmwMsg = super::srv::rmw::MovePointCmd_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to roarm_moveit__srv__MoveCircleCmd_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveCircleCmd_Request {
    /// 圆心的x坐标
    pub x: f64,

    /// 圆心的y坐标
    pub y: f64,

    /// 绘制圆的高度
    pub z: f64,

    /// 圆的半径
    pub radius: f64,

}



impl Default for MoveCircleCmd_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveCircleCmd_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveCircleCmd_Request {
  type RmwMsg = super::srv::rmw::MoveCircleCmd_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        radius: msg.radius,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      radius: msg.radius,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      radius: msg.radius,
    }
  }
}


// Corresponds to roarm_moveit__srv__MoveCircleCmd_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveCircleCmd_Response {
    /// 执行是否成功的标志
    pub success: bool,

    /// 执行结果的消息
    pub message: std::string::String,

}



impl Default for MoveCircleCmd_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveCircleCmd_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveCircleCmd_Response {
  type RmwMsg = super::srv::rmw::MoveCircleCmd_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}






#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__GetPoseCmd() -> *const std::ffi::c_void;
}

// Corresponds to roarm_moveit__srv__GetPoseCmd
#[allow(missing_docs, non_camel_case_types)]
pub struct GetPoseCmd;

impl rosidl_runtime_rs::Service for GetPoseCmd {
    type Request = GetPoseCmd_Request;
    type Response = GetPoseCmd_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__GetPoseCmd() }
    }
}




#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__MovePointCmd() -> *const std::ffi::c_void;
}

// Corresponds to roarm_moveit__srv__MovePointCmd
#[allow(missing_docs, non_camel_case_types)]
pub struct MovePointCmd;

impl rosidl_runtime_rs::Service for MovePointCmd {
    type Request = MovePointCmd_Request;
    type Response = MovePointCmd_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__MovePointCmd() }
    }
}




#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__MoveCircleCmd() -> *const std::ffi::c_void;
}

// Corresponds to roarm_moveit__srv__MoveCircleCmd
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveCircleCmd;

impl rosidl_runtime_rs::Service for MoveCircleCmd {
    type Request = MoveCircleCmd_Request;
    type Response = MoveCircleCmd_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__roarm_moveit__srv__MoveCircleCmd() }
    }
}


