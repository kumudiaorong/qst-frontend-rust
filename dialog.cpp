// #include "./ui_dialog.h"
#include <grpc/grpc.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/create_channel.h>
#include <grpcpp/security/credentials.h>
#include <qlineedit.h>
#include <qnamespace.h>
#include <qpushbutton.h>
#include <qstyle.h>
#include <qwindowdefs.h>

#include <memory>
#include <QFormLayout>
#include <QLabel>
#include <QMenuBar>
#include <QPushButton>
#include <QStyleOptionButton>

#include "comm/qst.grpc.pb.h"
#include "comm/qst.pb.h"
#include "dialog.h"
#include "edit.h"
#include "select.h"
namespace qst {
  Dialog::Dialog(QWidget *parent)
    : QDialog(parent)
    , index(0)
  // , ui(new Ui::Dialog)
  {
    // ui->setupUi(this);
    stub =
      std::make_unique<qst::Interact::Stub>(grpc::CreateChannel("localhost:50051", grpc::InsecureChannelCredentials()));
    setObjectName("Dialog");
    select = std::make_unique<Select>();
    menuBar = std::make_unique<QMenuBar>();
    lineEdit = std::make_unique<Edit>();
    // lineEdit->setPlaceholderText("123456");
    // lineEdit->setSizePolicy(QSizePolicy::Policy::Maximum,QSizePolicy::Policy::Maximum);
    QMenu *fileMenu = new QMenu(tr("&File"), this);
    // exitAction = fileMenu->addAction(tr("E&xit"));
    menuBar->addMenu(fileMenu);
    // QFormLayout *mainLayout = new QFormLayout;
    mainLayout = std::make_unique<QVBoxLayout>();
    mainLayout->setObjectName("mainLayout");
    // QLabel *label = new QLabel("Hello World");
    // label->setStyleSheet("border: 1px solid black;");
    // // label->setMinimumWidth(int minw)
    // label->setSizePolicy(QSizePolicy::Policy::MinimumExpanding,
    //                      QSizePolicy::Policy::MinimumExpanding);
    // label->setFixedWidth(lineEdit->width());
    // label->setScaledContents(true);
    mainLayout->setMenuBar(menuBar.get());
    // mainLayout->addWidget(label);
    mainLayout->addWidget(lineEdit.get());
    // mainLayout->addLayout(select.get());
    // mainLayout->addLayout()
    // mainLayout->setSpacing(1);
    // mainLayout->addStretch(1);
    // QSizePolicy sizePolicy(QSizePolicy::Policy::MinimumExpanding,
    // QSizePolicy::MinimumExpanding); this->setSizePolicy(sizePolicy);
    // label->setBaseSize(DefaultWidth, DefaultWidth);
    // this->setMaximumWidth(DefaultWidth);
    this->setMinimumWidth(DefaultWidth);
    this->setLayout(mainLayout.get());
    // this->setSizePolicy(QSizePolicy::Policy::MinimumExpanding,
    //                     QSizePolicy::Policy::Minimum);

    // this-
    // this->setFixedWidth(2000);
    // connect(exitAction, &QAction::triggered, this, &QDialog::accept);
    // connect(lineEdit, &QLineEdit::textChanged, label, &QLabel::setText);
    connect(lineEdit.get(), &QLineEdit::textChanged, this, &Dialog::updateList);
    connect(lineEdit.get(), &Edit::down, this, &Dialog::down);
    connect(lineEdit.get(), &Edit::up, this, &Dialog::up);
    connect(lineEdit.get(), &Edit::enter, this, &Dialog::finish);
  }
  // QSize Dialog::sizeHint() const { return this->layout()->sizeHint(); }
  void Dialog::updateList(const QString& text) {
    ::grpc::ClientContext context;
    Input request;
    request.set_str(text.toStdString());
    auto reader = stub->ListApp(&context, request);
    apps.clear();
    AppInfo app;
    while(reader->Read(&app)) {
      apps.push_back(app);
    }
    ::grpc::Status status = reader->Finish();
    // remove all widget
    index = 0;
    QLayoutItem *child;
    qDebug() << "UpdateList " << mainLayout->count();
    while((child = mainLayout->takeAt(1)) != nullptr) {
      delete child->widget();
      delete child;
    }
    for(auto& app : apps) {
      QPushButton *pb = new QPushButton;
      pb->setAutoExclusive(true);
      // pb->setDefault(false);
      pb->setCheckable(true);
      // pb->setAutoDefault(true);
      // pb->setShortcut(Qt::Key_Enter);
      connect(pb, &QPushButton::pressed, this, &Dialog::finish);
      pb->setText(QString::fromStdString(app.name()));
      pb->setObjectName(QString::fromStdString(app.name()));
      mainLayout->addWidget(pb);
    }
    mainLayout->addStretch(0);
  }
  void Dialog::down() {
    qDebug() << "down " << index << " " << mainLayout->count();
    if(index < mainLayout->count() - 1 - 1) {
      index++;
      dynamic_cast<QPushButton *>(mainLayout->itemAt(index)->widget())->setChecked(true);
    }
  }
  void Dialog::up() {
    qDebug() << "up " << index << " " << mainLayout->count();
    if(index > 1) {
      index--;
      dynamic_cast<QPushButton *>(mainLayout->itemAt(index)->widget())->setChecked(true);
    }
  }
  void Dialog::finish() {
    qDebug() << "finish " << index << " " << mainLayout->count();
    if(index > 0) {
      ::grpc::ClientContext context;
      Empty empty;
      auto status = stub->RunApp(&context, apps[index - 1], &empty);
      qDebug() << "finish " << status.ok();
    }
  }
  Dialog::~Dialog() {
    // delete ui;
  }
}  // namespace qst
