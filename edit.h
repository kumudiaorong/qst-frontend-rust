#ifndef QST_EDIT_H
#define QST_EDIT_H
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
  Q_SIGNALS:
    void down();
    void up();
    void enter();
  };
}  // namespace qst
#endif  // QST_Edit_H