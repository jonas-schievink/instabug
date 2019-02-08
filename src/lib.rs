#[cfg(test)]
mod tests {
    use insta::assert_snapshot_matches;

    #[test]
    fn bla() {
        assert_snapshot_matches!(
            "bla",
            r#"
First line
Second line
Third line
"#
        );
    }
}
