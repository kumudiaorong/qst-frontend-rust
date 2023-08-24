#pragma once
#ifndef QST_FRONTEND_CORE_H
#define QST_FRONTEND_CORE_H
#include <QApplication>

#include "comm.h"
#include "dialog.h"
#include "spdlog/spdlog.h"

namespace qst {
  class QstFrontendCore : public QApplication {
    Dialog dialog;
    Comm comm;
    std::shared_ptr<spdlog::logger> logger;
  public:
    QstFrontendCore(int& argc, char *argv[]);
    ~QstFrontendCore();
  };
}  // namespace qst
#endif  // QST_FRONTEND_CORE_HPP