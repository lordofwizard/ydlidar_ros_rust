use ros2_rust::rclrs;
use rclrs::{Context, Node, Publisher, Service, Timer};
use sensor_msgs::msg::LaserScan;
use std_srvs::srv::Empty;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// Constants
const SDK_ROS_VERSION: &str = "1.0.2";

struct YDLidar {
    is_scanning: bool,
}

impl YDLidar {
    fn new() -> Self {
        Self { is_scanning: false }
    }

    fn initialize(&self) -> bool {
        println!("Initializing LiDAR...");
        true
    }

    fn turn_on(&mut self) -> bool {
        if !self.is_scanning {
            println!("Turning on scanning...");
            self.is_scanning = true;
            true
        } else {
            println!("LiDAR is already scanning.");
            false
        }
    }

    fn turn_off(&mut self) -> bool {
        if self.is_scanning {
            println!("Turning off scanning...");
            self.is_scanning = false;
            true
        } else {
            println!("LiDAR is not scanning.");
            false
        }
    }
}

fn main() -> rclrs::Result<()> {
    let context = Context::new()?;
    let mut node = Node::new(&context, "ydlidar_ros_driver")?;

    println!("YDLIDAR ROS Driver Version: {}", SDK_ROS_VERSION);

    // Shared YDLidar instance
    let lidar = Arc::new(Mutex::new(YDLidar::new()));

    // Publishers
    let scan_publisher: Publisher<LaserScan> = node.create_publisher("scan", rclrs::QOS_PROFILE_DEFAULT)?;

    // Services
    {
        let lidar = Arc::clone(&lidar);
        let service_stop: Service<Empty> = node.create_service("stop_scan", move |_request, _response| {
            let mut lidar = lidar.lock().unwrap();
            lidar.turn_off();
            println!("Stop scan service called.");
        })?;

        let lidar = Arc::clone(&lidar);
        let service_start: Service<Empty> = node.create_service("start_scan", move |_request, _response| {
            let mut lidar = lidar.lock().unwrap();
            lidar.turn_on();
            println!("Start scan service called.");
        })?;
    }

    // Initialize and start LiDAR scanning
    let mut lidar_instance = lidar.lock().unwrap();
    if lidar_instance.initialize() {
        lidar_instance.turn_on();
    }

    // Timer for publishing data
    let lidar = Arc::clone(&lidar);
    let timer = node.create_wall_timer(Duration::from_millis(100), move || {
        let lidar = lidar.lock().unwrap();
        if lidar.is_scanning {
            let scan_msg = LaserScan::default(); // Populate with actual scan data
            println!("Publishing scan data...");
            scan_publisher.publish(scan_msg).unwrap();
        }
    })?;

    // Spin the node
    rclrs::spin(&node)?;
    Ok(())
}
