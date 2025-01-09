fn main() {
    let _ = MainWindow::new().expect("no valid slint").run();
}

slint::slint! {
    MainWindow := Window {
        Text {
            text: "rust slint example 1";
            color: red;
        }
    }
}
