# PIKA_MOCK
WinAPI to WASM for https://github.com/disjukr/PIKA_V

but there are only features for game animation, scene, event, keyboard Input(e.g DC, GDI, Keyboard, Timer, Rect...etc). so there are no audio like midi, windows event, File I/O, AI.


## build & run
- i used VS2019
- need install npm
- need install Rust
  - `cargo build` at `/`

### PIKA_MOCK (C++/WinAPI32)
- excute PIKA_MOCK.sln
- `F5`

### pika-game (Rust)
- this is just game logic & test

### pika-wasm (Rust, wasm-pack)
1. `wasm-pack build pika-wasm`
2. `cd pika-wasm`
3. `npm run serve`
   1. need `npm install`

### pika-winapi (Rust, winapi crate)
- `cargo run -p pika-winapi`


## ToDo

### target WinAPI
- [ ] http://soen.kr/lecture/win32api/reference/Function/MoveWindow.htm
- [ ] http://soen.kr/lecture/win32api/reference/Function/GetWindowRect.htm
- [ ] http://soen.kr/lecture/win32api/reference/Function/BeginPaint.htm
- [ ] http://soen.kr/lecture/win32api/reference/Function/GetDC.htm
- [ ] GetDeviceCaps : http://www.dreamy.pe.kr/zbxe/CodeClip/14889
- [ ] GetKeyboardState : http://soen.kr/project/dangeun/dg1/5-3-2.htm
- [ ] https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getnearestpaletteindex
- [ ] https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setrect
- [ ] https://m.blog.naver.com/PostView.nhn?blogId=tipsware&logNo=220975813784&proxyReferer=https%3A%2F%2Fwww.google.com%2F
- [ ] https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setdibcolortable
- [ ] https://m.blog.naver.com/PostView.nhn?blogId=sukdoo99&logNo=220984109882&proxyReferer=https%3A%2F%2Fwww.google.com%2F
- [ ] https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpalette
- [ ] http://soen.kr/lecture/win32api/reference/Function/SelectObject.htm

### retdec of PIKA
- https://github.com/disjukr/PIKA_V/blob/master/retdec/PIKA_V.exe.c


## reference

### winapi
- https://soen.kr

### wasm
- ~~https://github.com/timhutton/sdl-canvas-wasm~~
- https://developer.mozilla.org/ko/docs/WebAssembly/Rust_to_wasm
- https://developer.mozilla.org/ko/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/compileStreaming

### rust
- https://docs.rs/winapi/0.3.8/winapi/
- https://github.com/segfault87/ldraw.rs
- https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
