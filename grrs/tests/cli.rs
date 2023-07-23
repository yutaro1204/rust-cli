use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use assert_fs::prelude::*;

// テストには全てにアノテーションが必要
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    // ? は Result を返す関数の後に付く
    // 本来であればそれを展開する必要があるが、? を付けることでエラーだったらエラーを返し、エラーじゃなければ中身を返すをインラインで書けるようになる
    // https://www.reddit.com/r/rust/comments/8zsx61/when_we_need_to_end_line_of_code_with_in_rust/

    // assert_fs でファイルを用意しなくてもテストファイルを作成できる
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("phantom").arg("phantom.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
