use crate::states::{ClientState, GameState, StateUpdate};
use crate::{Components, ComponentsDef, ComponentsDefImpl, Panel, LCD};
use st_data::time::Instant;
use st_data::{ClientMessageProducer, GameMessage};

pub struct Client<'a, CD: ComponentsDef> {
    components: Components<'a, CD>,
    state: ClientState,
}

impl<'a, TPanel: Panel, TLCD: LCD> Client<'a, ComponentsDefImpl<TPanel, TLCD>> {
    pub fn new(producer: ClientMessageProducer<'a>, panel: TPanel, lcd: TLCD) -> Self {
        Client {
            components: Components::new(producer, panel, lcd),
            state: ClientState::GameState(GameState::new()),
        }
    }

    pub fn update(&mut self, now: Instant) {
        let result = match &mut self.state {
            ClientState::WaitingToStart(state) => state.update(&mut self.components, now),
            ClientState::GameState(state) => state.update(&mut self.components, now),
        };

        if let Some(state_update) = result {
            self.state = match state_update {
                StateUpdate::GameState => ClientState::GameState(GameState::new()),
            }
        }
    }

    pub fn handle(&mut self, now: Instant, msg: GameMessage) {
        match &mut self.state {
            ClientState::WaitingToStart(_state) => (),
            ClientState::GameState(state) => state.handle(now, msg),
        }
    }
}
