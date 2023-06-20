#include "select.h"
#include <QRandomGenerator>
#include <QPushButton>
Select::Select(QWidget *parent):QWidget(parent)
{
    mainLayout = new QVBoxLayout;
    this->setLayout(mainLayout);
}
void Select::setText(const QString &text)
{
    
    int number =QRandomGenerator::global()->bounded(10);
    //remove all widget
    QLayoutItem *child;
    while ((child = mainLayout->takeAt(0)) != 0) {
        delete child->widget();
        delete child;
    }
    for(int i=0;i<number;i++)
    {
        QPushButton *pb = new QPushButton;
        pb->setText(text+QString::number(i));
        mainLayout->addWidget(pb);
    }
    mainLayout->addStretch(0);
}
