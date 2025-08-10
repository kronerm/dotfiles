use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser as _;
use eyre::ContextCompat as _;

mod cli;

mod run {
    use std::{
        collections::HashMap,
        env, fs,
        marker::PhantomData,
        os,
        path::{Path, PathBuf},
        process::{Command, ExitStatus, Stdio},
    };

    /// gamescope -b ... -W ... -H ... -- proton run <exe>
    pub struct RunWrapper<T> {
        _phantom: PhantomData<T>,
        unmount_handle: crate::OverlayFsHandle,
        cwd: PathBuf,
        exe: String,
        args: Vec<String>,
        envs: HashMap<String, String>,
    }

    #[derive(Default)]
    pub struct WithExe;
    #[derive(Default)]
    pub struct WithOrWithoutProton;
    #[derive(Default)]
    pub struct WithOrWithoutGamescope;

    // impl<T> RunWrapper<T> {
    //     pub fn env(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
    //         self.envs.insert(key.into(), value.into()).unwrap();
    //         self
    //     }
    // }

    impl RunWrapper<WithExe> {
        pub fn new(
            cwd: PathBuf,
            exe_path: &Path,
            args: Vec<String>,
            unmount_handle: crate::OverlayFsHandle,
        ) -> Self {
            Self {
                _phantom: PhantomData,
                unmount_handle,
                exe: exe_path.to_str().unwrap().to_string(),
                cwd,
                args,
                envs: HashMap::new(),
            }
        }
    }

    impl RunWrapper<WithOrWithoutGamescope> {
        pub fn run(self) -> eyre::Result<ExitStatus> {
            tracing::info!(
                "running '{}' with args '{}' (cwd: {:?})",
                self.exe,
                self.args.join(" "),
                self.cwd
            );
            let mut child = Command::new(&self.exe)
                .current_dir(self.cwd)
                .args(&self.args)
                .envs(self.envs)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            tracing::info!("printing stdout and stderr of child process");

            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            use std::io::{BufRead as _, BufReader};

            let stdout_thread = std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    println!("{}", color_eyre::owo_colors::OwoColorize::dimmed(&line));
                }
            });
            let stderr_thread = std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    eprintln!("{}", color_eyre::owo_colors::OwoColorize::bright_red(&line));
                }
            });
            stdout_thread.join().unwrap();
            stderr_thread.join().unwrap();

            let output = child.wait_with_output()?;
            tracing::info!(
                "child process exited with exit code: {:?}",
                output.status.code()
            );

            Ok(output.status)
        }
    }

    impl RunWrapper<WithExe> {
        pub fn without_proton(self) -> RunWrapper<WithOrWithoutProton> {
            RunWrapper {
                _phantom: PhantomData,
                unmount_handle: self.unmount_handle,
                cwd: self.cwd,
                exe: self.exe,
                args: self.args,
                envs: self.envs,
            }
        }
        pub fn with_proton(
            self,
            gamerunner_dir: &Path,
            proton_path: &Path,
            prepend_proton_run: bool,
        ) -> eyre::Result<RunWrapper<WithOrWithoutProton>> {
            fs::create_dir_all(gamerunner_dir)?;

            let home_dir = gamerunner_dir.join("home");
            fs::create_dir_all(&home_dir)?;
            let prefix_dir = gamerunner_dir.join("prefix");
            let users_dir = prefix_dir.join("pfx").join("drive_c").join("users");
            fs::create_dir_all(&users_dir)?;
            let steamuser_dir = users_dir.join("steamuser");
            if !steamuser_dir.is_symlink() {
                if steamuser_dir.is_dir() {
                    fs::remove_dir_all(&steamuser_dir)?;
                };
                os::unix::fs::symlink(
                    pathdiff::diff_paths(&home_dir, &users_dir).unwrap(),
                    &steamuser_dir,
                )?;
            }

            let (exe, args) = if prepend_proton_run
                || ["winecfg", "winecfg.exe", "cmd", "cmd.exe"].contains(&self.exe.as_str())
            {
                (
                    proton_path.to_str().unwrap().to_string(),
                    ["run".to_string(), self.exe]
                        .into_iter()
                        .chain(self.args)
                        .collect(),
                )
            } else {
                (self.exe, self.args)
            };

            let ret = RunWrapper {
                _phantom: PhantomData,
                unmount_handle: self.unmount_handle,
                cwd: self.cwd,
                exe,
                args,
                envs: self
                    .envs
                    .into_iter()
                    .chain(
                        [
                            ("STEAM_COMPAT_CLIENT_INSTALL_PATH", "/dev/null"),
                            (
                                "STEAM_COMPAT_DATA_PATH",
                                fs::canonicalize(&prefix_dir)?.to_str().unwrap(),
                            ),
                            (
                                "WINEPREFIX",
                                fs::canonicalize(prefix_dir.join("pfx"))?.to_str().unwrap(),
                            ),
                            ("HOME", fs::canonicalize(gamerunner_dir)?.to_str().unwrap()),
                            ("PATH", &{
                                let proton_dir = proton_path.parent().unwrap();
                                let entry = [
                                    fs::canonicalize(proton_dir.join("files").join("bin"))?
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                    fs::canonicalize(proton_dir.join("protonfixes"))?
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                ]
                                .join(":");
                                if let Ok(var) = env::var("PATH") {
                                    format!("{}:{var}", entry)
                                } else {
                                    entry.to_string()
                                }
                            }),
                        ]
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string())),
                    )
                    .collect(),
            };

            Ok(ret)
        }
    }

    impl RunWrapper<WithOrWithoutProton> {
        pub fn without_gamescope(self) -> RunWrapper<WithOrWithoutGamescope> {
            RunWrapper {
                _phantom: PhantomData,
                unmount_handle: self.unmount_handle,
                cwd: self.cwd,
                exe: self.exe,
                args: self.args,
                envs: self.envs,
            }
        }
        pub fn with_gamescope(self) -> RunWrapper<WithOrWithoutGamescope> {
            RunWrapper {
                _phantom: PhantomData,
                unmount_handle: self.unmount_handle,
                cwd: self.cwd,
                exe: "gamescope".to_string(),
                args: [
                    "-b".to_string(),
                    "-W".to_string(),
                    "800".to_string(),
                    "-H".to_string(),
                    "600".to_string(),
                    "--".to_string(),
                    self.exe,
                ]
                .into_iter()
                .chain(self.args)
                .collect(),
                envs: self.envs,
            }
        }
    }
}

