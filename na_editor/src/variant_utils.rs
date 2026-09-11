use godot::builtin::real;
use godot::global::type_string;
use godot::prelude::*;

pub fn clamp(value: &Variant, min: f64, max: f64) -> Result<Variant, String> {
    let real_min = min as real;
    let real_max = max as real;
    let int_min = min.ceil();
    let int_max = max.floor();

    let clamped = match value.get_type() {
        VariantType::INT => value
            .to::<i64>()
            .max(int_min as i64)
            .min(int_max as i64)
            .to_variant(),
        VariantType::FLOAT => value.to::<f64>().max(min).min(max).to_variant(),
        VariantType::VECTOR2 => value
            .to::<Vector2>()
            .coord_max(Vector2::splat(real_min))
            .coord_min(Vector2::splat(real_max))
            .to_variant(),
        VariantType::VECTOR3 => value
            .to::<Vector3>()
            .coord_max(Vector3::splat(real_min))
            .coord_min(Vector3::splat(real_max))
            .to_variant(),
        VariantType::VECTOR4 => value
            .to::<Vector4>()
            .coord_max(Vector4::splat(real_min))
            .coord_min(Vector4::splat(real_max))
            .to_variant(),
        VariantType::VECTOR2I => value
            .to::<Vector2i>()
            .coord_max(Vector2i::splat(int_min as i32))
            .coord_min(Vector2i::splat(int_max as i32))
            .to_variant(),
        VariantType::VECTOR3I => value
            .to::<Vector3i>()
            .coord_max(Vector3i::splat(int_min as i32))
            .coord_min(Vector3i::splat(int_max as i32))
            .to_variant(),
        VariantType::VECTOR4I => value
            .to::<Vector4i>()
            .coord_max(Vector4i::splat(int_min as i32))
            .coord_min(Vector4i::splat(int_max as i32))
            .to_variant(),
        other => {
            return Err(format!(
                "cannot clamp a value of type {}",
                type_string(other.ord() as i64)
            ));
        }
    };

    Ok(clamped)
}
