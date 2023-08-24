#include <qnamespace.h>
#include <qpushbutton.h>

#include <QKeyEvent>

#include "edit.h"
namespace qst {
  Edit::Edit()
    : QLineEdit() {
    this->setFocusPolicy(Qt::StrongFocus);
    this->setObjectName("edit");
  }
  Edit::~Edit() {
  }
  void Edit::keyPressEvent(QKeyEvent *event) {
    switch(event->key()) {
      case Qt::Key_Down :
        // this->nextInFocusChain()->setFocus();
        // qDebug() << "down " << this->nextInFocusChain()->objectName();
        emit down();
        break;
      case Qt::Key_Up :
        // this->previousInFocusChain()->setFocus();
        // qDebug() << "up " << this->previousInFocusChain()->objectName();
        emit up();
        break;
      case Qt::Key_Enter :
      case Qt::Key_Return :
        emit enter();
        break;
      default :
        QLineEdit::keyPressEvent(event);
    }
  }
  // void Edit::focusInEvent(QFocusEvent *event) {
  //   // QLineEdit::focusInEvent(event);
  //   qDebug() << "focusInEvent " << this->nextInFocusChain()->objectName();
  // }
}  // namespace qst