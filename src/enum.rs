enum Week {
    Monday = 1, // 1
    Tuesday,    // 2
    Wednesday,  // 3
    Thursday,   // 4
    Friday,     // 5
    Saturday,   // 6
    Sunday,     // 7
}

impl Week {
    fn is_weekend(&self) -> bool {
        if (*self as u8) > 5 {
            return true;
        }
        false
    }
}