fn run_command(cmd: &str, args: &[&str]) -> eyre::Result<i32> {
    Command::new(cmd)
        .args(args)
        .status()?
        .code()
        .filter(|&code| code == 0)
        .context(format!("'{cmd} {}'", args.join(" ")))
}

trait Handle: std::fmt::Debug {}

#[derive(Debug)]
struct DwarfsMountHandle {
    _archive: PathBuf,
    dir: PathBuf,
}
impl Drop for DwarfsMountHandle {
    fn drop(&mut self) {
        tracing::info!("unmounting dwarfs: {self:#?}");
        run_command("umount", &[self.dir.to_str().unwrap()]).unwrap();
        fs::remove_dir(&self.dir).unwrap();
    }
}
impl Handle for DwarfsMountHandle {}

fn mount_dwarfs(archive: PathBuf, dir: PathBuf) -> eyre::Result<DwarfsMountHandle> {
    fs::create_dir_all(&dir)?;
    run_command(
        "dwarfs",
        &[archive.to_str().unwrap(), dir.to_str().unwrap()],
    )?;
    tracing::info!("mounted dwarfs ({archive:?} -> {dir:?})");
    Ok(DwarfsMountHandle {
        _archive: archive,
        dir,
    })
}

#[derive(Debug)]
struct SymlinkHandle {
    link: PathBuf,
}
impl Drop for SymlinkHandle {
    fn drop(&mut self) {
        fs::remove_file(&self.link).unwrap();
    }
}
impl Handle for SymlinkHandle {}

#[derive(Debug)]
struct OverlayFsHandle {
    lower_dirs: Vec<(Option<Box<dyn Handle>>, PathBuf)>,
    _upper_dir: PathBuf,
    work_dir: PathBuf,
    merged_dir: PathBuf,
}
impl Drop for OverlayFsHandle {
    fn drop(&mut self) {
        tracing::info!("unmounting overlayfs: {self:#?}");
        run_command("umount", &[self.merged_dir.to_str().unwrap()]).unwrap();

        fs::remove_dir(&self.merged_dir).unwrap();
        fs::remove_dir(self.work_dir.join("work")).unwrap();
        fs::remove_dir(&self.work_dir).unwrap();
        // we keep upper_dir, as we want to persist changes
        for (handle, _lower_dir) in &mut self.lower_dirs {
            drop(handle.take())
        }
        fs::remove_dir(self.lower_dirs[0].1.parent().unwrap()).unwrap();
    }
}
impl Handle for OverlayFsHandle {}

