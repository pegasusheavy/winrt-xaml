@echo off
echo Building XAML Islands Helper DLL...

if not exist build mkdir build
cd build

cmake .. -G "Visual Studio 17 2022" -A x64
if errorlevel 1 (
    echo CMake configuration failed!
    exit /b 1
)

cmake --build . --config Debug
if errorlevel 1 (
    echo Build failed!
    exit /b 1
)

echo.
echo ✅ Build successful!
echo DLL location: build\bin\Debug\xaml_islands_helper.dll
echo Copied to: ..\target\debug\

cd ..

