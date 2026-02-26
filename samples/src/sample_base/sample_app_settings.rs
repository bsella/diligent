use diligent::{AdapterType, RenderDeviceType, get_prefered_device_type};

use clap::{Arg, command, value_parser};

pub struct SampleAppSettings {
    pub device_type: RenderDeviceType,

    pub width: u32,
    pub height: u32,

    pub validation: bool,

    pub adapter_index: Option<usize>,

    pub adapter_type: AdapterType,

    pub adapters_dialog: bool,

    pub show_ui: bool,
    pub show_adapters_dialog: bool,

    pub vsync: bool,

    pub non_separable_progs: bool,

    #[cfg(feature = "vulkan")]
    pub vk_compatibility: bool,
}

impl SampleAppSettings {
    pub fn get_render_device_type(&self) -> RenderDeviceType {
        self.device_type
    }
    pub fn get_window_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub fn parse_sample_app_settings() -> SampleAppSettings {
    let args = command!()
        .arg(Arg::new("mode").long("mode").short('m').value_parser([
            #[cfg(feature = "d3d11")]
            "d3d11_sw",
            #[cfg(feature = "d3d12")]
            "d3d12_sw",
            #[cfg(feature = "vulkan")]
            "vk_sw",
            #[cfg(feature = "d3d11")]
            "d3d11",
            #[cfg(feature = "d3d12")]
            "d3d12",
            #[cfg(feature = "opengl")]
            "gl",
            "gles",
            #[cfg(feature = "vulkan")]
            "vk",
            "mtl",
            "wgpu",
        ]))
        .arg(Arg::new("adapter").long("adapter"))
        .arg(
            Arg::new("width")
                .long("width")
                .short('w')
                .value_parser(value_parser!(u32))
                .default_value("1024"),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .short('h')
                .value_parser(value_parser!(u32))
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
        .arg(
            Arg::new("non_separable_progs")
                .long("non_separable_progs")
                .value_parser(value_parser!(bool))
                .default_value("false"),
        )
        // TODO : add a help subcommand for the `--help` flag
        .disable_help_flag(true);

    #[cfg(feature = "vulkan")]
    let args = args.arg(
        Arg::new("vk_compatibility")
            .long("vk_compatibility")
            .value_parser(value_parser!(bool))
            .default_value("false"),
    );

    let matches = args.get_matches();

    let mut settings = SampleAppSettings {
        device_type: get_prefered_device_type(),
        width: *matches.get_one::<u32>("width").unwrap(),
        height: *matches.get_one::<u32>("height").unwrap(),
        adapter_index: None,
        adapter_type: AdapterType::Unknown,
        adapters_dialog: true,
        show_ui: *matches.get_one::<bool>("show_ui").unwrap(),
        show_adapters_dialog: *matches.get_one::<bool>("adapters_dialog").unwrap(),
        validation: true,
        vsync: *matches.get_one::<bool>("vsync").unwrap(),
        non_separable_progs: *matches.get_one::<bool>("non_separable_progs").unwrap(),
        #[cfg(feature = "vulkan")]
        vk_compatibility: *matches.get_one::<bool>("vk_compatibility").unwrap(),
    };

    if let Some(mode) = matches.get_one::<String>("mode") {
        let mode = mode.as_str();
        match mode {
            #[cfg(feature = "d3d11")]
            "d3d11_sw" => {
                settings.device_type = RenderDeviceType::D3D11;
                settings.adapter_type = AdapterType::Software;
            }
            #[cfg(feature = "d3d12")]
            "d3d12_sw" => {
                settings.device_type = RenderDeviceType::D3D12;
                settings.adapter_type = AdapterType::Software;
            }
            #[cfg(feature = "vulkan")]
            "vk_sw" => {
                settings.device_type = RenderDeviceType::VULKAN;
                settings.adapter_type = AdapterType::Software;
            }
            #[cfg(feature = "d3d11")]
            "d3d11" => {
                settings.device_type = RenderDeviceType::D3D11;
            }
            #[cfg(feature = "d3d12")]
            "d3d12" => {
                settings.device_type = RenderDeviceType::D3D12;
            }
            #[cfg(feature = "opengl")]
            "gl" => {
                settings.device_type = RenderDeviceType::GL;
            }
            //"gles" => {
            //    settings.device_type = RenderDeviceType::GLES;
            //}
            #[cfg(feature = "vulkan")]
            "vk" => {
                settings.device_type = RenderDeviceType::VULKAN;
            }
            #[cfg(feature = "metal")]
            "mtl" => {
                settings.device_type = RenderDeviceType::METAL;
            }
            #[cfg(feature = "webgpu")]
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
