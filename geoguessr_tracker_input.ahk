﻿; [[ Setup Stuff ]]
; vim: ts=4 : sw=4 : et : set filetype=autohotkey
#Requires AutoHotkey v2.0
#Warn   ; Enable warnings to assist with detecting common errors.
SendMode "Input"    ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir A_InitialWorkingDir   ; Ensures a consistent starting directory.
; #SingleInstance ; TODO
SetTitleMatchMode 2 ; A window's title can contain WinTitle anywhere inside it to be a match.
CoordMode "Mouse", "Client" ; Makes mouse movements relative to client
                            ; (what I've been grabbing from Window Spy)


; [[ End-user Configuration ]]

GEOGUESSR_WINDOW_TITLE := "GeoGuessr"    ; TODO: make sure this works
TRACKER_WINDOW_TITLE := "GeoMarathonTracker"

USERSCRIPT_TRIGGER_KEY := "{F19}"

IN_GAME_BANNER_X_COORD := 2170
IN_GAME_BANNER_Y_COORD := 215
IN_GAME_BANNER_COLOUR := "0x563B9A"

NEXT_GAME_BUTTON_X_COORD := 1480
NEXT_GAME_BUTTON_Y_COORD := 1420
NEXT_GAME_BUTTON_COLOUR := "0x6CB928"

CANCEL_BUTTON_X_OFFSET := 50
CANCEL_BUTTON_COLOUR := "0x484858"

ROUND_SCORE_LEFT_X_COORD := 1530
ROUND_SCORE_RIGHT_X_COORD := 1720
ROUND_SCORE_Y_COORD := 1420

INPUT_BOX_X_COORD := 6233
INPUT_BOX_Y_COORD := 1464

GEOGUESSR_WINDOW_X_COORD := 1300
GEOGUESSR_WINDOW_Y_COORD := 600

; Useful for testing purposes
; $*F3::
; {
;     MsgBox(WinGetTitle())
; }

; $*F4::
; {
;     Send(USERSCRIPT_TRIGGER_KEY)
; }


; [[ Actual Code Below ]]
; Don't touch unless you're prepared for things to break.

spam_check := 0

#HotIf WinActive(GEOGUESSR_WINDOW_TITLE)
$*F2::
{
    global spam_check
    if (spam_check = 1) {
        return
    }
    spam_check := 1

    if InGame() {
        SetKeyDelay(-1)
        Send("{F2 down}")
    }
    if NextGameButtonExists() {
        if CancelButtonExists() {
            Send("{Space}")
        }
        else {
            LogScore()
        }
    }
    else {
        SetKeyDelay(-1)
        Send("{F2 down}")
    }

    spam_check := 0
}

LogScore() {
    Sleep(10)
    MouseClickDrag("Left", ROUND_SCORE_LEFT_X_COORD, ROUND_SCORE_Y_COORD, ROUND_SCORE_RIGHT_X_COORD, ROUND_SCORE_Y_COORD, 100)
    Sleep(10)
    Send("^c")
    Sleep(40)

    WinActivate(TRACKER_WINDOW_TITLE)
    Sleep(10)
    DllCall("SetCursorPos", "int", INPUT_BOX_X_COORD, "int", INPUT_BOX_Y_COORD)
    Sleep(10)
    MouseClick
    Sleep(30)

    Send("^v")
    Sleep(10)
    Send("{Enter}")
    Sleep(30)

    WinActivate(GEOGUESSR_WINDOW_TITLE)
    Sleep(10)

    Send(USERSCRIPT_TRIGGER_KEY)

    DllCall("SetCursorPos", "int", GEOGUESSR_WINDOW_X_COORD, "int", GEOGUESSR_WINDOW_Y_COORD)
    Sleep(10)
    MouseClick
}

InGame()
{
    colour := PixelGetColor(IN_GAME_BANNER_X_COORD, IN_GAME_BANNER_Y_COORD)
    ; MsgBox(colour)
    If (colour = IN_GAME_BANNER_COLOUR)
    {
        return 1
    }
    Else
    {
        return 0
    }
}

NextGameButtonExists()
{
    pixel_exists := 0
    Loop 50
    {
        colour := PixelGetColor(NEXT_GAME_BUTTON_X_COORD, NEXT_GAME_BUTTON_Y_COORD)
        If (colour = NEXT_GAME_BUTTON_COLOUR)
        {
            pixel_exists := 1
            break
        }
        Sleep(20)
    }
    ;MsgBox("%pixel_exists%")
    return pixel_exists
}

CancelButtonExists()
{
    colour := PixelGetColor((NEXT_GAME_BUTTON_X_COORD + CANCEL_BUTTON_X_OFFSET), NEXT_GAME_BUTTON_Y_COORD)
    ; MsgBox(colour)
    If (colour = CANCEL_BUTTON_COLOUR)
    {
        return 1
    }
    Else
    {
        return 0
    }
}

; This broke things, so I'm commenting it out
; $*F2 UP::
; {
;     SetKeyDelay(-1)
;     ;Send "{F2 up}"
;     return
; }
