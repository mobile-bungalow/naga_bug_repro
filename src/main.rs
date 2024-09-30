use naga::ShaderStage;
use wgpu::*;

const BASIC_EXEC: &str = r"
#version 450

layout(push_constant) uniform PushConstants {
    float add_1;
};

layout(push_constant) uniform PushConstants {
    float add_2;
};

layout(local_size_x=1, local_size_y=1, local_size_z=1) in;
void main() {
    float example = add_1 + add_2;
}
";

fn main() {
    let (device, _queue) = set_up_wgpu();

    let mut parser = wgpu::naga::front::glsl::Frontend::default();
    let options = wgpu::naga::front::glsl::Options::from(ShaderStage::Compute);
    let naga = parser.parse(&options, BASIC_EXEC).unwrap();

    device.push_error_scope(ErrorFilter::Validation);

    // Create implicitly invalid pipeline
    let _module = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Naga(std::borrow::Cow::Owned(naga)),
    });

    match pollster::block_on(device.pop_error_scope()) {
        Some(e) => panic!("{e}"),
        None => println!("that worked out okay but... it souldn't have..."),
    }
}

fn set_up_wgpu() -> (wgpu::Device, wgpu::Queue) {
    let instance = if cfg!(windows) {
        let desc = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            ..Default::default()
        };

        wgpu::Instance::new(desc)
    } else {
        wgpu::Instance::default()
    };

    let adapter = pollster::block_on(async {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .expect("Failed to find an appropriate adapter")
    });
    let mut required_limits = wgpu::Limits::default().using_resolution(adapter.limits());
    required_limits.max_push_constant_size = 128;

    let (d, q) = pollster::block_on(async {
        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::PUSH_CONSTANTS
                        | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
                        | wgpu::Features::MAPPABLE_PRIMARY_BUFFERS
                        | wgpu::Features::CLEAR_TEXTURE,
                    required_limits,
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device")
    });

    (d, q)
}
