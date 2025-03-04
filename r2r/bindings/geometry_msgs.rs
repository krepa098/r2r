  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Accel {

                              pub linear: geometry_msgs::msg::Vector3,
pub angular: geometry_msgs::msg::Vector3,

                          }

                          impl WrappedTypesupport for Accel { 

            type CStruct = geometry_msgs__msg__Accel; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Accel() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Accel {

                unsafe { geometry_msgs__msg__Accel__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Accel) -> () {

                unsafe { geometry_msgs__msg__Accel__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Accel {
  Accel {
linear: geometry_msgs::msg::Vector3::from_native(&msg.linear),
angular: geometry_msgs::msg::Vector3::from_native(&msg.angular),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.linear.copy_to_native(&mut msg.linear);
self.angular.copy_to_native(&mut msg.angular);
}



        }


                          
                          impl Default for Accel {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Accel>::new();
                                  Accel::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AccelStamped {

                              pub header: std_msgs::msg::Header,
pub accel: geometry_msgs::msg::Accel,

                          }

                          impl WrappedTypesupport for AccelStamped { 

            type CStruct = geometry_msgs__msg__AccelStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__AccelStamped {

                unsafe { geometry_msgs__msg__AccelStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__AccelStamped) -> () {

                unsafe { geometry_msgs__msg__AccelStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AccelStamped {
  AccelStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
accel: geometry_msgs::msg::Accel::from_native(&msg.accel),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.accel.copy_to_native(&mut msg.accel);
}



        }


                          
                          impl Default for AccelStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AccelStamped>::new();
                                  AccelStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AccelWithCovariance {

                              pub accel: geometry_msgs::msg::Accel,
pub covariance: Vec<f64>,

                          }

                          impl WrappedTypesupport for AccelWithCovariance { 

            type CStruct = geometry_msgs__msg__AccelWithCovariance; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelWithCovariance() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__AccelWithCovariance {

                unsafe { geometry_msgs__msg__AccelWithCovariance__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__AccelWithCovariance) -> () {

                unsafe { geometry_msgs__msg__AccelWithCovariance__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AccelWithCovariance {
  AccelWithCovariance {
accel: geometry_msgs::msg::Accel::from_native(&msg.accel),
// is_upper_bound_: false
// member.array_size_ : 36
covariance: msg.covariance.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.accel.copy_to_native(&mut msg.accel);
assert_eq!(self.covariance.len(), 36, "Field {} is fixed size of {}!", "covariance", 36);
msg.covariance.copy_from_slice(&self.covariance[..36]);
}



        }


                          
                          impl Default for AccelWithCovariance {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AccelWithCovariance>::new();
                                  AccelWithCovariance::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct AccelWithCovarianceStamped {

                              pub header: std_msgs::msg::Header,
pub accel: geometry_msgs::msg::AccelWithCovariance,

                          }

                          impl WrappedTypesupport for AccelWithCovarianceStamped { 

            type CStruct = geometry_msgs__msg__AccelWithCovarianceStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__AccelWithCovarianceStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__AccelWithCovarianceStamped {

                unsafe { geometry_msgs__msg__AccelWithCovarianceStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__AccelWithCovarianceStamped) -> () {

                unsafe { geometry_msgs__msg__AccelWithCovarianceStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> AccelWithCovarianceStamped {
  AccelWithCovarianceStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
accel: geometry_msgs::msg::AccelWithCovariance::from_native(&msg.accel),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.accel.copy_to_native(&mut msg.accel);
}



        }


                          
                          impl Default for AccelWithCovarianceStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<AccelWithCovarianceStamped>::new();
                                  AccelWithCovarianceStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Inertia {

                              pub m: f64,
pub com: geometry_msgs::msg::Vector3,
pub ixx: f64,
pub ixy: f64,
pub ixz: f64,
pub iyy: f64,
pub iyz: f64,
pub izz: f64,

                          }

                          impl WrappedTypesupport for Inertia { 

            type CStruct = geometry_msgs__msg__Inertia; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Inertia() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Inertia {

                unsafe { geometry_msgs__msg__Inertia__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Inertia) -> () {

                unsafe { geometry_msgs__msg__Inertia__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Inertia {
  Inertia {
m: msg.m,
com: geometry_msgs::msg::Vector3::from_native(&msg.com),
ixx: msg.ixx,
ixy: msg.ixy,
ixz: msg.ixz,
iyy: msg.iyy,
iyz: msg.iyz,
izz: msg.izz,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.m = self.m;
self.com.copy_to_native(&mut msg.com);
msg.ixx = self.ixx;
msg.ixy = self.ixy;
msg.ixz = self.ixz;
msg.iyy = self.iyy;
msg.iyz = self.iyz;
msg.izz = self.izz;
}



        }


                          
                          impl Default for Inertia {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Inertia>::new();
                                  Inertia::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct InertiaStamped {

                              pub header: std_msgs::msg::Header,
pub inertia: geometry_msgs::msg::Inertia,

                          }

                          impl WrappedTypesupport for InertiaStamped { 

            type CStruct = geometry_msgs__msg__InertiaStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__InertiaStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__InertiaStamped {

                unsafe { geometry_msgs__msg__InertiaStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__InertiaStamped) -> () {

                unsafe { geometry_msgs__msg__InertiaStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> InertiaStamped {
  InertiaStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
inertia: geometry_msgs::msg::Inertia::from_native(&msg.inertia),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.inertia.copy_to_native(&mut msg.inertia);
}



        }


                          
                          impl Default for InertiaStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<InertiaStamped>::new();
                                  InertiaStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Point {

                              pub x: f64,
pub y: f64,
pub z: f64,

                          }

                          impl WrappedTypesupport for Point { 

            type CStruct = geometry_msgs__msg__Point; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Point {

                unsafe { geometry_msgs__msg__Point__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Point) -> () {

                unsafe { geometry_msgs__msg__Point__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Point {
  Point {
x: msg.x,
y: msg.y,
z: msg.z,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.z = self.z;
}



        }


                          
                          impl Default for Point {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Point>::new();
                                  Point::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Point32 {

                              pub x: f32,
pub y: f32,
pub z: f32,

                          }

                          impl WrappedTypesupport for Point32 { 

            type CStruct = geometry_msgs__msg__Point32; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Point32() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Point32 {

                unsafe { geometry_msgs__msg__Point32__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Point32) -> () {

                unsafe { geometry_msgs__msg__Point32__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Point32 {
  Point32 {
x: msg.x,
y: msg.y,
z: msg.z,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.z = self.z;
}



        }


                          
                          impl Default for Point32 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Point32>::new();
                                  Point32::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PointStamped {

                              pub header: std_msgs::msg::Header,
pub point: geometry_msgs::msg::Point,

                          }

                          impl WrappedTypesupport for PointStamped { 

            type CStruct = geometry_msgs__msg__PointStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PointStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PointStamped {

                unsafe { geometry_msgs__msg__PointStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PointStamped) -> () {

                unsafe { geometry_msgs__msg__PointStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PointStamped {
  PointStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
point: geometry_msgs::msg::Point::from_native(&msg.point),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.point.copy_to_native(&mut msg.point);
}



        }


                          
                          impl Default for PointStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PointStamped>::new();
                                  PointStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Polygon {

                              pub points: Vec<geometry_msgs::msg::Point32>,

                          }

                          impl WrappedTypesupport for Polygon { 

            type CStruct = geometry_msgs__msg__Polygon; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Polygon() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Polygon {

                unsafe { geometry_msgs__msg__Polygon__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Polygon) -> () {

                unsafe { geometry_msgs__msg__Polygon__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Polygon {
  Polygon {
// is_upper_bound_: false
// member.array_size_ : 0
points : {
let mut temp = Vec::with_capacity(msg.points.size);
let slice = unsafe { std::slice::from_raw_parts(msg.points.data, msg.points.size)};
for s in slice { temp.push(geometry_msgs::msg::Point32::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { geometry_msgs__msg__Point32__Sequence__fini(&mut msg.points) };
unsafe { geometry_msgs__msg__Point32__Sequence__init(&mut msg.points, self.points.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.points.data, msg.points.size)};
for (t,s) in slice.iter_mut().zip(&self.points) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Polygon {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Polygon>::new();
                                  Polygon::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PolygonStamped {

                              pub header: std_msgs::msg::Header,
pub polygon: geometry_msgs::msg::Polygon,

                          }

                          impl WrappedTypesupport for PolygonStamped { 

            type CStruct = geometry_msgs__msg__PolygonStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PolygonStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PolygonStamped {

                unsafe { geometry_msgs__msg__PolygonStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PolygonStamped) -> () {

                unsafe { geometry_msgs__msg__PolygonStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PolygonStamped {
  PolygonStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
polygon: geometry_msgs::msg::Polygon::from_native(&msg.polygon),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.polygon.copy_to_native(&mut msg.polygon);
}



        }


                          
                          impl Default for PolygonStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PolygonStamped>::new();
                                  PolygonStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Pose {

                              pub position: geometry_msgs::msg::Point,
pub orientation: geometry_msgs::msg::Quaternion,

                          }

                          impl WrappedTypesupport for Pose { 

            type CStruct = geometry_msgs__msg__Pose; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Pose() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Pose {

                unsafe { geometry_msgs__msg__Pose__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Pose) -> () {

                unsafe { geometry_msgs__msg__Pose__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Pose {
  Pose {
position: geometry_msgs::msg::Point::from_native(&msg.position),
orientation: geometry_msgs::msg::Quaternion::from_native(&msg.orientation),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.position.copy_to_native(&mut msg.position);
self.orientation.copy_to_native(&mut msg.orientation);
}



        }


                          
                          impl Default for Pose {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Pose>::new();
                                  Pose::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Pose2D {

                              pub x: f64,
pub y: f64,
pub theta: f64,

                          }

                          impl WrappedTypesupport for Pose2D { 

            type CStruct = geometry_msgs__msg__Pose2D; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Pose2D() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Pose2D {

                unsafe { geometry_msgs__msg__Pose2D__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Pose2D) -> () {

                unsafe { geometry_msgs__msg__Pose2D__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Pose2D {
  Pose2D {
x: msg.x,
y: msg.y,
theta: msg.theta,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.theta = self.theta;
}



        }


                          
                          impl Default for Pose2D {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Pose2D>::new();
                                  Pose2D::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PoseArray {

                              pub header: std_msgs::msg::Header,
pub poses: Vec<geometry_msgs::msg::Pose>,

                          }

                          impl WrappedTypesupport for PoseArray { 

            type CStruct = geometry_msgs__msg__PoseArray; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseArray() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PoseArray {

                unsafe { geometry_msgs__msg__PoseArray__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PoseArray) -> () {

                unsafe { geometry_msgs__msg__PoseArray__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PoseArray {
  PoseArray {
header: std_msgs::msg::Header::from_native(&msg.header),
// is_upper_bound_: false
// member.array_size_ : 0
poses : {
let mut temp = Vec::with_capacity(msg.poses.size);
let slice = unsafe { std::slice::from_raw_parts(msg.poses.data, msg.poses.size)};
for s in slice { temp.push(geometry_msgs::msg::Pose::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
unsafe { geometry_msgs__msg__Pose__Sequence__fini(&mut msg.poses) };
unsafe { geometry_msgs__msg__Pose__Sequence__init(&mut msg.poses, self.poses.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.poses.data, msg.poses.size)};
for (t,s) in slice.iter_mut().zip(&self.poses) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for PoseArray {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PoseArray>::new();
                                  PoseArray::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PoseStamped {

                              pub header: std_msgs::msg::Header,
pub pose: geometry_msgs::msg::Pose,

                          }

                          impl WrappedTypesupport for PoseStamped { 

            type CStruct = geometry_msgs__msg__PoseStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PoseStamped {

                unsafe { geometry_msgs__msg__PoseStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PoseStamped) -> () {

                unsafe { geometry_msgs__msg__PoseStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PoseStamped {
  PoseStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
pose: geometry_msgs::msg::Pose::from_native(&msg.pose),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.pose.copy_to_native(&mut msg.pose);
}



        }


                          
                          impl Default for PoseStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PoseStamped>::new();
                                  PoseStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PoseWithCovariance {

                              pub pose: geometry_msgs::msg::Pose,
pub covariance: Vec<f64>,

                          }

                          impl WrappedTypesupport for PoseWithCovariance { 

            type CStruct = geometry_msgs__msg__PoseWithCovariance; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseWithCovariance() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PoseWithCovariance {

                unsafe { geometry_msgs__msg__PoseWithCovariance__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PoseWithCovariance) -> () {

                unsafe { geometry_msgs__msg__PoseWithCovariance__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PoseWithCovariance {
  PoseWithCovariance {
pose: geometry_msgs::msg::Pose::from_native(&msg.pose),
// is_upper_bound_: false
// member.array_size_ : 36
covariance: msg.covariance.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.pose.copy_to_native(&mut msg.pose);
assert_eq!(self.covariance.len(), 36, "Field {} is fixed size of {}!", "covariance", 36);
msg.covariance.copy_from_slice(&self.covariance[..36]);
}



        }


                          
                          impl Default for PoseWithCovariance {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PoseWithCovariance>::new();
                                  PoseWithCovariance::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct PoseWithCovarianceStamped {

                              pub header: std_msgs::msg::Header,
pub pose: geometry_msgs::msg::PoseWithCovariance,

                          }

                          impl WrappedTypesupport for PoseWithCovarianceStamped { 

            type CStruct = geometry_msgs__msg__PoseWithCovarianceStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__PoseWithCovarianceStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__PoseWithCovarianceStamped {

                unsafe { geometry_msgs__msg__PoseWithCovarianceStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__PoseWithCovarianceStamped) -> () {

                unsafe { geometry_msgs__msg__PoseWithCovarianceStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> PoseWithCovarianceStamped {
  PoseWithCovarianceStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
pose: geometry_msgs::msg::PoseWithCovariance::from_native(&msg.pose),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.pose.copy_to_native(&mut msg.pose);
}



        }


                          
                          impl Default for PoseWithCovarianceStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<PoseWithCovarianceStamped>::new();
                                  PoseWithCovarianceStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Quaternion {

                              pub x: f64,
pub y: f64,
pub z: f64,
pub w: f64,

                          }

                          impl WrappedTypesupport for Quaternion { 

            type CStruct = geometry_msgs__msg__Quaternion; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Quaternion() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Quaternion {

                unsafe { geometry_msgs__msg__Quaternion__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Quaternion) -> () {

                unsafe { geometry_msgs__msg__Quaternion__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Quaternion {
  Quaternion {
x: msg.x,
y: msg.y,
z: msg.z,
w: msg.w,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.z = self.z;
msg.w = self.w;
}



        }


                          
                          impl Default for Quaternion {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Quaternion>::new();
                                  Quaternion::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct QuaternionStamped {

                              pub header: std_msgs::msg::Header,
pub quaternion: geometry_msgs::msg::Quaternion,

                          }

                          impl WrappedTypesupport for QuaternionStamped { 

            type CStruct = geometry_msgs__msg__QuaternionStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__QuaternionStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__QuaternionStamped {

                unsafe { geometry_msgs__msg__QuaternionStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__QuaternionStamped) -> () {

                unsafe { geometry_msgs__msg__QuaternionStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> QuaternionStamped {
  QuaternionStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
quaternion: geometry_msgs::msg::Quaternion::from_native(&msg.quaternion),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.quaternion.copy_to_native(&mut msg.quaternion);
}



        }


                          
                          impl Default for QuaternionStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<QuaternionStamped>::new();
                                  QuaternionStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Transform {

                              pub translation: geometry_msgs::msg::Vector3,
pub rotation: geometry_msgs::msg::Quaternion,

                          }

                          impl WrappedTypesupport for Transform { 

            type CStruct = geometry_msgs__msg__Transform; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Transform() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Transform {

                unsafe { geometry_msgs__msg__Transform__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Transform) -> () {

                unsafe { geometry_msgs__msg__Transform__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Transform {
  Transform {
translation: geometry_msgs::msg::Vector3::from_native(&msg.translation),
rotation: geometry_msgs::msg::Quaternion::from_native(&msg.rotation),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.translation.copy_to_native(&mut msg.translation);
self.rotation.copy_to_native(&mut msg.rotation);
}



        }


                          
                          impl Default for Transform {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Transform>::new();
                                  Transform::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TransformStamped {

                              pub header: std_msgs::msg::Header,
pub child_frame_id: std::string::String,
pub transform: geometry_msgs::msg::Transform,

                          }

                          impl WrappedTypesupport for TransformStamped { 

            type CStruct = geometry_msgs__msg__TransformStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TransformStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__TransformStamped {

                unsafe { geometry_msgs__msg__TransformStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__TransformStamped) -> () {

                unsafe { geometry_msgs__msg__TransformStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TransformStamped {
  TransformStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
child_frame_id: msg.child_frame_id.to_str().to_owned(),
transform: geometry_msgs::msg::Transform::from_native(&msg.transform),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
msg.child_frame_id.assign(&self.child_frame_id);
self.transform.copy_to_native(&mut msg.transform);
}



        }


                          
                          impl Default for TransformStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TransformStamped>::new();
                                  TransformStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Twist {

                              pub linear: geometry_msgs::msg::Vector3,
pub angular: geometry_msgs::msg::Vector3,

                          }

                          impl WrappedTypesupport for Twist { 

            type CStruct = geometry_msgs__msg__Twist; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Twist() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Twist {

                unsafe { geometry_msgs__msg__Twist__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Twist) -> () {

                unsafe { geometry_msgs__msg__Twist__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Twist {
  Twist {
linear: geometry_msgs::msg::Vector3::from_native(&msg.linear),
angular: geometry_msgs::msg::Vector3::from_native(&msg.angular),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.linear.copy_to_native(&mut msg.linear);
self.angular.copy_to_native(&mut msg.angular);
}



        }


                          
                          impl Default for Twist {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Twist>::new();
                                  Twist::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TwistStamped {

                              pub header: std_msgs::msg::Header,
pub twist: geometry_msgs::msg::Twist,

                          }

                          impl WrappedTypesupport for TwistStamped { 

            type CStruct = geometry_msgs__msg__TwistStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TwistStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__TwistStamped {

                unsafe { geometry_msgs__msg__TwistStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__TwistStamped) -> () {

                unsafe { geometry_msgs__msg__TwistStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TwistStamped {
  TwistStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
twist: geometry_msgs::msg::Twist::from_native(&msg.twist),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.twist.copy_to_native(&mut msg.twist);
}



        }


                          
                          impl Default for TwistStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TwistStamped>::new();
                                  TwistStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TwistWithCovariance {

                              pub twist: geometry_msgs::msg::Twist,
pub covariance: Vec<f64>,

                          }

                          impl WrappedTypesupport for TwistWithCovariance { 

            type CStruct = geometry_msgs__msg__TwistWithCovariance; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TwistWithCovariance() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__TwistWithCovariance {

                unsafe { geometry_msgs__msg__TwistWithCovariance__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__TwistWithCovariance) -> () {

                unsafe { geometry_msgs__msg__TwistWithCovariance__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TwistWithCovariance {
  TwistWithCovariance {
twist: geometry_msgs::msg::Twist::from_native(&msg.twist),
// is_upper_bound_: false
// member.array_size_ : 36
covariance: msg.covariance.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.twist.copy_to_native(&mut msg.twist);
assert_eq!(self.covariance.len(), 36, "Field {} is fixed size of {}!", "covariance", 36);
msg.covariance.copy_from_slice(&self.covariance[..36]);
}



        }


                          
                          impl Default for TwistWithCovariance {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TwistWithCovariance>::new();
                                  TwistWithCovariance::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct TwistWithCovarianceStamped {

                              pub header: std_msgs::msg::Header,
pub twist: geometry_msgs::msg::TwistWithCovariance,

                          }

                          impl WrappedTypesupport for TwistWithCovarianceStamped { 

            type CStruct = geometry_msgs__msg__TwistWithCovarianceStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__TwistWithCovarianceStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__TwistWithCovarianceStamped {

                unsafe { geometry_msgs__msg__TwistWithCovarianceStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__TwistWithCovarianceStamped) -> () {

                unsafe { geometry_msgs__msg__TwistWithCovarianceStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> TwistWithCovarianceStamped {
  TwistWithCovarianceStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
twist: geometry_msgs::msg::TwistWithCovariance::from_native(&msg.twist),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.twist.copy_to_native(&mut msg.twist);
}



        }


                          
                          impl Default for TwistWithCovarianceStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<TwistWithCovarianceStamped>::new();
                                  TwistWithCovarianceStamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Vector3 {

                              pub x: f64,
pub y: f64,
pub z: f64,

                          }

                          impl WrappedTypesupport for Vector3 { 

            type CStruct = geometry_msgs__msg__Vector3; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Vector3 {

                unsafe { geometry_msgs__msg__Vector3__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Vector3) -> () {

                unsafe { geometry_msgs__msg__Vector3__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Vector3 {
  Vector3 {
x: msg.x,
y: msg.y,
z: msg.z,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.x = self.x;
msg.y = self.y;
msg.z = self.z;
}



        }


                          
                          impl Default for Vector3 {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Vector3>::new();
                                  Vector3::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Vector3Stamped {

                              pub header: std_msgs::msg::Header,
pub vector: geometry_msgs::msg::Vector3,

                          }

                          impl WrappedTypesupport for Vector3Stamped { 

            type CStruct = geometry_msgs__msg__Vector3Stamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3Stamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Vector3Stamped {

                unsafe { geometry_msgs__msg__Vector3Stamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Vector3Stamped) -> () {

                unsafe { geometry_msgs__msg__Vector3Stamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Vector3Stamped {
  Vector3Stamped {
header: std_msgs::msg::Header::from_native(&msg.header),
vector: geometry_msgs::msg::Vector3::from_native(&msg.vector),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.vector.copy_to_native(&mut msg.vector);
}



        }


                          
                          impl Default for Vector3Stamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Vector3Stamped>::new();
                                  Vector3Stamped::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Wrench {

                              pub force: geometry_msgs::msg::Vector3,
pub torque: geometry_msgs::msg::Vector3,

                          }

                          impl WrappedTypesupport for Wrench { 

            type CStruct = geometry_msgs__msg__Wrench; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Wrench() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__Wrench {

                unsafe { geometry_msgs__msg__Wrench__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__Wrench) -> () {

                unsafe { geometry_msgs__msg__Wrench__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Wrench {
  Wrench {
force: geometry_msgs::msg::Vector3::from_native(&msg.force),
torque: geometry_msgs::msg::Vector3::from_native(&msg.torque),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.force.copy_to_native(&mut msg.force);
self.torque.copy_to_native(&mut msg.torque);
}



        }


                          
                          impl Default for Wrench {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Wrench>::new();
                                  Wrench::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct WrenchStamped {

                              pub header: std_msgs::msg::Header,
pub wrench: geometry_msgs::msg::Wrench,

                          }

                          impl WrappedTypesupport for WrenchStamped { 

            type CStruct = geometry_msgs__msg__WrenchStamped; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__WrenchStamped() }
            }

            fn create_msg() -> *mut geometry_msgs__msg__WrenchStamped {

                unsafe { geometry_msgs__msg__WrenchStamped__create() }

            }

            fn destroy_msg(msg: *mut geometry_msgs__msg__WrenchStamped) -> () {

                unsafe { geometry_msgs__msg__WrenchStamped__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> WrenchStamped {
  WrenchStamped {
header: std_msgs::msg::Header::from_native(&msg.header),
wrench: geometry_msgs::msg::Wrench::from_native(&msg.wrench),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.wrench.copy_to_native(&mut msg.wrench);
}



        }


                          
                          impl Default for WrenchStamped {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<WrenchStamped>::new();
                                  WrenchStamped::from_native(&msg_native)
                              }
                          }
             


                      }
