use windows::{core::*, Win32::System::Performance::*};

#[test]
fn test_counter() {
    unsafe {
        // 创建用于管理性能数据收集的新查询
        // PDH_FUNCTION PdhOpenQueryW(
        //  [in]  LPCWSTR    szDataSource, // 以 Null 结尾的字符串，指定要从中检索性能数据的日志文件的名称。 如果 为 NULL，则从实时数据源收集性能数据。
        //  [in]  DWORD_PTR  dwUserData, // 要与此查询关联的用户定义的值。
        //  [out] PDH_HQUERY *phQuery // 查询的句柄。 在后续调用中使用此句柄。
        // );
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);

        // 将指定的计数器添加到查询。
        let mut counter = 0;
        PdhAddCounterW(
            query,
            w!("\\Processor(0)\\% Processor Time"),
            0,
            &mut counter,
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            // 收集指定查询中所有计数器的当前原始数据值，并更新每个计数器的状态代码。
            PdhCollectQueryData(query);

            // 计算指定计数器的可显示值。
            let mut value = Default::default();
            if 0 == PdhGetFormattedCounterValue(
                counter,
                PDH_FMT_DOUBLE,
                None,
                &mut value,
            ) {
                println!("{:.2}", value.Anonymous.doubleValue);
            }
        }
    }
}
