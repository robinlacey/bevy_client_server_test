use bevy::app::App;
use bevy::{DefaultPlugins, MinimalPlugins};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::collections::HashMap;
use bevy_networking_turbulence::{NetworkingPlugin, MessageChannelSettings, MessageChannelMode, ReliableChannelSettings, NetworkResource, ConnectionChannelsBuilder};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::{panic, env};
use bevy::prelude::{ResMut, Res, IntoSystem, Time};
use bevy::prelude::stage::POST_UPDATE;
use bevy::ecs::Local;

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Message(pub String);
#[derive(Deserialize,Serialize)]
pub struct ServerAddress(pub SocketAddr);
#[derive(Default)]
pub struct Ticker(pub f32);

fn main() {
    let mut app = App::build();
    app.add_plugins(MinimalPlugins);
    app.add_plugin(NetworkingPlugin);
    println!("{}",&*format!("Getting Server address"));
    app.add_resource(ServerAddress(get_local_socket_address()));
    app.add_startup_system(server_networking_setup.system());
    app.add_startup_system(server_build_network_channels.system());
    app.add_system_to_stage(POST_UPDATE,post_update_send.system());
    app.run();
}

pub fn post_update_send(
    mut ticker: Local<Ticker>,
    time: Res<Time>,
    mut net: ResMut<NetworkResource>,
) {
    ticker.0 += time.delta_seconds();
    if ticker.0 >1. {
        let message = Message(time.seconds_since_startup().to_string());
        println!("sending {}", &message.0);
        net.broadcast_message(message);
        ticker.0 = 0.;
    }
}

fn get_local_socket_address() -> SocketAddr{
    // Tried 0.0.0.0 , 127.0.0.1 and find_my_ip_address
    // SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 14191)
    let ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    SocketAddr::new(ip_address, 14191)
}

pub fn server_networking_setup(mut net: ResMut<NetworkResource>, server_address: Res<ServerAddress>) {
    net.listen(server_address.0);
    println!("Server running {:?}",server_address.0);
}

fn server_build_network_channels(mut net: ResMut<NetworkResource>) {
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
