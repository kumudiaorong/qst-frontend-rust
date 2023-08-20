#ifndef DIALOG_H
#define DIALOG_H

#include <vector>
#include <memory>
#include <QDialog>
#include <QLineEdit>
#include <QMenuBar>
#include <QVBoxLayout>

#include "cpp/qst.grpc.pb.h"
#include "cpp/qst.pb.h"
#include "edit.h"
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
    std::unique_ptr<qst::Interact::Stub> stub;
    std::vector<AppInfo> apps;
    int32_t index = 0;
  public:
    Dialog(QWidget *parent = nullptr);
    ~Dialog();
  public Q_SLOTS:
    void updateList(const QString& text);
    void down();
    void up();
    void finish();
    // QSize sizeHint() const override;
  private:
    // Ui::Dialog *ui;
  };
}  // namespace qst
#endif  // DIALOG_H
