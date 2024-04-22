use super::types::OSVERSIONINFOEX;
use crate::version::Version;
use std::fmt::{self, Display, Formatter};
use std::{
    ffi::{OsStr, OsString},
    mem::{self, MaybeUninit},
    os::windows::ffi::{OsStrExt, OsStringExt},
    ptr,
};
use windows_sys::Win32::{
    Foundation::{ERROR_SUCCESS, FARPROC, NTSTATUS, STATUS_SUCCESS},
    System::{
        LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Registry::{
            RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, KEY_READ,
            REG_SZ,
        },
        SystemInformation::{
            GetNativeSystemInfo, GetSystemInfo, PROCESSOR_ARCHITECTURE_AMD64,
            PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_IA64,
            PROCESSOR_ARCHITECTURE_INTEL, SYSTEM_INFO,
        },
        SystemServices::{VER_NT_WORKSTATION, VER_SUITE_WH_SERVER},
    },
    UI::WindowsAndMessaging::{GetSystemMetrics, SM_SERVERR2},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Info {
    /// Operating system type. See `Type` for details.
    pub(crate) os_type: Type,
    /// Operating system version. See `Version` for details.
    pub(crate) version: Version,
    /// Operating system edition.
    pub(crate) edition: Option<String>,
    /// Operating system codename.
    pub(crate) codename: Option<String>,
    /// Operating system architecture in terms of how many bits compose the basic values it can deal
    /// with. See `Bitness` for details.
    pub(crate) bitness: Bitness,
    /// Processor architecture.
    pub(crate) architecture: Option<String>,
}

impl Default for Info {
    fn default() -> Self {
        Self::unknown()
    }
}

impl Info {
    pub fn unknown() -> Self {
        Self {
            os_type: Type::Unknown,
            version: Version::Unknown,
            edition: None,
            codename: None,
            bitness: Bitness::Unknown,
            architecture: None,
        }
    }

    pub fn os_type(&self) -> Type {
        self.os_type
    }
}
impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.os_type)?;
        if self.version != Version::Unknown {
            write!(f, " {}", self.version)?;
        }
        if let Some(ref edition) = self.edition {
            write!(f, " ({edition})")?;
        }
        if let Some(ref codename) = self.codename {
            write!(f, " ({codename})")?;
        }
        write!(f, " [{}]", self.bitness)
    }
}

/// Operating system architecture in terms of how many bits compose the basic values it can deal with.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Bitness {
    /// Unknown bitness (unable to determine).
    Unknown,
    /// 32-bit.
    X32,
    /// 64-bit.
    X64,
}

pub fn get() -> Info {
    let (version, edition) = version();
    let native_system_info = native_system_info();

    Info {
        os_type: Type::Windows,
        version,
        edition,
        bitness: bitness(),
        architecture: architecture(native_system_info),
        ..Default::default()
    }
}

fn version() -> (Version, Option<String>) {
    match version_info() {
        None => (Version::Unknown, None),
        Some(v) => (
            Version::Semantic(
                v.dwMajorVersion as u64,
                v.dwMinorVersion as u64,
                v.dwBuildNumber as u64,
            ),
            product_name(&v).or_else(|| edition(&v)),
        ),
    }
}

// According to https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info
// there is a variant for AMD64 CPUs, but it's not defined in generated bindings.
const PROCESSOR_ARCHITECTURE_ARM64: u16 = 12;

fn native_system_info() -> SYSTEM_INFO {
    let mut system_info: MaybeUninit<SYSTEM_INFO> = MaybeUninit::zeroed();
    unsafe {
        GetNativeSystemInfo(system_info.as_mut_ptr());
    };

    unsafe { system_info.assume_init() }
}

fn architecture(system_info: SYSTEM_INFO) -> Option<String> {
    let cpu_architecture =
        unsafe { system_info.Anonymous.Anonymous.wProcessorArchitecture };

    match cpu_architecture {
        PROCESSOR_ARCHITECTURE_AMD64 => Some("x86_64"),
        PROCESSOR_ARCHITECTURE_IA64 => Some("ia64"),
        PROCESSOR_ARCHITECTURE_ARM => Some("arm"),
        PROCESSOR_ARCHITECTURE_ARM64 => Some("aarch64"),
        PROCESSOR_ARCHITECTURE_INTEL => Some("i386"),
        _ => None,
    }
    .map(str::to_string)
}

