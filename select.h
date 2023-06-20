#ifndef SELECT_H
#define SELECT_H

#include <QVBoxLayout>
#include <QWidget>
class Select:public QWidget
{
    Q_OBJECT
    QVBoxLayout *mainLayout;
public:
    Select(QWidget *parent=nullptr);
    void setText(const QString &text);
    
};

#endif // SELECT_H
