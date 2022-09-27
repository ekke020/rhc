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
    // TODO: For some reason hash is incorrect after 55 chars
    let hash_224 = "90a3ed9e32b2aaf4c61c410eb925426119e1a9dc53d4286ade99a809";
    let hash_224 = convert_to_decimal_array(hash_224);
    let hash_256 = "49b794dcbb3afbe1a4c6602e86a193986eb0a6283740cfd032239560e1c9cc79";
    let hash_256 = convert_to_decimal_array(hash_256);
    let test = "iawbdjnawdjnajdadjawndadkadwakdajkdanjkdnjakdnjawkdkanjdnk.adnkawnkjdnjadnanjkdanjdnwanjdnjnjawnjdnjawdnjnjdnjadnjwanjdanjdnjadnjnjawndjnanwdnjanjdjnawdnjnadjnawjdnjwanjdanjdnkajwdnkjdnjnjawnjdanjk.dnanj.wd.njdnj.anjwdnjadnjnjadnjawndjanjdnjwanjdanjkdnjkawnjk.dwndjwadwnajwdnj".as_bytes();
    let mut sha256 = sha2::Sha256::new(test);
    let result = sha256.run().unwrap();
    println!("256: {:x?}", result);
    // use crate::sha2::wrapper::Hash;
    // let mut sha224 = sha2::Sha224::new(test);
    // sha224.run();
    // let test = sha224.extract().take();
    // println!("224: {:0>8x?}", test);
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