#[cfg(target_pointer_width = "64")]
fn bitness() -> Bitness {
    // x64 program can only run on x64 Windows.
    Bitness::X64
}

impl Display for Bitness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Bitness::Unknown => write!(f, "unknown bitness"),
            Bitness::X32 => write!(f, "32-bit"),
            Bitness::X64 => write!(f, "64-bit"),
        }
    }
}

/// A list of supported operating system types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum Type {
    /// IBM AIX (<https://en.wikipedia.org/wiki/IBM_AIX>).
    AIX,
    /// AlmaLinux (<https://en.wikipedia.org/wiki/AlmaLinux>).
    AlmaLinux,
    /// Alpaquita Linux (<https://bell-sw.com/alpaquita-linux/>).
    Alpaquita,
    /// Alpine Linux (<https://en.wikipedia.org/wiki/Alpine_Linux>).
    Alpine,
    /// Amazon Linux AMI (<https://en.wikipedia.org/wiki/Amazon_Machine_Image#Amazon_Linux_AMI>).
    Amazon,
    /// Android (<https://en.wikipedia.org/wiki/Android_(operating_system)>).
    Android,
    /// Arch Linux (<https://en.wikipedia.org/wiki/Arch_Linux>).
    Arch,
    /// Artix Linux (<https://en.wikipedia.org/wiki/Artix_Linux>).
    Artix,
    /// CentOS (<https://en.wikipedia.org/wiki/CentOS>).
    CentOS,
    /// Debian (<https://en.wikipedia.org/wiki/Debian>).
    Debian,
    /// DragonFly BSD (<https://en.wikipedia.org/wiki/DragonFly_BSD>).
    DragonFly,
    /// Emscripten (<https://en.wikipedia.org/wiki/Emscripten>).
    Emscripten,
    /// EndeavourOS (<https://en.wikipedia.org/wiki/EndeavourOS>).
    EndeavourOS,
    /// Fedora (<https://en.wikipedia.org/wiki/Fedora_(operating_system)>).
    Fedora,
    /// FreeBSD (<https://en.wikipedia.org/wiki/FreeBSD>).
    FreeBSD,
    /// Garuda Linux (<https://en.wikipedia.org/wiki/Garuda_Linux>)
    Garuda,
    /// Gentoo Linux (<https://en.wikipedia.org/wiki/Gentoo_Linux>).
    Gentoo,
    /// HardenedBSD (https://hardenedbsd.org/).
    HardenedBSD,
    /// Illumos (https://en.wikipedia.org/wiki/Illumos).
    Illumos,
    /// Kali Linux (https://en.wikipedia.org/wiki/Kali_Linux).
    Kali,
    /// Linux based operating system (<https://en.wikipedia.org/wiki/Linux>).
    Linux,
    /// Mabox (<https://maboxlinux.org/>).
    Mabox,
    /// Mac OS X/OS X/macOS (<https://en.wikipedia.org/wiki/MacOS>).
    Macos,
    /// Manjaro (<https://en.wikipedia.org/wiki/Manjaro>).
    Manjaro,
    /// Mariner (<https://en.wikipedia.org/wiki/CBL-Mariner>).
    Mariner,
    /// MidnightBSD (<https://en.wikipedia.org/wiki/MidnightBSD>).
    MidnightBSD,
    /// Mint (<https://en.wikipedia.org/wiki/Linux_Mint>).
    Mint,
    /// NetBSD (<https://en.wikipedia.org/wiki/NetBSD>).
    NetBSD,
    /// NixOS (<https://en.wikipedia.org/wiki/NixOS>).
    NixOS,
    /// Nobara (<https://nobaraproject.org/>).
    Nobara,
    /// Uos (<https://www.chinauos.com/>).
    Uos,
    /// OpenBSD (<https://en.wikipedia.org/wiki/OpenBSD>).
    OpenBSD,
    /// OpenCloudOS (<https://www.opencloudos.org>).
    OpenCloudOS,
    /// openEuler (<https://en.wikipedia.org/wiki/EulerOS>).
    openEuler,
    /// openSUSE (<https://en.wikipedia.org/wiki/OpenSUSE>).
    openSUSE,
    /// Oracle Linux (<https://en.wikipedia.org/wiki/Oracle_Linux>).
    OracleLinux,
    /// Pop!_OS (<https://en.wikipedia.org/wiki/Pop!_OS>)
    Pop,
    /// Raspberry Pi OS (<https://en.wikipedia.org/wiki/Raspberry_Pi_OS>).
    Raspbian,
    /// Red Hat Linux (<https://en.wikipedia.org/wiki/Red_Hat_Linux>).
    Redhat,
    /// Red Hat Enterprise Linux (<https://en.wikipedia.org/wiki/Red_Hat_Enterprise_Linux>).
    RedHatEnterprise,
    /// Redox (<https://en.wikipedia.org/wiki/Redox_(operating_system)>).
    Redox,
    /// Rocky Linux (<https://en.wikipedia.org/wiki/Rocky_Linux>).
    RockyLinux,
    /// Solus (<https://en.wikipedia.org/wiki/Solus_(operating_system)>).
    Solus,
    /// SUSE Linux Enterprise Server (<https://en.wikipedia.org/wiki/SUSE_Linux_Enterprise>).
    SUSE,
    /// Ubuntu (<https://en.wikipedia.org/wiki/Ubuntu_(operating_system)>).
    Ubuntu,
    /// Ultramarine (<https://ultramarine-linux.org/>).
    Ultramarine,
    /// Void Linux (<https://en.wikipedia.org/wiki/Void_Linux>).
    Void,
    /// Unknown operating system.
    Unknown,
    /// Windows (<https://en.wikipedia.org/wiki/Microsoft_Windows>).
    Windows,
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Alpaquita => write!(f, "Alpaquita Linux"),
            Type::Alpine => write!(f, "Alpine Linux"),
            Type::AlmaLinux => write!(f, "AlmaLinux"),
            Type::Amazon => write!(f, "Amazon Linux AMI"),
            Type::Arch => write!(f, "Arch Linux"),
            Type::Artix => write!(f, "Artix Linux"),
            Type::DragonFly => write!(f, "DragonFly BSD"),
            Type::Garuda => write!(f, "Garuda Linux"),
            Type::Gentoo => write!(f, "Gentoo Linux"),
            Type::Illumos => write!(f, "illumos"),
            Type::Kali => write!(f, "Kali Linux"),
            Type::Macos => write!(f, "Mac OS"),
            Type::MidnightBSD => write!(f, "Midnight BSD"),
            Type::Mint => write!(f, "Linux Mint"),
            Type::Nobara => write!(f, "Nobara Linux"),
            Type::Uos => write!(f, "Uos"),
            Type::openEuler => write!(f, "EulerOS"),
            Type::OracleLinux => write!(f, "Oracle Linux"),
            Type::Pop => write!(f, "Pop!_OS"),
            Type::Raspbian => write!(f, "Raspberry Pi OS"),
            Type::Redhat => write!(f, "Red Hat Linux"),
            Type::RedHatEnterprise => write!(f, "Red Hat Enterprise Linux"),
            Type::RockyLinux => write!(f, "Rocky Linux"),
            Type::SUSE => write!(f, "SUSE Linux Enterprise Server"),
            Type::Ultramarine => write!(f, "Ultramarine Linux"),
            Type::Void => write!(f, "Void Linux"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[cfg(target_pointer_width = "32")]
fn bitness() -> Bitness {
    use windows_sys::Win32::Foundation::{BOOL, FALSE, HANDLE};
    use windows_sys::Win32::System::Threading::GetCurrentProcess;

    // IsWow64Process is not available on all supported versions of Windows. Use GetModuleHandle to
    // get a handle to the DLL that contains the function and GetProcAddress to get a pointer to the
    // function if available.
    let is_wow_64 = match get_proc_address(b"kernel32\0", b"IsWow64Process\0")
    {
        None => return Bitness::Unknown,
        Some(val) => val,
    };

    type IsWow64 = unsafe extern "system" fn(HANDLE, *mut BOOL) -> BOOL;
    let is_wow_64: IsWow64 = unsafe { mem::transmute(is_wow_64) };

    let mut result = FALSE;
    if unsafe { is_wow_64(GetCurrentProcess(), &mut result) } == 0 {
        log::error!("IsWow64Process failed");
        return Bitness::Unknown;
    }

    if result == FALSE {
        Bitness::X32
    } else {
        Bitness::X64
    }
}

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
fn version_info() -> Option<OSVERSIONINFOEX> {
    let rtl_get_version =
        match get_proc_address(b"ntdll\0", b"RtlGetVersion\0") {
            None => return None,
            Some(val) => val,
        };

    type RtlGetVersion =
        unsafe extern "system" fn(&mut OSVERSIONINFOEX) -> NTSTATUS;
    let rtl_get_version: RtlGetVersion =
        unsafe { mem::transmute(rtl_get_version) };

    let mut info: OSVERSIONINFOEX = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEX>() as u32;

    if unsafe { rtl_get_version(&mut info) } == STATUS_SUCCESS {
        Some(info)
    } else {
        None
    }
}

fn product_name(info: &OSVERSIONINFOEX) -> Option<String> {
    let sub_key = to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    let mut key = Default::default();
    if unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            sub_key.as_ptr(),
            0,
            KEY_READ,
            &mut key,
        )
    } != ERROR_SUCCESS
        || key == 0
    {
        log::error!("RegOpenKeyExW(HKEY_LOCAL_MACHINE, ...) failed");
        return None;
    }

    let is_win_11 = info.dwMajorVersion == 10 && info.dwBuildNumber >= 22000;

    // Get size of the data.
    let name = to_wide(if is_win_11 { "EditionID" } else { "ProductName" });
    let mut data_type = 0;
    let mut data_size = 0;
    if unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null_mut(),
            &mut data_type,
            ptr::null_mut(),
            &mut data_size,
        )
    } != ERROR_SUCCESS
        || data_type != REG_SZ
        || data_size == 0
        || data_size % 2 != 0
    {
        log::error!("RegQueryValueExW failed");
        return None;
    }

    // Get the data.
    let mut data = vec![0u16; data_size as usize / 2];
    if unsafe {
        RegQueryValueExW(
            key,
            name.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            data.as_mut_ptr().cast(),
            &mut data_size,
        )
    } != ERROR_SUCCESS
        || data_size as usize != data.len() * 2
    {
        return None;
    }

    // If the data has the REG_SZ, REG_MULTI_SZ or REG_EXPAND_SZ type, the string may not have been
    // stored with the proper terminating null characters.
    match data.last() {
        Some(0) => {
            data.pop();
        }
        _ => {}
    }

    let value =
        OsString::from_wide(data.as_slice()).to_string_lossy().into_owned();

    if is_win_11 {
        Some(format!("Windows 11 {}", value))
    } else {
        Some(value)
    }
}

