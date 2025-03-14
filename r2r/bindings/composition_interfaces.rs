  pub mod srv {
#[allow(non_snake_case)]
    pub mod ListNodes {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__ListNodes()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = composition_interfaces__srv__ListNodes_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__ListNodes_Request() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__ListNodes_Request {

                unsafe { composition_interfaces__srv__ListNodes_Request__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__ListNodes_Request) -> () {

                unsafe { composition_interfaces__srv__ListNodes_Request__destroy(msg) };

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

                              pub full_node_names: Vec<std::string::String>,
pub unique_ids: Vec<u64>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = composition_interfaces__srv__ListNodes_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__ListNodes_Response() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__ListNodes_Response {

                unsafe { composition_interfaces__srv__ListNodes_Response__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__ListNodes_Response) -> () {

                unsafe { composition_interfaces__srv__ListNodes_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
full_node_names: msg.full_node_names.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
unique_ids: msg.unique_ids.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.full_node_names.update(&self.full_node_names);
msg.unique_ids.update(&self.unique_ids);
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
    pub mod LoadNode {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__LoadNode()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub package_name: std::string::String,
pub plugin_name: std::string::String,
pub node_name: std::string::String,
pub node_namespace: std::string::String,
pub log_level: u8,
pub remap_rules: Vec<std::string::String>,
pub parameters: Vec<rcl_interfaces::msg::Parameter>,
pub extra_arguments: Vec<rcl_interfaces::msg::Parameter>,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = composition_interfaces__srv__LoadNode_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Request() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__LoadNode_Request {

                unsafe { composition_interfaces__srv__LoadNode_Request__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__LoadNode_Request) -> () {

                unsafe { composition_interfaces__srv__LoadNode_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
package_name: msg.package_name.to_str().to_owned(),
plugin_name: msg.plugin_name.to_str().to_owned(),
node_name: msg.node_name.to_str().to_owned(),
node_namespace: msg.node_namespace.to_str().to_owned(),
log_level: msg.log_level,
// is_upper_bound_: false
// member.array_size_ : 0
remap_rules: msg.remap_rules.to_vec(),
// is_upper_bound_: false
// member.array_size_ : 0
parameters : {
let mut temp = Vec::with_capacity(msg.parameters.size);
let slice = unsafe { std::slice::from_raw_parts(msg.parameters.data, msg.parameters.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
// is_upper_bound_: false
// member.array_size_ : 0
extra_arguments : {
let mut temp = Vec::with_capacity(msg.extra_arguments.size);
let slice = unsafe { std::slice::from_raw_parts(msg.extra_arguments.data, msg.extra_arguments.size)};
for s in slice { temp.push(rcl_interfaces::msg::Parameter::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.package_name.assign(&self.package_name);
msg.plugin_name.assign(&self.plugin_name);
msg.node_name.assign(&self.node_name);
msg.node_namespace.assign(&self.node_namespace);
msg.log_level = self.log_level;
msg.remap_rules.update(&self.remap_rules);
unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.parameters) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.parameters, self.parameters.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.parameters.data, msg.parameters.size)};
for (t,s) in slice.iter_mut().zip(&self.parameters) { s.copy_to_native(t);}
unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(&mut msg.extra_arguments) };
unsafe { rcl_interfaces__msg__Parameter__Sequence__init(&mut msg.extra_arguments, self.extra_arguments.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.extra_arguments.data, msg.extra_arguments.size)};
for (t,s) in slice.iter_mut().zip(&self.extra_arguments) { s.copy_to_native(t);}
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
pub error_message: std::string::String,
pub full_node_name: std::string::String,
pub unique_id: u64,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = composition_interfaces__srv__LoadNode_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__LoadNode_Response() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__LoadNode_Response {

                unsafe { composition_interfaces__srv__LoadNode_Response__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__LoadNode_Response) -> () {

                unsafe { composition_interfaces__srv__LoadNode_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
error_message: msg.error_message.to_str().to_owned(),
full_node_name: msg.full_node_name.to_str().to_owned(),
unique_id: msg.unique_id,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.error_message.assign(&self.error_message);
msg.full_node_name.assign(&self.full_node_name);
msg.unique_id = self.unique_id;
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
    pub mod UnloadNode {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__composition_interfaces__srv__UnloadNode()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub unique_id: u64,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = composition_interfaces__srv__UnloadNode_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__UnloadNode_Request() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__UnloadNode_Request {

                unsafe { composition_interfaces__srv__UnloadNode_Request__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__UnloadNode_Request) -> () {

                unsafe { composition_interfaces__srv__UnloadNode_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
unique_id: msg.unique_id,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.unique_id = self.unique_id;
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
pub error_message: std::string::String,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = composition_interfaces__srv__UnloadNode_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__composition_interfaces__srv__UnloadNode_Response() }
            }

            fn create_msg() -> *mut composition_interfaces__srv__UnloadNode_Response {

                unsafe { composition_interfaces__srv__UnloadNode_Response__create() }

            }

            fn destroy_msg(msg: *mut composition_interfaces__srv__UnloadNode_Response) -> () {

                unsafe { composition_interfaces__srv__UnloadNode_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
success: msg.success,
error_message: msg.error_message.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.success = self.success;
msg.error_message.assign(&self.error_message);
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
