#[allow(unused)]
#[repr(u64)]
enum WeechatApiVersions {
    V4_0_0 = 20230220,
    V4_1_0 = 20230908,
    V4_2_0 = 20240105,
    V4_3_0 = 20240402,
    V4_4_0 = 20240727,
    V4_5_0 = 20241124,
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WEECHAT_BUNDLED");
    println!("cargo:rerun-if-env-changed=WEECHAT_PLUGIN_FILE");
    println!("cargo::rustc-check-cfg=cfg(weechat408)");
    println!("cargo::rustc-check-cfg=cfg(weechat410)");
    println!("cargo::rustc-check-cfg=cfg(weechat420)");
    println!("cargo::rustc-check-cfg=cfg(weechat430)");
    println!("cargo::rustc-check-cfg=cfg(weechat440)");
    println!("cargo::rustc-check-cfg=cfg(weechat450)");

    let (version, _) = std::str::from_utf8(weechat_sys::WEECHAT_PLUGIN_API_VERSION)
        .unwrap()
        .split_once('-')
        .unwrap();

    println!("cargo::warning=WEECHAT_PLUGIN_API_VERSION: {version}");

    let version: u64 = version.parse().unwrap();

    use crate::WeechatApiVersions::*;
    match version {
        v if v >= V4_5_0 as _ => {
            println!("cargo::rustc-cfg=weechat450");
        }
        v if v >= V4_4_0 as _ => {
            println!("cargo::rustc-cfg=weechat440");
        }
        v if v >= V4_3_0 as _ => {
            println!("cargo::rustc-cfg=weechat430");
        }
        v if v >= V4_2_0 as _ => {
            println!("cargo::rustc-cfg=weechat420");
        }
        v if v >= V4_1_0 as _ => {
            println!("cargo::rustc-cfg=weechat410");
        }
        v if v < V4_1_0 as _ => {
            println!("cargo::rustc-cfg=weechat400");
        }
        _ => {
            println!("cargo::warning=Failed to match weechat API version: {version}");
        }
    }
}
