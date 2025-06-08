use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::cell::RefCell;

    use crate::{
        browser_state::{Export, Import},
        particles::{model::Model, simulation::SimulationParams},
    };
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

    #[wasm_bindgen]
    extern "C" {
        fn wasm_get_state();
        fn wasm_set_state(state: JsValue);
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct State {
        #[serde(flatten)]
        params: SimulationParams,

        #[serde(flatten)]
        model: Model,
    }

    pub fn import(_trigger: Trigger<Import>) {
        wasm_get_state();
    }

    thread_local! {
        static PENDING_IMPORT: RefCell<Option<State>> = RefCell::new(None);
    }

    #[wasm_bindgen]
    pub fn import_settings_from_js(value: JsValue) {
        match serde_wasm_bindgen::from_value::<State>(value) {
            Ok(state) => {
                PENDING_IMPORT.with_borrow_mut(|cell| {
                    *cell = Some(state);
                });
            }
            Err(err) => {
                tracing::warn!(?err, "failed to parse state from url");
            }
        }
    }

    pub fn async_import(mut commands: Commands) {
        let state = PENDING_IMPORT.with_borrow_mut(|cell| cell.take());

        if let Some(state) = state {
            commands.insert_resource(state.params);
            commands.insert_resource(state.model);
        }
    }

    pub fn export(trigger: Trigger<Export>, params: Res<SimulationParams>, model: Res<Model>) {
        let state = State {
            params: *params,
            model: *model,
        };

        wasm_set_state(serde_wasm_bindgen::to_value(&state).unwrap());
    }
}

mod native {
    use crate::browser_state::{Export, Import};
    use bevy::prelude::*;

    pub fn import(_: Trigger<Import>) {
        tracing::warn!("import not supported on native");
    }

    pub fn export(_: Trigger<Export>) {
        tracing::warn!("export not supported on native");
    }
}

pub struct BrowserStatePlugin;

impl Plugin for BrowserStatePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        app.add_systems(PreUpdate, wasm::async_import)
            .add_observer(wasm::import)
            .add_observer(wasm::export);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_observer(native::import)
            .add_observer(native::export);
    }
}

#[derive(Debug, Default, Event, Copy, Clone)]
pub struct Export;

#[derive(Debug, Default, Event, Copy, Clone)]
pub struct Import;
