use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Tabs, Widget},
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
            Tabs::new(vec!["Gaming 1", "Gaming 2", "Gaming 3"]).select(self.current_tab),
            layout[0],
        );
        frame.render_widget(
            Paragraph::new("pookie").block(Block::new().borders(Borders::ALL)),
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
        match key_event.code {
            // KeyCode::Char(c @ '1'..='3') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            //     self.current_tab = (c.to_digit(10).unwrap() as usize) - 1
            // }
            KeyCode::Char(c) if ['!', '@', '#'].contains(&c) => {
                let index = ['!', '@', '#'].iter().position(|&x| x == c);
                self.current_tab = index.unwrap()
            }

            KeyCode::Char('q') => self.exit(),
            _ => {
                println!("{:?}", key_event)
            }
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

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
