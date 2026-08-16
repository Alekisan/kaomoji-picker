#include "clipboard.h"
#include <QGuiApplication>
#include <QClipboard>

void kaomoji_copy_to_clipboard(const QString& text) {
    QGuiApplication::clipboard()->setText(text);
}
