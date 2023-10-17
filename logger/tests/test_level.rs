use logger::*;

#[test]
fn test_parse() {
    let actual = "ERROR".parse::<Level>();
    let expect = Ok(Level::Error);
    assert_eq!(actual, expect);

    let actual = "OFF".parse::<Level>();
    let expect = Err(ParseLevelError(()));
    assert_eq!(actual, expect);
}

#[test]
fn test_levelfilter_from_str() {
    let tests = [
        ("off", Ok(LevelFilter::Off)),
        ("error", Ok(LevelFilter::Error)),
        ("warn", Ok(LevelFilter::Warn)),
        ("info", Ok(LevelFilter::Info)),
        ("debug", Ok(LevelFilter::Debug)),
        ("trace", Ok(LevelFilter::Trace)),
        ("OFF", Ok(LevelFilter::Off)),
        ("ERROR", Ok(LevelFilter::Error)),
        ("WARN", Ok(LevelFilter::Warn)),
        ("INFO", Ok(LevelFilter::Info)),
        ("DEBUG", Ok(LevelFilter::Debug)),
        ("TRACE", Ok(LevelFilter::Trace)),
        ("asdf", Err(ParseLevelError(()))),
    ];
    for (s, expect) in tests {
        assert_eq!(s.parse(), expect);
    }
}

#[test]
fn test_level_from_str() {
    let tests = [
        ("OFF", Err(ParseLevelError(()))),
        ("error", Ok(Level::Error)),
        ("warn", Ok(Level::Warn)),
        ("info", Ok(Level::Info)),
        ("debug", Ok(Level::Debug)),
        ("trace", Ok(Level::Trace)),
        ("ERROR", Ok(Level::Error)),
        ("WARN", Ok(Level::Warn)),
        ("INFO", Ok(Level::Info)),
        ("DEBUG", Ok(Level::Debug)),
        ("TRACE", Ok(Level::Trace)),
        ("asdf", Err(ParseLevelError(()))),
    ];
    for (s, expect) in tests {
        assert_eq!(s.parse(), expect);
    }
}

#[test]
fn test_level_as_str() {
    let tests = [
        (Level::Error, "ERROR"),
        (Level::Warn, "WARN"),
        (Level::Info, "INFO"),
        (Level::Debug, "DEBUG"),
        (Level::Trace, "TRACE"),
    ];
    for (input, expected) in tests {
        assert_eq!(expected, input.as_str());
    }
}

#[test]
fn test_level_show() {
    assert_eq!("INFO", Level::Info.to_string());
    assert_eq!("ERROR", Level::Error.to_string());
}

#[test]
fn test_levelfilter_show() {
    assert_eq!("OFF", LevelFilter::Off.to_string());
    assert_eq!("ERROR", LevelFilter::Error.to_string());
}

#[test]
fn test_cross_cmp() {
    assert!(Level::Debug > LevelFilter::Error);
    assert!(LevelFilter::Warn < Level::Trace);
    assert!(LevelFilter::Off < Level::Error);
}

#[test]
fn test_cross_eq() {
    assert!(Level::Error == LevelFilter::Error);
    assert!(LevelFilter::Off != Level::Error);
    assert!(Level::Trace == LevelFilter::Trace);
}

#[test]
fn test_to_level() {
    assert_eq!(Some(Level::Error), LevelFilter::Error.to_level());
    assert_eq!(None, LevelFilter::Off.to_level());
    assert_eq!(Some(Level::Debug), LevelFilter::Debug.to_level());
}

#[test]
fn test_to_level_filter() {
    assert_eq!(LevelFilter::Error, Level::Error.to_level_filter());
    assert_eq!(LevelFilter::Trace, Level::Trace.to_level_filter());
}

#[test]
fn test_level_filter_as_str() {
    let tests = &[
        (LevelFilter::Off, "OFF"),
        (LevelFilter::Error, "ERROR"),
        (LevelFilter::Warn, "WARN"),
        (LevelFilter::Info, "INFO"),
        (LevelFilter::Debug, "DEBUG"),
        (LevelFilter::Trace, "TRACE"),
    ];
    for (input, expected) in tests {
        assert_eq!(*expected, input.as_str());
    }
}
