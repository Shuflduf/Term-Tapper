use std::{error::Error, io, thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use midir::{MidiOutput, MidiOutputPort};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border::{self, ROUNDED},
    text::{Line, Text},
    widgets::{Block, Paragraph, Tabs, Widget},
};

#[derive(Default)]
pub struct App {
    current_tab: usize,
    exit: bool,
}
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        // frame.render_widget(self, frame.area());
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Min(0)])
            .split(frame.area());

        frame.render_widget(
            Tabs::new(vec!["Tracks", "Composition", "Preview"]).select(self.current_tab),
            layout[0],
        );
        frame.render_widget(
            match self.current_tab {
                0 => Paragraph::new("tracks go here mayb")
                    .block(Block::bordered().border_set(ROUNDED)),
                1 => Paragraph::new("uhhhh").block(Block::bordered().border_set(ROUNDED)),
                2 => Paragraph::new("sick af preview").block(Block::bordered().border_set(ROUNDED)),
                _ => Paragraph::new("what the fuck did you do"),
            },
            layout[1],
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        // if key_event.code == KeyCode::Char('q') {
        //     self.exit();
        // }
        const SHIFT_NUM_KEYS: [char; 3] = ['!', '@', '#'];
        match key_event.code {
            KeyCode::Char(c) if SHIFT_NUM_KEYS.contains(&c) => {
                let index = SHIFT_NUM_KEYS.iter().position(|&x| x == c);
                self.current_tab = index.unwrap()
            }
            KeyCode::Char(' ') => play_sound().unwrap(),

            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }
}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Counter Test".bold());
        let block = Block::bordered()
            .title(Line::from("LEFT").left_aligned())
            .title(title.centered())
            .border_set(border::ROUNDED);

        let counter_text = Text::from(vec![Line::from(vec!["Value: ".into()])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn play_sound() -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1.. => &out_ports[0],
    };

    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        play_note(74, 1);
        play_note(74, 1);
        play_note(86, 1);
        play_note(0, 1);
        play_note(81, 1);
    }
    sleep(Duration::from_millis(150));
    Ok(())
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