fn to_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

// Examines data in the OSVERSIONINFOEX structure to determine the Windows edition:
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
fn edition(version_info: &OSVERSIONINFOEX) -> Option<String> {
    match (
        version_info.dwMajorVersion,
        version_info.dwMinorVersion,
        version_info.wProductType as u32,
    ) {
        // Windows 10.
        (10, 0, VER_NT_WORKSTATION) => {
            if version_info.dwBuildNumber >= 22000 {
                Some("Windows 11")
            } else {
                Some("Windows 10")
            }
        }
        (10, 0, _) => Some("Windows Server 2016"),
        // Windows Vista, 7, 8 and 8.1.
        (6, 3, VER_NT_WORKSTATION) => Some("Windows 8.1"),
        (6, 3, _) => Some("Windows Server 2012 R2"),
        (6, 2, VER_NT_WORKSTATION) => Some("Windows 8"),
        (6, 2, _) => Some("Windows Server 2012"),
        (6, 1, VER_NT_WORKSTATION) => Some("Windows 7"),
        (6, 1, _) => Some("Windows Server 2008 R2"),
        (6, 0, VER_NT_WORKSTATION) => Some("Windows Vista"),
        (6, 0, _) => Some("Windows Server 2008"),
        // Windows 2000, Home Server, 2003 Server, 2003 R2 Server, XP and XP Professional x64.
        (5, 1, _) => Some("Windows XP"),
        (5, 0, _) => Some("Windows 2000"),
        (5, 2, _) if unsafe { GetSystemMetrics(SM_SERVERR2) } == 0 => {
            let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { GetSystemInfo(&mut info) };

            if Into::<u32>::into(version_info.wSuiteMask) & VER_SUITE_WH_SERVER
                == VER_SUITE_WH_SERVER
            {
                Some("Windows Home Server")
            } else if version_info.wProductType == VER_NT_WORKSTATION as u8
                && unsafe { info.Anonymous.Anonymous.wProcessorArchitecture }
                    == PROCESSOR_ARCHITECTURE_AMD64
            {
                Some("Windows XP Professional x64 Edition")
            } else {
                Some("Windows Server 2003")
            }
        }
        _ => None,
    }
    .map(str::to_string)
}

