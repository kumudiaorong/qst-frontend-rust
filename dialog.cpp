// #include "./ui_dialog.h"
#include "dialog.h"
#include <QFormLayout>
#include <QLabel>
#include <QMenuBar>
#include <cstdlib>
#include <qlabel.h>
Dialog::Dialog(QWidget *parent)
    : QDialog(parent)
// , ui(new Ui::Dialog)
{
  // ui->setupUi(this);
  selectWidget = new Select;
  menuBar = new QMenuBar;
  lineEdit = new QLineEdit;
  lineEdit->setPlaceholderText("123456");
  // lineEdit->setSizePolicy(QSizePolicy::Policy::Maximum,QSizePolicy::Policy::Maximum);
  QMenu *fileMenu = new QMenu(tr("&File"), this);
  // exitAction = fileMenu->addAction(tr("E&xit"));
  menuBar->addMenu(fileMenu);

  // QFormLayout *mainLayout = new QFormLayout;
  mainLayout = new QVBoxLayout;

  QLabel *label = new QLabel("Hello World");
  label->setStyleSheet("border: 1px solid black;");
  // label->setMinimumWidth(int minw)
  label->setSizePolicy(QSizePolicy::Policy::MinimumExpanding,
                       QSizePolicy::Policy::MinimumExpanding);
  // label->setFixedWidth(lineEdit->width());
  // label->setScaledContents(true);
  mainLayout->setMenuBar(menuBar);
  // mainLayout->addWidget(label);
  mainLayout->addWidget(lineEdit);
  mainLayout->addWidget(selectWidget);
  // mainLayout->addLayout()
  // mainLayout->setSpacing(1);
  // mainLayout->addStretch(1);
  // QSizePolicy sizePolicy(QSizePolicy::Policy::MinimumExpanding,
  // QSizePolicy::MinimumExpanding); this->setSizePolicy(sizePolicy);
  // label->setBaseSize(DefaultWidth, DefaultWidth);
  // this->setMaximumWidth(DefaultWidth);
  this->setMinimumWidth(DefaultWidth);
  this->setLayout(mainLayout);
  // this->setSizePolicy(QSizePolicy::Policy::MinimumExpanding,
  //                     QSizePolicy::Policy::Minimum);

  // this-
  // this->setFixedWidth(2000);
  // connect(exitAction, &QAction::triggered, this, &QDialog::accept);
  connect(lineEdit, &QLineEdit::textChanged, label, &QLabel::setText);
  connect(lineEdit, &QLineEdit::textChanged, selectWidget, &Select::setText);
}
QSize Dialog::sizeHint() const { return this->layout()->sizeHint(); }

Dialog::~Dialog() {
  // delete ui;
}
