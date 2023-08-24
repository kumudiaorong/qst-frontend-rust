#pragma once
#ifndef QST_FRONTEND_EDIT_H
#define QST_FRONTEND_EDIT_H
#include <QLineEdit>
namespace qst {
  class Edit : public QLineEdit {
    Q_OBJECT
  public:
    Edit();
    ~Edit();
  protected:
    void keyPressEvent(QKeyEvent *) override;
    // void focusInEvent(QFocusEvent *) override;
  signals:
    void down();
    void up();
    void enter();
  };
}  // namespace qst
#endif  // QST_Edit_H