#include <iostream>
#include "core.h"
int main(int argc, char *argv[]) {
  qst::QstFrontendCore core(argc, argv);
  // QObject::connect(&core, &QApplication::focusChanged, [](QWidget *old, QWidget *now) {
  //   if(now) {
  //     qDebug() << "now:" << now->objectName();
  //   }
  //   if(old) {
  //     qDebug() << "old:" << old->objectName();
  //   }
  // });
  return core.exec();
}
