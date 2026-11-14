use std::io;

use crate::header::CabecerasDePaquete;
use crate::packet::Paquete;
use crate::ui::Regiones;

use crossterm::event::self;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::widgets::ListState;
use ratatui::widgets::TableState;

#[derive(Default)] // definimos nuesta app, variable de control para salir si pulso 'q'
pub struct App {
    pub exit: bool,
    pub paquetes : Vec<Paquete>,
    pub region_actual : Regiones,
    pub estado_tabla_paquetes : TableState,
    pub estado_lista_cabeceras : ListState,
    pub cabeceras_de_paquete_seleccionado : Option<CabecerasDePaquete>,
}

impl App { // implementar rasgos
    pub fn new(paquetes: Vec<Paquete>) -> App {
        App {
            exit : false, 
            paquetes : paquetes,
            region_actual : Regiones::default(), // por defecto es la region de paquetes
            estado_tabla_paquetes : TableState::default(), // por defecto el seleccionado es None
            estado_lista_cabeceras : ListState::default(), // por defecto el seleccionado es None
            cabeceras_de_paquete_seleccionado : None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?; // funcion lambda que recibe un frame, como parametro se llama a draw() para crear el frame
            self.handle_events()?; // gestiono diferentes eventos
        }
        Ok(())
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn draw(&mut self, frame: &mut Frame) -> () { // dibujar toda mi app en todo el area de la terminal (la que hay disponible)
        frame.render_widget(self, frame.area());
    }

    pub fn down_paquetes(&mut self) {
        if self.paquetes.is_empty() {
            return;
        }
        let siguiente: usize = match self.estado_tabla_paquetes.selected() {
            None => 0,
            Some(indice) if indice + 1 < self.paquetes.len() => indice + 1,
            Some(indice) => indice,
        };
        if self.estado_tabla_paquetes.selected() != Some(siguiente) {
            self.estado_tabla_paquetes.select(Some(siguiente));
            self.actualizar_cabeceras_de_paquete_seleccionado();
        }
    }

    pub fn up_paquetes(&mut self) {
        if self.paquetes.is_empty() {
            return;
        }
        let siguiente: usize = match self.estado_tabla_paquetes.selected() {
            None => 0,
            Some(indice) if indice == 0 => indice,
            Some(indice) => indice - 1,
        };
        if self.estado_tabla_paquetes.selected() != Some(siguiente) {
            self.estado_tabla_paquetes.select(Some(siguiente));
            self.actualizar_cabeceras_de_paquete_seleccionado();
        }
    }

    pub fn cambiar_region(&mut self) {
        self.region_actual = match self.region_actual {
            Regiones::Paquetes => Regiones::Cabeceras,
            Regiones::Cabeceras => Regiones::Paquetes,
        }
    }

    pub fn actualizar_cabeceras_de_paquete_seleccionado(&mut self) {
        if let Some(indice) = self.estado_tabla_paquetes.selected() {
            if let Some(paquete) = self.paquetes.get(indice) {
                self.cabeceras_de_paquete_seleccionado = Some(CabecerasDePaquete::new(paquete));
                self.estado_lista_cabeceras.select(Some(0));
            }
        }
    }

    pub fn up_cabeceras(&mut self) {
        if self.cabeceras_de_paquete_seleccionado == None {
            return;
        }
        if let Some(cabeceras_seleccionadas) = &self.cabeceras_de_paquete_seleccionado {
            let total = cabeceras_seleccionadas.elementos.len();
            if total == 0 {
                return;
            }
            let siguiente = match self.estado_lista_cabeceras.selected() {
                None => 0,
                Some(0) => 0,
                Some(indice) => indice - 1,
            };
            self.estado_lista_cabeceras.select(Some(siguiente));
        }
    }

    pub fn down_cabeceras(&mut self) {
        if let Some(cabeceras_seleccionadas) = &self.cabeceras_de_paquete_seleccionado {
            let total = cabeceras_seleccionadas.elementos.len();
            if total == 0 {
                return;
            }
            let siguiente = match self.estado_lista_cabeceras.selected() {
                None => 0,
                Some(indice) if indice + 1 < total => indice + 1,
                Some(indice) => indice  
            };
            self.estado_lista_cabeceras.select(Some(siguiente));
        }
    }

    pub fn alternar_cabecera(&mut self) {
        if let Some(cabeceras) = &mut self.cabeceras_de_paquete_seleccionado {
            if let Some(indice) = self.estado_lista_cabeceras.selected() {
                if let Some(cabecera) = cabeceras.elementos.get_mut(indice) {
                    cabecera.desplegada = !cabecera.desplegada;
                }
            }
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent){ // KeyCode tiene sus propios metodos para gestionar diferentes teclas
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.cambiar_region(),
            KeyCode::Down => {
                match self.region_actual {
                    Regiones::Paquetes => {self.down_paquetes();}
                    Regiones::Cabeceras => {self.down_cabeceras();}
                }
            }
            KeyCode::Up => {
                match self.region_actual {
                    Regiones::Paquetes => {self.up_paquetes();}
                    Regiones::Cabeceras => {self.up_cabeceras();}
                }
            }
            KeyCode::Enter => {
                match self.region_actual {
                    Regiones::Paquetes => {/* no hacemos nada */}
                    Regiones::Cabeceras => {self.alternar_cabecera();}
                }
            }
            _ => { /* para cualquier otra tecla no hacemos nada */}
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? { // leo el evento que pase
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => { // de momento solo contemplo que se pulse una tecla
                self.handle_key_event(key_event) // le paso el evento de tecla al majeador de eventos de tecla
            }
            _ => {} // en caso de que se necesite otro evento se añaden opciones al match, por ejemplo Event::Mouse()
        }
        Ok(())
    }
}