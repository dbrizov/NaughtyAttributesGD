use godot::classes::{Object, Script};
use godot::global::type_string;
use godot::prelude::*;

pub fn resolve(object: &Gd<Object>, name: &str) -> Result<Variant, String> {
    let name = StringName::from(name);

    if object.has_method(&name) {
        return object
            .clone()
            .try_call(&name, &[])
            .map_err(|_| format!("'{name}' must be a method that takes no arguments"));
    }

    let value = object.get(&name);
    if value.is_nil() {
        return Err(format!(
            "could not resolve '{name}' to a property or a parameterless method{}",
            tool_hint(object)
        ));
    }

    Ok(value)
}

pub fn resolve_number(object: &Gd<Object>, name: &str) -> Result<f64, String> {
    let value = resolve(object, name)?;

    match value.get_type() {
        VariantType::INT => Ok(value.to::<i64>() as f64),
        VariantType::FLOAT => Ok(value.to::<f64>()),
        other => Err(format!(
            "'{name}' is {}, not a number",
            type_string(other.ord() as i64)
        )),
    }
}

pub fn tool_hint(object: &Gd<Object>) -> &'static str {
    let is_tool = object
        .get("script")
        .try_to::<Gd<Script>>()
        .map(|script| script.is_tool())
        .unwrap_or(true);

    if is_tool {
        ""
    } else {
        " (calling a method needs @tool on the script)"
    }
}
