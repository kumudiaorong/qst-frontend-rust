#ifndef DIALOG_H
#define DIALOG_H

#include "select.h"
#include <QDialog>
#include <QLineEdit>
#include <QVBoxLayout>
#include <QMenuBar>
#define DefaultWidth 360
QT_BEGIN_NAMESPACE
// namespace Ui { class Dialog; }
QT_END_NAMESPACE

class Dialog : public QDialog {
  Q_OBJECT
  QVBoxLayout *mainLayout;
  QMenuBar *menuBar;
  QLineEdit *lineEdit;
  Select *selectWidget;

public:
  Dialog(QWidget *parent = nullptr);
  QSize sizeHint() const override;
  ~Dialog();

private:
  // Ui::Dialog *ui;
};
#endif // DIALOG_H
