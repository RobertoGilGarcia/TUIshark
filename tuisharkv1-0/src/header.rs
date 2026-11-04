use crate::packet::{OperacionArp::self, Paquete, PaqueteArp, PaqueteUdp, PaqueteIcmp, PaqueteTcp, TipoPaquete};

pub struct Cabeceras{
    pub elementos : Vec<String>,
}

impl Cabeceras {
    pub fn new(paquete: &Paquete) -> Cabeceras{
        let mut elementos :Vec<String> = Vec::new();
        let numero_paquete = &paquete.num;
        let longitud_paquete = &paquete.length;
        elementos.push(format!("Frame {}: Packet, {} bytes on wire ({} bits), {} bytes captured ({} bits)", numero_paquete, longitud_paquete, longitud_paquete*8, longitud_paquete, longitud_paquete*8));
        match &paquete.tipo {
            TipoPaquete::Arp(datos_arp) => {
                Self::construir_cabecera_arp(paquete, &mut elementos, datos_arp);
            },
            TipoPaquete::Icmp(datos_icmp) => {
                Self::construir_cabecera_icmp(paquete, &mut elementos, datos_icmp);
            }
            TipoPaquete::Tcp(datos_tcp) => {
                Self::construir_cabecera_tcp(paquete, &mut elementos, datos_tcp);
            }
            TipoPaquete::Udp(datos_udp) => {
                Self::construir_cabecera_udp(paquete, &mut elementos, datos_udp);
            }
            _ => {}
        }
        Cabeceras { 
            elementos,
        }
    }

    pub fn construir_cabecera_udp(paquete: &Paquete, elementos: &mut Vec<String>, datos_udp: &PaqueteUdp) {
        elementos.push(format!("Internet Protocol Version 4, Src: {}, Dst {}", paquete.source, paquete.destination));
        elementos.push(format!("User Datagram Protocol, Src Port: {}, Dst Port: {}", datos_udp.source_port, datos_udp.destination_port));
        elementos.push(format!("Data ({} bytes)", datos_udp.data));
    }

    pub fn construir_cabecera_tcp(paquete: &Paquete, elementos: &mut Vec<String>, datos_tcp: &PaqueteTcp) {
        elementos.push(format!("Internet Protocol Version 4, Src: {}, Dst {}", paquete.source, paquete.destination));
        elementos.push(format!("Transmission Control Protocol, Src Port: {}, Dst Port; {}, Seq: {}", datos_tcp.source_port, datos_tcp.destination_port, datos_tcp.sequence));
    }

    pub fn construir_cabecera_icmp(paquete: &Paquete, elementos: &mut Vec<String>, datos_icmp: &PaqueteIcmp) {
        elementos.push(format!("Internet Protocol Version 4, Src: {}, Dst {}", paquete.source, paquete.destination));
        elementos.push(format!("Internet Control Message Protocol"));
    }

    pub fn construir_cabecera_arp(paquete: &Paquete, elementos: &mut Vec<String>, datos_arp: &PaqueteArp) {
        let destino: &str = match paquete.destination.as_str() {
            "ff:ff:ff:ff:ff:ff" => {"Broadcast"},
            _ => {paquete.destination.as_str()},
        };
        elementos.push(format!("Ethernet II, Src: {} ({}), Dst: {} ({})", paquete.source, paquete.source, destino, paquete.destination));
        let operacion = match datos_arp.operacion {
            OperacionArp::Desconocido => {"Desconocido"},
            OperacionArp::Reply => {"reply"},
            OperacionArp::Request => {"request"},
        };
        elementos.push(format!("Address Resolution Protocol ({})", operacion));
    }
}