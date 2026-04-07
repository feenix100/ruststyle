// src/main.rs is intentionally small.
// All layout and styling lives in dedicated Slint files for tutorial clarity.
slint::include_modules!();

fn compute_window_scale(window: &slint::Window) -> f32 {
    let logical_size = window.size().to_logical(window.scale_factor());
    let width_scale = logical_size.width / 1220.0;
    let height_scale = logical_size.height / 940.0;

    width_scale.min(height_scale).clamp(0.55, 3.2)
}

fn main() -> Result<(), slint::PlatformError> {
    std::thread::Builder::new()
        .name("ui-main".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| -> Result<(), slint::PlatformError> {
            let app = AppWindow::new()?;
            app.window().set_maximized(true);
            let timer = slint::Timer::default();
            let app_weak = app.as_weak();
            let start = std::time::Instant::now();

            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    if let Some(app) = app_weak.upgrade() {
                        let phase = (start.elapsed().as_secs_f32() / 3.6).fract();
                        app.set_animation_phase(phase);
                        app.set_window_scale(compute_window_scale(&app.window()));
                    }
                },
            );

            app.run()
        })
        .expect("failed to spawn UI thread")
        .join()
        .expect("UI thread panicked")
}
