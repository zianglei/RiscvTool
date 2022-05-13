#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod isa;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{
   State, Manager
};
use isa::IsaMapType;

type IsaStoreMap = HashMap<String, IsaMapType>;

struct IsaStore {
  isa: Mutex<IsaStoreMap>,
}

#[tauri::command]
fn get_isa_store(isa_store: State<'_, IsaStore>) -> String {
  return serde_json::to_string(&*isa_store.isa.lock().unwrap()).unwrap();
}

fn main() {

  tauri::Builder::default()
    .manage(IsaStore { isa: Default::default() })
    .setup(move |app| {
      let isa_dir = app
        .path_resolver()
        .resource_dir()
        .unwrap()
        .join("assets/isa/")
        .to_string_lossy()
        .to_string();
      
      let isa_map: State<IsaStore> = app.state();
      isa_map.isa.lock().unwrap().insert("riscv32".to_string(), isa::parse_isa(&isa_dir, "riscv32"));

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_isa_store,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
