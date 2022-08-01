use walle_core::segment::{Mention, MentionAll, MessageSegment, Segments};
use walle_core::util::{Value, ValueMapExt};
use walle_core::value_map;

pub fn to_11(mut seg: MessageSegment) -> MessageSegment {
    match seg.ty.as_str() {
        "mention" => MessageSegment {
            ty: "at".to_string(),
            data: value_map! {
                "qq": seg.data.remove_downcast::<String>("user_id").unwrap_or_default()
            },
        },
        "mention_all" => MessageSegment {
            ty: "at".to_string(),
            data: value_map! {
                "qq": "all"
            },
        },
        _ => seg,
    }
}

pub fn segs_to_11(segs: &mut Value) {
    if let Ok(s) = segs.clone().downcast::<Segments>() {
        *segs = s.into_iter().map(to_11).collect::<Vec<_>>().into();
    }
}

pub fn to_12(mut seg: MessageSegment) -> MessageSegment {
    match seg.ty.as_str() {
        "qq" => match seg
            .data
            .remove_downcast::<String>("qq")
            .unwrap_or_default()
            .as_str()
        {
            "all" => MentionAll {}.into(),
            user_id => Mention {
                user_id: user_id.to_string(),
            }
            .into(),
        },
        _ => seg,
    }
}

pub fn segs_to_12(segs: &mut Value) {
    if let Ok(s) = segs.clone().downcast::<Segments>() {
        *segs = s.into_iter().map(to_12).collect::<Vec<_>>().into();
    }
}
