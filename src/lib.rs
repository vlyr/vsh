pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::utils::append_to_path;

    #[test]
    fn path_append() {
        let dotdot = append_to_path("/home/user/files/videos/".into(), "../photos");
        assert_eq!(dotdot, "/home/user/files/photos");

        let dot = append_to_path("/home/user/files/videos/".into(), "./good-stuff");
        assert_eq!(dot, "/home/user/files/videos/good-stuff");
    }
}
