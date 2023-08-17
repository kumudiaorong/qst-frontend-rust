#include <QApplication>
#include <QWidget>

#include "dialog.h"
int main(int argc, char *argv[]) {
  QApplication a(argc, argv);
  QObject::connect(&a, &QApplication::focusChanged, [](QWidget *old, QWidget *now) {
    if(now) {
      qDebug() << "now:" << now->objectName();
    }
    if(old) {
      qDebug() << "old:" << old->objectName();
    }
  });
  qst::Dialog w;
  w.show();
  return a.exec();
  return 0;
}
