import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

GridView {
    id: grid
    clip: true
    cellWidth: 80
    cellHeight: 80

    signal kaomojiClicked(string chars)
    signal kaomojiRightClicked(int index)

    delegate: KaomojiCard {
        width: grid.cellWidth
        height: grid.cellHeight
        chars: model.chars
        description: model.description
        onClicked: grid.kaomojiClicked(model.chars)
        onRightClicked: grid.kaomojiRightClicked(model.index)
    }

    add: Transition {
        NumberAnimation { property: "opacity"; from: 0; to: 1; duration: 150 }
    }
}
