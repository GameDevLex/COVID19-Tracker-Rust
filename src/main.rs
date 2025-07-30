// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod covid_details;
use std::error::Error;
slint::include_modules!();
use covid_details::CovidData;
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_get_details({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match covid_details::get_covid_details() {
                Ok(json_data) => {
                    let total_cases = json_data.cases.to_string();
                    let total_deaths = json_data.deaths.to_string();
                    let today_cases = json_data.today_cases.to_string();
                    let today_deaths = json_data.today_deaths.to_string();
                    let today_recovered = json_data.today_recovered.to_string();

                    ui.set_total_cases_counter(slint::SharedString::from(total_cases).clone().parse().unwrap());
                    ui.set_total_deaths_counter(slint::SharedString::from(total_deaths).clone().parse().unwrap());
                    ui.set_today_cases_counter(slint::SharedString::from(today_cases).clone().parse().unwrap());
                    ui.set_today_deaths_counter(slint::SharedString::from(today_deaths).clone().parse().unwrap());
                    ui.set_today_recovered_counter(slint::SharedString::from(today_recovered).clone().parse().unwrap());

                },
                Err(_) => {
                    println!("Failed to grab price");
                },
            }

        }
    });

    ui.run()?;

    Ok(())
}
