import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Row {
    id: bar
    spacing: 6
    signal categoryChanged(string category)

    property string selectedCategory: "all"
    property var categories: ["all", "happy", "sad", "angry", "confused", "love", "animals", "people", "hands", "tables", "misc"]

    Repeater {
        model: bar.categories

        delegate: Rectangle {
            required property string modelData
            required property int index
            width: chipLabel.implicitWidth + 20
            height: 30
            radius: 15
            color: bar.selectedCategory === modelData ? palette.highlight : palette.alternateBase

            Label {
                id: chipLabel
                anchors.centerIn: parent
                text: modelData.charAt(0).toUpperCase() + modelData.slice(1)
                font.pixelSize: 12
                color: bar.selectedCategory === modelData ? palette.highlightedText : palette.text
            }

            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    bar.selectedCategory = modelData;
                    bar.categoryChanged(modelData);
                }
            }
        }
    }
}
