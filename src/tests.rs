#[cfg(test)]
mod tests {
    use crate::get_amount_from_title;
    use crate::calculate_duration_in_hours_to_minutes;
    use crate::get_name_or_contact_name;

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
        assert_eq!(1.,hours)
    }

    #[test]
    fn test_duration_convertor_90mins() {
        let hours = calculate_duration_in_hours_to_minutes("90".to_string());
        assert_eq!(1.5,hours)
    }

    #[test]
    fn test_duration_convertor_135mins() {
        let hours = calculate_duration_in_hours_to_minutes("135".to_string());
        assert_eq!(2.25,hours)
    }

    #[test]
    fn test_duration_convertor_garbage_input() {
        let hours = calculate_duration_in_hours_to_minutes("fiets".to_string());
        assert_eq!(1.,hours)
    }

    #[test]
    fn test_duration_convertor_partial_input() {
        let hours = calculate_duration_in_hours_to_minutes("90]".to_string());
        assert_eq!(1.,hours)
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
}