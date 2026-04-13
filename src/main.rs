slint::include_modules!();
use slint::Image;
use slint::winit_030::WinitWindowAccessor;

fn main() -> Result<(), slint::PlatformError> {
    unsafe { 
        std::env::set_var("SLINT_BACKEND", "winit");  //forcer winit au lieu de qt par défaut sur mon système
        std::env::remove_var("WAYLAND_DISPLAY"); // force XWayland
    } 
    
    let overlay = Image_Window::new()?;
    let settings_window = AppWindows2::new()?;

    let image = Image::load_from_path(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/image.png")
    ).unwrap();

    overlay.set_mon_image(image.clone());
    settings_window.set_mon_image(image);

    let overlay_weak = overlay.as_weak();        // référence faible à la fenêtre (évite les cycles mémoire)
    overlay.on_start_drag(move || {         // quand le callback start-drag est déclenché depoverlays le .slint
        let overlay = overlay_weak.unwrap();     // récupère la fenêtre depoverlays la référence faible
        overlay.window().with_winit_window(|winit_win: &winit::window::Window| {  // accède au handle winit natif
            winit_win.drag_window().ok();  // dit au OS de déplacer la fenêtre, ignore l'erreur si ça rate
        });
    });

    overlay.show()?;
    settings_window.show()?;
    settings_window.window().on_close_requested(move || {
        slint::run_event_loop().unwrap();
        slint::CloseRequestResponse::KeepWindowShown
    });

    // Délai pour que les fenêtres soient bien affichées avant wmctrl
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500));
        std::process::Command::new("wmctrl")
            .args(["-a", "overlay-image", "-b", "add,above"])
            .output()
            .ok();
    });
    slint::run_event_loop()
}