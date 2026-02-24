use std::collections::HashMap;

/**
 * I implemeneted a aimple template rendering setup here since this is an MVP.
 * TODO: Will surely have to replace {{key}} with value & implement a proper templating engine.
 */

pub fn render_template(body: &str, vars: &HashMap<String, String>) -> String {
    let mut out = body.to_string();
    for (k, v) in vars {
        let placeholder = format!("{{{{{}}}}}", k);
        out = out.replace(&placeholder, v);
    }
    out
}