fn prepare_game_directory(
    gamerunner_dir: &Path,
    sources: Vec<PathBuf>,
) -> eyre::Result<OverlayFsHandle> {
    let overlay_dir = gamerunner_dir.join("overlay");
    let lower_dir = overlay_dir.join("lower");
    let upper_dir = overlay_dir.join("upper");
    let work_dir = overlay_dir.join("work");
    let merged_dir = overlay_dir.join("merged");

    fs::create_dir_all(&lower_dir)?;
    fs::create_dir_all(&upper_dir)?;
    fs::create_dir_all(&work_dir)?;
    fs::create_dir_all(&merged_dir)?;

    let mut lower_dirs: Vec<(Option<Box<dyn Handle>>, PathBuf)> = vec![];

    for (index, source) in sources.into_iter().enumerate() {
        if !source.exists() {
            eyre::bail!("source does not exist: {source:?}");
        }
        let lower_dir_path = lower_dir.join(index.to_string());
        if source.is_file() {
            lower_dirs.push((
                Some(Box::new(mount_dwarfs(source, lower_dir_path.clone())?)),
                lower_dir_path,
            ))
            // let (index, _archive) = dwarfs::Archive::new(fs::File::open(source)?)?;
            // if index
            //     .get_path(relative_exe_path.to_string_lossy().split('/'))
            //     .is_none()
            // {
            //     eyre::bail!("exe not found inside dwarfs archive");
            // }
        } else {
            std::os::unix::fs::symlink(source, &lower_dir_path)?;
            lower_dirs.push((
                Some(Box::new(SymlinkHandle {
                    link: lower_dir_path.clone(),
                })),
                lower_dir_path,
            ))
        }
    }

    run_command(
        "fuse-overlayfs",
        &[
            "-o",
            &format!(
                "lowerdir={},upperdir={},workdir={}",
                lower_dirs
                    .iter()
                    .map(|(_handle, lower_dir)| lower_dir.to_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(":"),
                upper_dir.to_str().unwrap(),
                work_dir.to_str().unwrap()
            ),
            merged_dir.to_str().unwrap(),
        ],
    )?;

    tracing::info!(
        "mounted overlayfs from sources: {:#?}",
        lower_dirs
            .iter()
            .map(|(handle_opt, _)| handle_opt.as_ref().unwrap())
            .collect::<Vec<_>>()
    );
    Ok(OverlayFsHandle {
        lower_dirs,
        _upper_dir: upper_dir,
        work_dir,
        merged_dir,
    })
}

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt().init();
    color_eyre::install()?;

    for cmd in ["dwarfs", "fuse-overlayfs"] {
        if which::which(cmd).is_err() {
            eyre::bail!("{cmd} command not available");
        }
    }

    let args = cli::CliArgs::parse();

    let wrapper =
        match args.game {
            cli::Game::NonSteam(non_steam_game) => {
                let current_dir = env::current_dir()?;
                let gamerunner_dir = current_dir.join(".gamerunner");

                let handle = prepare_game_directory(&gamerunner_dir, non_steam_game.sources)?;
                let game_dir = handle
                    .merged_dir
                    .clone()
                    .join(non_steam_game.relative_working_directory);

                match non_steam_game.platform {
                    cli::Platform::Native { path, args } => {
                        run::RunWrapper::new(game_dir, &path, args, handle).without_proton()
                    }
                    cli::Platform::Proton {
                        proton_path,
                        runnable,
                    } => match runnable {
                        cli::ProtonRunnable::Command { command_path, args } => {
                            run::RunWrapper::new(game_dir, &command_path, args, handle)
                                .with_proton(&gamerunner_dir, &proton_path, false)?
                        }
                        cli::ProtonRunnable::ExeFile { exe, args } => run::RunWrapper::new(
                            game_dir, &exe, args, handle,
                        )
                        .with_proton(&gamerunner_dir, &proton_path, true)?,
                    },
                }
            }
        };

    let gamescope = false;
    let wrapper = if gamescope {
        wrapper.with_gamescope()
    } else {
        wrapper.without_gamescope()
    };
    wrapper.run()?;

    Ok(())
}
