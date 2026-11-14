mod app;
mod ui;
mod packet;
mod header;
mod config;
use crate::app::App;
use crate::packet::Paquete;

use std::env;
use pcap::Capture;

const NUMBER_OF_ARGS : usize = 2;

fn main() -> Result<(), String> {
    if std::env::args().len() != NUMBER_OF_ARGS {
        return Err(String::from("Wrong number of args"))
    }
    let Some(fichero) = env::args().nth(1) else {
        return Err(String::from("You must specify the name of the pcap file on the command line."))
    };
    let Ok(mut captura) = Capture::from_file(&fichero) else {
        return Err(format!("The file {} does not exist or does not contain a pcap capture.", fichero))
    };
    
    let mut paquetes : Vec<Paquete> = Vec::new();
    let mut primer_tiempo_paquetes: Option<i64> = None;
    
    while let Ok(packet) = captura.next_packet() {
        let datos: &[u8] = packet.data;
        let tiempo_actual: i64 = packet.header.ts.tv_sec as i64 * 1_000_00 + packet.header.ts.tv_usec as i64;
        let inicio: i64 = *primer_tiempo_paquetes.get_or_insert(tiempo_actual);
        let tiempo_relativo: f64 = (tiempo_actual - inicio) as f64 / 1_000_000.0;
        let paquete: Paquete = Paquete::new(datos, paquetes.len() + 1, tiempo_relativo.to_string());
        paquetes.push(paquete);
    }
        
    ratatui::run(|terminal| App::new(paquetes).run(terminal)).map_err(|e| e.to_string())

}