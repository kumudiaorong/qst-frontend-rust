#include <grpcpp/create_channel.h>

#include <algorithm>
#include <iostream>
#include <string_view>
#include <vector>

#include "comm.h"
#include "qst.pb.h"
namespace qst {
  Comm::Comm() {
  }
  Comm::Comm(const char *addr)
    : stub(std::make_unique<qst::Interact::Stub>(grpc::CreateChannel(addr, grpc::InsecureChannelCredentials()))) {
  }
  Comm::~Comm() {
  }
  void Comm::setAddr(const char *addr) {
    stub = std::make_unique<qst::Interact::Stub>(grpc::CreateChannel(addr, grpc::InsecureChannelCredentials()));
  }
  void Comm::listApp(std::string_view text) {
    //test
    std::cout << "listApp : " << text << std::endl;
    ::grpc::ClientContext context;
    Input request;
    request.set_str(text);
    auto reader = this->stub->ListApp(&context, request);
    std::vector<Display> apps;
    Display app;
    while(reader->Read(&app)) {
      apps.emplace_back(std::move(app));
    }
    ::grpc::Status status = reader->Finish();
    emit appListed(apps);
  }
  void Comm::runApp(const ExecHint& hint) {
    ::grpc::ClientContext context;
    Empty empty;
    auto status = stub->RunApp(&context, hint, &empty);
  }
}  // namespace qst