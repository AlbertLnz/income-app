slint::include_modules!();

const TAXPER: f64 = 0.30; // float 64
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXPER;
        let result: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", {tax}, {owner}, {profit}, {opex});
        
        ui.set_results(result.into())
    });

    ui.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_100() {
        let result = TAXPER + OWNERPER + PROFITPER + OPEXPER;
        let formatted = f64::trunc(result  * 100.0) / 100.0;
        assert_eq!(formatted, 1.00);
    }
}
