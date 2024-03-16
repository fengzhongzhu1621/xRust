use windows::core::Result;
use windows::{
    core::*, Win32::Foundation::*, Win32::System::DataExchange::*,
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*, Win32_UI_WindowsAndMessaging,
};

fn main() -> Result<()> {
    // 执行事件循环
    let mut window = Window::new()?;
    window.run()
}

struct Window {
    handle: HWND, // 窗口句柄
}

impl Window {
    pub fn new() -> Result<Self> {
        Ok(Window { handle: HWND(0) })
    }

    pub fn run(&mut self) -> Result<()> {
        unsafe {
            // 检索指定模块的模块句柄。 模块必须已由调用进程加载。
            let instance = GetModuleHandleA(None)?;
            debug_assert!(instance.0 != 0);

            // 将字符串常量转换为字符串所在的指针
            let window_class = s!("ClipboardMonitor");

            // 窗口类属性
            let wc = WNDCLASSA {
                // 实例的句柄，该实例包含类的窗口过程。
                hInstance: instance.into(), // HMODULE -> HINSTANCE
                // 窗口类名
                lpszClassName: window_class,
                // 指向窗口过程的指针。 必须使用 CallWindowProc 函数调用窗口过程。
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            // 注册一个窗口类，以便在调用 CreateWindow 或 CreateWindowEx 函数时使用。
            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            // 创建一个可见窗口，仅用于消息循环
            let handle = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                // WS_EX_TOOLWINDOW
                // | WS_EX_NOACTIVATE
                // | WS_EX_TRANSPARENT
                // | WS_EX_LAYERED
                // | WS_EX_TOPMOST,
                // 窗口类名
                window_class,
                // 窗口名称。 如果窗口样式指定标题栏，则 lpWindowName 指向的窗口标题将显示在标题栏中。
                s!("MessageLoop"),
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
                Some(self as *mut _ as _), // 用于窗口内全局变量共享
            );
            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);

            // 从调用线程的消息队列中检索消息。 函数调度传入的已发送消息，直到已发布的消息可供检索。
            let mut msg = MSG::default();
            // 从线程消息中取出一条消息
            while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
                // 把取出的消息发送到目的窗口
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }

            // 如果函数检索 WM_QUIT 消息，则返回值为零。
            Ok(())
        }

        Ok(())
    }

    /// 当前窗口的处理过程
    extern "system" fn wndproc(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                // 获得窗口创建参数，用于窗口线程共享全局变量
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                // 设置与窗口关联的用户数据。 此数据供创建窗口的应用程序使用。 其值最初为零。
                SetWindowLongPtrA(window, GWLP_USERDATA, this as _);
            } else if message == WM_PAINT {
                // Note：需要处理重绘，如果不处理，会导致接收大量的WM_PAINT，引起CPU升高
                _ = ValidateRect(window, None);
            } else {
                // 获得使用SetWindowLongPtrA设置的窗口关联的用户数据
                let this =
                    GetWindowLongPtrA(window, GWLP_USERDATA) as *mut Self;

                // 处理接收到的窗口消息
                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            // 调用默认窗口过程，为应用程序不处理的任何窗口消息提供默认处理。
            DefWindowProcA(window, message, wparam, lparam)
        }
    }

    fn message_handler(
        &mut self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_CREATE => {}
                WM_DESTROY => {
                    // 此方法会产生一个WM_QUIT消息到消息队列中并马上返回。
                    PostQuitMessage(0)
                }
                WM_CLOSE => {
                    return DefWindowProcA(
                        self.handle,
                        message,
                        wparam,
                        lparam,
                    );
                }
                _ => {
                    return DefWindowProcA(
                        self.handle,
                        message,
                        wparam,
                        lparam,
                    );
                }
            }
        }

        LRESULT(0)
    }
}
