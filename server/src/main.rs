use bevy::{
    app::{App, EventReader, Events, ScheduleRunnerSettings},
    core::Time,
    ecs::prelude::*,
    MinimalPlugins,
};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};

use std::{net::SocketAddr, time::Duration};
use std::net::{Ipv4Addr, IpAddr};

const SERVER_PORT: u16 = 14191;

fn main() {
    simple_logger::SimpleLogger::from_env().init().expect("A logger was already initialized");
    
    App::build()
        // minimal plugins necessary for timers + headless loop
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        // The NetworkingPlugin
        .add_plugin(NetworkingPlugin)
        // Our networking
        .add_startup_system(startup.system())
        .init_resource::<NetworkReader>()
        .add_system(handle_packets.system())
        .run();
}

fn startup(mut net: ResMut<NetworkResource>) {
    log::info!("Starting server");
    net.listen(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 14191));
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
                    log::info!("Got a ping");
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
