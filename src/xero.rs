/// The Xero invoice import definition, the field names are slightly different as it contains non-rust allowable ones.
/// It supports the debug derive for easy debugging.
#[derive(Debug)]
pub struct XeroType {
    pub contact_name: String,
    pub email_address: String,
    pub poaddress_line1: String,
    pub poaddress_line2: String,
    pub poaddress_line3: String,
    pub poaddress_line4: String,
    pub pocity: String,
    pub poregion: String,
    pub popostal_code: String,
    pub pocountry: String,
    pub invoice_number: String,
    pub reference: String,
    pub invoice_date: String,
    pub due_date: String,
    pub inventory_item_code: String,
    pub description: String,
    pub quantity: String,
    pub unit_amount: String,
    pub discount: String,
    pub account_code: String,
    pub tax_type: String,
    pub tracking_name1: String,
    pub tracking_option1: String,
    pub tracking_name2: String,
    pub tracking_option2: String,
    pub currency: String,
    pub branding_theme: String,
}
///Methods for the Xero type.
impl XeroType {
    ///Returns the headers that the CSV Xero import expects. It has static TTL as it needs to exist while the application is running
    pub fn get_headers() -> Vec<&'static str> {
        vec!["*ContactName", "EmailAddress", "POAddressLine1", "POAddressLine2", "POAddressLine3", "POAddressLine4", "POCity", "PORegion", "POPostalCode", "POCountry", "*InvoiceNumber", "Reference", "*InvoiceDate", "*DueDate", "InventoryItemCode", "*Description", "*Quantity", "*UnitAmount", "Discount", "*AccountCode", "*TaxType", "TrackingName1", "TrackingOption1", "TrackingName2", "TrackingOption2", "Currency", "BrandingTheme"]
    }

    /// Returns the struct as a vector of string slices as this is what the *CSV writer* expects.
    pub fn get_item_as_vector(&self) -> Vec<&str> {
        vec![&self.contact_name, &self.email_address, &self.poaddress_line1, &self.poaddress_line2, &self.poaddress_line3, &self.poaddress_line4, &self.pocity, &self.poregion, &self.popostal_code, &self.pocountry, &self.invoice_number, &self.reference, &self.invoice_date, &self.due_date, &self.inventory_item_code, &self.description, &self.quantity, &self.unit_amount, &self.discount, &self.account_code, &self.tax_type, &self.tracking_name1, &self.tracking_option1, &self.tracking_name2, &self.tracking_option2, &self.currency, &self.branding_theme]
    }
}