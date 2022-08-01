use crate::parse::message;
use walle_core::action::Action;

pub fn to_12(mut action: Action) -> Action {
    if let Some(segs) = action.params.get_mut("message") {
        message::segs_to_12(segs)
    }
    match action.action.as_str() {
        //todo
        _ => action,
    }
}

pub fn to_11(mut action: Action) -> Action {
    if let Some(segs) = action.params.get_mut("message") {
        message::segs_to_11(segs)
    }
    match action.action.as_str() {
        //todo
        _ => action,
    }
}
