  pub mod srv {
#[allow(non_snake_case)]
    pub mod ConfigLogger {
    use super::super::super::*;

        #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
        pub struct Service();
        impl WrappedServiceTypeSupport for Service {
            type Request = Request;
            type Response = Response;
            fn get_ts() -> &'static rosidl_service_type_support_t {
                unsafe {
                    &*rosidl_typesupport_c__get_service_type_support_handle__logging_demo__srv__ConfigLogger()
                }
            }
        }

            
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct Request {

                              pub logger_name: std::string::String,
pub level: std::string::String,

                          }

                          impl WrappedTypesupport for Request { 

            type CStruct = logging_demo__srv__ConfigLogger_Request; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__logging_demo__srv__ConfigLogger_Request() }
            }

            fn create_msg() -> *mut logging_demo__srv__ConfigLogger_Request {

                unsafe { logging_demo__srv__ConfigLogger_Request__create() }

            }

            fn destroy_msg(msg: *mut logging_demo__srv__ConfigLogger_Request) -> () {

                unsafe { logging_demo__srv__ConfigLogger_Request__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> Request {
  Request {
logger_name: msg.logger_name.to_str().to_owned(),
level: msg.level.to_str().to_owned(),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.logger_name.assign(&self.logger_name);
msg.level.assign(&self.level);
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

            type CStruct = logging_demo__srv__ConfigLogger_Response; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__logging_demo__srv__ConfigLogger_Response() }
            }

            fn create_msg() -> *mut logging_demo__srv__ConfigLogger_Response {

                unsafe { logging_demo__srv__ConfigLogger_Response__create() }

            }

            fn destroy_msg(msg: *mut logging_demo__srv__ConfigLogger_Response) -> () {

                unsafe { logging_demo__srv__ConfigLogger_Response__destroy(msg) };

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
