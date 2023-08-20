
#include <memory>
#include <QKeyEvent>
#include <QLabel>
#include <QPushButton>
#include <QRandomGenerator>

#include "select.h"
namespace qst {
  Select::Select(QWidget *parent)
    : QVBoxLayout()
    , index(-1) {
    // mainLayout->addWidget(new QLabel("test"));
    this->setObjectName("select");
  }
  void Select::setText(const std::vector<qst::AppInfo>& apps) {
    // int number = QRandomGenerator::global()->bounded(10);
    // remove all widget
    QLayoutItem *child;
    // QLayoutItem *child = mainLayout->itemAt(0);
    // QLabel *label = dynamic_cast<QLabel *>(child->widget());
    while((child = this->takeAt(0)) != 0) {
      delete child->widget();
      delete child;
    }
    for(auto& app : apps) {
      QPushButton *pb = new QPushButton;
      pb->setText(QString::fromStdString(app.name()));
      this->addWidget(pb);
    }
    // this->addStretch(0);
    // label->setText("index: " + QString::number(index) + " size: " + QString::number(mainLayout->count()));
  }
  // void Select::focusInEvent(QFocusEvent *event) {
  //   event->gotFocus()
  //   QLayoutItem *child = mainLayout->itemAt(0);
  //   // QLabel *label = dynamic_cast<QLabel *>(child->widget());
  //   if(child != nullptr) {
  //     QPushButton *pb = dynamic_cast<QPushButton *>(child->widget());
  //     if(pb != nullptr) {
  //       pb->setFocus();
  //     }
  //   }
  //   // label->setText("index: " + QString::number(index) + " size: " + QString::number(mainLayout->count()));
  // }
  // void Select::keyPressEvent(QKeyEvent *event) {
  //   QLayoutItem *child = mainLayout->itemAt(0);
  //   // QLabel *label = dynamic_cast<QLabel *>(child->widget());
  //   switch(event->key()) {
  //     case Qt::Key_Down :
  //       if(index < 3) {
  //         ++index;
  //         child = mainLayout->itemAt(index);
  //         if(child != nullptr) {
  //           QPushButton *pb = dynamic_cast<QPushButton *>(child->widget());
  //           if(pb != nullptr) {
  //             pb->setFocus();
  //           }
  //         }
  //       }
  //       break;
  //     case Qt::Key_Up :
  //       if(index > 1) {
  //         --index;
  //         child = mainLayout->itemAt(index);
  //         if(child != nullptr) {
  //           QPushButton *pb = dynamic_cast<QPushButton *>(child->widget());
  //           if(pb != nullptr) {
  //             pb->setFocus();
  //           }
  //         }
  //       }
  //       break;
  //     default :
  //       QWidget::keyPressEvent(event);
  //   }
  //   qDebug() << "index: " << index << " size: " << mainLayout->count();
  //   // label->setText("index: ");
  // }
  void Select::focusFirst() {
    index = 0;
    qDebug() << "focusFirst index: " << index << " size: " << this->count();
    // QLayoutItem *child = mainLayout->itemAt(0);
    // QLabel *label = dynamic_cast<QLabel *>(child->widget());
    // label->setText("index: " + QString::number(index) + " size: " + QString::number(mainLayout->count()));
    QLayoutItem *child = this->itemAt(index);
    if(child != nullptr) {
      QPushButton *pb = dynamic_cast<QPushButton *>(child->widget());
      if(pb != nullptr) {
        pb->setFocus();
      }
    }
  }
  void Select::focusLast() {
    index = this->count() - 2;
    qDebug() << "focusLast index: " << index << " size: " << this->count();
    // QLayoutItem *child = mainLayout->itemAt(0);
    // QLabel *label = dynamic_cast<QLabel *>(child->widget());
    // label->setText("index: " + QString::number(index) + " size: " + QString::number(mainLayout->count()));
    QLayoutItem *child = this->itemAt(index);
    if(child != nullptr) {
      QPushButton *pb = dynamic_cast<QPushButton *>(child->widget());
      if(pb != nullptr) {
        pb->setFocus();
      }
    }
  }
}  // namespace qst
