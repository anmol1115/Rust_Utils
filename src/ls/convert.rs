pub fn numeric_to_symbolic_conv(permissions: u32) -> String {
    let mut permissions = format!("{:b}", permissions);
    let start = permissions.len() - 9;

    let mut res = String::new();
    permissions = permissions[start..].to_string();
    let access = vec!['r', 'w', 'x'];
    let mut access_idx = 0;

    for chunk in permissions.chars() {
        if chunk == '1' {
            res.push(access[access_idx % 3]);
        } else {
            res.push('-');
        }
        access_idx += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        assert_eq!(String::from("rw-r--r--"), super::numeric_to_symbolic_conv(33188));
    }
}