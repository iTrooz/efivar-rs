REM Install cargo make if needed
cargo make -V

IF %ERRORLEVEL% NEQ 0 (
    wget -qO cargo-make.zip "https://github.com/sagiegurari/cargo-make/releases/download/0.18.0/cargo-make-v0.18.0-x86_64-pc-windows-msvc.zip"
    7z x cargo-make.zip
    move cargo-make.exe %USERPROFILE%\.cargo\bin
)

REM Build
cargo make workspace-ci-flow --no-workspace
