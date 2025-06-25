use std::{error::Error, io, thread::sleep, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use midir::{MidiOutput, MidiOutputPort};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border::{self, ROUNDED},
    text::{Line, Text},
    widgets::{Block, Paragraph, Tabs, Widget},
};

const TRACKS_NUM: usize = 6;

#[derive(Default)]
pub struct TracksView {
    selected_track: usize,
}
impl TracksView {
    fn new(selected_track: usize) -> Self {
        TracksView { selected_track }
    }
    fn wrap(input: usize) -> usize {
        input % TRACKS_NUM
    }
}
impl Widget for TracksView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Length(20)].repeat(TRACKS_NUM))
            .split(area);
        for i in 0..TRACKS_NUM {
            Paragraph::new((i + 1).to_string())
                .block(Block::bordered().border_set(ROUNDED).border_style(
                    if self.selected_track == i {
                        Style::new().fg(Color::Red)
                    } else {
                        Style::reset()
                    },
                ))
                .render(layout[i], buf);
        }
    }
}

#[derive(Default)]
pub struct App {
    current_tab: usize,
    exit: bool,
    selected_track: usize,
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

        match self.current_tab {
            0 => frame.render_widget(TracksView::new(self.selected_track), layout[1]),
            1 => frame.render_widget(
                Paragraph::new("uhhhh").block(Block::bordered().border_set(ROUNDED)),
                layout[1],
            ),
            2 => frame.render_widget(
                Paragraph::new("sick af preview").block(Block::bordered().border_set(ROUNDED)),
                layout[1],
            ),
            _ => frame.render_widget(Paragraph::new("what the fuck did you do"), layout[1]),
        }
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
            KeyCode::Char('H') => {
                self.selected_track = TracksView::wrap(self.selected_track + TRACKS_NUM - 1)
            }
            KeyCode::Char('L') => self.selected_track = TracksView::wrap(self.selected_track + 1),
            KeyCode::Char(' ') => play_sound().unwrap(),
            KeyCode::Char('q') => self.exit(),
            _ => {
                println!()
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }
}
impl Widget for App {
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

        play_note(note_to_midi_value("D5").unwrap(), 1);
        play_note(note_to_midi_value("D5").unwrap(), 1);
        play_note(note_to_midi_value("D6").unwrap(), 1);
        play_note(0, 1);
        play_note(note_to_midi_value("A5").unwrap(), 1);
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

fn note_to_midi_value(note_name: &str) -> Option<u8> {
    let mut chars = note_name.chars().peekable();
    let note_char = chars.next()?;

    let mut is_sharp = false;
    let mut is_flat = false;

    if let Some(&next_char) = chars.peek() {
        if next_char == '#' {
            is_sharp = true;
            chars.next();
        } else if next_char == 'b' {
            is_flat = true;
            chars.next();
        }
    }

    let octave_char = chars.next()?;
    let octave: i8 = octave_char.to_digit(10)?.try_into().ok()?;

    let mut midi_value: i8;

    match note_char.to_ascii_uppercase() {
        'C' => midi_value = 0,
        'D' => midi_value = 2,
        'E' => midi_value = 4,
        'F' => midi_value = 5,
        'G' => midi_value = 7,
        'A' => midi_value = 9,
        'B' => midi_value = 11,
        _ => return None,
    }

    midi_value += (octave + 1) * 12;

    if is_sharp {
        midi_value += 1;
    } else if is_flat {
        midi_value -= 1;
    }

    if midi_value >= 0 {
        Some(midi_value as u8)
    } else {
        None
    }
}
