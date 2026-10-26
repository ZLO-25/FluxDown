//! Windows 安装版（Inno Setup，每用户）：不在运行期替换文件，而是校验安装器，
//! 由重启计划在 agent 完全退出后静默运行它（`CloseApplications=force` 关残余进程，
//! `[Run]` 在静默安装结束时带 `--after-update` 重新拉起桌面）。
//!
//! 校验除 PE 头外还比对 Authenticode 签名：规则见 [`super::authenticode`]。
//!
//! 便携版走 [`super::apply_portable`]。

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::authenticode::{self, Signature};
use super::{InstallError, detect, output_within};
use crate::update::restart::RestartPlan;

/// 安装器日志文件名（位于该版本的暂存目录）。
const SETUP_LOG: &str = "setup.log";

/// 读取签名的 PowerShell 脚本；目标路径经环境变量传入，不拼进脚本源码。
const SIGNATURE_SCRIPT: &str = "$ErrorActionPreference = 'Stop'; \
$s = Get-AuthenticodeSignature -LiteralPath $env:FLUXDOWN_SIGNATURE_PATH; \
Write-Output $s.Status.ToString(); \
if ($null -ne $s.SignerCertificate) { Write-Output $s.SignerCertificate.Subject } else { Write-Output '' }";
const SIGNATURE_PATH_ENV: &str = "FLUXDOWN_SIGNATURE_PATH";
const SIGNATURE_TIMEOUT: Duration = Duration::from_secs(60);

pub(super) async fn apply_setup(
    package: &Path,
    work_dir: &Path,
) -> Result<RestartPlan, InstallError> {
    let installer = package.to_path_buf();
    let work = work_dir.to_path_buf();
    super::blocking(move || check_executable(&installer, &work)).await?;
    verify_signature(package).await?;
    Ok(RestartPlan::installer(
        package.to_path_buf(),
        work_dir.join(SETUP_LOG),
    ))
}

fn check_executable(package: &Path, work_dir: &Path) -> Result<(), InstallError> {
    let is_exe = package
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"));
    if !is_exe {
        return Err(InstallError::InvalidPackage(format!(
            "installer must be an .exe: {}",
            package.display()
        )));
    }
    let mut magic = [0_u8; 2];
    File::open(package)
        .and_then(|mut file| file.read_exact(&mut magic))
        .map_err(InstallError::storage(format!(
            "read installer {}",
            package.display()
        )))?;
    if &magic != b"MZ" {
        return Err(InstallError::InvalidPackage(format!(
            "{} is not a Windows executable",
            package.display()
        )));
    }
    std::fs::create_dir_all(work_dir).map_err(InstallError::storage(format!(
        "create {}",
        work_dir.display()
    )))
}

/// 新安装器与正在运行的程序比对 Authenticode 签名。
async fn verify_signature(installer: &Path) -> Result<(), InstallError> {
    let current_exe = detect::current_exe_path().ok_or_else(|| InstallError::Storage {
        context: "locate the running executable".to_owned(),
        source: io::Error::new(io::ErrorKind::NotFound, "current executable unknown"),
    })?;
    let current = read_signature(&current_exe).await?;
    let new = read_signature(installer).await?;
    authenticode::check(&current, &new).map_err(InstallError::InvalidPackage)?;
    if current.status != authenticode::SignatureStatus::Valid {
        tracing::warn!(
            installer = %installer.display(),
            "running build has no valid Authenticode signature; installer signer not compared"
        );
    }
    Ok(())
}

async fn read_signature(path: &Path) -> Result<Signature, InstallError> {
    let powershell = powershell_path();
    let mut command = tokio::process::Command::new(&powershell);
    command
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            SIGNATURE_SCRIPT,
        ])
        .env(SIGNATURE_PATH_ENV, path);
    let output = output_within(command, &powershell, SIGNATURE_TIMEOUT).await?;
    if !output.status.success() {
        return Err(InstallError::Command(format!(
            "read the Authenticode signature of {}: {}",
            path.display(),
            super::stderr_tail(&output.stderr)
        )));
    }
    Ok(Signature::parse(&String::from_utf8_lossy(&output.stdout)))
}

/// 系统自带的 Windows PowerShell 绝对路径（不经 PATH 查找，避免被同名程序劫持）。
fn powershell_path() -> PathBuf {
    std::env::var_os("SystemRoot")
        .map_or_else(|| PathBuf::from(r"C:\Windows"), PathBuf::from)
        .join("System32")
        .join("WindowsPowerShell")
        .join("v1.0")
        .join("powershell.exe")
}
