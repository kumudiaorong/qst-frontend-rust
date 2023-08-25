#include <QApplication>

#include "comm.h"
#include "core.h"
#include "dialog.h"
// #include "spdlog/async.h"
#include "spdlog/common.h"
// #include "spdlog/sinks/stdout_color_sinks.h"
#include "spdlog/spdlog.h"
namespace qst {
  QstFrontendCore::QstFrontendCore(int& argc, char *argv[])
    : QApplication(argc, argv)
  // , logger(
  // spdlog::create_async<spdlog::sinks::stdout_color_sink_mt>("backend")
  // )
  {
    // spdlog::set_default_logger(logger);
    spdlog::set_level(spdlog::level::trace);
    this->setApplicationDisplayName("Qst");
    bool ok = false;
    for(int i = 1; i < argc; i++) {
      if(strcmp(argv[i], "--addr") == 0) {
        comm.setAddr(argv[++i]);
        ok = true;
      }
    }
    if(!ok) {
      spdlog::error("No address specified");
      std::exit(1);
    }
    QObject::connect(&dialog, &Dialog::inputChanged, &comm, &Comm::listApp);
    QObject::connect(&comm, &Comm::appListed, &dialog, &Dialog::updateList);
    QObject::connect(&dialog, &Dialog::runApp, &comm, &Comm::runApp);
    dialog.show();
  }
  QstFrontendCore::~QstFrontendCore() {
  }
}  // namespace qst