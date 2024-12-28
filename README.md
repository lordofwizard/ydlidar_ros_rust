# Introduction
hey this is just a reimplementation of the ydlidar library written in rust. The motive is to build a standardised optimised sdk which garentees memory safety for getting optimal input from Lazer based YDLidars.   

# Setup
### Install the base ros2_rust and also the colcon-cargo things.
```shell
# Install Rust, e.g. as described in https://rustup.rs/
# Install ROS 2 as described in https://docs.ros.org/en/humble/Installation.html
# Assuming you installed the minimal version of ROS 2, you need these additional packages:
sudo apt install -y git libclang-dev python3-pip python3-vcstool # libclang-dev is required by bindgen
# Install these plugins for cargo and colcon:
pip install git+https://github.com/colcon/colcon-cargo.git
pip install git+https://github.com/colcon/colcon-ros-cargo.git

mkdir -p workspace/src && cd workspace
git clone https://github.com/ros2-rust/ros2_rust.git src/ros2_rust
vcs import src < src/ros2_rust/ros2_rust_humble.repos
. /opt/ros/humble/setup.sh
colcon build
```

### Then, to run the minimal pub-sub example, do this:

```shell
# In a new terminal (or tmux window)
. ./install/setup.sh
```

Setup a 
