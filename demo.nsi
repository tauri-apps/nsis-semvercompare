Name "demo"
OutFile "demo.exe"
Unicode true
ShowInstDetails show

!addplugindir ".\target\i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_TARGET_DIR%\i686-pc-windows-msvc\release"
!addplugindir "$%CARGO_BUILD_TARGET_DIR%\i686-pc-windows-msvc\release"

!include "MUI2.nsh"

!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_LANGUAGE "English"

Section
    nsis_semvercompare::SemverCompare "1.0.0" "1.1.0"
    Pop $1
    DetailPrint $1
    nsis_process::FindProcess "explorer.exe"
    Pop $1
    DetailPrint $1
    nsis_process::FindProcess "abcdef.exe"
    Pop $1
    DetailPrint $1
    ; nsis_download::Download "https://go.microsoft.com/fwlink/p/?LinkId=2124703" "wv2setup.exe"
    ; Pop $1
    ; DetailPrint $1
SectionEnd