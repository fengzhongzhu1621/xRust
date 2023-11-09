use windows::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

#[test]
/// 遍历所有可见的窗口，打印标题和坐标
fn test_enum_window() -> windows::core::Result<()> {
    unsafe { EnumWindows(Some(enum_window), LPARAM(0)) }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        // 获得窗口的标题
        let len = GetWindowTextW(window, &mut text);
        // 解码 UTF-16 编码的切片v成一个String, 将无效数据替换为替换字符(U+FFFD).
        let text = String::from_utf16_lossy(&text[..len as usize]);

        // 检索有关指定窗口的信息。
        let mut info = WINDOWINFO {
            // 结构大小（以字节为单位）。 调用方必须将此成员设置为 sizeof(WINDOWINFO)。
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(window, &mut info).unwrap();

        if !text.is_empty() && info.dwStyle.contains(WS_VISIBLE) {
            println!(
                "{} ({}, {})",
                text, info.rcWindow.left, info.rcWindow.top
            );
        }

        true.into()
    }
}
