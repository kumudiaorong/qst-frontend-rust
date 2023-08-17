/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>

#include <chrono>
#include <iostream>
#include <memory>
#include <optional>
#include <random>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

#include "comm/qst.grpc.pb.h"
#include "comm/qst.pb.h"

using grpc::Channel;
using grpc::ClientContext;
using grpc::ClientReader;
using grpc::ClientReaderWriter;
using grpc::ClientWriter;
using grpc::Status;

class QstClient {
public:
  QstClient(std::shared_ptr<Channel> channel)
    : stub_(qst::Interact::NewStub(channel)) {
  }

  std::optional<qst::AppInfo> Query(std::string_view input) {
    ClientContext context;
    qst::Input request{};
    qst::AppInfo info{};
    request.set_str(input);
    Status status = stub_->Query(&context, request, &info);
    if(!status.ok()) {
      std::cout << "GetFeature rpc failed." << std::endl;
      return std::nullopt;
    }
    return info;
  }
  std::vector<qst::AppInfo> ListApp(std::string_view input) {
    ClientContext context;
    qst::Input request{};
    qst::AppInfo info{};
    request.set_str(input);
    std::vector<qst::AppInfo> infos{};
    auto reader(stub_->ListApp(&context, request));
    while(reader->Read(&info)) {
      infos.push_back(info);
      std::cout << info.name() << ' ' << info.exec() << std::endl;
    }
    Status status = reader->Finish();
    if(!status.ok()) {
      std::cout << "ListApp rpc failed." << std::endl;
    }
    return infos;
  }
private:
  std::unique_ptr<qst::Interact::Stub> stub_;
};

int main(int argc, char **argv) {
  // Expect only arg: --db_path=path/to/route_guide_db.json.
  QstClient qst(grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials()));
  std::cout << "--------------" << std::endl;
  std::string input;
  while(std::cin >> input) {
    auto infos = qst.ListApp(input);
  }

  return 0;
}
