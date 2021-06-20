const SIZE: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub const NO_BRANCH_SPACE: &str = "    ";
pub const BRANCH_SPACE: &str = "│   ";
pub const MIDDLE_ITEM: &str = "├─";
pub const LAST_ITEM: &str = "└─";

pub fn size_human(size: u64) -> String {
    let mut sizef: f64 = size as f64;
    let mut index: usize = 0;
    let max = 1024_f64;
    while sizef > max {
        sizef /= max;
        index += 1;
    }

    format!("{:.1} {}", sizef, SIZE[index])
}

pub fn get_leading_entry(path: &str) -> &str {
    let mut retval = path;
    if let Some(0) = retval.find('/') {
        retval = &retval[1..];
    }

    if let Some(index) = retval.find('/') {
        &retval[0..index]
    } else {
        retval
    }
}

pub fn split_last_entry(path: &str) -> (&str, &str) {
    let mut retval = path;
    let len = retval.len();

    if let Some(index) = retval.rfind('/') {
        let idx = index + 1;
        if idx == len {
            retval = &retval[0..len - 1];
            if let Some(index) = retval.rfind('/') {
                (&retval[0..index], &retval[index + 1..])
            } else {
                ("", retval)
            }
        } else {
            (&retval[0..idx], &retval[idx..len])
        }
    } else {
        ("", retval)
    }
}

pub fn get_remaining(path: &str) -> &str {
    let mut retval = path;
    if let Some(0) = retval.find('/') {
        retval = &retval[1..];
    }

    if let Some(index) = retval.find('/') {
        &retval[index..]
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::{get_leading_entry, get_remaining, split_last_entry};

    #[test]
    fn gets_leading_entry() {
        let path1 = "usr/local/bin/";
        assert_eq!(get_leading_entry(path1), "usr");
        assert_eq!(get_remaining(path1), "/local/bin/");
        let (p1, f1) = split_last_entry(path1);
        assert_eq!(p1, "usr/local");
        assert_eq!(f1, "bin");

        let path2 = "/dev/null";
        assert_eq!(get_leading_entry(path2), "dev");
        assert_eq!(get_remaining(path2), "/null");
        let (p2, f2) = split_last_entry(path2);
        assert_eq!(p2, "/dev/");
        assert_eq!(f2, "null");

        let path3 = "usrlocal";
        assert_eq!(get_leading_entry(path3), "usrlocal");
        assert_eq!(get_remaining(path3), "");
        let (p3, f3) = split_last_entry(path3);
        assert_eq!(p3, "");
        assert_eq!(f3, "usrlocal");

        let path4 = "test/";
        assert_eq!(get_leading_entry(path4), "test");
        assert_eq!(get_remaining(path4), "/");
        let (p4, f4) = split_last_entry(path4);
        assert_eq!(p4, "");
        assert_eq!(f4, "test");
    }
}
