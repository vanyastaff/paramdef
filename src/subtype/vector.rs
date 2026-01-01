//! Standard vector subtypes.
//!
//! Vector subtypes are categorized by their size:
//!
//! ## Size 2
//! - [`Position2D`] - 2D position
//! - [`Size2D`] - Width/height
//! - [`Uv`] - Texture coordinates
//! - [`LatLong`] - Geographic coordinates
//! - [`MinMax`] - Range bounds
//! - [`Direction2D`] - 2D direction (normalized)
//! - [`Scale2D`] - 2D scale factors
//! - [`Vector2`] - Generic 2D vector
//!
//! ## Size 3
//! - [`Position3D`] - 3D position
//! - [`Direction3D`] - 3D direction (normalized)
//! - [`Normal`] - Surface normal (normalized)
//! - [`Scale3D`] - 3D scale factors
//! - [`Euler`] - Euler angles (pitch, yaw, roll)
//! - [`ColorRgb`] - RGB color (0-1)
//! - [`ColorHsv`] - HSV color
//! - [`Vector3`] - Generic 3D vector
//!
//! ## Size 4
//! - [`Quaternion`] - Rotation quaternion (normalized)
//! - [`AxisAngle`] - Axis-angle rotation
//! - [`ColorRgba`] - RGBA color (0-1)
//! - [`Bounds2D`] - 2D bounding box (minX, minY, maxX, maxY)
//! - [`Vector4`] - Generic 4D vector
//!
//! ## Larger Sizes
//! - [`Bounds3D`] - 3D bounding box (size 6)
//! - [`Matrix3x3`] - 3x3 matrix (size 9)
//! - [`Matrix4x4`] - 4x4 transformation matrix (size 16)

use crate::define_vector_subtype;

// === Size 2 ===

define_vector_subtype!(Position2D, 2, "position_2d", labels: ["X", "Y"]);
define_vector_subtype!(Size2D, 2, "size_2d", labels: ["W", "H"]);
define_vector_subtype!(Uv, 2, "uv", labels: ["U", "V"], range: (0.0, 1.0));
define_vector_subtype!(LatLong, 2, "lat_long", labels: ["Lat", "Long"]);
define_vector_subtype!(MinMax, 2, "min_max", labels: ["Min", "Max"]);
define_vector_subtype!(Direction2D, 2, "direction_2d", labels: ["X", "Y"], normalized: true);
define_vector_subtype!(Scale2D, 2, "scale_2d", labels: ["X", "Y"]);
define_vector_subtype!(Vector2, 2, "vector2", labels: ["X", "Y"]);

// === Size 3 ===

define_vector_subtype!(Position3D, 3, "position_3d", labels: ["X", "Y", "Z"]);
define_vector_subtype!(Direction3D, 3, "direction_3d", labels: ["X", "Y", "Z"], normalized: true);
define_vector_subtype!(Normal, 3, "normal", labels: ["X", "Y", "Z"], normalized: true);
define_vector_subtype!(Scale3D, 3, "scale_3d", labels: ["X", "Y", "Z"]);
define_vector_subtype!(Euler, 3, "euler", labels: ["Pitch", "Yaw", "Roll"]);
define_vector_subtype!(ColorRgb, 3, "color_rgb", labels: ["R", "G", "B"], range: (0.0, 1.0));
define_vector_subtype!(ColorHsv, 3, "color_hsv", labels: ["H", "S", "V"]);
define_vector_subtype!(Vector3, 3, "vector3", labels: ["X", "Y", "Z"]);

// === Size 4 ===

define_vector_subtype!(Quaternion, 4, "quaternion", labels: ["X", "Y", "Z", "W"], normalized: true);
define_vector_subtype!(AxisAngle, 4, "axis_angle", labels: ["X", "Y", "Z", "Angle"]);
define_vector_subtype!(ColorRgba, 4, "color_rgba", labels: ["R", "G", "B", "A"], range: (0.0, 1.0));
define_vector_subtype!(Bounds2D, 4, "bounds_2d", labels: ["MinX", "MinY", "MaxX", "MaxY"]);
define_vector_subtype!(Vector4, 4, "vector4", labels: ["X", "Y", "Z", "W"]);

// === Size 6 ===

define_vector_subtype!(Bounds3D, 6, "bounds_3d", labels: ["MinX", "MinY", "MinZ", "MaxX", "MaxY", "MaxZ"]);

// === Size 9 ===

define_vector_subtype!(Matrix3x3, 9, "matrix_3x3", labels: ["M00", "M01", "M02", "M10", "M11", "M12", "M20", "M21", "M22"]);

