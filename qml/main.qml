import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import com.kaomoji.picker 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 480
    height: 600
    title: "Kaomoji Picker"
    color: palette.window
    flags: Qt.WindowStaysOnTopHint

    property bool editorOpen: false
    property string toastText: ""
    property bool toastVisible: false

    function showToast(msg) {
        root.toastText = msg
        root.toastVisible = true
        toastTimer.restart()
    }

    function refresh() {
        gridModel.clear()
        for (var i = 0; i < controller.filtered_count; i++) {
            gridModel.append({
                chars: controller.getChars(i),
                description: controller.getDescription(i),
                categories: controller.getCategories(i),
                index: i
            })
        }
    }

    Timer {
        id: toastTimer
        interval: 1500
        onTriggered: root.toastVisible = false
    }

    KaomojiController {
        id: controller
    }

    Component.onCompleted: {
        controller.loadAll()
        root.refresh()
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 12
        spacing: 10

        TextField {
            id: searchField
            Layout.fillWidth: true
            placeholderText: "Search kaomoji..."
            onTextChanged: {
                controller.setSearchText(text)
                root.refresh()
            }
            leftPadding: 30
            background: Rectangle {
                radius: 8
                color: palette.alternateBase
                border.color: searchField.activeFocus ? palette.highlight : "transparent"
                border.width: 1
                Image {
                    anchors.left: parent.left
                    anchors.leftMargin: 8
                    anchors.verticalCenter: parent.verticalCenter
                    source: "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%23888' stroke-width='2'%3E%3Ccircle cx='11' cy='11' r='8'/%3E%3Cline x1='21' y1='21' x2='16.65' y2='16.65'/%3E%3C/svg%3E"
                }
            }
        }

        CategoryBar {
            id: categoryBar
            Layout.fillWidth: true
            onCategoryChanged: function(cat) {
                controller.setCategory(cat)
                root.refresh()
            }
        }

        RowLayout {
            Layout.fillWidth: true
            Label {
                text: controller.filtered_count + " kaomoji"
                font.pixelSize: 12
                color: palette.placeholderText
            }
            Item { Layout.fillWidth: true }
            Button {
                text: "+ Add"
                flat: true
                onClicked: root.editorOpen = !root.editorOpen
            }
            Button {
                text: "Import"
                flat: true
                onClicked: importDialog.open()
            }
            Button {
                text: "Export"
                flat: true
                onClicked: exportDialog.open()
            }
        }

        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: root.editorOpen ? 1 : 0

            GridView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                cellWidth: 80
                cellHeight: 80
                model: gridModel

                delegate: KaomojiCard {
                    width: 80
                    height: 80
                    chars: model.chars
                    description: model.description
                    onClicked: function(kaomoji) {
                        controller.copyToClipboard(kaomoji)
                        root.showToast("Copied: " + kaomoji)
                    }
                }
            }

            EditorPanel {
                Layout.fillWidth: true
                Layout.fillHeight: true
                onAdded: function(chars, desc, cats) {
                    controller.addKaomoji(chars, desc, cats)
                    root.refresh()
                    root.showToast("Added: " + chars)
                    root.editorOpen = false
                }
                onCancelled: {
                    root.editorOpen = false
                }
            }
        }
    }

    Toast {
        anchors.bottom: parent.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottomMargin: 20
        text: root.toastText
        visible: root.toastVisible
    }

    FileDialog {
        id: importDialog
        title: "Import Kaomoji"
        nameFilters: ["JSON files (*.json)"]
        onAccepted: {
            controller.importJson(selectedFile.toString().replace("file://", ""))
            root.refresh()
        }
    }

    FileDialog {
        id: exportDialog
        title: "Export Custom Kaomoji"
        nameFilters: ["JSON files (*.json)"]
        fileMode: FileDialog.SaveFile
        onAccepted: {
            var result = controller.exportJson(selectedFile.toString().replace("file://", ""))
            if (result === "ok") {
                root.showToast("Exported successfully")
            } else {
                root.showToast("Export failed: " + result)
            }
        }
    }

    ListModel {
        id: gridModel
    }
}
