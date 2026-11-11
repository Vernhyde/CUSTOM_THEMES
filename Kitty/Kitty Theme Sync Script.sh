#!/usr/bin/env bash
# Sync Kitty with KDE Plasma Day/Night Color Scheme

KITTY_DIR="$HOME/.config/kitty"

apply_kitty_theme() {
    local scheme
    scheme=$(kreadconfig6 --file kdeglobals --group General --key ColorScheme)

    if [[ "$scheme" == *"Light"* ]]; then
        ln -sf "$KITTY_DIR/vernhyde-light.conf" "$KITTY_DIR/current-theme.conf"
    else
        ln -sf "$KITTY_DIR/vernhyde-dark.conf" "$KITTY_DIR/current-theme.conf"
    fi

    # Push live color update to all running Kitty windows via socket
    for sock in "$XDG_RUNTIME_DIR"/kitty.sock*; do
        [[ -S "$sock" ]] && kitty @ --to "unix:$sock" set-colors --all --configured "$KITTY_DIR/current-theme.conf" 2>/dev/null
    done
}

# Run once on startup to ensure sync
apply_kitty_theme

# Listen for KDE Plasma color scheme changes over DBus
dbus-monitor --session "type='signal',interface='org.kde.KGlobalSettings',member='notifyChange'" | \
while read -r line; do
    if [[ "$line" == *"notifyChange"* ]]; then
        apply_kitty_theme
    fi
done
