////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// 
///  Este módulo gestiona los diferentes paquetes de cada protocolo
/// 
///  Se encarga de construir un "Paquete" el cual contiene dentro los datos y estructuras de los diferentes protocolos
///  encapsulados unos dentro de otros
/// 
///  Utiliza el crate pnet para poder hacer uso de los métodos que nos brinda, pero teniendo una estructura propia para controlar
///  lo que se quiere que se vea, de manera personalizada
/// 
///  Si el paquete que se tiene que construir es un paquete TCP, este se construye en cascada, primero
///  se construye un "Paquete" y en el campo "tipo" se introduce un PaqueteTcp con los campos característicos del protocolo TCP
/// 
///  Funciona igual en los protocolos ICMP, UDP, TCP y ARP, todos se construyen en cascada, pero al final, tenemos un Paquete, el cual tiene
///  campos comunes y el campo tipo, que es un tipo de paquete específico de cada protocolo
/// 
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


use pnet::packet::arp::ArpPacket;
use pnet::packet::arp::ArpOperations;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpFlags;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;


#[derive(Debug, Default)]
pub enum OperacionArp {
    Request,
    Reply,
    #[default]
    Desconocido,
}

#[derive(Debug, Default)]
pub enum TipoPaquete {
    Arp(PaqueteArp),
    Icmp(PaqueteIcmp),
    Tcp(PaqueteTcp),
    Udp(PaqueteUdp),
    #[default]
    Desconocido,
    // en caso de necesitar mas protocolos se pueden añadir al tipo enumerado
}

#[derive(Debug, Default)]
pub struct PaqueteIcmp {
    pub icmp_type : u8,
    pub icmp_code : u8,
}

#[derive(Debug, Default)]
pub struct PaqueteArp {
    pub sender_ip : String,
    pub target_ip : String,
    pub sender_mac : String,
    pub target_mac : String,
    pub operacion : OperacionArp,
}

#[derive(Debug, Default)]
pub struct PaqueteTcp {
    pub source_port : u16,
    pub destination_port : u16,
    pub flags : String,
    pub sequence : u32,
}

#[derive(Debug, Default)]
pub struct PaqueteUdp {
    pub source_port : u16,
    pub destination_port : u16,
    pub data : usize,
}

#[derive(Debug, Default)]
pub struct PaqueteIpv4 {
    pub source : String,
    pub destination : String,
    pub protocol : String,
}

#[derive(Debug, Default)]
pub struct PaqueteEthernet {
    pub source : String,
    pub destination : String,
    pub tipo : u16,
}

#[derive(Debug, Default)]
pub struct Paquete {
    pub num : String,
    pub time : String,
    pub source : String,
    pub destination : String,
    pub protocol : String,
    pub length : usize,
    pub info : String,

    pub ethernet: PaqueteEthernet,
    pub ipv4: Option<PaqueteIpv4>, //option debido a que puede o no trabajar con IP, ARP sería None.
    pub tipo : TipoPaquete,
}

impl Paquete {
    pub fn new(datos:&[u8], num_paquete: usize, tiempo_relativo: String ) -> Paquete {
        let ethernet_packet = EthernetPacket::new(datos).unwrap();
        let datos_ethernet : PaqueteEthernet = PaqueteEthernet {
            source : ethernet_packet.get_source().to_string(),
            destination : ethernet_packet.get_destination().to_string(),
            tipo : ethernet_packet.get_ethertype().0, // codigo de cada protocolo
        };
        match ethernet_packet.get_ethertype() {
            EtherTypes::Arp => {
                let (tipo, info) = Self::construir_paquete_arp(&ethernet_packet);
                return Paquete { 
                    num: num_paquete.to_string(),
                    time: tiempo_relativo,
                    source: ethernet_packet.get_source().to_string(),
                    destination: ethernet_packet.get_destination().to_string(),
                    protocol: "ARP".to_string(), 
                    length: datos.len(),
                    info,
                    ethernet: datos_ethernet,
                    ipv4 : None,
                    tipo,
                }
            }
            EtherTypes::Ipv4 => {
                let paquete_ipv4 = Ipv4Packet::new(ethernet_packet.payload()).unwrap();
                let info_ipv4 = PaqueteIpv4 {
                    source : paquete_ipv4.get_source().to_string(),
                    destination : paquete_ipv4.get_destination().to_string(),
                    protocol : paquete_ipv4.get_next_level_protocol().to_string(),
                };
                let (tipo, protocol, info, info_ipv4) = match paquete_ipv4.get_next_level_protocol() {
                    IpNextHeaderProtocols::Icmp => {
                        let paquete_icmp: PaqueteIcmp = Self::construir_paquete_icmp(&paquete_ipv4);
                        let info: String = format!("Type={} Code={}", paquete_icmp.icmp_type, paquete_icmp.icmp_code);
                        (TipoPaquete::Icmp(paquete_icmp), "ICMP".to_string(), info, info_ipv4)
                    }
                    IpNextHeaderProtocols::Tcp => {
                        let paquete_tcp: PaqueteTcp = Self::construir_paquete_tcp(&paquete_ipv4);
                        let info: String = format!("{} -> {} [{}] Seq={}", paquete_tcp.source_port, paquete_tcp.destination_port, paquete_tcp.flags.trim(), paquete_tcp.sequence);
                        (TipoPaquete::Tcp(paquete_tcp), "TCP".to_string(), info, info_ipv4)
                    }
                    IpNextHeaderProtocols::Udp => {
                        let paquete_udp:PaqueteUdp = Self::construir_paquete_udp(&paquete_ipv4);
                        let info: String = format!("{} -> {}", paquete_udp.source_port, paquete_udp.destination_port);
                        (TipoPaquete::Udp(paquete_udp), "UDP".to_string(), info, info_ipv4)
                    }
                    _ => {(TipoPaquete::Desconocido, "Ipv4".to_string(), "Unsopported Protocol".to_string(), PaqueteIpv4::default())}
                };
                return Paquete {
                    num : num_paquete.to_string(),
                    time : tiempo_relativo,
                    source : paquete_ipv4.get_source().to_string(),
                    destination : paquete_ipv4.get_destination().to_string(),
                    protocol,
                    length : datos.len(),
                    info,
                    ethernet : datos_ethernet,
                    ipv4 : Some(info_ipv4),
                    tipo,
                }
            }
            _ => {return Paquete::default();} // en caso de necesitar mas protocolos se añaden aqui para construir los diferentes paquetes de cada protocolo
            // por ejemplo, si queremos añadir OSPF pues se añade otro caso al match
        }
    }

