#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__GetPoseCmd_Request() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__GetPoseCmd_Request__init(msg: *mut GetPoseCmd_Request) -> bool;
    fn roarm_moveit__srv__GetPoseCmd_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Request>, size: usize) -> bool;
    fn roarm_moveit__srv__GetPoseCmd_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Request>);
    fn roarm_moveit__srv__GetPoseCmd_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPoseCmd_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Request>) -> bool;
}

// Corresponds to roarm_moveit__srv__GetPoseCmd_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetPoseCmd_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetPoseCmd_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__GetPoseCmd_Request__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__GetPoseCmd_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPoseCmd_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPoseCmd_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPoseCmd_Request where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/GetPoseCmd_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__GetPoseCmd_Request() }
  }
}


#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__GetPoseCmd_Response() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__GetPoseCmd_Response__init(msg: *mut GetPoseCmd_Response) -> bool;
    fn roarm_moveit__srv__GetPoseCmd_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Response>, size: usize) -> bool;
    fn roarm_moveit__srv__GetPoseCmd_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Response>);
    fn roarm_moveit__srv__GetPoseCmd_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetPoseCmd_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetPoseCmd_Response>) -> bool;
}

// Corresponds to roarm_moveit__srv__GetPoseCmd_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__GetPoseCmd_Response__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__GetPoseCmd_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetPoseCmd_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__GetPoseCmd_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetPoseCmd_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetPoseCmd_Response where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/GetPoseCmd_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__GetPoseCmd_Response() }
  }
}


#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MovePointCmd_Request() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__MovePointCmd_Request__init(msg: *mut MovePointCmd_Request) -> bool;
    fn roarm_moveit__srv__MovePointCmd_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Request>, size: usize) -> bool;
    fn roarm_moveit__srv__MovePointCmd_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Request>);
    fn roarm_moveit__srv__MovePointCmd_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovePointCmd_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Request>) -> bool;
}

// Corresponds to roarm_moveit__srv__MovePointCmd_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__MovePointCmd_Request__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__MovePointCmd_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovePointCmd_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovePointCmd_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovePointCmd_Request where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/MovePointCmd_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MovePointCmd_Request() }
  }
}


#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MovePointCmd_Response() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__MovePointCmd_Response__init(msg: *mut MovePointCmd_Response) -> bool;
    fn roarm_moveit__srv__MovePointCmd_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Response>, size: usize) -> bool;
    fn roarm_moveit__srv__MovePointCmd_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Response>);
    fn roarm_moveit__srv__MovePointCmd_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovePointCmd_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovePointCmd_Response>) -> bool;
}

// Corresponds to roarm_moveit__srv__MovePointCmd_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovePointCmd_Response {
    /// 执行是否成功的标志
    pub success: bool,

    /// 执行结果的消息
    pub message: rosidl_runtime_rs::String,

}



impl Default for MovePointCmd_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__MovePointCmd_Response__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__MovePointCmd_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovePointCmd_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MovePointCmd_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovePointCmd_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovePointCmd_Response where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/MovePointCmd_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MovePointCmd_Response() }
  }
}


#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MoveCircleCmd_Request() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__MoveCircleCmd_Request__init(msg: *mut MoveCircleCmd_Request) -> bool;
    fn roarm_moveit__srv__MoveCircleCmd_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Request>, size: usize) -> bool;
    fn roarm_moveit__srv__MoveCircleCmd_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Request>);
    fn roarm_moveit__srv__MoveCircleCmd_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveCircleCmd_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Request>) -> bool;
}

// Corresponds to roarm_moveit__srv__MoveCircleCmd_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__MoveCircleCmd_Request__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__MoveCircleCmd_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveCircleCmd_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveCircleCmd_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveCircleCmd_Request where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/MoveCircleCmd_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MoveCircleCmd_Request() }
  }
}


#[link(name = "roarm_moveit__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MoveCircleCmd_Response() -> *const std::ffi::c_void;
}

#[link(name = "roarm_moveit__rosidl_generator_c")]
extern "C" {
    fn roarm_moveit__srv__MoveCircleCmd_Response__init(msg: *mut MoveCircleCmd_Response) -> bool;
    fn roarm_moveit__srv__MoveCircleCmd_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Response>, size: usize) -> bool;
    fn roarm_moveit__srv__MoveCircleCmd_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Response>);
    fn roarm_moveit__srv__MoveCircleCmd_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveCircleCmd_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveCircleCmd_Response>) -> bool;
}

// Corresponds to roarm_moveit__srv__MoveCircleCmd_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveCircleCmd_Response {
    /// 执行是否成功的标志
    pub success: bool,

    /// 执行结果的消息
    pub message: rosidl_runtime_rs::String,

}



impl Default for MoveCircleCmd_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !roarm_moveit__srv__MoveCircleCmd_Response__init(&mut msg as *mut _) {
        panic!("Call to roarm_moveit__srv__MoveCircleCmd_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveCircleCmd_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { roarm_moveit__srv__MoveCircleCmd_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveCircleCmd_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveCircleCmd_Response where Self: Sized {
  const TYPE_NAME: &'static str = "roarm_moveit/srv/MoveCircleCmd_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__roarm_moveit__srv__MoveCircleCmd_Response() }
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


