fn main() {
    let _ = MainWindow::new().expect("no valid slint").run();
}

slint::slint! {
    component ImageLabel inherits Rectangle {
        width: 100px;
        height: 100px;
        background: #0000ff;

        Image {
            source: @image-url("images/config.png");
            width: parent.width;
            height: parent.height;
        }
    }

    export component MainWindow inherits Window {
        ImageLabel {}
    }
}
