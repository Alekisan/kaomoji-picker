import QtQuick
import QtQuick.Controls

Label {
    padding: 12
    font.pixelSize: 13
    background: Rectangle {
        radius: 8
        color: palette.highlight
    }
    color: palette.highlightedText
    opacity: visible ? 1.0 : 0.0
    Behavior on opacity {
        NumberAnimation { duration: 200 }
    }
}
