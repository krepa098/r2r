  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ConnectedClient {

                              pub ip_address: std::string::String,
pub connection_time: builtin_interfaces::msg::Time,

                          }

                          impl WrappedTypesupport for ConnectedClient { 

            type CStruct = rosbridge_msgs__msg__ConnectedClient; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbridge_msgs__msg__ConnectedClient() }
            }

            fn create_msg() -> *mut rosbridge_msgs__msg__ConnectedClient {

                unsafe { rosbridge_msgs__msg__ConnectedClient__create() }

            }

            fn destroy_msg(msg: *mut rosbridge_msgs__msg__ConnectedClient) -> () {

                unsafe { rosbridge_msgs__msg__ConnectedClient__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ConnectedClient {
  ConnectedClient {
ip_address: msg.ip_address.to_str().to_owned(),
connection_time: builtin_interfaces::msg::Time::from_native(&msg.connection_time),
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.ip_address.assign(&self.ip_address);
self.connection_time.copy_to_native(&mut msg.connection_time);
}



        }


                          
                          impl Default for ConnectedClient {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ConnectedClient>::new();
                                  ConnectedClient::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct ConnectedClients {

                              pub clients: Vec<rosbridge_msgs::msg::ConnectedClient>,

                          }

                          impl WrappedTypesupport for ConnectedClients { 

            type CStruct = rosbridge_msgs__msg__ConnectedClients; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__rosbridge_msgs__msg__ConnectedClients() }
            }

            fn create_msg() -> *mut rosbridge_msgs__msg__ConnectedClients {

                unsafe { rosbridge_msgs__msg__ConnectedClients__create() }

            }

            fn destroy_msg(msg: *mut rosbridge_msgs__msg__ConnectedClients) -> () {

                unsafe { rosbridge_msgs__msg__ConnectedClients__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> ConnectedClients {
  ConnectedClients {
// is_upper_bound_: false
// member.array_size_ : 0
clients : {
let mut temp = Vec::with_capacity(msg.clients.size);
let slice = unsafe { std::slice::from_raw_parts(msg.clients.data, msg.clients.size)};
for s in slice { temp.push(rosbridge_msgs::msg::ConnectedClient::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {unsafe { rosbridge_msgs__msg__ConnectedClient__Sequence__fini(&mut msg.clients) };
unsafe { rosbridge_msgs__msg__ConnectedClient__Sequence__init(&mut msg.clients, self.clients.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.clients.data, msg.clients.size)};
for (t,s) in slice.iter_mut().zip(&self.clients) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for ConnectedClients {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<ConnectedClients>::new();
                                  ConnectedClients::from_native(&msg_native)
                              }
                          }
             


                      }
