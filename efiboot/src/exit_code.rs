#[derive(Debug)]
pub enum ExitCode {
    SUCCESS,
    FAILURE,
    FAILURE1(u8),
}

impl PartialEq for ExitCode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FAILURE1(l0), Self::FAILURE1(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(exit_code: ExitCode) -> Self {
        match exit_code {
            ExitCode::SUCCESS => std::process::ExitCode::SUCCESS,
            ExitCode::FAILURE => std::process::ExitCode::FAILURE,
            ExitCode::FAILURE1(code) => std::process::ExitCode::from(code),
        }
    }
}
