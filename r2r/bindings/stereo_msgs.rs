  pub mod msg {
    use super::super::*;

                          #[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
                          #[serde(default)]
                          pub struct DisparityImage {

                              pub header: std_msgs::msg::Header,
pub image: sensor_msgs::msg::Image,
pub f: f32,
pub t: f32,
pub valid_window: sensor_msgs::msg::RegionOfInterest,
pub min_disparity: f32,
pub max_disparity: f32,
pub delta_d: f32,

                          }

                          impl WrappedTypesupport for DisparityImage { 

            type CStruct = stereo_msgs__msg__DisparityImage; 


            fn get_ts() -> &'static rosidl_message_type_support_t { 

                unsafe { &*rosidl_typesupport_c__get_message_type_support_handle__stereo_msgs__msg__DisparityImage() }
            }

            fn create_msg() -> *mut stereo_msgs__msg__DisparityImage {

                unsafe { stereo_msgs__msg__DisparityImage__create() }

            }

            fn destroy_msg(msg: *mut stereo_msgs__msg__DisparityImage) -> () {

                unsafe { stereo_msgs__msg__DisparityImage__destroy(msg) };

            }

            fn from_native(msg: &Self::CStruct) -> DisparityImage {
  DisparityImage {
header: std_msgs::msg::Header::from_native(&msg.header),
image: sensor_msgs::msg::Image::from_native(&msg.image),
f: msg.f,
t: msg.t,
valid_window: sensor_msgs::msg::RegionOfInterest::from_native(&msg.valid_window),
min_disparity: msg.min_disparity,
max_disparity: msg.max_disparity,
delta_d: msg.delta_d,
      }
    }



            fn copy_to_native(&self, msg: &mut Self::CStruct) {self.header.copy_to_native(&mut msg.header);
self.image.copy_to_native(&mut msg.image);
msg.f = self.f;
msg.t = self.t;
self.valid_window.copy_to_native(&mut msg.valid_window);
msg.min_disparity = self.min_disparity;
msg.max_disparity = self.max_disparity;
msg.delta_d = self.delta_d;
}



        }


                          
                          impl Default for DisparityImage {
                              fn default() -> Self {
                                  let msg_native = WrappedNativeMsg::<DisparityImage>::new();
                                  DisparityImage::from_native(&msg_native)
                              }
                          }
             


                      }
