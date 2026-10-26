//! Windows 安装器的 Authenticode 签名判定（纯逻辑，便于在任意平台测试）。
//!
//! 签名信息由 `windows.rs` 经 PowerShell `Get-AuthenticodeSignature` 读取，输出两行：
//! 第一行 `Status`（`Valid` / `NotSigned` / `HashMismatch` / `NotTrusted` …），
//! 第二行签名证书的 `Subject`（无签名时为空）。
//!
//! 规则与 macOS 的 TeamIdentifier 校验同一思路——新包必须和正在运行的程序同源：
//! - 新安装器带签名但状态不是 `Valid`（被篡改、证书不受信任）：一律拒绝；
//! - 当前程序有有效签名：新安装器必须同样有效，且签名证书 Subject 一致（证书续期 Subject 不变）；
//! - 当前程序没有有效签名（自行构建、未签名的发行版）：无从比对，允许 `Valid` 或 `NotSigned`。

/// 一个文件的 Authenticode 签名状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Signature {
    pub status: SignatureStatus,
    /// 签名证书 Subject；没有签名时为 `None`。
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SignatureStatus {
    Valid,
    NotSigned,
    /// 其余状态（`HashMismatch`、`NotTrusted`、`UnknownError` …），原样保留便于报错。
    Invalid(String),
}

impl Signature {
    /// 解析 PowerShell 脚本的两行输出；状态行缺失按无效处理。
    pub(super) fn parse(output: &str) -> Self {
        let mut lines = output.lines().map(str::trim);
        let status = match lines.next().unwrap_or_default() {
            "Valid" => SignatureStatus::Valid,
            "NotSigned" => SignatureStatus::NotSigned,
            "" => SignatureStatus::Invalid("no status".to_owned()),
            other => SignatureStatus::Invalid(other.to_owned()),
        };
        let subject = lines
            .next()
            .filter(|subject| !subject.is_empty())
            .map(str::to_owned);
        Self { status, subject }
    }

    fn valid_subject(&self) -> Option<&str> {
        match self.status {
            SignatureStatus::Valid => self.subject.as_deref(),
            _ => None,
        }
    }
}

/// 判定新安装器能否替换当前程序；`Err` 为拒绝原因。
pub(super) fn check(current: &Signature, installer: &Signature) -> Result<(), String> {
    if let SignatureStatus::Invalid(status) = &installer.status {
        return Err(format!("installer signature is not valid ({status})"));
    }
    let Some(expected) = current.valid_subject() else {
        return Ok(());
    };
    match installer.valid_subject() {
        Some(subject) if subject == expected => Ok(()),
        Some(subject) => Err(format!(
            "installer is signed by \"{subject}\", expected \"{expected}\""
        )),
        None => Err(format!(
            "installer is not signed, expected a signature from \"{expected}\""
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUBJECT: &str = "CN=FluxDown, O=FluxDown, C=CN";

    fn valid(subject: &str) -> Signature {
        Signature {
            status: SignatureStatus::Valid,
            subject: Some(subject.to_owned()),
        }
    }

    fn unsigned() -> Signature {
        Signature {
            status: SignatureStatus::NotSigned,
            subject: None,
        }
    }

    #[test]
    fn parses_status_and_subject() {
        assert_eq!(
            Signature::parse(&format!("Valid\r\n{SUBJECT}\r\n")),
            valid(SUBJECT)
        );
        assert_eq!(Signature::parse("NotSigned\r\n\r\n"), unsigned());
        assert_eq!(
            Signature::parse("HashMismatch\n"),
            Signature {
                status: SignatureStatus::Invalid("HashMismatch".to_owned()),
                subject: None,
            }
        );
        assert_eq!(
            Signature::parse("").status,
            SignatureStatus::Invalid("no status".to_owned())
        );
    }

    #[test]
    fn signed_install_requires_the_same_signer() {
        assert!(check(&valid(SUBJECT), &valid(SUBJECT)).is_ok());
        assert!(check(&valid(SUBJECT), &valid("CN=Someone Else")).is_err());
        assert!(check(&valid(SUBJECT), &unsigned()).is_err());
    }

    #[test]
    fn tampered_installer_is_always_rejected() {
        let tampered = Signature::parse(&format!("HashMismatch\n{SUBJECT}"));
        assert!(check(&valid(SUBJECT), &tampered).is_err());
        assert!(check(&unsigned(), &tampered).is_err());
    }

    #[test]
    fn unsigned_install_accepts_signed_or_unsigned_installers() {
        assert!(check(&unsigned(), &unsigned()).is_ok());
        assert!(check(&unsigned(), &valid(SUBJECT)).is_ok());
        let untrusted_current = Signature::parse(&format!("NotTrusted\n{SUBJECT}"));
        assert!(check(&untrusted_current, &unsigned()).is_ok());
    }
}
