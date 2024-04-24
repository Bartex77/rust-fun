slint::include_modules!();

const BLOW_THRESHOLD: u32 = 85;
use fastrand;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

     ui.on_dices_rolled({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let dices: u32 = fastrand::i32(1..100).try_into().unwrap();            
            ui.set_dices(dices.try_into().unwrap());

            if dices >= BLOW_THRESHOLD {
                ui.set_hassan("Blow, Hassan, blooow!".into());
            } else {
                ui.set_hassan("Relax, Hassan, relax...".into());
            }
        }
    }); 

    ui.run()
}
