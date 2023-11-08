use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn test_create_window_ex_a() -> Result<()> {
    unsafe {
        // 检索指定模块的模块句柄。 模块必须已由调用进程加载。
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        // 窗口类属性。
        let wc = WNDCLASSA {
            // 类游标的句柄。 此成员必须是游标资源的句柄。
            // 如果此成员为 NULL，则每当鼠标移动到应用程序的窗口中时，应用程序都必须显式设置光标形状。
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            // 实例的句柄，该实例包含类的窗口过程。
            hInstance: instance.into(),
            // 窗口类名
            lpszClassName: window_class,
            // 类样式的任意组合
            style: CS_HREDRAW | CS_VREDRAW,
            // 指向窗口过程的指针。 必须使用 CallWindowProc 函数调用窗口过程。
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        // 注册一个窗口类，以便在调用 CreateWindow 或 CreateWindowEx 函数时使用。
        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        // 创建具有扩展窗口样式的重叠窗口、弹出窗口窗口或子窗口
        CreateWindowExA(
            // 正在创建的窗口的扩展窗口样式
            WINDOW_EX_STYLE::default(),
            // 窗口类名
            window_class,
            // 窗口名称。 如果窗口样式指定标题栏，则 lpWindowName 指向的窗口标题将显示在标题栏中。
            s!("This is a sample window"),
            // 正在创建的窗口的样式。
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            // 窗口的初始水平位置。 对于重叠或弹出窗口， x 参数是窗口左上角的初始 x 坐标（以屏幕坐标表示）。
            // 对于子窗口， x 是窗口左上角相对于父窗口工作区左上角的 x 坐标。
            CW_USEDEFAULT,
            // 窗口的初始垂直位置。 对于重叠或弹出窗口， y 参数是窗口左上角的初始 y 坐标（以屏幕坐标表示）。
            // 对于子窗口， y 是子窗口左上角相对于父窗口工作区左上角的初始 y 坐标。
            CW_USEDEFAULT,
            // 窗口的宽度（以设备单位为单位）。 对于重叠窗口， nWidth 是窗口的宽度、屏幕坐标或 CW_USEDEFAULT。 如果 nWidthCW_USEDEFAULT，系统将为窗口选择默认宽度和高度;默认宽度从初始 x 坐标扩展到屏幕的右边缘;默认高度从初始 y 坐标扩展到图标区域的顶部。
            // CW_USEDEFAULT 仅对重叠窗口有效;
            // 如果为弹出窗口或子窗口指定 了CW_USEDEFAULT ，则 nWidth 和 nHeight 参数设置为零。
            CW_USEDEFAULT,
            // 窗口的高度（以设备单位为单位）。
            // 对于重叠窗口， nHeight 是窗口的高度（以屏幕坐标为单位）。
            // 如果 nWidth 参数设置为 CW_USEDEFAULT，则系统将忽略 nHeight。
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        // 从调用线程的消息队列中检索消息。 函数调度传入的已发送消息，直到已发布的消息可供检索。
        // 如果函数检索 WM_QUIT以外的消息，则返回值为非零值。
        // 如果函数检索 WM_QUIT 消息，则返回值为零。
        while GetMessageA(&mut message, None, 0, 0).into() {
            // 将消息调度到窗口过程。 它通常用于调度 GetMessage 函数检索到的消息。
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

// 一个回调函数，可在应用程序中定义，用于处理发送到窗口的消息。
// WNDPROC 类型定义指向此回调函数的指针。 WndProc 名称是应用程序中定义的函数名称的占位符。
extern "system" fn wndproc(
    window: HWND,   // 窗口的句柄。 此参数通常名为 hWnd。
    message: u32,   // 消息。 此参数通常命名为 uMsg。
    wparam: WPARAM, // 其他消息信息。 此参数通常名为 wParam。参数的内容取决于 uMsg 参数的值。
    lparam: LPARAM, // 其他消息信息。 此参数通常名为 lParam。参数的内容取决于 uMsg 参数的值。
) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                // 通过从指定窗口的更新区域中删除矩形来验证矩形中的工作区。
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                // 向系统指示线程已发出终止请求， (退出) 。 它通常用于响应 WM_DESTROY 消息。
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
