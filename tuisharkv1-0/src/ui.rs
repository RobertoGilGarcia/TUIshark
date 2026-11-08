use std::rc::Rc;


use ratatui::buffer::Buffer;
use ratatui::style::Color; 
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Cell;
use ratatui::widgets::List;
use ratatui::widgets::Row;
use ratatui::widgets::StatefulWidget;
use ratatui::widgets::Table;
use ratatui::widgets::Widget;


use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::header::CabecerasDePaquete;
use crate::app::App;

const REGION_PAQUETES : Regiones = Regiones::Paquetes;
const REGION_CABECERAS : Regiones = Regiones::Cabeceras;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Regiones {
    #[default]
    Paquetes,
    Cabeceras,
}

impl App {
    pub fn dibujar_paquetes(&mut self, area_bloque: Rect , buf:&mut Buffer) {
        let banner = Row::new(vec![
            "No.",
            "Time",
            "Source",
            "Destination",
            "Protocol",
            "Length",
            "Info",
        ]);

        let mut filas_paquetes : Vec<Row> = Vec::new();
        for (_indice, paquete) in self.paquetes.iter().enumerate() {
            let fila = Row::new(vec![
                Cell::from(paquete.num.to_string()),
                Cell::from(paquete.time.as_str()),
                Cell::from(paquete.source.as_str()),
                Cell::from(paquete.destination.as_str()),
                Cell::from(paquete.protocol.as_str()),
                Cell::from(paquete.length.to_string()),
                Cell::from(paquete.info.as_str()),
            ]);
            filas_paquetes.push(fila);
        }
        
        let tabla = Table::new(
            filas_paquetes,
            [
                Constraint::Length(5),
                Constraint::Length(10),
                Constraint::Length(18),
                Constraint::Length(18),
                Constraint::Length(10),
                Constraint::Length(8),
                Constraint::Min(10),
            ],
        ).header(banner).row_highlight_style(Style::default().bg(Color::Red));
        StatefulWidget::render(tabla, area_bloque, buf, &mut self.estado_tabla_paquetes);
    }

    pub fn dibujar_cabeceras(&mut self, area_bloque: Rect, buf: &mut Buffer) {
        if let Some(indice_paquete_seleccionado) = self.estado_tabla_paquetes.selected() {
            let paquete_seleccionado = &self.paquetes[indice_paquete_seleccionado];
            let cabeceras_paquete: CabecerasDePaquete = CabecerasDePaquete::new(&paquete_seleccionado);
            let lista = List::new(CabecerasDePaquete::obtener_lineas(&cabeceras_paquete));
            StatefulWidget::render(lista, area_bloque, buf, &mut self.estado_lista_cabeceras);
        }
    }

    
    pub fn dibujar_datos_en_bloque(&mut self, area_bloque: Rect, buf: &mut Buffer, region: Regiones) {
        match region {
            Regiones::Paquetes => {self.dibujar_paquetes(area_bloque, buf);},
            Regiones::Cabeceras => {self.dibujar_cabeceras(area_bloque, buf);},
        }
    }

    pub fn dibujar_bloques(&mut self, regiones: &Rc<[Rect]>, buf: &mut Buffer) {
        //PAQUETES
        let color_borde_bloque_arriba = if self.region_actual == Regiones::Paquetes {Color::Red} else {Color::default()};
        let bloque_arriba = Block::bordered().border_style(Style::default().fg(color_borde_bloque_arriba)); // Se le puede añadir .title("PAQUETES"), pero en wireshark no hay ese nombre, son solo selectores
        let area_bloque_arriba = bloque_arriba.inner(regiones[0]);
        bloque_arriba.render(regiones[0], buf);
        self.dibujar_datos_en_bloque(area_bloque_arriba, buf, REGION_PAQUETES);

        //CABECERAS
        let color_borde_bloque_abajo = if self.region_actual == Regiones::Cabeceras {Color::Red} else {Color::default()};
        let bloque_abajo = Block::bordered().title("CABECERAS").border_style(Style::default().fg(color_borde_bloque_abajo)); // titulo orientativo
        let area_bloque_abajo = bloque_abajo.inner(regiones[1]);
        bloque_abajo.render(regiones[1], buf);
        self.dibujar_datos_en_bloque(area_bloque_abajo, buf, REGION_CABECERAS);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()// definimos layout, lo cual hace que se establezcan las normas para dividir en regiones el area de la terminal
                            .direction(Direction::Vertical)
                            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)]);
        let regiones = layout.split(area); // dividimos el layout definido en regiones con split, que sigue las normas del layout para dibujar, devuelve un "array" de regiones
        self.dibujar_bloques(&regiones, buf);
    }
}