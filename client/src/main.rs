use bevy::{
    app::{App, EventReader, Events, ScheduleRunnerSettings},
    core::Time,
    ecs::prelude::*,
    MinimalPlugins,
};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};

use std::{net::SocketAddr, time::Duration};

const SERVER_PORT: u16 = 14191;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("cannot initialize console_log");

    App::build()
        // minimal plugins necessary for timers + headless loop
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        // The NetworkingPlugin
        .add_plugin(NetworkingPlugin)
        .add_startup_system(startup.system())
        .add_system(send_packets.system())
        .init_resource::<NetworkReader>()
        .add_system(handle_packets.system())
        .run();
}

fn startup(mut net: ResMut<NetworkResource>) {
    let mut socket_address: SocketAddr = "127.0.0.1:0".parse().unwrap();
    socket_address.set_port(14191);

    log::info!("Starting client. Connected to {:?}", socket_address);
    net.connect(socket_address);
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        log::info!("PING");
        net.broadcast(Packet::from("PING"));
    }
}

#[derive(Default)]
struct NetworkReader {
    network_events: EventReader<NetworkEvent>,
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut state: ResMut<NetworkReader>,
    network_events: Res<Events<NetworkEvent>>,
) {
    for event in state.network_events.iter(&network_events) {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                log::info!("Got packet on [{}]: {}", handle, message);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            log::info!("Sent PONG");
                        }
                        Err(error) => {
                            log::info!("PONG send error: {}", error);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
