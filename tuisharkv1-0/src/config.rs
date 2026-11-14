use serde::Serialize; 
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ColoresProtocolos {
    color_letra: String,
    color_fondo: String,
}

#[derive(Debug, Deserialize)]
pub struct Configuracion {
    pub colores_protocolos : HashMap<String, ColoresProtocolos>
}