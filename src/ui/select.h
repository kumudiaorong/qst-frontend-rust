#pragma once
#ifndef QST_FRONTEND_SELECT_H
#define QST_FRONTEND_SELECT_H

#include <memory>
#include <QVBoxLayout>
#include <QWidget>
#include <vector>

#include "qst.pb.h"
namespace qst {
  class Select : public QVBoxLayout {
    Q_OBJECT
    int32_t index;
  public:
    Select(QWidget *parent = nullptr);
  protected:
    // void focusInEvent(QFocusEvent *event) override;
    // void keyPressEvent(QKeyEvent *event) override;

  public slots:
    void setText(const std::vector<qst::Display>& apps);
    void focusFirst();
    void focusLast();
  };
}  // namespace qst

#endif  // SELECT_H
