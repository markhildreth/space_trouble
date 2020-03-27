use crate::game_screen::GameScreen;
use crate::strings::get_action_text;
use crate::timing::{SpanStatus, TimeSpan};
use crate::{Panel, LCD};
use st_data::time::*;
use st_data::{ClientMessageProducer, GameMessage};

fn calc_blocks(remaining: Duration, total: Duration) -> u8 {
    return (20 * remaining.as_millis() / total.as_millis()) as u8;
}

pub struct GameState<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    producer: ClientMessageProducer<'a>,
    panel: TPanel,
    screen: GameScreen<TLCD>,
    directive_time_span: Option<TimeSpan>,
}

impl<'a, TPanel, TLCD> GameState<'a, TPanel, TLCD>
where
    TPanel: Panel,
    TLCD: LCD,
{
    pub fn new(producer: ClientMessageProducer<'a>, panel: TPanel, lcd: TLCD) -> Self {
        let screen = GameScreen::new(lcd);
        GameState {
            producer,
            panel,
            screen,
            directive_time_span: None,
        }
    }

    pub fn update(&mut self, now: Instant) {
        self.screen.update();
        self.panel.update(&mut self.producer, now);

        if let Some(span) = &self.directive_time_span {
            let status = span.status(now);
            match status {
                SpanStatus::Ongoing { remaining, total } => {
                    let blocks = calc_blocks(remaining, total);
                    self.screen.update_timer(blocks);
                }
                SpanStatus::Completed => {
                    self.screen.update_command_text(None, None);
                    self.screen.update_timer(0);
                    self.directive_time_span = None;
                }
            }
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {
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
                let blocks = calc_blocks(Duration::from_millis(0), directive.time_limit);
                self.screen.update_timer(blocks);
                self.directive_time_span = Some(TimeSpan::new(now, directive.time_limit));
            }
            GameMessage::DirectiveCompleted => {
                self.screen.update_command_text(None, None);
                self.screen.update_timer(0);
                self.directive_time_span = None;
            }
        }
    }
}
