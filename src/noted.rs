/// The representation of the Noted CSV structure.
#[derive(Debug)]
pub struct NotedType {
    pub title: String,
    pub create_date: String,
    pub duration: i16,
    pub category: String,
    pub type_therapy: String,
    pub full_name: String,
    pub email: String,
    pub external_agenecy_contacts_data: String,
    pub contacts_agency_organisation: String,
    pub contact_association_client: String,
    pub contacts_email: String,
    pub contact_name: String,
}
