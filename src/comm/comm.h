#pragma once
#ifndef QST_FRONTEND_COMM_H
#define QST_FRONTEND_COMM_H

#include <memory>
#include <QObject>
#include <string_view>
#include <vector>
// #include "qst.grpc.pb.h"
#include "qst.grpc.pb.h"
#include "qst.pb.h"
namespace qst {
  class Comm : public QObject {
    Q_OBJECT
    std::unique_ptr<qst::Interact::Stub> stub;
  public:
    Comm();
    Comm(const char *addr);
    ~Comm();
    void setAddr(const char *addr);
  public slots:
    void listApp(std::string_view text);
    void runApp(const ExecHint& hint);
  signals:
    void appListed(const std::vector<Display>& apps);
  };
}  // namespace qst
#endif