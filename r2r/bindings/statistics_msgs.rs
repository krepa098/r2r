  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct MetricsMessage {

                              pub measurement_source_name: std::string::String,
pub metrics_source: std::string::String,
pub unit: std::string::String,
pub window_start: builtin_interfaces::msg::Time,
pub window_stop: builtin_interfaces::msg::Time,
pub statistics: Vec<statistics_msgs::msg::StatisticDataPoint>,

                          }

                          impl WrappedTypesupport for MetricsMessage { 

            type CStruct = statistics_msgs__msg__MetricsMessage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__MetricsMessage() }
            }

            fn create_msg() -> *mut statistics_msgs__msg__MetricsMessage {

                unsafe { statistics_msgs__msg__MetricsMessage__create() }

            }

            fn destroy_msg(msg: *mut statistics_msgs__msg__MetricsMessage) -> () {

                unsafe { statistics_msgs__msg__MetricsMessage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> MetricsMessage {
  MetricsMessage {
measurement_source_name: msg.measurement_source_name.to_str().to_owned(),
metrics_source: msg.metrics_source.to_str().to_owned(),
unit: msg.unit.to_str().to_owned(),
window_start: builtin_interfaces::msg::Time::from_native(&msg.window_start),
window_stop: builtin_interfaces::msg::Time::from_native(&msg.window_stop),
// is_upper_bound_: false
// member.array_size_ : 0
statistics : {
let mut temp = Vec::with_capacity(msg.statistics.size);
let slice = unsafe { std::slice::from_raw_parts(msg.statistics.data, msg.statistics.size)};
for s in slice { temp.push(statistics_msgs::msg::StatisticDataPoint::from_native(s)); }
temp },
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.measurement_source_name.assign(&self.measurement_source_name);
msg.metrics_source.assign(&self.metrics_source);
msg.unit.assign(&self.unit);
self.window_start.copy_to_native(&mut msg.window_start);
self.window_stop.copy_to_native(&mut msg.window_stop);
unsafe { statistics_msgs__msg__StatisticDataPoint__Sequence__fini(&mut msg.statistics) };
unsafe { statistics_msgs__msg__StatisticDataPoint__Sequence__init(&mut msg.statistics, self.statistics.len()) };
let slice = unsafe { std::slice::from_raw_parts_mut(msg.statistics.data, msg.statistics.size)};
for (t,s) in slice.iter_mut().zip(&self.statistics) { s.copy_to_native(t);}
}



        }


                          
                          impl Default for MetricsMessage {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<MetricsMessage>::new();
                                  MetricsMessage::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct StatisticDataPoint {

                              pub data_type: u8,
pub data: f64,

                          }

                          impl WrappedTypesupport for StatisticDataPoint { 

            type CStruct = statistics_msgs__msg__StatisticDataPoint; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__StatisticDataPoint() }
            }

            fn create_msg() -> *mut statistics_msgs__msg__StatisticDataPoint {

                unsafe { statistics_msgs__msg__StatisticDataPoint__create() }

            }

            fn destroy_msg(msg: *mut statistics_msgs__msg__StatisticDataPoint) -> () {

                unsafe { statistics_msgs__msg__StatisticDataPoint__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> StatisticDataPoint {
  StatisticDataPoint {
data_type: msg.data_type,
data: msg.data,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {msg.data_type = self.data_type;
msg.data = self.data;
}



        }


                          
                          impl Default for StatisticDataPoint {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<StatisticDataPoint>::new();
                                  StatisticDataPoint::from_native(&msg_native)
                              }
                          }
             


                    
                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct StatisticDataType {

                              
                          }

                          impl WrappedTypesupport for StatisticDataType { 

            type CStruct = statistics_msgs__msg__StatisticDataType; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__StatisticDataType() }
            }

            fn create_msg() -> *mut statistics_msgs__msg__StatisticDataType {

                unsafe { statistics_msgs__msg__StatisticDataType__create() }

            }

            fn destroy_msg(msg: *mut statistics_msgs__msg__StatisticDataType) -> () {

                unsafe { statistics_msgs__msg__StatisticDataType__destroy(msg) };

            }

            fn from_native(_msg: &Self::CStruct) -> StatisticDataType {
  StatisticDataType {
      }
    }



            fn copy_to_native(&self, _msg: &mut Self::CStruct) {}



        }


                          
                          impl Default for StatisticDataType {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<StatisticDataType>::new();
                                  StatisticDataType::from_native(&msg_native)
                              }
                          }
             


                      }
