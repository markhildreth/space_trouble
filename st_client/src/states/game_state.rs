use crate::game_screen::GameScreen;
use crate::panels::Panel;
use crate::strings::get_action_text;
use crate::timing::{SpanStatus, TimeSpan};
use st_data::{ClientMessageProducer, GameMessage};
use st_device::Device;

fn calc_blocks(remaining_ms: u32, total_ms: u32) -> u8 {
    return (20 * remaining_ms / total_ms) as u8;
}

pub struct GameState<'a> {
    producer: ClientMessageProducer<'a>,
    panel: Panel,
    screen: GameScreen,
    directive_time_span: Option<TimeSpan>,
}

impl<'a> GameState<'a> {
    pub fn new(producer: ClientMessageProducer<'a>, panel: Panel, device: &mut Device) -> Self {
        let mut screen = GameScreen::new();
        screen.init(&mut device.lcd);
        GameState {
            producer,
            panel,
            screen,
            directive_time_span: None,
        }
    }

    pub fn update(&mut self, device: &mut Device) {
        let ms = device.ms();
        self.screen.update(&mut device.lcd);
        if let Some(span) = &self.directive_time_span {
            let status = span.status(ms);
            match status {
                SpanStatus::Ongoing {
                    remaining_ms,
                    total_ms,
                } => {
                    let blocks = calc_blocks(remaining_ms, total_ms);
                    self.screen.update_timer(blocks);
                }
                SpanStatus::Completed => {
                    self.screen.update_command_text(None, None);
                    self.screen.update_timer(0);
                    self.directive_time_span = None;
                }
            }
        }

        self.panel.update(&mut self.producer, device);
    }

    pub fn handle(&mut self, ms: u32, msg: GameMessage) {
        match msg {
            GameMessage::ShipDistanceUpdated(distance) => {
                self.screen.update_distance(distance);
            }
            GameMessage::HullHealthUpdated(health) => {
                self.screen.update_hull_health(health);
            }
            GameMessage::NewDirective(directive) => {
                let (text_1, text_2) = get_action_text(directive.action);
                self.screen.update_command_text(Some(text_1), Some(text_2));
                let blocks = calc_blocks(0, directive.expiration);
                self.screen.update_timer(blocks);
                self.directive_time_span = Some(TimeSpan::new(ms, directive.expiration as u32));
            }
            GameMessage::DirectiveCompleted => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}