fn get_proc_address(module: &[u8], proc: &[u8]) -> Option<FARPROC> {
    assert!(
        *module.last().expect("Empty module name") == 0,
        "Module name should be zero-terminated"
    );
    assert!(
        *proc.last().expect("Empty procedure name") == 0,
        "Procedure name should be zero-terminated"
    );

    let handle = unsafe { GetModuleHandleA(module.as_ptr()) };
    if handle == 0 {
        log::error!(
            "GetModuleHandleA({}) failed",
            String::from_utf8_lossy(module)
        );
        return None;
    }

    unsafe { Some(GetProcAddress(handle, proc.as_ptr())) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn version() {
        let info = get();
        assert_eq!(Type::Windows, info.os_type());
    }

    #[test]
    fn get_version_info() {
        let version = version_info();
        assert!(version.is_some());
    }

    #[test]
    fn get_edition() {
        let test_data = [
            (10, 0, 0, "Windows Server 2016"),
            (6, 3, VER_NT_WORKSTATION, "Windows 8.1"),
            (6, 3, 0, "Windows Server 2012 R2"),
            (6, 2, VER_NT_WORKSTATION, "Windows 8"),
            (6, 2, 0, "Windows Server 2012"),
            (6, 1, VER_NT_WORKSTATION, "Windows 7"),
            (6, 1, 0, "Windows Server 2008 R2"),
            (6, 0, VER_NT_WORKSTATION, "Windows Vista"),
            (6, 0, 0, "Windows Server 2008"),
            (5, 1, 0, "Windows XP"),
            (5, 1, 1, "Windows XP"),
            (5, 1, 100, "Windows XP"),
            (5, 0, 0, "Windows 2000"),
            (5, 0, 1, "Windows 2000"),
            (5, 0, 100, "Windows 2000"),
        ];

        let mut info = version_info().unwrap();

        for &(major, minor, product_type, expected_edition) in &test_data {
            info.dwMajorVersion = major;
            info.dwMinorVersion = minor;
            info.wProductType = product_type as u8;

            let edition = edition(&info).unwrap();
            assert_eq!(edition, expected_edition);
        }
    }

    #[test]
    fn get_bitness() {
        let b = bitness();
        assert_ne!(b, Bitness::Unknown);
    }

    #[test]
    #[should_panic(expected = "Empty module name")]
    fn empty_module_name() {
        get_proc_address(b"", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Module name should be zero-terminated")]
    fn non_zero_terminated_module_name() {
        get_proc_address(b"ntdll", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Empty procedure name")]
    fn empty_proc_name() {
        get_proc_address(b"ntdll\0", b"");
    }

    #[test]
    #[should_panic(expected = "Procedure name should be zero-terminated")]
    fn non_zero_terminated_proc_name() {
        get_proc_address(b"ntdll\0", b"RtlGetVersion");
    }

    #[test]
    fn proc_address() {
        let address = get_proc_address(b"ntdll\0", b"RtlGetVersion\0");
        assert!(address.is_some());
    }

    #[test]
    fn get_architecture() {
        let cpu_types: [(u16, Option<String>); 6] = [
            (PROCESSOR_ARCHITECTURE_AMD64, Some("x86_64".to_owned())),
            (PROCESSOR_ARCHITECTURE_ARM, Some("arm".to_owned())),
            (PROCESSOR_ARCHITECTURE_ARM64, Some("aarch64".to_owned())),
            (PROCESSOR_ARCHITECTURE_IA64, Some("ia64".to_owned())),
            (PROCESSOR_ARCHITECTURE_INTEL, Some("i386".to_owned())),
            (0xffff, None),
        ];

        let mut native_info = native_system_info();

        for cpu_type in cpu_types {
            native_info.Anonymous.Anonymous.wProcessorArchitecture =
                cpu_type.0;
            assert_eq!(architecture(native_info), cpu_type.1);
        }
    }

    #[test]
    fn get_product_name() {
        let version = version_info().expect("version_info() failed");
        let edition = product_name(&version).expect("edition() failed");
        assert!(!edition.is_empty());
    }

    #[test]
    fn to_wide_str() {
        let data = [
            ("", [0x0000].as_ref()),
            ("U", &[0x0055, 0x0000]),
            ("你好", &[0x4F60, 0x597D, 0x0000]),
        ];

        for (s, expected) in &data {
            let wide = to_wide(s);
            assert_eq!(&wide, expected);
        }
    }
}
