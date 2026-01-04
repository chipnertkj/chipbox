use std::fmt::Write as _;

pub trait DisplayJsValue {
    fn pretty_display(&self) -> String;
}

impl DisplayJsValue for rquickjs::Value<'_> {
    fn pretty_display(&self) -> String {
        display_js_value(self)
    }
}

// display objects recursively with indentation
#[must_use]
fn display_js_value(value: &rquickjs::Value<'_>) -> String {
    let mut seen = std::collections::HashSet::new();
    inner(value, 0, &mut seen)
}

fn get_object_id(value: &rquickjs::Value) -> Option<usize> {
    let raw = value.as_raw();
    // Tags that use the ptr field: negative tags and some special ones
    match raw.tag {
        tag if tag.is_negative() => {
            Some(
                // SAFETY: we checked union tag is negative - all JsValues with negative tags are managed by heap
                unsafe { raw.u.ptr as usize },
            )
        }
        _ => None,
    }
}

fn format_object(
    obj: &rquickjs::Object<'_>,
    depth: usize,
    seen: &mut std::collections::HashSet<usize>,
) -> String {
    let props: Vec<_> = obj.props().filter_map(Result::ok).collect();
    if props.is_empty() {
        return "{}".to_string();
    }

    let indent = "  ".repeat(depth);
    let mut s = String::from("{\n");
    for (key, val) in props {
        let key_str = inner(&key, depth + 1, seen);
        let val_str = inner(&val, depth + 1, seen);
        let _ = writeln!(s, "{indent}  {key_str}: {val_str},");
    }
    s.push_str(&indent);
    s.push('}');
    s
}

fn format_array(
    arr: &rquickjs::Array<'_>,
    depth: usize,
    seen: &mut std::collections::HashSet<usize>,
) -> String {
    let items: Vec<_> = arr.iter().filter_map(Result::ok).collect();
    if items.is_empty() {
        return "[]".to_string();
    }

    let indent = "  ".repeat(depth);
    let mut s = String::from("[\n");
    for val in items {
        let val_str = inner(&val, depth + 1, seen);
        let _ = writeln!(s, "{indent}  {val_str},");
    }
    s.push_str(&indent);
    s.push(']');
    s
}

fn format_function(func: &rquickjs::Function<'_>) -> String {
    let name = func
        .get::<_, String>("name")
        .ok()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "<unnamed>".to_string());
    let length = func.get::<_, i32>("length").unwrap_or(0);
    format!(
        "{name}({}) {{ <native code> }}",
        (0..length).map(|_| "_").collect::<Vec<_>>().join(", ")
    )
}

fn inner(
    value: &rquickjs::Value<'_>,
    depth: usize,
    seen: &mut std::collections::HashSet<usize>,
) -> String {
    const MAX_DEPTH: usize = 10;

    if depth > MAX_DEPTH {
        return "<max depth reached>".to_string();
    }

    // Check for circular reference early
    if let Some(id) = get_object_id(value)
        && !seen.insert(id)
    {
        return "<circular>".to_string();
    }

    let indent = "  ".repeat(depth);

    let result = match value.type_of() {
        rquickjs::Type::Object => value.clone().as_object().map_or_else(
            || format!("{indent}<unprintable object>"),
            |obj| format_object(obj, depth, seen),
        ),
        rquickjs::Type::Array => value.clone().as_array().map_or_else(
            || format!("{indent}<unprintable array>"),
            |arr| format_array(arr, depth, seen),
        ),
        rquickjs::Type::String => value.clone().as_string().map_or_else(
            || "<error converting string>".to_string(),
            |v| {
                format!(
                    "\"{}\"",
                    v.to_string()
                        .unwrap_or_else(|_| "<error converting string>".to_string())
                )
            },
        ),
        rquickjs::Type::Int | rquickjs::Type::Float => value.as_number().map_or_else(
            || "<error converting number>".to_string(),
            |n| n.to_string(),
        ),
        rquickjs::Type::Bool => value
            .as_bool()
            .map_or_else(|| "<error converting bool>".to_string(), |b| b.to_string()),
        rquickjs::Type::Null => "null".to_string(),
        rquickjs::Type::Undefined => "undefined".to_string(),
        rquickjs::Type::Function => value
            .clone()
            .as_function()
            .map_or_else(|| "<anonymous function>".to_string(), format_function),
        _ => format!("{value:?}"),
    };

    // Clean up from seen set
    if let Some(id) = get_object_id(value) {
        seen.remove(&id);
    }

    result
}
