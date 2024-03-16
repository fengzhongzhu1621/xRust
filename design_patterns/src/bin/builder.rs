use chrono::prelude::*;
use std::{
    cell::RefCell, ffi::OsStr, fmt, io, io::Write, path::Path, rc::Rc, str,
    time::SystemTime,
};

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// 定义需要构造的协议
#[derive(Debug, Default, Clone)]
struct Record<'a> {
    event_time: Option<SystemTime>,
    var_a: Option<String>,
    var_b: Option<&'a Path>,
    var_c: Option<i32>,
    var_d: Option<&'a OsStr>,
}

/// Record -> RecordBuilder
impl<'a> Record<'a> {
    /// Returns a new builder.
    #[inline]
    fn builder() -> RecordBuilder<'a> {
        RecordBuilder::new()
    }

    #[inline]
    fn event_time(&self) -> Option<SystemTime> {
        self.event_time
    }

    #[inline]
    fn var_a(&self) -> &Option<String> {
        &self.var_a
    }

    #[inline]
    fn var_b(&self) -> Option<&'a Path> {
        self.var_b
    }

    #[inline]
    fn var_c(&self) -> Option<i32> {
        self.var_c
    }

    #[inline]
    fn var_d(&self) -> Option<&'a OsStr> {
        self.var_d
    }
}

impl<'a> Record<'a> {}

/// 用于构造协议，通过 Record 和 RecordBuidler 将协议的读写分离
#[derive(Debug)]
struct RecordBuilder<'a> {
    record: Record<'a>,
}

impl<'a> RecordBuilder<'a> {
    /// Construct new `RecordBuilder`.
    #[inline]
    fn new() -> RecordBuilder<'a> {
        RecordBuilder { record: Record::default() }
    }

    #[inline]
    fn event_time(
        &mut self,
        event_time: Option<SystemTime>,
    ) -> &mut RecordBuilder<'a> {
        self.record.event_time = event_time;
        self
    }

    #[inline]
    fn var_a(&mut self, var_a: Option<String>) -> &mut RecordBuilder<'a> {
        self.record.var_a = var_a;
        self
    }

    #[inline]
    fn var_b(&mut self, var_b: Option<&'a Path>) -> &mut RecordBuilder<'a> {
        self.record.var_b = var_b;
        self
    }

    #[inline]
    fn var_c(&mut self, var_c: Option<i32>) -> &mut RecordBuilder<'a> {
        self.record.var_c = var_c;
        self
    }

    #[inline]
    fn var_d(&mut self, var_d: Option<&'a OsStr>) -> &mut RecordBuilder<'a> {
        self.record.var_d = var_d;
        self
    }

    /// Invoke the builder and return a `Record`
    #[inline]
    fn build(&mut self) -> Record<'a> {
        // todo 添加业务逻辑

        self.record.clone()
    }
}

impl<'a> Default for RecordBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// 定义一个写缓存
#[derive(Debug)]
struct Buffer(Vec<u8>);

impl Buffer {
    /// 初始化缓存
    fn new() -> Self {
        Self(vec![])
    }

    /// 清空缓存
    fn clear(&mut self) {
        self.0.clear();
    }

    /// 写缓存
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    /// 刷新缓存
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// 获得缓存的内容
    fn bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

/// 定义缓存内容的格式器
struct FormatterBuffer {
    buf: Rc<RefCell<Buffer>>, // RefCell可以修改buf，Rc可以避免使用作用域标识
}

impl FormatterBuffer {
    fn new(buffer: Rc<RefCell<Buffer>>) -> Self {
        FormatterBuffer { buf: buffer }
    }

    fn clear(&mut self) {
        self.buf.borrow_mut().clear()
    }

    fn buf(&self) -> Rc<RefCell<Buffer>> {
        self.buf.clone()
    }
}
impl io::Write for FormatterBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.borrow_mut().flush()
    }
}

