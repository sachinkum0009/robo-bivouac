use rclrs::*;
use std::{thread, time::Duration};
use std_msgs::msg::String as StringMsg;
use geometry_msgs::msg::PoseStamped as PoseStampedMsg;
use nav2_msgs::msg::SpeedLimit as SpeedLimitMsg;

struct SimplePublisherNode {
    publisher: Publisher<StringMsg>,
}

impl SimplePublisherNode {
    fn new(executor: &Executor) -> Result<Self, RclrsError> {
        let node = executor.create_node("simple_publisher").unwrap();
        let publisher = node.create_publisher("publish_hello").unwrap();
        Ok(Self { publisher })
    }

    fn publish_data(&self, increment: i32) -> Result<i32, RclrsError> {
        let msg: StringMsg = StringMsg {
            data: format!("Hello World: {}", increment),
        };
        println!("Publishing data: {:?}", msg);
        self.publisher.publish(msg).unwrap();
        Ok(increment + 1_i32)
    }
}

fn main() -> Result<(), RclrsError> {
    let mut executor = Context::default_from_env().unwrap().create_basic_executor();
    let node = SimplePublisherNode::new(&executor).unwrap();
    let mut count: i32 = 0;
    let mut pose_stamped_msg = PoseStampedMsg::default();
    pose_stamped_msg.header.frame_id =  String::from("base_link");

    let mut speed = SpeedLimitMsg::default();
    speed.header.frame_id = String::from("base_link");
    speed.percentage = true;

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000));
        count = node.publish_data(count).unwrap();
    });
    executor.spin(SpinOptions::default()).first_error()
}
