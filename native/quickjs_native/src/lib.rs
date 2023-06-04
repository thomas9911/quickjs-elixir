use rquickjs::{Context, Func, Object, Runtime};
use rustler::types::atom;
use rustler::Atom;
mod serde_json_wrapper;

fn value_printer(value: &rquickjs::Value, buffer: &mut String) -> Result<(), rquickjs::Error> {
    let type_ = value.type_of();
    use rquickjs::Type::*;
    match type_ {
        Bool | Int | Float => match type_ {
            Bool => buffer.push_str(&value.as_bool().unwrap().to_string()),
            Int => buffer.push_str(&value.as_int().unwrap().to_string()),
            Float => buffer.push_str(&value.as_float().unwrap().to_string()),
            _ => unreachable!(),
        },
        String => {
            let txt = unsafe { value.ref_string() }.to_string();
            buffer.push('"');
            buffer.push_str(&txt?);
            buffer.push('"');
        }
        Symbol | Object | Array | Function => {
            buffer.push_str("{ ..other }");
        }
        _ => (),
    }
    Ok(())
}

fn debug(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::debug!("{}", buffer);
}

fn log(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::info!("{}", buffer);
}

fn info(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::info!("{}", buffer);
}

fn warn(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::warn!("{}", buffer);
}

fn error(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::error!("{}", buffer);
}

#[rustler::nif]
fn run(script: &str) -> Result<(Atom, String), rustler::Error> {
    let rt = Runtime::new().map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;
    let ctx = Context::full(&rt).map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;

    let out = ctx
        .with(|ctx| {
            let global = ctx.globals();
            let console = Object::new(ctx)?;
            console.set("debug", Func::new("debug", debug))?;
            console.set("info", Func::new("info", info))?;
            console.set("log", Func::new("log", log))?;
            console.set("warn", Func::new("warn", warn))?;
            console.set("error", Func::new("error", error))?;
            global.set("console", console)?;

            let out = ctx.eval(script)?;
            Ok::<serde_json_wrapper::JsonValue, rquickjs::Error>(out)
        })
        .map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;

    Ok((atom::ok(), out.0.to_string()))
}

rustler::init!("Elixir.QuickJs.Native", [run]);
