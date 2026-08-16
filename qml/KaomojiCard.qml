import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: card
    property string chars: ""
    property string description: ""
    radius: 8
    color: mouseArea.containsMouse ? palette.highlight : palette.base
    border.color: mouseArea.containsMouse ? palette.highlight : "transparent"
    border.width: 1

    signal clicked(string kaomoji)
    signal rightClicked()

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 4
        spacing: 2

        Label {
            Layout.fillWidth: true
            Layout.fillHeight: true
            text: card.chars
            font.pixelSize: 20
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight
            wrapMode: Text.NoWrap
            color: mouseArea.containsMouse ? palette.highlightedText : palette.text
        }

        Label {
            Layout.fillWidth: true
            text: card.description
            font.pixelSize: 9
            horizontalAlignment: Text.AlignHCenter
            elide: Text.ElideRight
            color: palette.placeholderText
            visible: text.length > 0
        }
    }

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        hoverEnabled: true
        acceptedButtons: Qt.LeftButton | Qt.RightButton
        onClicked: function(mouse) {
            if (mouse.button === Qt.RightButton) {
                card.rightClicked();
            } else {
                card.clicked(card.chars);
            }
        }
        ToolTip {
            parent: mouseArea.parent
            visible: mouseArea.containsMouse
            text: card.chars + "\n" + card.description
            delay: 500
        }
    }
}
