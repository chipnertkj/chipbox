//! Implementation for the `build` command.

use miette::{Context as _, IntoDiagnostic as _};

use crate::{
    command::{CargoProfile, Command},
    program::{Program, ProgramAction, ProgramDisplay, Task},
};

pub async fn program() -> miette::Result<Program> {
    let cargo_profile = CargoProfile::Release;

    let mut display = ProgramDisplay::new();

    // For status updates.
    // Stuff like "compiled sucessfully" etc.
    let status_pane = display.create_pane("status");
    // For build output from the commands we run.
    let output_pane = display.create_pane("output");

    // The only output in the status pane.
    let status_output = display.create_output("status", status_pane);
    // Outputs for each command.
    let pnpm_output = display.create_output("pnpm", output_pane);
    let cargo_output = display.create_output("cargo", output_pane);
    let cc_output = display.create_output("cc", output_pane);

    let mut program = Program::new("build", display);

    // Initialize commands.
    let (cmd_pnpm_install, cmd_build_frontend, cmd_gen_render_bindings, cmd_build_bin) =
        tokio::try_join!(
            Command::pnpm_install(),
            Command::pnpm_build_frontend(),
            Command::cargo_generate_render_bindings(),
            Command::cargo_build_chipbox(cargo_profile),
        )
        .into_diagnostic()
        .wrap_err("construct commands")?;

    // Define program tasks.
    let build_portaudio = Task::builder()
        .name("cc portaudio")
        .on_start([
            status_output.msg_action("Building PortAudio..."),
            ProgramAction::CompilePortAudio(cc_output),
        ])
        .add_to(&mut program);
    let frontend_deps = Task::builder()
        .name("pnpm install")
        .on_start([
            status_output.msg_action("Installing frontend dependencies..."),
            cmd_pnpm_install.with_output_to(pnpm_output),
        ])
        .add_to(&mut program);
    let gen_render_bindings = Task::builder()
        .name("gen render bindings")
        .on_start([
            status_output.msg_action("Generating render bindings..."),
            cmd_gen_render_bindings.with_output_to(cargo_output),
        ])
        .add_to(&mut program);
    let build_frontend = Task::builder()
        .name("pnpm build")
        .on_start([
            status_output.msg_action("Compiling frontend module..."),
            cmd_build_frontend.with_output_to(pnpm_output),
        ])
        .depend_on_tasks([frontend_deps, gen_render_bindings])
        .add_to(&mut program);
    let build_bin = Task::builder()
        .name("cargo build")
        .on_start([
            status_output.msg_action("Building final binary..."),
            cmd_build_bin.with_output_to(cargo_output),
        ])
        .on_finish([])
        .depend_on_tasks([build_frontend, build_portaudio])
        .add_to(&mut program);
    // Output dir message.
    let cargo_output_dir = crate::fs::cargo_output_dir(cargo_profile)
        .await
        .into_diagnostic()
        .wrap_err("find cargo output dir")?;
    let _finish = Task::builder()
        .on_start([
            status_output.msg_action(format!("Cargo output dir: {}", cargo_output_dir.display())),
            ProgramAction::finish(),
        ])
        .depend_on_task(build_bin);

    Ok(program)
}
