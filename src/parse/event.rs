use super::message;
use crate::event::Event as V11Event;
use walle_core::{event::Event as V12Event, util::ValueMapExt};

fn detail_type(ty: &str) -> &'static str {
    match ty {
        "message" => "message_type",
        "notice" => "notice_type",
        "request" => "request_type",
        _ => "meta_event_type", // "meta" as default
    }
}

pub fn to_11(mut event: V12Event) -> V11Event {
    event
        .extra
        .insert(detail_type(&event.ty).to_string(), event.detail_type.into());
    event
        .extra
        .insert("sub_type".to_string(), event.sub_type.into());
    if let Some(r) = event.extra.get_mut("message") {
        message::segs_to_11(r);
    }
    V11Event {
        time: event.time as u64,
        self_id: event.self_id.parse().unwrap_or_default(),
        post_type: event.ty,
        content: event.extra,
    }
}

pub fn to_12(mut event: V11Event) -> V12Event {
    let detail_type = detail_type(&event.post_type);
    if let Some(r) = event.content.get_mut("message") {
        message::segs_to_12(r);
    }
    V12Event {
        id: "".to_string(), // todo
        implt: "Walle-V11".to_string(),
        platform: "OneBot V11".to_string(),
        self_id: event.self_id.to_string(),
        time: event.time as f64,
        detail_type: event
            .content
            .remove_downcast(detail_type)
            .unwrap_or_default(),
        ty: event.post_type,
        sub_type: event
            .content
            .remove_downcast("sub_type")
            .unwrap_or_default(),
        extra: event.content,
    }
}
