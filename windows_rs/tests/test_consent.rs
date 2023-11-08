/// consent.exe
/// 当用户开启用户账户控制（UAC）功能时，一个程序要更改或者使用一些比较高权限才能做的事情的功能的时候、还有当你使用管理员账户运行程序的时候，会弹出一个对话框，询问您是否允许程序修改计算机设置，这个对话框的进程就是consent.exe，
/// consent的意思是同意的意思，启动这个进程的用户是SYSTEM，
/// 或者完全看不到是谁启动的，这个进程在正常情况下无法结束。
use windows::{
    core::*, Foundation::*, Security::Credentials::UI::*,
    Win32::Foundation::*, Win32::System::WinRT::*,
};

#[test]
fn test_() -> Result<()> {
    unsafe {
        let interop =
            factory::<UserConsentVerifier, IUserConsentVerifierInterop>()?;

        let window = HWND(0); // <== replace with your app's window handle

        let operation: IAsyncOperation<UserConsentVerificationResult> =
            interop.RequestVerificationForWindowAsync(
                window,
                h!("Hello from Rust"),
            )?;

        let result: UserConsentVerificationResult = operation.get()?;

        println!("{result:?}");

        Ok(())
    }
}
