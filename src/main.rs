use axum::Server;
use socketioxide::extract::{SocketRef, SocketIo, AckSender, Data};
use socketioxide::SocketIo;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};

pub fn handle_ping(socket: SocketRef, Data(data): Data<String>, ack: AckSender, signal_light: Arc<Mutex<SignalLight>>) {
    info!("Received event: {:?}", data);
    match data.as_str() {
        "S" => {
            if let Ok(mut light) = signal_light.lock() {
                light.enable();
            }
        },
        "P" => {
            if let Ok(mut light) = signal_light.lock() {
                light.disable();
            }
        },
        _ => {}
    }
    ack.send("pong").ok();
    socket.emit("pong", "pong").ok();
}

const PIN_SIGNAL_LIGHT: u8 = 21;

struct SignalLight {
    pin: OutputPin,
}

impl SignalLight {
    fn new() -> SignalLight {
        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(PIN_SIGNAL_LIGHT).unwrap().into_output();
        SignalLight { pin }
    }

    fn enable(&mut self) {
        self.pin.set_high();
    }

    fn disable(&mut self) {
        self.pin.set_low();
    }
}

impl Drop for SignalLight {
    fn drop(&mut self) {
        self.disable();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let signal_light = Arc::new(Mutex::new(SignalLight::new()));

    let (layer, io) = SocketIo::new_layer();

    let signal_light_clone = Arc::clone(&signal_light);
    io.ns("/control-station", move |socket: SocketRef| {
        info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
        let signal_light_clone = Arc::clone(&signal_light_clone);
        socket.on("ping", move |socket, data, ack| handle_ping(socket, data, ack, Arc::clone(&signal_light_clone)));
    });

    let app = axum::Router::new().layer(layer);

    info!("Starting server on port 5000");
    let server = Server::bind(&"127.0.0.1:5000".parse().unwrap()).serve(app.into_make_service());

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}
