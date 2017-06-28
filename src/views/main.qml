import QtQuick 2.2
import QtQuick.Controls 1.1
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

ApplicationWindow
{
  x: 400
  y: 200
  width: 400
  height: 300
  title: "Tickets"
  Component.onCompleted: visible = true

  ListView {
    width: 400
    height: 300

    model: tickets
    delegate: Text {
      text: title + ": " + description + "@ " + created_at
    }
  }
}
