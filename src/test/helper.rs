
#[cfg(test)]
mod tests {
    use crate::helper::{calculate_wind_components, normalize_angle};
    use approx::assert_relative_eq;
    

    /// Tests for the `normalize_angle` function.
    #[test]
    fn test_normalize_angle() {
        // Angles within 0-360
        assert_relative_eq!(normalize_angle(0.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(45.0), 45.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(359.999), 359.999, epsilon = 1e-6);

        // Angles exactly 360 and multiples
        assert_relative_eq!(normalize_angle(360.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(720.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(-360.0), 0.0, epsilon = 1e-6);

        // Negative angles
        assert_relative_eq!(normalize_angle(-45.0), 315.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(-720.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(-370.0), 350.0, epsilon = 1e-6);

        // Angles greater than 360
        assert_relative_eq!(normalize_angle(450.0), 90.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(1080.0), 0.0, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(765.0), 45.0, epsilon = 1e-6);

        // Non-integer angles
        assert_relative_eq!(normalize_angle(123.456), 123.456, epsilon = 1e-6);
        assert_relative_eq!(normalize_angle(-123.456), 236.544, epsilon = 1e-6);
    }

    /// Tests for the `calculate_wind_components` function.
    #[test]
    fn test_calculate_wind_components() {
        

        let (crosswind, headwind) = calculate_wind_components(8.0, 137, 180);
        assert_relative_eq!(crosswind, -5.46, epsilon = 1e-2);
        assert_relative_eq!(headwind, 5.85, epsilon = 1e-2);

        let (crosswind, headwind) = calculate_wind_components(8.0, 180, 180);
        assert_relative_eq!(crosswind, 0.0, epsilon = 1e-2);
        assert_relative_eq!(headwind, 8.0, epsilon = 1e-2);
        
    }
}
