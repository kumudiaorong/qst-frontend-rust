#include <iostream>
#include "core.h"
int main(int argc, char *argv[]) {
  std::cout << "main : argc = " << argc << ", argv = ";
  for(int i = 1; i < argc; i++) {
    std::cout <<"\"" << argv[i] << "\" ";
  }
  std::cout << std::endl;
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
