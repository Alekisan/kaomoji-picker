import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: editor
    spacing: 10

    signal added(string chars, string description, string categories)
    signal cancelled()

    Label {
        text: "Add Custom Kaomoji"
        font.bold: true
        font.pixelSize: 16
    }

    TextField {
        id: charsField
        Layout.fillWidth: true
        placeholderText: "Kaomoji characters (e.g. (◕‿◕))"
        font.pixelSize: 18
    }

    TextField {
        id: descField
        Layout.fillWidth: true
        placeholderText: "Description (e.g. happy face)"
    }

    Label {
        text: "Categories (comma-separated):"
        font.pixelSize: 12
        color: palette.placeholderText
    }

    TextField {
        id: catsField
        Layout.fillWidth: true
        placeholderText: "e.g. happy, misc"
        text: "misc"
    }

    RowLayout {
        Layout.fillWidth: true
        Item { Layout.fillWidth: true }
        Button {
            text: "Cancel"
            flat: true
            onClicked: editor.cancelled()
        }
        Button {
            text: "Add Kaomoji"
            enabled: charsField.text.length > 0
            onClicked: {
                editor.added(charsField.text, descField.text, catsField.text);
                charsField.text = "";
                descField.text = "";
                catsField.text = "misc";
            }
        }
    }
}