    fn construir_paquete_udp(paquete_ipv4: &Ipv4Packet) -> PaqueteUdp {
        let paquete_udp: UdpPacket = UdpPacket::new(paquete_ipv4.payload()).unwrap();
        PaqueteUdp { 
            source_port: paquete_udp.get_source(),
            destination_port: paquete_udp.get_destination(),
            data : paquete_udp.payload().len(),
        }

    }

    fn construir_paquete_tcp(paquete_ipv4: &Ipv4Packet) -> PaqueteTcp {
        let paquete_tcp :TcpPacket = TcpPacket::new(paquete_ipv4.payload()).unwrap();
        let mut flags = String::from("");
        let flag = paquete_tcp.get_flags();
        if (flag & TcpFlags::URG) != 0 {
            flags.push_str(" URG");
        }
        if (flag & TcpFlags::ACK) != 0 {
            flags.push_str(" ACK");
        }
        if (flag & TcpFlags::PSH) != 0 {
            flags.push_str(" PSH");
        }
        if (flag & TcpFlags::RST) != 0 {
            flags.push_str(" RST");
        }
        if (flag & TcpFlags::SYN) != 0 {
            flags.push_str(" SYN");
        }
        if (flag & TcpFlags::FIN) != 0 {
            flags.push_str(" FIN");
        }
        PaqueteTcp {
            source_port: paquete_tcp.get_source(),
            destination_port : paquete_tcp.get_destination(),
            flags,
            sequence : paquete_tcp.get_sequence(),
        }

    }

    fn construir_paquete_icmp(paquete_ipv4: &Ipv4Packet) -> PaqueteIcmp {
        let paquete_icmp = IcmpPacket::new(paquete_ipv4.payload()).unwrap();
        PaqueteIcmp {
            icmp_type : paquete_icmp.get_icmp_type().0,
            icmp_code : paquete_icmp.get_icmp_code().0,
        }
    }

    fn construir_paquete_arp(paquete_ethernet : &EthernetPacket) -> (TipoPaquete, String) {
        let paquete_arp = ArpPacket::new(paquete_ethernet.payload()).unwrap();
        let operacion: OperacionArp = match paquete_arp.get_operation() {
            ArpOperations::Request => {OperacionArp::Request},
            ArpOperations::Reply => {OperacionArp::Reply},
            _ => {OperacionArp::Desconocido},
        };
        let datos_arp: PaqueteArp = PaqueteArp {
            sender_ip: paquete_arp.get_sender_proto_addr().to_string(),
            target_ip: paquete_arp.get_target_proto_addr().to_string(),
            sender_mac: paquete_arp.get_sender_hw_addr().to_string(),
            target_mac: paquete_arp.get_target_hw_addr().to_string(),
            operacion,
        };
        let info = match paquete_arp.get_operation() {
            ArpOperations::Request => {
                format!("Who has {}? Tell {}", datos_arp.target_ip, datos_arp.sender_ip)
            }
            ArpOperations::Reply => {
                format!("{} is at {}", datos_arp.sender_ip, datos_arp.sender_mac)
            }
            _ => {return (TipoPaquete::Arp(datos_arp), "Unknown ARP Operation".to_string());}
        };
        (TipoPaquete::Arp(datos_arp), info)
    }
}