pub fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

pub fn find_extension_smpl(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i + 1..]),
    }
}

fn local_map<F, T, A>(option: Option<T>, f: F) -> Option<A>
where
    F: FnOnce(T) -> A,
{
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}

pub fn find_extension_map(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i + 1..])
}

pub fn find_extension_local_map(file_name: &str) -> Option<&str> {
    let f = find(file_name, '.');
    local_map(f, |i| &file_name[i + 1..])
}
