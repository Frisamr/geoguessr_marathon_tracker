; vim: ts=4 sw=4 et
#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.

; #SingleInstance ; TODO
SetTitleMatchMode, 2 ; A window's title can contain WinTitle anywhere inside it to be a match.
CoordMode Mouse, Client ; Makes mouse movements relative to client
                        ; (what I've been grabbing from Window Spy)

windows_to_switch_from := ["Edge", "PowerShell"] ; TODO: remove

; Coords on my screen
; 1250, 1020
; 1365, 1050

; F1::
;     MouseClickDrag Left, 1250, 1020, 1365, 1050, 100
;     SendInput, ^c
; return

F2::
    if checkForNextGameButton()
    {
        addScore()
    }
return

addScore()
{
    ; WinActivate, LibreWolf
    ; Sleep, 10

    MouseClickDrag Left, 1250, 1020, 1365, 1050, 100
    SendInput, ^c
    Sleep, 10

    WinActivate, PowerShell
    Sleep, 10

    SendInput, ^v
    Sleep, 10
    SendInput, {Enter}
    Sleep, 10

    WinActivate, LibreWolf
    Sleep, 10

    SendInput, {Space}
}

; Button colour: 6C B9 28, so 28 B9 6C in BGR
; left edge coords: 1200, 1030
checkForNextGameButton()
{
    pixel_exists := 0
    Loop, 50
    {
        PixelGetColor, colour, 1200, 1030
        If (colour = "0x28B96C")
        {
            pixel_exists := 1
            break
        }
        Sleep, 20
    }
    ;MsgBox, next game button check: %pixel_exists%
    return pixel_exists
}

; F1::
;     ; PixelGetColor, colour, 1200, 1030
;     ; MsgBox, colour: |%colour%|
;
;     x := checkForNextGameButton()
;     MsgBox check: %x%
; return

; OUTDATED STUFF BELOW

$F5::
    If MultiWinActive(windows_to_switch_from*)
    {
        5kTrackerCommand("n")
    }
    Else
    {
        repeatOriginalKey("F5")
    }
return

$F3::
    If MultiWinActive(windows_to_switch_from*)
    {
        5kTrackerCommand("y")
    }
    Else
    {
        repeatOriginalKey("F3")
    }
return

$F4::
    If MultiWinActive(windows_to_switch_from*)
    {
        5kTrackerCommand("d")
    }
    Else
    {
        repeatOriginalKey("F4")
    }
return

5kTrackerCommand(command)
{
    ;MsgBox, "5k tracker command"
    WinActivate, PowerShell
    Sleep, 30

    SendInput, %command%
    Sleep, 30
    SendInput, {Enter}

    Sleep, 30
    WinActivate, Edge
}

repeatOriginalKey(original_key)
{
        ;MsgBox, "original key repeated"
        SendInput, {%original_key% down}
        Sleep, 30
        SendInput, {%original_key% up}
}

MultiWinActive(win_titles*)
{
    result := 0
    for index,win_title in win_titles
        If WinActive(win_title)
        {
            result := 1
        }
    return result
}

/*F7::
    If MultiWinActive(windows_to_switch_from*) {
        MsgBox, window list matched
    } Else {
        MsgBox, window list failed to match
    }
return*/

/*F4::
    WinGetActiveTitle, Title
    MsgBox, The active window is "%Title%".
return*/
