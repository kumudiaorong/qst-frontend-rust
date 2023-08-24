#include <QApplication>

#include "comm.h"
#include "core.h"
#include "dialog.h"
namespace qst {
  QstFrontendCore::QstFrontendCore(int& argc, char *argv[])
    : QApplication(argc, argv) {
    this->setApplicationDisplayName("Qst");
    for(int i = 1; i < argc; i++) {
      if(strcmp(argv[i], "--addr") == 0) {
        comm.setAddr(argv[++i]);
      }
    }
    QObject::connect(&dialog, &Dialog::inputChanged, &comm, &Comm::listApp);
    QObject::connect(&comm, &Comm::appListed, &dialog, &Dialog::updateList);
    QObject::connect(&dialog, &Dialog::runApp, &comm, &Comm::runApp);
    dialog.show();
  }
  QstFrontendCore::~QstFrontendCore(){}
}  // namespace qst