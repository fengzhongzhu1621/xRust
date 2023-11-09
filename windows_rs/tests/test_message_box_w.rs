use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*,
    Win32::System::Threading::*, Win32::UI::Shell::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn test_message_box_w() -> Result<()> {
    let doc = XmlDocument::new()?;
    doc.LoadXml(h!("<html>hello world</html>"))?;

    let root = doc.DocumentElement()?;
    assert!(root.NodeName()? == "html");
    assert!(root.InnerText()? == "hello world");

    unsafe {
        // 创建或打开一个命名或未命名的事件对象。
        let event = CreateEventW(None, true, false, None)?;
        // 将指定的事件对象设置为信号状态。
        SetEvent(event).ok();
        // 等待指定的对象处于信号状态或超时间隔已过。
        // 如果 dwMilliseconds 为零，则如果未向对象发出信号，则函数不会进入等待状态;它始终立即返回。
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok();

        // MessageBox是在库里声明了一个宏 当你使用宽字符的时候,也就是unicode的时候,自动帮你转换使用 MessageBoxW 而当你使用窄字符的时候,会自动帮你转换到 MEssageBoxA

        // 显示一个模式对话框，其中包含一个系统图标、一组按钮和一条简短的应用程序特定消息
        MessageBoxA(None, s!("Ansi"), s!("Caption"), MB_OK);
        // 显示一个模式对话框，其中包含一个系统图标、一组按钮和一条简短的应用程序特定消息
        MessageBoxW(None, w!("Wide"), w!("Caption"), MB_OK);
    }

    Ok(())
}

#[test]
fn test_shell_message() {
    unsafe {
        MessageBoxA(None, s!("Ansi"), s!("World"), MB_OK);
        MessageBoxW(None, h!("WinRT"), h!("World"), MB_OK);
        // ShellMessageBox 是 MessageBox 的一个特殊实例，它提供使用所有者窗口标题作为消息框标题的选项。
        ShellMessageBoxW(None, None, w!("Wide"), w!("World"), MB_ICONERROR);
    }
}
