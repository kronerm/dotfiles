# source /etc/profile with bash
if status is-login
    exec bash -c "test -e /etc/profile && source /etc/profile;\
    exec fish"
end

# set dotfiles directory
set -x DOTFILES "$HOME/.dotfiles"

# set default programs
set -x EDITOR nvim
set -x TERMINAL foot

# use `bat` as `man` pager
set -x MANPAGER "sh -c 'col -bx | bat -l man -p'"
set -x MANROFFOPT -c

# `delta` mouse scrolling
set -x DELTA_PAGER "less -R --mouse"

# set xdg variables
set -x XDG_CACHE_HOME "$HOME/.cache"
set -x XDG_CONFIG_HOME "$HOME/.config"
set -x XDG_DATA_HOME "$HOME/.local/share"
set -x XDG_STATE_HOME "$HOME/.local/state"

# clean up $HOME
set -x ANDROID_USER_HOME "$XDG_DATA_HOME/android"
set -x ANSIBLE_HOME "$XDG_DATA_HOME/ansible"
set -x CARGO_HOME "$XDG_DATA_HOME/cargo"
set -x CUDA_CACHE_PATH "$XDG_CACHE_HOME/nv"
set -x DOT_SAGE "$XDG_CONFIG_HOME/sage"
set -x GNUPGHOME "$XDG_DATA_HOME/gnupg"
set -x GOPATH "$XDG_DATA_HOME/go"
set -x GTK2_RC_FILES "$XDG_CONFIG_HOME/gtk-2.0/gtkrc"
set -x IPYTHONDIR "$XDG_CONFIG_HOME/ipython"
set -x JUPYTER_CONFIG_DIR "$XDG_CONFIG_HOME/jupyter"
set -x LESSHISTFILE -
set -x MATHEMATICA_USERBASE "$XDG_CONFIG_HOME/mathematica"
set -x NPM_CONFIG_CACHE "$XDG_CACHE_HOME/npm"
set -x NPM_CONFIG_TMP "$XDG_RUNTIME_DIR/npm"
set -x NPM_CONFIG_USERCONFIG "$XDG_DATA_HOME/npm/config"
set -x NUGET_PACKAGES "$XDG_CACHE_HOME/NuGetPackages"
set -x PASSWORD_STORE_DIR "$XDG_DATA_HOME/password-store"
set -x PSQL_HISTORY "$XDG_DATA_HOME/psql_history"
set -x RUSTUP_HOME "$XDG_DATA_HOME/rustup"
set -x TS3_CONFIG_DIR "$XDG_CONFIG_HOME/ts3client"
set -x XINITRC "$XDG_CONFIG_HOME/X11/xinitrc"
set -x _JAVA_OPTIONS "-Djava.util.prefs.userRoot=$XDG_CONFIG_HOME/java"

# update PATH
function prepend_path
    if echo "$PATH" | grep -Fqv "$argv"
        set -x PATH "$argv:$PATH"
    end
end
function append_path
    if echo "$PATH" | grep -Fqv "$argv"
        set -x PATH "$PATH:$argv"
    end
end

append_path /usr/bin/vendor_perl
prepend_path "$HOME/.dotnet/tools"
prepend_path "$CARGO_HOME/bin"
prepend_path "$HOME/.local/share/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin"
prepend_path "$HOME/.local/share/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin"
prepend_path "$HOME/.local/bin"
prepend_path "$DOTFILES/.scripts"

# firefox with wayland
set -x MOZ_ENABLE_WAYLAND 1

# explicit specification of video driver
set -x VDPAU_DRIVER radeonsi
set -x LIBVA_DRIVER_NAME radeonsi

# qt wayland
set -x QT_QPA_PLATFORM wayland

# anki wayland
set -x ANKI_WAYLAND 1

# disable cows in ansible
set -x ANSIBLE_NOCOWS 1

# force sdl wayland
set -x SDL_VIDEODRIVER wayland

set -x OBSIDIAN_USE_WAYLAND 1
