#!/bin/bash
# Sync Kitty theme with KDE Plasma Day/Night Color Scheme

dbus-monitor "path='/KGlobalSettings',member='notifyChange'" | while read -r line; do
    if echo "$line" | grep -q 'notifyChange'; then
        # Query current KDE scheme
        CURRENT_THEME=$(kreadconfig6 --group General --key ColorScheme)
        
        if [[ "$CURRENT_THEME" == *"Light"* ]]; then
            # Apply Light mode to all running Kitty instances instantly
            kitty +kitten themes --reload-in=all vernhyde-light
        else
            # Apply Dark mode
            kitty +kitten themes --reload-in=all vernhyde-dark
        fi
    fi
done