use super::*;

pub enum ArgumentCheckResult {
    Ok,
    Wrong,
    Pending,
}

pub fn check_argument_types(
    env: &Environment,
    t: &str,
    f: &str,
    args: &[Value],
) -> ArgumentCheckResult {
    let f = env
        .fn_map
        .get(t)
        .and_then(|n| n.get(f))
        .unwrap_or_else(|| panic!("tried to get undefined function '{f}' on '{t}'"));
    if f.types.len() > args.len() {
        ArgumentCheckResult::Pending
    } else if f.types.len() < args.len() {
        ArgumentCheckResult::Wrong
    } else if f
        .types
        .iter()
        .zip(args.iter())
        .all(|(n, m)| n == &m.get_typeid())
    {
        ArgumentCheckResult::Ok
    } else {
        ArgumentCheckResult::Wrong
    }
}
