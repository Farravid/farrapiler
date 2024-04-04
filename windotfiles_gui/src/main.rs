#![windows_subsystem = "windows"] // This attribute hides the console window

use druid::widget::{Button, Flex, Label, CrossAxisAlignment, WidgetExt}; // Import WidgetExt
use druid::{AppLauncher, Env, LocalizedString, Widget, WindowDesc};
use std::process::Command as SysCommand;
use nfd::Response;

fn main() {
    // Create the main window
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Windotfiles GUI"))
        .window_size((400.0, 250.0));

    // Launch the application
    let data = ();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
        .expect("Failed to launch application");
}

// Build the UI
fn build_ui() -> impl Widget<()> {
    let button_python = Button::new("Launch Python Script")
        .on_click(|_, _: &mut (), _| {
            let _ = SysCommand::new("wt")
                .arg("python C:/Users/david/windotfiles/scripts/startup.py") // Change this to your Python script path
                .spawn()
                .expect("Failed to execute Python script");
        });

    let button_powershell = Button::new("Execute PowerShell Command")
        .on_click(|_, _: &mut (), _| {
            let _ = SysCommand::new("powershell")
                .arg("-Command")
                .arg("Write-Host 'Hello from PowerShell!'")
                .spawn()
                .expect("Failed to execute PowerShell command");
        });

    let button_file_explorer = Button::new("Select File")
        .on_click(|_, _: &mut (), _env: &Env| {
            if let Ok(Response::Okay(file_path)) = nfd::open_file_dialog(None, None) {
                println!("Selected file: {:?}", file_path);
                // Do something with the selected file path
            }
        });

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(Label::new("Welcome to Rust GUI App").with_text_size(20.0))
        .with_default_spacer()
        .with_child(button_python)
        .with_default_spacer()
        .with_child(button_powershell)
        .with_default_spacer()
        .with_child(button_file_explorer)
        .padding(20.0) // Now padding method is available
}