// === Size 16 ===

define_vector_subtype!(Matrix4x4, 16, "matrix_4x4", labels: [
    "M00", "M01", "M02", "M03",
    "M10", "M11", "M12", "M13",
    "M20", "M21", "M22", "M23",
    "M30", "M31", "M32", "M33"
]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::VectorSubtype;

    // === Size 2 Tests ===

    #[test]
    fn test_position_2d() {
        assert_eq!(Position2D::name(), "position_2d");
        assert_eq!(Position2D::SIZE, 2);
        assert_eq!(Position2D::component_labels(), ["X", "Y"]);
        assert!(!Position2D::is_normalized());
    }

    #[test]
    fn test_size_2d() {
        assert_eq!(Size2D::name(), "size_2d");
        assert_eq!(Size2D::component_labels(), ["W", "H"]);
    }

    #[test]
    fn test_uv() {
        assert_eq!(Uv::name(), "uv");
        assert_eq!(Uv::default_range(), Some((0.0, 1.0)));
    }

    #[test]
    fn test_lat_long() {
        assert_eq!(LatLong::name(), "lat_long");
        assert_eq!(LatLong::component_labels(), ["Lat", "Long"]);
    }

    #[test]
    fn test_direction_2d() {
        assert_eq!(Direction2D::name(), "direction_2d");
        assert!(Direction2D::is_normalized());
    }

    // === Size 3 Tests ===

    #[test]
    fn test_position_3d() {
        assert_eq!(Position3D::name(), "position_3d");
        assert_eq!(Position3D::SIZE, 3);
        assert_eq!(Position3D::component_labels(), ["X", "Y", "Z"]);
    }

    #[test]
    fn test_direction_3d() {
        assert_eq!(Direction3D::name(), "direction_3d");
        assert!(Direction3D::is_normalized());
    }

    #[test]
    fn test_normal() {
        assert_eq!(Normal::name(), "normal");
        assert!(Normal::is_normalized());
    }

    #[test]
    fn test_euler() {
        assert_eq!(Euler::name(), "euler");
        assert_eq!(Euler::component_labels(), ["Pitch", "Yaw", "Roll"]);
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!(ColorRgb::name(), "color_rgb");
        assert_eq!(ColorRgb::component_labels(), ["R", "G", "B"]);
        assert_eq!(ColorRgb::default_range(), Some((0.0, 1.0)));
    }

    #[test]
    fn test_color_hsv() {
        assert_eq!(ColorHsv::name(), "color_hsv");
        assert_eq!(ColorHsv::component_labels(), ["H", "S", "V"]);
    }

    // === Size 4 Tests ===

    #[test]
    fn test_quaternion() {
        assert_eq!(Quaternion::name(), "quaternion");
        assert_eq!(Quaternion::SIZE, 4);
        assert_eq!(Quaternion::component_labels(), ["X", "Y", "Z", "W"]);
        assert!(Quaternion::is_normalized());
    }

    #[test]
    fn test_axis_angle() {
        assert_eq!(AxisAngle::name(), "axis_angle");
        assert_eq!(AxisAngle::component_labels(), ["X", "Y", "Z", "Angle"]);
    }

    #[test]
    fn test_color_rgba() {
        assert_eq!(ColorRgba::name(), "color_rgba");
        assert_eq!(ColorRgba::component_labels(), ["R", "G", "B", "A"]);
        assert_eq!(ColorRgba::default_range(), Some((0.0, 1.0)));
    }

    #[test]
    fn test_bounds_2d() {
        assert_eq!(Bounds2D::name(), "bounds_2d");
        assert_eq!(Bounds2D::SIZE, 4);
    }

    // === Larger Size Tests ===

    #[test]
    fn test_bounds_3d() {
        assert_eq!(Bounds3D::name(), "bounds_3d");
        assert_eq!(Bounds3D::SIZE, 6);
        assert_eq!(
            Bounds3D::component_labels(),
            ["MinX", "MinY", "MinZ", "MaxX", "MaxY", "MaxZ"]
        );
    }

    #[test]
    fn test_matrix_3x3() {
        assert_eq!(Matrix3x3::name(), "matrix_3x3");
        assert_eq!(Matrix3x3::SIZE, 9);
    }

    #[test]
    fn test_matrix_4x4() {
        assert_eq!(Matrix4x4::name(), "matrix_4x4");
        assert_eq!(Matrix4x4::SIZE, 16);
    }
}
