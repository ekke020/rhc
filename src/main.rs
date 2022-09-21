mod core;
mod sha2;
mod systems;
use std::convert::TryInto;
use std::env;
use vulkano::buffer::{BufferContents, BufferUsage, CpuAccessibleBuffer};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Features, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::sync::{self, GpuFuture};
fn main() {
    // let password_info = systems::input::take();
    // systems::spawner::run_threads(password_info);
    // gpu_test();

    // sha2::sha256::test("hello world");
    let hash = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
    let hash = convert_to_decimal_array(hash);
    let test = "test".as_bytes();
    let mut k = sha2::Sha256::new(hash, test);
    let result = k.run();
    println!("{:?}", result.unwrap());
}
// TODO: Implement a global value with the decimal array?
fn convert_to_decimal_array(hash: &str) -> Vec<u32> {
    use std::u32;
    hash
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|s| u32::from_str_radix(s, 16).unwrap())
        .collect::<Vec<u32>>()
}

fn gpu_test() {
    let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue families that we want to use
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    println!(
        "The name of the graphics card is: {}",
        device.physical_device().properties().device_name
    );

    let data_iter = 0..65536;
    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    let shader = cs::load(device.clone()).expect("failed to create shader module");

    use vulkano::pipeline::ComputePipeline;

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        shader.entry_point("main").unwrap(),
        &(),
        None,
        |_| {},
    )
    .expect("failed to create compute pipeline");

    use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
    use vulkano::pipeline::Pipeline;

    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        layout.clone(),
        [WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
    )
    .unwrap();

    use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
    use vulkano::pipeline::PipelineBindPoint;

    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0, // 0 is the index of our set
            set,
        )
        .dispatch([1024, 1, 1])
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        println!("{}", val);
        assert_eq!(*val, n as u32 * 2);
    }

    println!("Everything succeeded!");
}

mod cs {
    vulkano_shaders::shader! {
        ty: "compute",
        src: "
        #version 450

        layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

        layout(set = 0, binding = 0) buffer Data {
            uint data[];
        } buf;

        void main() {
            uint idx = gl_GlobalInvocationID.x;
            buf.data[idx] *= 2;
        }"
    }
}
