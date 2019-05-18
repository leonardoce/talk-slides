#[macro_use]
extern crate diesel;

sql_function! {
    #[sql_name = "ABS"]
    fn abs(expr: diesel::sql_types::Double) -> diesel::sql_types::Double;
}

pub mod notes;
pub mod schema;

#[cfg(test)]
mod tests {
    use super::notes;

    #[test]
    fn test_temperament_ratio() {
        assert_eq!(notes::temperament_ratio(0), 1f64);
        assert!(2f64 - notes::temperament_ratio(12) < 0.001);
    }

    #[test]
    fn test_note_names() {
        assert_eq!(notes::NOTES[9], "A");
    }

    #[test]
    fn test_c0_modern_temperament() {
        assert!((16.352 - notes::find_c0(440_f64).abs()) < 0.001);
    }
}
