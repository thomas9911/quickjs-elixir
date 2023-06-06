use rquickjs::{Context, Func, Object, Runtime};

fn run(script: &str) -> Result<(), String> {
    let rt = Runtime::new().map_err(|e| e.to_string())?;
    rt.set_memory_limit(1024 * 1024);
    rt.set_max_stack_size(1024);
    let ctx = Context::full(&rt).map_err(|e| e.to_string())?;

    let out = ctx
        .with(|ctx| {
            let out = ctx.eval(script)?;
            Ok::<(), rquickjs::Error>(out)
        })
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
    function ack(m,n)
    {
        if (m == 0)
            {
                return n + 1;
            }
            else if((m > 0) && (n == 0))
            {
                return ack(m - 1, 1);
            }
            else if((m > 0) && (n > 0))
            {
                return ack(m - 1, ack(m, n - 1));
            }else
        return n + 1;
    }

    ack(5, 3);
    12
    
    "#;

    run(script)?;

    Ok(())
}
