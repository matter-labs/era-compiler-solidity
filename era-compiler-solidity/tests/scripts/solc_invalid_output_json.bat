@ECHO OFF

if "%~1"=="--version" (
    echo solc, the solidity compiler commandline interface
    echo Version: 0.8.28+commit.deadbeef.platform.toolchain
    echo ZKsync: 0.8.28-1.0.1
    exit /b 0
)

echo Invalid JSON output
