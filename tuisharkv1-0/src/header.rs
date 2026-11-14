use crate::packet::OperacionArp::self;
use crate::packet::Paquete;
use crate::packet::PaqueteArp;
use crate::packet::PaqueteEthernet;
use crate::packet::PaqueteUdp;
use crate::packet::PaqueteIcmp;
use crate::packet::PaqueteTcp; 
use crate::packet::TipoPaquete;

pub enum ElementosCabecera {
    Titulo(usize), //titulo de la cabecera x seleccionada
    Campos(usize, usize), // campos x de la cabecera y seleccionada
}

#[derive(PartialEq)]
pub struct Cabecera {
    pub titulo : String,
    pub campos : Vec<String>,
    pub desplegada : bool,
}

impl Cabecera {

    pub fn alternar_despliegue(&mut self) {
        self.desplegada != self.desplegada;
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

    fn construir_cabecera_ipv4(paquete: &Paquete) -> Cabecera {
        let titulo_cabecera_ipv4 : String = format!("Internet Protocol Version 4, Src: {}, Dst: {}", paquete.source, paquete.destination);
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_ipv4,
            campos : campos,
            desplegada : false,
        }
    }

    fn construir_cabecera_icmp(paquete: &PaqueteIcmp) -> Cabecera {
        let titulo_cabecera_icmp : String = format!("Internet Control Message Protocol");
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_icmp,
            campos : campos,
            desplegada : false,
        }
    }

    pub fn construir_titulo_tcp(paquete_tcp: &PaqueteTcp) -> String{ // funcion muy tonta, pero es necesaria para escalarla despues
        format!("Transmission Control Protocol, Src Port: {}, Dst Port {}, {:?}", paquete_tcp.source_port, paquete_tcp.destination_port, paquete_tcp.flags)
    }

    pub fn construir_cabecera_tcp(paquete_tcp: &PaqueteTcp) -> Cabecera {
        let titulo_cabecera_tcp : String = Self::construir_titulo_tcp(paquete_tcp);
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_tcp,
            campos : campos,
            desplegada : false,
        }
    }

    pub fn construir_cabecera_udp(paquete_udp: &PaqueteUdp) -> Cabecera {
        let titulo_cabecera_udp = format!("User Datagram Protocol, Src port: {}, Dst Port: {}", paquete_udp.source_port, paquete_udp.destination_port);
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_udp,
            campos : campos,
            desplegada : false,
        }
    }

        pub fn construir_cabecera_data_udp(paquete_udp: &PaqueteUdp) -> Cabecera {
        let titulo_cabecera_data_udp = format!("Data ({} bytes)", paquete_udp.data);
        let mut campos : Vec<String> = Vec::new();
        //TEMPORAL
        for i in 0..16 {
            campos.push("prueba".to_string());
        }
        Cabecera {
            titulo : titulo_cabecera_data_udp,
            campos : campos,
            desplegada : false,
        }
    }


}

#[derive(PartialEq)]
pub struct CabecerasDePaquete{
    pub elementos : Vec<Cabecera>,
}

impl CabecerasDePaquete {
    pub fn new(paquete: &Paquete) -> CabecerasDePaquete{
        let mut cabeceras: Vec<Cabecera> = Vec::new();
        let cabecera_frame: Cabecera = Cabecera::construir_cabecera_frame(paquete);
        let cabecera_ethernet: Cabecera = Cabecera::construir_cabecera_ethernet(paquete); 
        cabeceras.push(cabecera_frame);
        cabeceras.push(cabecera_ethernet);
        match &paquete.tipo {
            TipoPaquete::Arp(datos_arp) => {
                cabeceras.push(Cabecera::construir_cabecera_arp( datos_arp));
            },
            
            TipoPaquete::Icmp(datos_icmp) => {
                cabeceras.push(Cabecera::construir_cabecera_ipv4(paquete));
                cabeceras.push(Cabecera::construir_cabecera_icmp(datos_icmp));
            }
            
            TipoPaquete::Tcp(datos_tcp) => {
                cabeceras.push(Cabecera::construir_cabecera_ipv4(paquete));
                cabeceras.push(Cabecera::construir_cabecera_tcp(datos_tcp));
            }
            
            TipoPaquete::Udp(datos_udp) => {
                cabeceras.push(Cabecera::construir_cabecera_ipv4(paquete));
                cabeceras.push(Cabecera::construir_cabecera_udp(datos_udp));
                cabeceras.push(Cabecera::construir_cabecera_data_udp(datos_udp));
            }

            _ => {}
        }
        CabecerasDePaquete { 
            elementos : cabeceras,
        }
    }

    pub fn elementos_visibles(&self) -> Vec<ElementosCabecera> {
        let mut elementos_visibles : Vec<ElementosCabecera> = Vec::new();
        for (indice_titulo_cabecera, cabecera) in self.elementos.iter().enumerate() {
            elementos_visibles.push(ElementosCabecera::Titulo(indice_titulo_cabecera));

            if cabecera.desplegada {
                for indice_campo in 0..cabecera.campos.len() {
                    elementos_visibles.push(ElementosCabecera::Campos(indice_titulo_cabecera, indice_campo));
                }
            }
        }
        elementos_visibles
    }
}