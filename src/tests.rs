#[cfg(test)]
mod tests {
    use crate::n2x_core::get_amount_from_title;
    use crate::n2x_core::get_name_or_contact_name;
    use crate::n2x_core::map_noted_to_xero;
    use crate::{n2x_core::calculate_duration_in_hours_to_minutes, noted::NotedType};

    fn standard_noted_struct() -> NotedType {
        NotedType {
            title: "This is a value [100]".to_string(),
            category: "Invoice".to_string(),
            contact_name: "Jan Jansen".to_string(),
            create_date: "2021-05-01".to_string(),
            duration: 60,
            full_name: "Peter Petersen".to_string(),
            type_therapy: "Counselling".to_string(),
            contact_association_client: "None".to_string(),
            contacts_agency_organisation: "test".to_string(),
            contacts_email: "".to_string(),
            email: "test@test.com".to_string(),
            external_agenecy_contacts_data: "None".to_string(),
        }
    }

    #[test]
    fn test_desc_splitter() {
        let (desc, amount) = get_amount_from_title("test [0]".to_string());
        assert_eq!("test", desc);
        assert_eq!(0., amount);
    }

    #[test]
    fn test_desc_splitter_no_amount() {
        let (desc, amount) = get_amount_from_title("test".to_string());
        assert_eq!("test", desc);
        assert_eq!(120., amount);
    }

    #[test]
    fn test_desc_splitter_floating_amount() {
        let (desc, amount) = get_amount_from_title("test one two three [80.55] ".to_string());
        assert_eq!("test one two three", desc);
        assert_eq!(80.55, amount);
    }

    #[test]
    fn test_duration_convertor() {
        let hours = calculate_duration_in_hours_to_minutes("60".to_string());
        assert_eq!(1., hours)
    }

    #[test]
    fn test_duration_convertor_90mins() {
        let hours = calculate_duration_in_hours_to_minutes("90".to_string());
        assert_eq!(1.5, hours)
    }

    #[test]
    fn test_duration_convertor_135mins() {
        let hours = calculate_duration_in_hours_to_minutes("135".to_string());
        assert_eq!(2.25, hours)
    }

    #[test]
    fn test_duration_convertor_garbage_input() {
        let hours = calculate_duration_in_hours_to_minutes("fiets".to_string());
        assert_eq!(1., hours)
    }

    #[test]
    fn test_duration_convertor_partial_input() {
        let hours = calculate_duration_in_hours_to_minutes("90]".to_string());
        assert_eq!(1., hours)
    }

    #[test]
    fn test_get_name_or_contact_name() {
        let res = get_name_or_contact_name("one".to_string(), "".to_string());
        assert_eq!("one", res)
    }

    #[test]
    fn test_get_name_or_contact_name_contacname_not_empty() {
        let res = get_name_or_contact_name("".to_string(), "two".to_string());
        assert_eq!("two", res)
    }

    #[test]
    fn test_get_name_or_contact_name_prefer_contact_name() {
        let res = get_name_or_contact_name("one".to_string(), "two".to_string());
        assert_eq!("two", res)
    }

    #[test]
    fn test_map_noted_to_zero() {
        let noted_collection = [standard_noted_struct()];
        let result = map_noted_to_xero(&noted_collection, Some(3000));
        assert_eq!(1, result.len());
        assert_eq!(
            "INV-3000",
            result.into_iter().nth(0).unwrap().invoice_number
        );
    }

    #[test]
    fn test_map_noted_to_zero_amount_scraper() {
        let noted_collection = [standard_noted_struct()];
        let result = map_noted_to_xero(&noted_collection, Some(3000));
        assert_eq!(1, result.len());
        assert_eq!("100", result.into_iter().nth(0).unwrap().unit_amount);
    }

    #[test]
    fn test_map_noted_to_date_therapy_in_description() {
        let noted_collection = [standard_noted_struct()];
        let result = map_noted_to_xero(&noted_collection, Some(3000));
        assert_eq!(1, result.len());
        assert_eq!(
            "This is a value; Therapy date: 2021-05-01",
            result.into_iter().nth(0).unwrap().description
        );
    }
}
