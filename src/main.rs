#![allow(warnings)]
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

use crate::systems::printer::print;
fn main() {
    // let password_info = systems::input::take();
    // systems::spawner::run_threads(password_info);
    // gpu_test();
    let hashed_256_value_of_test = "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08";
    let hashed_224_value_of_test = "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809";
    let test = "test";
    let mut sha256 = sha2::Sha256::new(test);
    sha256.run();
    // let result = sha256.extract_as_upper_hex();
    // println!("256: {:?}", sha2::convert_to_decimal_array(hashed_256_value_of_test));
    // let mut sha224 = sha2::Sha224::new(test);
    // sha224.run();
    // let result = sha224.extract_as_lower_hex();
    // println!("224: {}", result);
    // println!("224: {:?}", sha2::convert_to_decimal_array(hashed_224_value_of_test));
    let correct = "ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff";
    let mut sha512 = sha2::Sha512::new(test);
    sha512.run();
    assert_eq!(correct, sha512.extract_as_lower_hex());
    //ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db2
    //7ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff
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
