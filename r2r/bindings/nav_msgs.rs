  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct GridCells {

                              pub header: std_msgs::msg::Header,
pub cell_width: f32,
pub cell_height: f32,
pub cells: Vec<geometry_msgs::msg::Point>,

                          }

                          impl WrappedTypesupport for GridCells { 

            type CStruct = nav_msgs__msg__GridCells; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__GridCells() }
            }

            fn create_msg() -> *mut nav_msgs__msg__GridCells {

                unsafe { nav_msgs__msg__GridCells__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__msg__GridCells) -> () {

                unsafe { nav_msgs__msg__GridCells__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> GridCells {
  GridCells {
header: std_msgs::msg::Header::from_native(&msg.header),
cell_width: msg.cell_width,
cell_height: msg.cell_height,
// is_upper_bound_: false
// member.array_size_ : 0
cells : {
let mut temp = Vec::with_capacity(msg.cells.size);
let slice = unsafe { std::slice::from_raw_parts(msg.cells.data, msg.cells.size)};
for s in slice { temp.push(geometry_msgs::msg::Point::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.cell_width = self.cell_width;
msg.cell_height = self.cell_height;
unsafe { geometry_msgs__msg__Point__Sequence__fini(&mut msg.cells) };
unsafe { geometry_msgs__msg__Point__Sequence__init(&mut msg.cells, self.cells.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.cells.data, msg.cells.size)};
for (t,s) in slice.iter_mut().zip(&self.cells) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for GridCells {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<GridCells>::new();
                                  GridCells::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MapMetaData {

                              pub map_load_time: builtin_interfaces::msg::Time,
pub resolution: f32,
pub width: u32,
pub height: u32,
pub origin: geometry_msgs::msg::Pose,

                          }

                          impl WrappedTypesupport for MapMetaData { 

            type CStruct = nav_msgs__msg__MapMetaData; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__MapMetaData() }
            }

            fn create_msg() -> *mut nav_msgs__msg__MapMetaData {

                unsafe { nav_msgs__msg__MapMetaData__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__msg__MapMetaData) -> () {

                unsafe { nav_msgs__msg__MapMetaData__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MapMetaData {
  MapMetaData {
map_load_time: builtin_interfaces::msg::Time::from_native(&msg.map_load_time),
resolution: msg.resolution,
width: msg.width,
height: msg.height,
origin: geometry_msgs::msg::Pose::from_native(&msg.origin),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.map_load_time.copy_to_native(&mut msg.map_load_time);
msg.resolution = self.resolution;
msg.width = self.width;
msg.height = self.height;
self.origin.copy_to_native(&mut msg.origin);
}



        }


                          
                          impl Default for MapMetaData {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MapMetaData>::new();
                                  MapMetaData::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct OccupancyGrid {

                              pub header: std_msgs::msg::Header,
pub info: nav_msgs::msg::MapMetaData,
pub data: Vec<i8>,

                          }

                          impl WrappedTypesupport for OccupancyGrid { 

            type CStruct = nav_msgs__msg__OccupancyGrid; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__OccupancyGrid() }
            }

            fn create_msg() -> *mut nav_msgs__msg__OccupancyGrid {

                unsafe { nav_msgs__msg__OccupancyGrid__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__msg__OccupancyGrid) -> () {

                unsafe { nav_msgs__msg__OccupancyGrid__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> OccupancyGrid {
  OccupancyGrid {
header: std_msgs::msg::Header::from_native(&msg.header),
info: nav_msgs::msg::MapMetaData::from_native(&msg.info),
// is_upper_bound_: false
// member.array_size_ : 0
data: msg.data.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.info.copy_to_native(&mut msg.info);
msg.data.update(&self.data);
}



        }


                          
                          impl Default for OccupancyGrid {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<OccupancyGrid>::new();
                                  OccupancyGrid::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Odometry {

                              pub header: std_msgs::msg::Header,
pub child_frame_id: std::string::String,
pub pose: geometry_msgs::msg::PoseWithCovariance,
pub twist: geometry_msgs::msg::TwistWithCovariance,

                          }

                          impl WrappedTypesupport for Odometry { 

            type CStruct = nav_msgs__msg__Odometry; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Odometry() }
            }

            fn create_msg() -> *mut nav_msgs__msg__Odometry {

                unsafe { nav_msgs__msg__Odometry__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__msg__Odometry) -> () {

                unsafe { nav_msgs__msg__Odometry__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Odometry {
  Odometry {
header: std_msgs::msg::Header::from_native(&msg.header),
child_frame_id: msg.child_frame_id.to_str().to_owned(),
pose: geometry_msgs::msg::PoseWithCovariance::from_native(&msg.pose),
twist: geometry_msgs::msg::TwistWithCovariance::from_native(&msg.twist),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.child_frame_id.assign(&self.child_frame_id);
self.pose.copy_to_native(&mut msg.pose);
self.twist.copy_to_native(&mut msg.twist);
}



        }


                          
                          impl Default for Odometry {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Odometry>::new();
                                  Odometry::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Path {

                              pub header: std_msgs::msg::Header,
pub poses: Vec<geometry_msgs::msg::PoseStamped>,

                          }

                          impl WrappedTypesupport for Path { 

            type CStruct = nav_msgs__msg__Path; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__msg__Path() }
            }

            fn create_msg() -> *mut nav_msgs__msg__Path {

                unsafe { nav_msgs__msg__Path__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__msg__Path) -> () {

                unsafe { nav_msgs__msg__Path__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Path {
  Path {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
poses : {
let mut temp = Vec::with_capacity(msg.poses.size);
let slice = unsafe { std::slice::from_raw_parts(msg.poses.data, msg.poses.size)};
for s in slice { temp.push(geometry_msgs::msg::PoseStamped::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { geometry_msgs__msg__PoseStamped__Sequence__fini(&mut msg.poses) };
unsafe { geometry_msgs__msg__PoseStamped__Sequence__init(&mut msg.poses, self.poses.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.poses.data, msg.poses.size)};
for (t,s) in slice.iter_mut().zip(&self.poses) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Path {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Path>::new();
                                  Path::from_native(&msg_native)
                              }
                          }
             


                      }
  pub mod srv {
#[allow(non_snake_case)]
    pub mod GetMap {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetMap()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = nav_msgs__srv__GetMap_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__GetMap_Request() }
            }

            fn create_msg() -> *mut nav_msgs__srv__GetMap_Request {

                unsafe { nav_msgs__srv__GetMap_Request__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__GetMap_Request) -> () {

                unsafe { nav_msgs__srv__GetMap_Request__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> Request {
  Request {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub map: nav_msgs::msg::OccupancyGrid,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = nav_msgs__srv__GetMap_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__GetMap_Response() }
            }

            fn create_msg() -> *mut nav_msgs__srv__GetMap_Response {

                unsafe { nav_msgs__srv__GetMap_Response__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__GetMap_Response) -> () {

                unsafe { nav_msgs__srv__GetMap_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
map: nav_msgs::msg::OccupancyGrid::from_native(&msg.map),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.map.copy_to_native(&mut msg.map);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod GetPlan {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__GetPlan()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub start: geometry_msgs::msg::PoseStamped,
pub goal: geometry_msgs::msg::PoseStamped,
pub tolerance: f32,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = nav_msgs__srv__GetPlan_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__GetPlan_Request() }
            }

            fn create_msg() -> *mut nav_msgs__srv__GetPlan_Request {

                unsafe { nav_msgs__srv__GetPlan_Request__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__GetPlan_Request) -> () {

                unsafe { nav_msgs__srv__GetPlan_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
start: geometry_msgs::msg::PoseStamped::from_native(&msg.start),
goal: geometry_msgs::msg::PoseStamped::from_native(&msg.goal),
tolerance: msg.tolerance,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.start.copy_to_native(&mut msg.start);
self.goal.copy_to_native(&mut msg.goal);
msg.tolerance = self.tolerance;
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub plan: nav_msgs::msg::Path,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = nav_msgs__srv__GetPlan_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__GetPlan_Response() }
            }

            fn create_msg() -> *mut nav_msgs__srv__GetPlan_Response {

                unsafe { nav_msgs__srv__GetPlan_Response__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__GetPlan_Response) -> () {

                unsafe { nav_msgs__srv__GetPlan_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
plan: nav_msgs::msg::Path::from_native(&msg.plan),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.plan.copy_to_native(&mut msg.plan);
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod LoadMap {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__LoadMap()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub map_url: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = nav_msgs__srv__LoadMap_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Request() }
            }

            fn create_msg() -> *mut nav_msgs__srv__LoadMap_Request {

                unsafe { nav_msgs__srv__LoadMap_Request__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__LoadMap_Request) -> () {

                unsafe { nav_msgs__srv__LoadMap_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
map_url: msg.map_url.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.map_url.assign(&self.map_url);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub map: nav_msgs::msg::OccupancyGrid,
pub result: u8,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = nav_msgs__srv__LoadMap_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__LoadMap_Response() }
            }

            fn create_msg() -> *mut nav_msgs__srv__LoadMap_Response {

                unsafe { nav_msgs__srv__LoadMap_Response__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__LoadMap_Response) -> () {

                unsafe { nav_msgs__srv__LoadMap_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
map: nav_msgs::msg::OccupancyGrid::from_native(&msg.map),
result: msg.result,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.map.copy_to_native(&mut msg.map);
msg.result = self.result;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
#[allow(non_snake_case)]
    pub mod SetMap {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__nav_msgs__srv__SetMap()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub map: nav_msgs::msg::OccupancyGrid,
pub initial_pose: geometry_msgs::msg::PoseWithCovarianceStamped,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = nav_msgs__srv__SetMap_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__SetMap_Request() }
            }

            fn create_msg() -> *mut nav_msgs__srv__SetMap_Request {

                unsafe { nav_msgs__srv__SetMap_Request__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__SetMap_Request) -> () {

                unsafe { nav_msgs__srv__SetMap_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
map: nav_msgs::msg::OccupancyGrid::from_native(&msg.map),
initial_pose: geometry_msgs::msg::PoseWithCovarianceStamped::from_native(&msg.initial_pose),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.map.copy_to_native(&mut msg.map);
self.initial_pose.copy_to_native(&mut msg.initial_pose);
}



        }


                          
                          impl Default for Request {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Request>::new();
                                  Request::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Response {

                              pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = nav_msgs__srv__SetMap_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__nav_msgs__srv__SetMap_Response() }
            }

            fn create_msg() -> *mut nav_msgs__srv__SetMap_Response {

                unsafe { nav_msgs__srv__SetMap_Response__create() }

            }

            fn destroy_msg(msg: *mut nav_msgs__srv__SetMap_Response) -> () {

                unsafe { nav_msgs__srv__SetMap_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
}



        }


                          
                          impl Default for Response {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Response>::new();
                                  Response::from_native(&msg_native)
                              }
                          }
             


                        }
  }
