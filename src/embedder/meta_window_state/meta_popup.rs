use serde::{Deserialize, Serialize};
use serde_json::json;
use smithay::utils::Logical;

use crate::{backend::Backend, flutter_engine::wayland_messages::MyPoint, state::State};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetaPopup {
    pub id: String,
    pub parent: String,
    pub position: MyPoint<i32, Logical>,
    pub surface_id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MetaPopupPatch {
    UpdatePosition {
        id: String,
        value: MyPoint<i32, Logical>,
    },
}

impl<BackendData: Backend + 'static> State<BackendData> {
    pub fn create_meta_popup(&mut self, meta_popup: MetaPopup) -> MetaPopup {
        self.meta_window_state
            .meta_popups
            .insert(meta_popup.id.clone(), meta_popup.clone());
        let platform_method_channel = &mut self.flutter_engine_mut().platform_method_channel;
        platform_method_channel.invoke_method(
            "meta_popup_created",
            Some(Box::new(json!(meta_popup))),
            None,
        );
        meta_popup
    }

    pub fn patch_meta_popup(&mut self, patch: MetaPopupPatch) {
        let platform_method_channel = &mut self.flutter_engine_mut().platform_method_channel;
        platform_method_channel.invoke_method(
            "meta_popup_patch",
            Some(Box::new(json!(patch))),
            None,
        );
    }
    pub fn remove_meta_popup(&mut self, meta_popup_id: &String) {
        self.meta_window_state.meta_popups.remove(meta_popup_id);
        let platform_method_channel = &mut self.flutter_engine_mut().platform_method_channel;
        platform_method_channel.invoke_method(
            "meta_popup_removed",
            Some(Box::new(json!({"id":meta_popup_id.clone()}))),
            None,
        );
    }
}