impl fmt::Debug for FormatterBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FormatterBuffer").finish()
    }
}

#[derive(Debug)]
/// 格式化器
struct Format<'a> {
    buf: &'a mut FormatterBuffer, // 数据缓存
    sep: &'a str,                 // 分隔符
}

impl<'a> Format<'a> {
    /// 写数据到缓存中
    fn write(mut self, record: &Record) -> io::Result<()> {
        let _ = self.write_event_time(record);
        let _ = self.write_var_a(record);
        let _ = self.write_var_b(record);
        let _ = self.write_var_c(record);
        let _ = self.write_var_d(record);

        Ok(())
    }

    fn write_event_time(&mut self, record: &Record) -> io::Result<()> {
        match record.event_time() {
            Some(event_time) => {
                let datetime_str = format_system_time(event_time);
                write!(self.buf, "{}{}", datetime_str, self.sep)
            }
            None => {
                write!(self.buf, "{}", self.sep)
            }
        }
    }

    fn write_var_a(&mut self, record: &Record) -> io::Result<()> {
        match record.var_a() {
            Some(var_a) => {
                write!(self.buf, "{}{}", var_a, self.sep)
            }
            None => write!(self.buf, "{}", self.sep),
        }
    }

    fn write_var_b(&mut self, record: &Record) -> io::Result<()> {
        match record.var_b() {
            Some(var_b) => {
                write!(
                    self.buf,
                    "{}{}",
                    var_b.to_string_lossy(), // 操作系统对路径处理的差异性可能会丢失部分数据
                    self.sep
                )
            }
            None => write!(self.buf, "{}", self.sep),
        }
    }

    fn write_var_c(&mut self, record: &Record) -> io::Result<()> {
        match record.var_c() {
            Some(var_c) => {
                write!(self.buf, "{}{}", var_c, self.sep)
            }
            None => write!(self.buf, "{}", self.sep),
        }
    }

    fn write_var_d(&mut self, record: &Record) -> io::Result<()> {
        match record.var_d() {
            Some(var_d) => {
                write!(
                    self.buf,
                    "{}{}",
                    var_d.to_os_string().to_str().unwrap(), // 操作系统对路径处理的差异性可能会panic
                    self.sep
                )
            }
            None => write!(self.buf, "{}", self.sep),
        }
    }
}

/// 将 SystemTime 格式的时间转换为指定格式的字符串
fn format_system_time(st: SystemTime) -> String {
    let local_datetime: DateTime<Local> = st.clone().into();
    let datetime_str = local_datetime.format(DATETIME_FORMAT).to_string();

    datetime_str
}

fn main() {
    // 创建缓存
    let buffer = Rc::new(RefCell::new(Buffer::default()));
    let mut format_buffer = FormatterBuffer::new(buffer.clone());
    format_buffer.clear();

    // 创建一个格式化器
    let format = Format { buf: &mut format_buffer, sep: "|" };

    // 构造事件发生时间
    let no_timezone =
        NaiveDateTime::parse_from_str("2024-01-02 03:04:05", DATETIME_FORMAT)
            .unwrap();
    let event_time = Local.from_local_datetime(&no_timezone).unwrap().into();
    // 构造路径
    let path = Path::new("./foo/bar.txt");
    let os_str = OsStr::new("1.png");
    // 构造一条上报记录
    let record = Record::builder()
        .event_time(Some(event_time))
        .var_a(Some("hello world".to_string()))
        .var_b(Some(path))
        .var_c(Some(999))
        .var_d(Some(os_str))
        .build();

    // 写记录到缓存
    let _ = format.write(&record);

    // 获得RefCell对象的内部值
    let ref_cell_inner_value = buffer.borrow();
    let actual = str::from_utf8(ref_cell_inner_value.bytes()).unwrap();

    let expect = "2024-01-02 03:04:05|hello world|./foo/bar.txt|999|1.png|";
    assert_eq!(actual, expect);
}
