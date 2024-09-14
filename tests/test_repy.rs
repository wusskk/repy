#[cfg(test)]
mod tests {
    use mockito;
    use repy::{check_version, download_file, modify_pth_file, unzip_file};
    use std::fs::File;
    use std::io::{Read, Write};
    use tempfile::tempdir;
    use zip::write::FileOptions;
    use zip::ZipWriter;

    #[test]
    fn test_check_version_valid() {
        assert!(check_version("3.12.6"));
    }

    #[test]
    fn test_check_version_invalid_format() {
        assert!(!check_version("3.12"));
        assert!(!check_version("3.12.6.1"));
    }

    #[test]
    fn test_check_version_invalid_major() {
        assert!(!check_version("2.12.6"));
    }

    #[test]
    fn test_check_version_non_numeric() {
        assert!(!check_version("3.12.a"));
    }

    // #[test]
    // fn test_download_file() {
    //     let dir = tempdir().unwrap();
    //     let file_path = dir.path().join("testfile");

    //     let mut server = mockito::Server::new();
    //     // Mock the HTTP request
    //     let url = server.url();
    //     let _mock = server.mock("GET", "/").with_status(200).with_body();

    //     download_file(&url, &file_path).unwrap();

    //     let content = std::fs::read_to_string(file_path).unwrap();
    //     assert_eq!(content, "test content");
    // }

    // #[test]
    // fn test_unzip_file() {
    //     let dir = tempdir().unwrap();
    //     let zip_path = dir.path().join("test.zip");
    //     let extract_path = dir.path().join("extract");

    //     // Create a zip file
    //     {
    //         let file = File::create(&zip_path).unwrap();
    //         let mut zip = ZipWriter::new(file);
    //         let options: FileOptions= FileOptions::default();
    //         zip.start_file("test.txt", options).unwrap();
    //         zip.write_all(b"Hello, world!").unwrap();
    //         zip.finish().unwrap();
    //     }

    //     // Unzip the file
    //     unzip_file(&zip_path, &extract_path).unwrap();

    //     // Verify the contents
    //     let extracted_file_path = extract_path.join("test.txt");
    //     let mut extracted_file = File::open(extracted_file_path).unwrap();
    //     let mut contents = String::new();
    //     extracted_file.read_to_string(&mut contents).unwrap();
    //     assert_eq!(contents, "Hello, world!");
    // }

    // #[test]
    // fn test_modify_pth_file() {
    //     let dir = tempdir().unwrap();
    //     let pth_path = dir.path().join("test.pth");

    //     // Create a .pth file
    //     {
    //         let mut file = File::create(&pth_path).unwrap();
    //         writeln!(file, "import site").unwrap();
    //         writeln!(file, "some other line").unwrap();
    //     }

    //     modify_pth_file(&pth_path).unwrap();

    //     let content = std::fs::read_to_string(&pth_path).unwrap();
    //     assert!(content.contains("import site"));
    //     assert!(content.contains("some other line"));
    // }
}
