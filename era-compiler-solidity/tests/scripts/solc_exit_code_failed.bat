@ECHO OFF

if "%~1"=="--version" (
    echo solc, the solidity compiler commandline interface
    echo Version: 0.8.30+commit.deadbeef.platform.toolchain
    echo ZKsync: 0.8.30-1.0.2
    exit /b 0
)

setlocal EnableDelayedExpansion
set "INPUT="
:readloop
    set "LINE="
    set /p LINE=
    if errorlevel 1 goto done

    set "INPUT=!INPUT!!LINE!
    "
    goto readloop
:done

exit /b 1
