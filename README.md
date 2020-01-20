# PIKA_MOCK
WinAPI to WASM for https://github.com/disjukr/PIKA_V

but there are only features for game animation, scene, event, keyboard Input(e.g DC, GDI, Keyboard, Timer, Rect...etc). so there are no audio like midi, windows event, File I/O, AI.

:감기 휴무중:

## reference
- https://soen.kr
- ~~https://github.com/timhutton/sdl-canvas-wasm~~
- https://docs.rs/winapi/0.3.8/winapi/

## target WinAPI
- http://soen.kr/lecture/win32api/reference/Function/MoveWindow.htm
- http://soen.kr/lecture/win32api/reference/Function/GetWindowRect.htm
- http://soen.kr/lecture/win32api/reference/Function/BeginPaint.htm
- http://soen.kr/lecture/win32api/reference/Function/GetDC.htm
- GetDeviceCaps : http://www.dreamy.pe.kr/zbxe/CodeClip/14889
- GetKeyboardState : http://soen.kr/project/dangeun/dg1/5-3-2.htm
- https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getnearestpaletteindex
- https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setrect
- https://m.blog.naver.com/PostView.nhn?blogId=tipsware&logNo=220975813784&proxyReferer=https%3A%2F%2Fwww.google.com%2F
- https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdibcolortable
- https://m.blog.naver.com/PostView.nhn?blogId=sukdoo99&logNo=220984109882&proxyReferer=https%3A%2F%2Fwww.google.com%2F
- https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpalette
- http://soen.kr/lecture/win32api/reference/Function/SelectObject.htm

## (WIP) retdec
