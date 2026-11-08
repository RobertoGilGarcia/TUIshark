use std::io;

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

#[derive(Debug, Default)] // definimos nuesta app, variable de control para salir si pulso 'q'
pub struct App {
    pub exit: bool,
    pub paquetes : Vec<Paquete>,
    pub region_actual : Regiones,
    pub estado_tabla_paquetes : TableState,
    pub estado_lista_cabeceras : ListState,
}

impl App { // implementar rasgos
    pub fn new(paquetes: Vec<Paquete>) -> Self {
        App {
            exit : false, 
            paquetes : paquetes,
            region_actual : Regiones::default(), // por defecto es la region de paquetes
            estado_tabla_paquetes : TableState::default(), // por defecto el seleccionado es None
            estado_lista_cabeceras : ListState::default(), // por defecto el seleccionado es None
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
            Some(indice) if indice <= self.paquetes.len() => indice + 1, // si no es el final
            Some(indice) => indice,
        };
        self.estado_tabla_paquetes.select(Some(siguiente))
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
        self.estado_tabla_paquetes.select(Some(siguiente));
    }

    pub fn cambiar_region(&mut self) {
        self.region_actual = match self.region_actual {
            Regiones::Paquetes => Regiones::Cabeceras,
            Regiones::Cabeceras => Regiones::Paquetes,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent){ // KeyCode tiene sus propios metodos para gestionar diferentes teclas
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.cambiar_region(),
            KeyCode::Down => {
                match self.region_actual {
                    Regiones::Paquetes => {self.down_paquetes();}
                    Regiones::Cabeceras => {/* nada de momento */}
                }
            }
            KeyCode::Up => {
                match self.region_actual {
                    Regiones::Paquetes => {self.up_paquetes();}
                    Regiones::Cabeceras => {/* nada de momento */}
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