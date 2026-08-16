use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.kaomoji.picker")
            .qml_file("qml/main.qml")
            .qml_file("qml/KaomojiGrid.qml")
            .qml_file("qml/KaomojiCard.qml")
            .qml_file("qml/CategoryBar.qml")
            .qml_file("qml/EditorPanel.qml")
            .qml_file("qml/Toast.qml"),
    )
    .file("src/kaomoji/model.rs")
    .cpp_file("cpp/clipboard.cpp")
    .include_dir("cpp")
    .build();
}
