use std::fmt::Display;
use std::thread::JoinHandle;

use rquickjs::{Context, Func, Object, Runtime};
use rustler::types::atom;
use rustler::Atom;
mod serde_json_wrapper;

const STACK_SIZE: usize = 256 * 1024;

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

enum InnerError {
    String(String),
}

// impl Into<rustler::Error> for InnerError {
//     fn into(self) -> rustler::Error {
//         match self {
//             InnerError::String(error) => rustler::Error::RaiseTerm(Box::new(error))
//         }
//     }
//     // fn into(error: InnerError) -> rustler::Error {
//     //     // rustler::Error::RaiseTerm(Box::new(error))

//     //     match error {
//     //         InnerError::String(error) => rustler::Error::RaiseTerm(Box::new(error))
//     //     }
//     // }
// }

impl From<InnerError> for rustler::Error {
    fn from(error: InnerError) -> rustler::Error {
        match error {
            InnerError::String(error) => rustler::Error::RaiseTerm(Box::new(error)),
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn run(script: &str) -> Result<(Atom, String), rustler::Error> {
    // let child = std::thread::spawn(move || {
    //     let res = std::panic::catch_unwind(|| {
    //         // let rt =
    //         //     Runtime::new()
    //         //     .map_err(|e| InnerError::String(e.to_string()))?;
    //         // // .map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;
    //         // rt.set_memory_limit(10 * 1024 * 1024);
    //         // let ctx = Context::full(&rt)
    //         //     // .map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;
    //         //     .map_err(|e| InnerError::String(e.to_string()))?;

    //         // let out = ctx
    //         //     .with(|ctx| {
    //         //         let global = ctx.globals();
    //         //         let console = Object::new(ctx)?;
    //         //         console.set("debug", Func::new("debug", debug))?;
    //         //         console.set("info", Func::new("info", info))?;
    //         //         console.set("log", Func::new("log", log))?;
    //         //         console.set("warn", Func::new("warn", warn))?;
    //         //         console.set("error", Func::new("error", error))?;
    //         //         global.set("console", console)?;

    //         //         let out = ctx.eval(script)?;
    //         //         Ok::<serde_json_wrapper::JsonValue, rquickjs::Error>(out)
    //         //     })
    //         //     .map_err(|e| InnerError::String(e.to_string()))?;
    //         // // .map_err(|e| rustler::Error::RaiseTerm(Box::new(e.to_string())))?;
    //         // Ok(out.0.to_string())
    //         inner_run(script)
    //     })
    //     .map_err(|e| {
    //         let err = match e.downcast_ref::<Box<dyn Display>>() {
    //             Some(e) => e.to_string(),
    //             None => String::from("BIEM!"),
    //         };
    //         InnerError::String(err)
    //         // rustler::Error::RaiseTerm(Box::new(err))

    //         // rquickjs::Error::Exception {
    //         //     message: err,
    //         //     file: String::from("none"),
    //         //     line: 0,
    //         //     stack: String::new(),
    //         // }
    //     });

    //     Ok(res)
    // });

    let child = inner_spawn_run(script);

    // let out = child.join().map_err(|e| {
    // let err = match e.downcast_ref::<Box<dyn Display>>() {
    //     Some(e) => e.to_string(),
    //     None => String::from("BIEM!"),
    // };

    //     // rustler::Error::RaiseTerm(Box::new(err))
    //     InnerError::String(err)

    //     // rquickjs::Error::Exception {
    //     //     message: err,
    //     //     file: String::from("none"),
    //     //     line: 0,
    //     //     stack: String::new(),
    //     // }
    // })?;

    let out = child.join().map_err(|e| {
        let err = match e.downcast_ref::<Box<dyn Display>>() {
            Some(e) => e.to_string(),
            None => String::from("BIEM!"),
        };

        InnerError::String(err)
    })??;

    Ok((atom::ok(), out))
}

fn inner_spawn_run(script: &str) -> JoinHandle<Result<String, InnerError>> {
    let script = script.to_string();
    let builder = std::thread::Builder::new().stack_size(STACK_SIZE);

    let child = builder
        .spawn(move || {
            let res = std::panic::catch_unwind(|| inner_run(&script)).map_err(|e| {
                let err = match e.downcast_ref::<Box<dyn Display>>() {
                    Some(e) => e.to_string(),
                    None => String::from("BIEM!"),
                };
                InnerError::String(err)
            })??;

            Ok(res)
        })
        .expect("IO error");

    child
}

fn inner_run(script: &str) -> Result<String, InnerError> {
    let rt = Runtime::new().map_err(|e| InnerError::String(e.to_string()))?;
    rt.set_memory_limit(10 * 1024 * 1024);
    rt.set_max_stack_size(STACK_SIZE);

    let ctx = Context::full(&rt).map_err(|e| InnerError::String(e.to_string()))?;

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
        .map_err(|e| InnerError::String(e.to_string()))?;
    Ok(out.0.to_string())
}

rustler::init!("Elixir.QuickJs.Native", [run]);
