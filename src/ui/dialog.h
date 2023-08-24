#pragma once
#ifndef QST_FRONTEND_DIALOG_H
#define QST_FRONTEND_DIALOG_H

#include <memory>
#include <QDialog>
#include <QLineEdit>
#include <QMenuBar>
#include <QVBoxLayout>
#include <string_view>
#include <vector>

#include "edit.h"
#include "qst.pb.h"
#include "select.h"
#define DefaultWidth 360
QT_BEGIN_NAMESPACE
// namespace Ui { class Dialog; }
QT_END_NAMESPACE
namespace qst {
  class Dialog : public QDialog {
    Q_OBJECT
    std::unique_ptr<QVBoxLayout> mainLayout;
    std::unique_ptr<QMenuBar> menuBar;
    std::unique_ptr<Edit> lineEdit;
    std::unique_ptr<Select> select;
    std::vector<Display> apps;
    int32_t index = 0;
  public:
    Dialog(QWidget *parent = nullptr);
    ~Dialog();
  signals:
    void inputChanged(std::string_view text);
    void runApp(const ExecHint& hint);
  public slots:
    void updateList(const std::vector<Display>& apps);
    void down();
    void up();
    void finish();
    // QSize sizeHint() const override;
  private:
  protected slots:
    void _inputChanged(const QString& text);

    // Ui::Dialog *ui;
  };
}  // namespace qst
#endif  // DIALOG_H
