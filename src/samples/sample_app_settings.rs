use clap::{command, value_parser, Arg};

use crate::{
    core::graphics_types::{AdapterType, RenderDeviceType},
    tools::native_app::app_settings::AppSettings,
};

pub struct SampleAppSettings {
    pub device_type: RenderDeviceType,

    pub width: u16,
    pub height: u16,

    pub validation: bool,

    pub adapter_index: Option<usize>,

    pub adapter_type: AdapterType,

    pub adapters_dialog: bool,

    pub show_ui: bool,
    pub show_adapters_dialog: bool,

    pub vsync: bool,
}

impl AppSettings for SampleAppSettings {
    fn get_window_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

pub fn parse_sample_app_settings() -> SampleAppSettings {
    let matches = command!()
        .arg(Arg::new("mode").long("mode").short('m').value_parser([
            "d3d11_sw", "d3d12_sw", "vk_sw", "d3d11", "d3d12", "gl", "gles", "vk", "mtl", "wgpu",
        ]))
        .arg(Arg::new("adapter").long("adapter"))
        .arg(
            Arg::new("width")
                .long("width")
                .short('w')
                .value_parser(value_parser!(u16))
                .default_value("1024"),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .short('h')
                .value_parser(value_parser!(u16))
                .default_value("786"),
        )
        .arg(
            Arg::new("show_ui")
                .long("show_ui")
                .value_parser(value_parser!(bool))
                .default_value("true"),
        )
        .arg(
            Arg::new("vsync")
                .long("vsync")
                .value_parser(value_parser!(bool))
                .default_value("false"),
        )
        .arg(
            Arg::new("adapters_dialog")
                .long("adapters_dialog")
                .value_parser(value_parser!(bool))
                .default_value("true"),
        )
        // TODO : add a help subcommand for the `--help` flag
        .disable_help_flag(true)
        .get_matches();

    let mut settings = SampleAppSettings {
        device_type: RenderDeviceType::VULKAN,
        width: *matches.get_one::<u16>("width").unwrap(),
        height: *matches.get_one::<u16>("height").unwrap(),
        adapter_index: None,
        adapter_type: AdapterType::Unknown,
        adapters_dialog: true,
        show_ui: *matches.get_one::<bool>("show_ui").unwrap(),
        show_adapters_dialog: *matches.get_one::<bool>("adapters_dialog").unwrap(),
        validation: true,
        vsync: *matches.get_one::<bool>("vsync").unwrap(),
    };

    if let Some(mode) = matches.get_one::<String>("mode") {
        let mode = mode.as_str();
        match mode {
            "d3d11_sw" => {
                settings.device_type = RenderDeviceType::D3D11;
                settings.adapter_type = AdapterType::Software;
            }
            "d3d12_sw" => {
                settings.device_type = RenderDeviceType::D3D12;
                settings.adapter_type = AdapterType::Software;
            }
            "vk_sw" => {
                settings.device_type = RenderDeviceType::VULKAN;
                settings.adapter_type = AdapterType::Software;
            }
            "d3d11" => {
                settings.device_type = RenderDeviceType::D3D11;
            }
            "d3d12" => {
                settings.device_type = RenderDeviceType::D3D12;
            }
            "gl" => {
                settings.device_type = RenderDeviceType::GL;
            }
            "gles" => {
                settings.device_type = RenderDeviceType::GLES;
            }
            "vk" => {
                settings.device_type = RenderDeviceType::VULKAN;
            }
            "mtl" => {
                settings.device_type = RenderDeviceType::METAL;
            }
            "wgpu" => {
                settings.device_type = RenderDeviceType::WEBGPU;
            }
            &_ => {
                panic!("Unknown device type")
            }
        }
    }

    if let Some(adapter_type) = matches.get_one::<String>("adapter") {
        if adapter_type == "sw" {
            settings.adapter_type = AdapterType::Software;
        }
    } else if let Some(adapter_index) = matches.get_one::<usize>("adapter") {
        settings.adapter_index = Some(*adapter_index);
    }

    settings
}
