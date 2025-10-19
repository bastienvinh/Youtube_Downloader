; NSIS Installer for Youtube Downloader
; This installer is used with the Docker build for Windows

!include "MUI2.nsh"
!include "x64.nsh"

; Basic Settings
Name "Youtube Downloader"
OutFile "$OUTDIR\youtube-dl-installer.exe"
InstallDir "$PROGRAMFILES\youtube-dl"
InstallDirRegKey HKCU "Software\youtube-dl" ""

; Require admin privileges
RequestExecutionLevel admin

; MUI Settings
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

; Installer Sections
Section "Install"
  SetOutPath "$INSTDIR"
  
  ; Copy application files from build output
  File /r "target\release\youtube-dl.exe"
  
  ; Create Start Menu shortcuts
  SetShellVarContext all
  CreateDirectory "$SMPROGRAMS\youtube-dl"
  CreateShortcut "$SMPROGRAMS\youtube-dl\Youtube Downloader.lnk" "$INSTDIR\youtube-dl.exe"
  CreateShortcut "$SMPROGRAMS\youtube-dl\Uninstall.lnk" "$INSTDIR\uninstall.exe"
  
  ; Create Desktop shortcut
  CreateShortcut "$DESKTOP\Youtube Downloader.lnk" "$INSTDIR\youtube-dl.exe"
  
  ; Write uninstaller
  WriteUninstaller "$INSTDIR\uninstall.exe"
  
  ; Write registry entries
  WriteRegStr HKCU "Software\youtube-dl" "" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\youtube-dl" "DisplayName" "Youtube Downloader"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\youtube-dl" "UninstallString" "$INSTDIR\uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\youtube-dl" "DisplayVersion" "0.1.0"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\youtube-dl" "Publisher" "strifep"
SectionEnd

; Uninstaller Section
Section "Uninstall"
  SetShellVarContext all
  
  ; Remove shortcuts
  RMDir /r "$SMPROGRAMS\youtube-dl"
  Delete "$DESKTOP\Youtube Downloader.lnk"
  
  ; Remove files
  Delete "$INSTDIR\youtube-dl.exe"
  Delete "$INSTDIR\uninstall.exe"
  RMDir "$INSTDIR"
  
  ; Remove registry entries
  DeleteRegKey HKCU "Software\youtube-dl"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\youtube-dl"
SectionEnd
