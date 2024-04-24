extern crate winapi;

use winapi::um::winuser;
use winapi::shared::windef::POINT;
use std::thread;
use std::time::Duration;

fn main() {
    let mut old_cursor_pos = POINT { x: 0, y: 0 };
    unsafe {winuser::GetCursorPos(&mut old_cursor_pos);}
    loop {
        thread::sleep(Duration::from_nanos(300));
        unsafe {
            let mut cursor_pos = POINT { x: 0, y: 0 };
            winuser::GetCursorPos(&mut cursor_pos);
            if cursor_pos.x != old_cursor_pos.x || cursor_pos.y != old_cursor_pos.y{
                cursor_pos.x = (old_cursor_pos.x - cursor_pos.x)*3 + old_cursor_pos.x;
                cursor_pos.y = (old_cursor_pos.y - cursor_pos.y)*3 + old_cursor_pos.y;
                if cursor_pos.x.abs() <= winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) && cursor_pos.y.abs() <= winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN){
                    winuser::SetCursorPos(cursor_pos.x, cursor_pos.y);
                }
                if old_cursor_pos.x.abs() >= winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN)-5 {
                    winuser::SetCursorPos(winuser::GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN)-20, old_cursor_pos.y);
                }
                if old_cursor_pos.y.abs() >= winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN)-5 {
                    winuser::SetCursorPos(old_cursor_pos.x, winuser::GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN)-20);
                }
                if old_cursor_pos.x.abs() <= 5 {
                    winuser::SetCursorPos(20, old_cursor_pos.y);
                }
                if old_cursor_pos.y.abs() <= 5 {
                    winuser::SetCursorPos(old_cursor_pos.x, 20);
                }
            }
            winuser::GetCursorPos(&mut old_cursor_pos);
        }
    }
}
