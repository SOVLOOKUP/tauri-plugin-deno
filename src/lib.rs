use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

#[derive(Default)]
struct MyState {}

#[tauri::command]
// this will be accessible with `invoke('plugin:deno|load')`.
// 添加脚本
fn load<R: Runtime>(_app: AppHandle<R>, state: State<'_, MyState>) {}

#[tauri::command]
// this will be accessible with `invoke('plugin:deno|unload')`.
// 卸载脚本
fn unload<R: Runtime>(_app: AppHandle<R>, state: State<'_, MyState>) {}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("deno")
        .invoke_handler(tauri::generate_handler![load, unload])
        .setup(|app_handle| {
            // setup plugin specific state here
            app_handle.manage(MyState::default());

            // todo 将 app_handle.emit 和 app_handle.listen 通过 ops 透传至 deno 脚本
            Ok(())
        })
        .build()
}
