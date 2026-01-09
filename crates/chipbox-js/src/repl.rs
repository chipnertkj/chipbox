use std::io::Write as _;

use miette::{Context as _, IntoDiagnostic as _};

use crate::{DisplayJsValue as _, runtime::Runtime};

/// ## `!Send`
/// This function uses a `QuickJS` runtime, which is `!Send`.
#[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
pub async fn repl(js_runtime: &Runtime) -> miette::Result<()> {
    loop {
        print!("js > ");
        std::io::stdout()
            .flush()
            .into_diagnostic()
            .wrap_err("flush stdout")?;
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .into_diagnostic()
            .wrap_err("read line")?;
        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if input.starts_with('/') {
            match input.get(1..).unwrap_or_default() {
                "exit" => {
                    println!("exiting js repl...");
                    break Ok(());
                }
                name => println!("unknown command: `{name}`"),
            }
            continue;
        }
        let result = js_runtime
            .with_async_eval(line, |_ctx, js_value| Ok(js_value.pretty_display()))
            .await;
        let value = match result {
            Ok(result) => result,
            Err(e) => {
                let stack_trace = e.stack_trace().map(ToString::to_string);
                eprintln!("{:?}", miette::Report::from_err(e));
                if let Some(stack_trace) = stack_trace {
                    eprintln!("stack trace: {stack_trace}");
                }
                continue;
            }
        };
        println!("{value}");
    }
}
