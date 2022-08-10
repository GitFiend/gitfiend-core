use std::env;
use std::path::PathBuf;

pub fn _fake_action_script_path() -> Option<PathBuf> {
  Some(
    env::current_dir()
      .ok()?
      .parent()?
      .join("git-fiend")
      .join("scripts")
      .join("fake-action.sh"),
  )
}

// pub fn run_fake_action(_: &ActionOptions) -> Result<ActionOutput, ActionError> {
//   let mut cmd = Command::new(_fake_action_script_path().expect("Fake action script path"))
//     .stdout(Stdio::piped())
//     .spawn()?;
//
//   let mut lines: Vec<String> = Vec::new();
//
//   let stdout = cmd
//     .stdout
//     .as_mut()
//     .ok_or_else(|| ActionError::IO("Failed to get stdout as mut".to_string()))?;
//
//   let stdout_reader = BufReader::new(stdout);
//   let stdout_lines = stdout_reader.lines();
//
//   for line in stdout_lines.flatten() {
//     ACTION_LOGS.push(ActionProgress::Out(line.clone()));
//     println!("{}", line);
//
//     lines.push(line);
//   }
//
//   let status = cmd.wait()?;
//
//   let mut stderr = String::new();
//
//   if let Some(mut err) = cmd.stderr {
//     if let Ok(len) = err.read_to_string(&mut stderr) {
//       if len > 0 {
//         ACTION_LOGS.push(ActionProgress::Err(stderr.clone()));
//       }
//     }
//   }
//
//   if !status.success() {
//     return if has_credential_error(&stderr) {
//       Err(Credential)
//     } else {
//       Err(ActionError::Git {
//         stdout: lines.join(""),
//         stderr: stderr.clone(),
//       })
//     };
//   }
//
//   Ok(ActionOutput {
//     stdout: lines.join("\n"),
//     stderr,
//   })
// }
