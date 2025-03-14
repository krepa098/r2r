  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Mesh {

                              pub triangles: Vec<shape_msgs::msg::MeshTriangle>,
pub vertices: Vec<geometry_msgs::msg::Point>,

                          }

                          impl WrappedTypesupport for Mesh { 

            type CStruct = shape_msgs__msg__Mesh; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__Mesh() }
            }

            fn create_msg() -> *mut shape_msgs__msg__Mesh {

                unsafe { shape_msgs__msg__Mesh__create() }

            }

            fn destroy_msg(msg: *mut shape_msgs__msg__Mesh) -> () {

                unsafe { shape_msgs__msg__Mesh__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Mesh {
  Mesh {
// is_upper_bound_: false
// member.array_size_ : 0
triangles : {
let mut temp = Vec::with_capacity(msg.triangles.size);
let slice = unsafe { std::slice::from_raw_parts(msg.triangles.data, msg.triangles.size)};
for s in slice { temp.push(shape_msgs::msg::MeshTriangle::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
vertices : {
let mut temp = Vec::with_capacity(msg.vertices.size);
let slice = unsafe { std::slice::from_raw_parts(msg.vertices.data, msg.vertices.size)};
for s in slice { temp.push(geometry_msgs::msg::Point::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { shape_msgs__msg__MeshTriangle__Sequence__fini(&mut msg.triangles) };
unsafe { shape_msgs__msg__MeshTriangle__Sequence__init(&mut msg.triangles, self.triangles.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.triangles.data, msg.triangles.size)};
for (t,s) in slice.iter_mut().zip(&self.triangles) { s.copy_to_native(t);}
unsafe { geometry_msgs__msg__Point__Sequence__fini(&mut msg.vertices) };
unsafe { geometry_msgs__msg__Point__Sequence__init(&mut msg.vertices, self.vertices.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.vertices.data, msg.vertices.size)};
for (t,s) in slice.iter_mut().zip(&self.vertices) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for Mesh {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Mesh>::new();
                                  Mesh::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MeshTriangle {

                              pub vertex_indices: Vec<u32>,

                          }

                          impl WrappedTypesupport for MeshTriangle { 

            type CStruct = shape_msgs__msg__MeshTriangle; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__MeshTriangle() }
            }

            fn create_msg() -> *mut shape_msgs__msg__MeshTriangle {

                unsafe { shape_msgs__msg__MeshTriangle__create() }

            }

            fn destroy_msg(msg: *mut shape_msgs__msg__MeshTriangle) -> () {

                unsafe { shape_msgs__msg__MeshTriangle__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MeshTriangle {
  MeshTriangle {
// is_upper_bound_: false
// member.array_size_ : 3
vertex_indices: msg.vertex_indices.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {assert_eq!(self.vertex_indices.len(), 3, "Field {} is fixed size of {}!", "vertex_indices", 3);
msg.vertex_indices.copy_from_slice(&self.vertex_indices[..3]);
}



        }


                          
                          impl Default for MeshTriangle {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MeshTriangle>::new();
                                  MeshTriangle::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Plane {

                              pub coef: Vec<f64>,

                          }

                          impl WrappedTypesupport for Plane { 

            type CStruct = shape_msgs__msg__Plane; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__Plane() }
            }

            fn create_msg() -> *mut shape_msgs__msg__Plane {

                unsafe { shape_msgs__msg__Plane__create() }

            }

            fn destroy_msg(msg: *mut shape_msgs__msg__Plane) -> () {

                unsafe { shape_msgs__msg__Plane__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Plane {
  Plane {
// is_upper_bound_: false
// member.array_size_ : 4
coef: msg.coef.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {assert_eq!(self.coef.len(), 4, "Field {} is fixed size of {}!", "coef", 4);
msg.coef.copy_from_slice(&self.coef[..4]);
}



        }


                          
                          impl Default for Plane {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<Plane>::new();
                                  Plane::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct SolidPrimitive {

                              pub type_: u8,
pub dimensions: Vec<f64>,

                          }

                          impl WrappedTypesupport for SolidPrimitive { 

            type CStruct = shape_msgs__msg__SolidPrimitive; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__SolidPrimitive() }
            }

            fn create_msg() -> *mut shape_msgs__msg__SolidPrimitive {

                unsafe { shape_msgs__msg__SolidPrimitive__create() }

            }

            fn destroy_msg(msg: *mut shape_msgs__msg__SolidPrimitive) -> () {

                unsafe { shape_msgs__msg__SolidPrimitive__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> SolidPrimitive {
  SolidPrimitive {
type_: msg.type_,
// is_upper_bound_: true
// member.array_size_ : 3
dimensions: msg.dimensions.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.type_ = self.type_;
assert!(self.dimensions.len() <= 3, "Field {} is upper bounded by {}!", "dimensions", 3);
msg.dimensions.update(&self.dimensions);
}



        }


                          
                          impl Default for SolidPrimitive {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<SolidPrimitive>::new();
                                  SolidPrimitive::from_native(&msg_native)
                              }
                          }
             


                      }
