use crate::packet::OperacionArp::self;
use crate::packet::Paquete;
use crate::packet::PaqueteArp;
use crate::packet::PaqueteUdp;
use crate::packet::PaqueteIcmp;
use crate::packet::PaqueteTcp; 
use crate::packet::TipoPaquete;

pub struct Cabecera {
    pub titulo : String,
    pub campos : Vec<String>,
    pub desplegada : bool,
}

impl Cabecera {

    fn desplegar_cabecera(&mut self) {
        self.desplegada = true;
    }

    fn esconder_cabecera(&mut self) {
        self.desplegada = false;
    }

    fn construir_cabecera_frame(paquete: &Paquete) -> Cabecera {
        let titulo_cabecera_frame: String = format!("Frame {}: Packet, {} bytes on wire ({} bits), {} bytes captured ({} bits)", paquete.num, paquete.length, paquete.length*8, paquete.length, paquete.length*8);
        let mut campos: Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_frame,
            campos : campos,
            desplegada : false,
        }
    }

    fn construir_cabecera_ethernet(paquete: &Paquete) -> Cabecera {
        let titulo_cabecera_ethernet : String = format!("Ethernet II, Src: {} ({}), Dst: {} ({})", paquete.source, paquete.source, paquete.destination, paquete.destination);
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_ethernet,
            campos : campos,
            desplegada : false,
        }
    }

    fn construir_cabecera_arp(paquete: &PaqueteArp) -> Cabecera{
        let titulo_cabecera_arp : String = match paquete.operacion {
            OperacionArp::Desconocido => format!("Address Resolution Protocol (unknown)"),
            OperacionArp::Reply => format!("Address Resolution Protocol (reply)"),
            OperacionArp::Request => format!("Address Resolution Protocol (request)"),
        };
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_arp,
            campos : campos,
            desplegada : false,
        }
    }
}

pub struct CabecerasDePaquete{
    pub elementos : Vec<Cabecera>,
}

impl CabecerasDePaquete {
    pub fn new(paquete: &Paquete) -> CabecerasDePaquete{
        let mut elementos :Vec<Cabecera> = Vec::new();
        let cabecera_frame: Cabecera = Cabecera::construir_cabecera_frame(paquete);
        let cabecera_ethernet: Cabecera = Cabecera::construir_cabecera_ethernet(paquete); 
        elementos.push(cabecera_frame);
        elementos.push(cabecera_ethernet);
        match &paquete.tipo {
            TipoPaquete::Arp(datos_arp) => {
                elementos.push(Cabecera::construir_cabecera_arp( datos_arp));
            },
            /*
            TipoPaquete::Icmp(datos_icmp) => {
                Self::construir_cabecera_icmp(paquete, &mut elementos, datos_icmp);
            }
            TipoPaquete::Tcp(datos_tcp) => {
                Self::construir_cabecera_tcp(paquete, &mut elementos, datos_tcp);
            }
            TipoPaquete::Udp(datos_udp) => {
                Self::construir_cabecera_udp(paquete, &mut elementos, datos_udp);
            }
            */
            _ => {}
        }
        CabecerasDePaquete { 
            elementos,
        }
    }

    pub fn obtener_lineas(&self) -> Vec<String> {
        let mut lineas : Vec<String> = Vec::new();
        for cabecera in &self.elementos {
            let simbolo: &str = if cabecera.desplegada {"▼"} else {"▶"};
            lineas.push(format!("{} {}", simbolo, cabecera.titulo));
            if cabecera.desplegada {
                for campo in &cabecera.campos {
                    lineas.push(format!("\t{}", campo));
                }
            }
        }
        lineas
    }
}