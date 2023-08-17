#ifndef SELECT_H
#define SELECT_H

#include <memory>
#include <QVBoxLayout>
#include <QWidget>
#include <vector>

#include "comm/qst.pb.h"
namespace qst {
  class Select : public QVBoxLayout {
    Q_OBJECT
    int32_t index;
  public:
    Select(QWidget *parent = nullptr);
  protected:
    // void focusInEvent(QFocusEvent *event) override;
    // void keyPressEvent(QKeyEvent *event) override;

  public Q_SLOTS:
    void setText(const std::vector<qst::AppInfo>& apps);
    void focusFirst();
    void focusLast();
  };
}  // namespace qst

#endif  // SELECT_H
