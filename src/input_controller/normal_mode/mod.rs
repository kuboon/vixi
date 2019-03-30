mod actions;
mod nouns;
mod verbs;

use self::actions::{Action, Actions};
use self::nouns::Nouns;
use self::verbs::Verbs;
use crate::devices::keyboard::KeyStroke;
use crate::input_controller::rpc::*;
use crate::input_controller::Response;

use failure::Error;
use xi_rpc::Peer;

#[derive(Default)]
pub struct Config {
    verbs: verbs::Config,
    nouns: nouns::Config,
    actions: actions::Config,
}

#[derive(Default)]
pub struct NormalMode {
    verbs: Verbs,
    nouns: Nouns,
    actions: Actions,
}

impl NormalMode {
    pub fn from_config(config_map: &Config) -> Result<Self, Error> {
        Ok(NormalMode {
            verbs: Verbs::from_config(&config_map.verbs)?,
            nouns: Nouns::from_config(&config_map.nouns)?,
            actions: Actions::from_config(&config_map.actions)?,
        })
    }

    pub fn handle_keystroke(&self, key: KeyStroke, view_id: &str, core: &dyn Peer) -> Response {
        let action = self.actions.get(key);
        if let Some(action) = action {
            return match action {
                Action::SwitchToInsertMode => Response::SwitchToInsertMode,
                Action::Exit => exit(view_id, core),
                Action::MoveUp => move_up(view_id, core),
                Action::MoveDown => move_down(view_id, core),
                Action::MoveLeft => move_left(view_id, core),
                Action::MoveRight => move_right(view_id, core),
                Action::PageUp => page_up(view_id, core),
                Action::PageDown => page_down(view_id, core),
            };
        }

        Response::Continue
    }
}