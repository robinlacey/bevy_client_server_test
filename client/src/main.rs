use bevy::app::App;
use bevy::prelude::Msaa;
use bevy::DefaultPlugins;
use std::net::{SocketAddr, IpAddr, Ipv4Addr,ToSocketAddrs};
use std::collections::HashMap;
use url::Url;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use bevy_networking_turbulence::{NetworkingPlugin, MessageChannelSettings, MessageChannelMode, ReliableChannelSettings, NetworkResource, ConnectionChannelsBuilder};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::panic;
use bevy::prelude::stage::PRE_UPDATE;

extern crate console_error_panic_hook;
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Message(pub String);
#[derive(Deserialize,Serialize,Debug)]
pub struct ServerAddress(pub SocketAddr);
fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(NetworkingPlugin);
    console_log!("{}",&*format!("Getting Server address"));
    app.add_resource(ServerAddress(get_server_socket_address()));
    app.add_startup_system(client_networking_setup.system());
    app.add_startup_system(client_build_network_channels.system());
    app.add_system_to_stage(PRE_UPDATE,pre_update_receive.system());
    app.run();
}
pub fn pre_update_receive(
    mut net: ResMut<NetworkResource>
){
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(mut m) = channels.recv::<Message>() {
            console_log!("Got Message {:?}",m.0);
        }
    }
}

fn get_server_socket_address() -> SocketAddr{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let parsed_url = Url::parse(&*document.url().unwrap()).unwrap();
    let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();
    let mut return_ip = get_local_socket_address();
    if hash_query.contains_key("server_ip") {
        let server = hash_query["server_ip"].parse::<SocketAddr>();
        match server {
            Ok(ip) => {
                return_ip = ip
            }
            Err(_) => {
                console_log!("{}",(&*format!("Invalid IP {:?}. Setting to local address.", hash_query["server_ip"])));
            }
        };
    }
    console_log!("{}",&*format!("Address of Server set to: {:?}", return_ip));
    return_ip
}

fn get_local_socket_address() -> SocketAddr{
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234)
}

pub fn client_networking_setup(mut net: ResMut<NetworkResource>, server_address: Res<ServerAddress>) {
    net.connect(server_address.0);
    console_log!("Client running {:?}",server_address.0);
}

fn client_build_network_channels(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder.register::<Message>(MESSAGE_SETTINGS).unwrap();
    });
}

const MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};
