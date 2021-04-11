pub mod noted;
pub mod xero;
mod tests;
mod logging;
pub mod constants;



pub mod n2x_core {
    use crate::{logging, constants};
    use log::{warn, debug, trace};
    use chrono;
    use chrono::Duration;
    use chrono::Local;

    use crate::noted::NotedType;
    use crate::xero::XeroType;

    /// Utility function: Calculate duration into hours, if no number in the duration, to assumes the default of 1.
    pub fn calculate_duration_in_hours_to_minutes(duration: String) -> f64 {
        match duration.parse::<f64>() {
            Ok(val) => { val / 60. }
            Err(err) => {
                warn!("Could not parse this string, return 1 hour as default: Error: {:?}", err);
                1.
            }
        }
    }

    /// Utility function: Returns the first string if the second is empty, else the second string
    pub fn get_name_or_contact_name(name: String, contact_name: String) -> String {
        trace!("Processing {} against {} returning the second if not null else only the first", name, contact_name);
        if contact_name.is_empty() {
            name
        } else {
            contact_name
        }
    }

    pub fn init_logging() {
        logging::initialize_logging();
    }

    /// Utility : Will get the amount from the number in brackets at the end of a title, returns the title
    /// without the amount and the amount as a number in a touple.
    /// If title has no brackets, the title is returned unchanged and the amount is set to the default defined in the constant `STANDARD_RATE`
    pub fn get_amount_from_title(title_string: String) -> (String, f64) {
        let components: Vec<&str> = title_string.split('[').collect();
        // If there is no price in brackets, just take the defaults
        if components.len() == 1 {
            return (components[0].to_string(), constants::STANDARD_RATE);
        }
        trace!("DEBUG:: first part is '{}'", components[0]);
        trace!("DEBUG:: second part is '{}'", components[1]);
        let title = components[0].trim().to_string();
        let mut amount_intemediate = components[1].to_string();
        amount_intemediate = amount_intemediate.trim().to_string();
        amount_intemediate.pop(); // Lose the closing bracket of the amount in the title, or it cannot be parsed
        debug!("second part is now '{}'", amount_intemediate);
        let amount = match amount_intemediate.parse::<f64>() {
            Ok(amount) => { amount }
            Err(_) => {
                warn!("Could not parse the amount {} so we take the default {}", &amount_intemediate, constants::STANDARD_RATE);
                constants::STANDARD_RATE
            }
        };

        (title, amount)
    }

    /// Does the actual mapping, get a noted_collection and returns a collection of Xero import lines.
    pub fn map_noted_to_xero(noted_collection: &Vec<NotedType>) -> Vec<XeroType> {
        let invoice_arguments: Vec<String> = std::env::args().collect();
        if invoice_arguments.len() == 1 {
            panic!("Please pass the starting invoice number as the start parameter. Bye now");
        }
        let invoice_counter = &invoice_arguments[1].to_string().parse::<i32>().unwrap_or(0);
        let mut invoice_counter_cp = invoice_counter.clone();
        let mut result: Vec<XeroType> = Vec::new();


        for noted_item in noted_collection.iter() {
            let (title, rate) = get_amount_from_title(noted_item.title.to_string());
            let today = Local::now()  + Duration::days(constants::INVOICE_DAYS_TODAY as i64);
            let xero_item = XeroType {
                contact_name: get_name_or_contact_name(noted_item.full_name.to_string(), noted_item.contact_name.to_string()),
                email_address: get_name_or_contact_name(noted_item.email.to_string(), noted_item.contacts_email.to_string()),
                poaddress_line1: "".to_string(),
                poaddress_line2: "".to_string(),
                poaddress_line3: "".to_string(),
                poaddress_line4: "".to_string(),
                pocity: "".to_string(),
                poregion: "".to_string(),
                popostal_code: "".to_string(),
                pocountry: "".to_string(),
                invoice_number: format!("INV-{}", invoice_counter_cp.to_string()),
                reference: "".to_string(),
                invoice_date: today.format("%d-%m-%Y").to_string(),
                due_date: (today + Duration::days(constants::INVOICE_DAYS_TO_PAY as i64)).format("%d-%m-%Y").to_string(),
                inventory_item_code: constants::XERO_INVENTORY_ACCOUNT_NUMBER.to_string(),
                description: title,
                quantity: calculate_duration_in_hours_to_minutes(noted_item.duration.to_string()).to_string(),
                unit_amount: rate.to_string(),
                discount: "".to_string(),
                account_code: constants::XERO_INCOME_ACCOUNT_NUMBER.to_string(),
                tax_type: constants::TAX_RATE.to_string(),
                tracking_name1: "".to_string(),
                tracking_option1: "".to_string(),
                tracking_name2: "".to_string(),
                tracking_option2: "".to_string(),
                currency: constants::CURRENCY.to_string(),
                branding_theme: constants::BRANDING_THEME.to_string(),
            };
            invoice_counter_cp += 1;
            result.push(xero_item);
        }
        result
    }
}

