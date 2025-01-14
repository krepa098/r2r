  pub mod srv {
#[allow(non_snake_case)]
    pub mod MuxAdd {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__topic_tools_interfaces__srv__MuxAdd()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = topic_tools_interfaces__srv__MuxAdd_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxAdd_Request() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxAdd_Request {

                unsafe { topic_tools_interfaces__srv__MuxAdd_Request__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxAdd_Request) -> () {

                unsafe { topic_tools_interfaces__srv__MuxAdd_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

            type CStruct = topic_tools_interfaces__srv__MuxAdd_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxAdd_Response() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxAdd_Response {

                unsafe { topic_tools_interfaces__srv__MuxAdd_Response__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxAdd_Response) -> () {

                unsafe { topic_tools_interfaces__srv__MuxAdd_Response__destroy(msg) };

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
#[allow(non_snake_case)]
    pub mod MuxDelete {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__topic_tools_interfaces__srv__MuxDelete()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = topic_tools_interfaces__srv__MuxDelete_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxDelete_Request() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxDelete_Request {

                unsafe { topic_tools_interfaces__srv__MuxDelete_Request__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxDelete_Request) -> () {

                unsafe { topic_tools_interfaces__srv__MuxDelete_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

            type CStruct = topic_tools_interfaces__srv__MuxDelete_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxDelete_Response() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxDelete_Response {

                unsafe { topic_tools_interfaces__srv__MuxDelete_Response__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxDelete_Response) -> () {

                unsafe { topic_tools_interfaces__srv__MuxDelete_Response__destroy(msg) };

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
#[allow(non_snake_case)]
    pub mod MuxList {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__topic_tools_interfaces__srv__MuxList()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              
                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = topic_tools_interfaces__srv__MuxList_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxList_Request() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxList_Request {

                unsafe { topic_tools_interfaces__srv__MuxList_Request__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxList_Request) -> () {

                unsafe { topic_tools_interfaces__srv__MuxList_Request__destroy(msg) };

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

                              pub topics: Vec<std::string::String>,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = topic_tools_interfaces__srv__MuxList_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxList_Response() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxList_Response {

                unsafe { topic_tools_interfaces__srv__MuxList_Response__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxList_Response) -> () {

                unsafe { topic_tools_interfaces__srv__MuxList_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
// is_upper_bound_: false
// member.array_size_ : 0
topics: msg.topics.to_vec(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topics.update(&self.topics);
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
    pub mod MuxSelect {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__topic_tools_interfaces__srv__MuxSelect()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub topic: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = topic_tools_interfaces__srv__MuxSelect_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxSelect_Request() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxSelect_Request {

                unsafe { topic_tools_interfaces__srv__MuxSelect_Request__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxSelect_Request) -> () {

                unsafe { topic_tools_interfaces__srv__MuxSelect_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
topic: msg.topic.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.topic.assign(&self.topic);
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

                              pub prev_topic: std::string::String,
pub success: bool,

                          }

                          impl WrappedTypesupport for Response { 

            type CStruct = topic_tools_interfaces__srv__MuxSelect_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__topic_tools_interfaces__srv__MuxSelect_Response() }
            }

            fn create_msg() -> *mut topic_tools_interfaces__srv__MuxSelect_Response {

                unsafe { topic_tools_interfaces__srv__MuxSelect_Response__create() }

            }

            fn destroy_msg(msg: *mut topic_tools_interfaces__srv__MuxSelect_Response) -> () {

                unsafe { topic_tools_interfaces__srv__MuxSelect_Response__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Response {
  Response {
prev_topic: msg.prev_topic.to_str().to_owned(),
success: msg.success,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.prev_topic.assign(&self.prev_topic);
msg.success = self.success;
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
