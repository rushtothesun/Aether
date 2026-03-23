@echo off
echo Starting Aether Production Build...
echo.

cd Website
echo Checking dependencies...
call npm install
echo.

cd src-tauri
call npx tauri build --no-bundle

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo BUILD FAILED! Please check the errors above.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo ======================================================
echo BUILD COMPLETE!
echo.
echo Your executable is located at:
echo Website\src-tauri\target\release\Aether.exe
echo ======================================================
echo.
pause
