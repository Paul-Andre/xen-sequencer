import QtQuick 2.0



    Item {
        id: key
        x: 56
        y: 277
        width: 40
        height: 40
        property string text: ""
        property int fontSize: 10

        signal clicked

        BorderImage {
            id: keyUp
            anchors.fill: parent
            visible: !mouseArea.pressed
            source: "../../Inkscape Crafts/path3336.png"
        }

        BorderImage {
            id: keyDown
            anchors.right: parent.right
            anchors.rightMargin: 0
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.top: parent.top
            visible: mouseArea.pressed
            source: "../../Inkscape Crafts/path4192.png"
        }

        Text {
            id: text
            x: 8
            y: 13
            color: "#fff7f7"
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.verticalCenter: parent.verticalCenter
            font.pixelSize: parent.text
            font.underline: none.none
        }

        MouseArea {
            id: mouseArea
            x: 0
            y: 0
            width: 40
            height: 40
        }


    Connections {
        target: mouseArea
        onClicked: item.clicked()
    }
    }


