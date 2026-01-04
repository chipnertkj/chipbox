//! Implementation for the `dev` command.

use miette::{Context as _, IntoDiagnostic as _};

use crate::{
    command::{CargoProfile, Command},
    program::{Program, ProgramAction, ProgramDisplay, Task},
};

pub async fn program() -> miette::Result<Program> {
    let mut display = ProgramDisplay::new();

    // For status updates.
    // Stuff like "compiled sucessfully" etc.
    let status_pane = display.create_pane("status");
    // For build output from the commands we run.
    let output_pane = display.create_pane("output");

    // The only output in the status pane.
    let status_output = display.create_output("status", status_pane);
    // Outputs for different commands.
    let pnpm_install_output = display.create_output("pnpm install", output_pane);
    let pnpm_dev_output = display.create_output("pnpm dev", output_pane);
    let cargo_build_output = display.create_output("cargo build", output_pane);
    let cargo_bindings_output = display.create_output("cargo bindings", output_pane);
    let chipbox_output = display.create_output("chipbox", output_pane);

    let mut program = Program::new("build", display);

    // Initialize commands.
    let (cmd_pnpm_install, cmd_pnpm_dev, cmd_gen_render_bindings, cmd_build_bin, cmd_run) =
        tokio::try_join!(
            Command::pnpm_install(),
            Command::pnpm_dev(),
            Command::cargo_generate_render_bindings(),
            Command::cargo_build_chipbox(CargoProfile::Dev),
            Command::chipbox(CargoProfile::Dev, [] as [&str; _]),
        )
        .into_diagnostic()
        .wrap_err("construct commands")?;

    // Add tasks to run.
    let interrupt = Task::builder()
        .on_start([ProgramAction::WaitForSigint])
        .on_finish([ProgramAction::finish()])
        .add_to(&mut program);
    let frontend_deps = Task::builder()
        .name("pnpm install")
        .on_start([
            status_output.msg_action("Installing frontend dependencies..."),
            cmd_pnpm_install.with_output_to(pnpm_install_output),
        ])
        .watch_npm_pkg_defs(["chipbox-frontend", "chipbox-solid-render"])
        .abort_on_task_complete(interrupt)
        .add_to(&mut program);
    let gen_render_bindings = Task::builder()
        .name("gen render bindings")
        .on_start([
            status_output.msg_action("Generating render bindings..."),
            cmd_gen_render_bindings.with_output_to(cargo_bindings_output),
        ])
        .watch_crate("chipbox-render")
        .abort_on_task_complete(interrupt)
        .add_to(&mut program);
    let _pnpm_dev = Task::builder()
        .name("pnpm dev")
        .on_start([
            status_output.msg_action("Running vite dev server..."),
            cmd_pnpm_dev.with_output_to(pnpm_dev_output),
        ])
        .depend_on_tasks([frontend_deps, gen_render_bindings])
        .abort_on_task_complete(interrupt)
        .add_to(&mut program);
    let cargo_build = Task::builder()
        .name("cargo build")
        .on_start([
            status_output.msg_action("Building binary..."),
            cmd_build_bin.with_output_to(cargo_build_output),
        ])
        .watch_crate("chipbox")
        .abort_on_task_complete(interrupt)
        .add_to(&mut program);
    let _run = Task::builder()
        .name("run")
        .depend_on_task(cargo_build)
        .on_start([
            status_output.msg_action("Running binary..."),
            cmd_run.with_output_to(chipbox_output),
        ])
        .abort_on_task_complete(interrupt)
        .add_to(&mut program);

    Ok(program)
}
