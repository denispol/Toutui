#!/usr/bin/env bash
# Install Toutui and dependencies automagically.

set -eo pipefail

main() {
    do_not_run_as_root

    # Grab essential variables
    OS=$(identify_os)
    USER=${USER:-$(grab_username)}
    HOME=${HOME:-$(grab_home_dir)}
    CONFIG_DIR="${XDG_CONFIG_HOME:-$(grab_config_dir)}/toutui"
    INSTALL_DIR="${2:-$(grab_install_dir)}"

    load_dependencies
    load_exit_codes

    # Adjust script to OS
    case $OS in
        linux) DISTRO="$(get_distro)";;
	macOS) DISTRO="hungry for apples?";;
        *)     install_from_source;;
    esac

    case $1 in
        --install|install) install_toutui && exit $EXIT_OK || exit $EXIT_FAIL;;
        --update|update) update_toutui && exit $EXIT_OK || exit $EXIT_FAIL;;
        *) usage "INCORRECT_ARG";;
    esac
}

load_dependencies() {
    # Hard Coded dependencies here.
    # os:package_to_install(:cmd)?
    HC_DEPS=(
	linux:curl \
	linux:vlc  \
	linux:pkg-config \
	linux:git \
	debian:libssl-dev:no_check \
	linux:sqlite3 \
	debian:libsqlite3-dev:no_check \
	centos:libsqlite3-dev:no_check \
	macOS:git \
	macOS:sqlite3 \
	macOS:vlc \
	macOS:curl \
	macOS:pkg-config \
	%macOS:openssl \
	*centos:epel-release \
	*linux:kitty \
	*macOS:kitty \
	macOS:netcat\
	debian:netcat \
	fedora:nc \
	centos:nc \
	arch:gnu-netcat:netcat \
	opensuse:netcat \
	)
    # Dependencies starting with a '*' are optional
    # Starting with "linux:" for all linux distros
    # Starting with "macOS:" for macOS specific
    # Starting with "debian: for debian only
    # See also "arch:", "fedora:", "opensuse:", "centos:"
    # Ending with ":cmd" is optional but serves to
    # know how to check if the desired program is
    # installed on the machine.
    # For example: "linux:sqlite3:sqlite" means checking
    # for sqlite3 installation by launching sqlite.
    # ":no_check" means do not check program's existence
    # by launching it.
}

identify_os() {
    case $OSTYPE in
	darwin*) os="macOS";;
	linux*)  os="linux";;
	*) os="unknown";;
    esac
    echo $os
}

grab_username() {
    local user=${USER:-$(whoami 2>/dev/null)}
    user=${user:-$(id -un 2>/dev/null)}
    if [[ -z "$user" ]]; then
	echo "[ERROR] Cannot find username."
	exit 1
    fi
    echo "$user"
}

grab_home_dir() {
    local home=${HOME:-~/$USER}
    if ! [[ -d "$home" ]]; then home=${home:-/home/$USER}; fi
    if ! [[ -d "$home" ]]; then home=${home:-/Users/$USER}; fi
    if ! [[ -d "$home" ]]; then
	echo "[ERROR] Cannot find \"$USER\" home directory."
	exit 1
    fi
    echo $home
}

grab_config_dir() {
    local config="${XDG_CONFIG_HOME}"
    if [[ $OS == "macOS" && ! -d "$config" ]]; then config="${config:-$HOME/Library/Preferences}"; fi
    if [[ $OS == "macOS" && ! -d "$config" ]]; then config="${config:-$HOME/Library/Application Support}"; fi
    if ! [[ -d "$config" ]]; then config="${config:-$HOME/.config}"; fi
    if ! [[ -d "$config" ]]; then
	echo "[ERROR] Cannot find \"$USER\" config directory."
	exit $EXIT_CONFIG
    fi
    echo "${config}"
}

grab_install_dir() {
    local install_dir="${INSTALL_DIR}"
    if [[ $OS == "linux" ]]; then
        case $DISTRO in
	    *) install_dir="${install_dir:-/usr/bin}" ;;
	esac
    elif [[ $OS == "macOS" ]]; then
        install_dir="${install_dir:-/usr/local/bin}"
    fi
    if ! [[ -d "$install_dir" ]]; then
	echo "[ERROR] Cannot locate install directory \"$install_dir\"."
	exit $EXIT_INSTALL_DIR
    fi
    echo "${install_dir}"
}

usage() {
    local exit_code=$1
    echo "Usage: $ /bin/bash ./$(basename $0) <install|update> [install_directory]"
    echo "Help:"
    echo " --install: install toutui and dependencies."
    echo " --update: update toutui and dependencies."
    echo "Example: /bin/bash ./$(basename $0) install /usr/bin"
    eval "exit \$EXIT_${exit_code}"
}

get_distro() {
    local distro=$(head -n1 /etc/os-release 2>/dev/null| sed -E "s%.*\"([^\"]*).*\"%\1%")
    if [[ -z $distro ]]; then distro=$(lsb_release -a 2>/dev/null | grep Description | sed "s/Description:\s*//") ;fi
    if [[ -z $distro ]]; then distro=$(hostnamectl | grep "Operating System" | sed "s/Operating System:\s*//"); fi
    if [[ -z $distro ]]; then distro="unknown"; fi
    # rename distro to a lowercase general name (easier for package handling later)
    case "$distro" in
	Arch*) distro="archlinux";;
    	Debian*|Ubuntu*) distro="debian";;
    	Fedora*) distro="fedora";;
    	CentOS*) distro="centos";;
    	OpenSUSE*) distro="opensuse";;
	unknown|*) distro="unknown";;
    esac
    echo "$distro"
}

install_brew() {
    # adapted from https://brew.sh/
    curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh | /usr/bin/env bash
}

install_from_source() {
    echo "[ERROR] Could not identify OS/Distro."
    echo "Please follow the instructions here:"
    echo "https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#git"
    exit $EXIT_UNKNOWN_OS
}

propose_optional_dependencies() {
    local optionals="$@"
    if [[ $(( ${#optionals[@]} )) == 0 || "${optionals[@]}" =~ ^\ *$ ]]; then return; fi
    echo "[INFO] Toutui's experience could be improved by these optional packages:"
    for opt in "${optionals[@]}"; do
        echo -e "\t- ${opt}"
    done
    local answer=
    while :; do
        read -p "Would you like to install these packages? (y/N) : " answer
        if [[ $answer == "" || $answer =~ (n|N) ]]; then answer=no; break; fi
        if [[ $answer =~ (y|Y) ]]; then answer=yes; break; fi
    done
    case $answer in
        no)
	   echo "[INFO] Ignoring optional dependencies.";;
        yes)
	   echo "[INFO] Installing optional dependencies."
    	   install_packages "${optionals[@]}"
	   echo "[OK] Optional dependencies installed."
	   ;;
    esac
}

install_rust() {
    if ! command -v rustc >/dev/null 2>&1; then
	echo "[INFO] Cannot find \"rustc\" in your \$PATH. Installing rust..."
    	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source_cargo_env
    else
	echo "[OK] \"rustc\" exists."
    fi
}

source_cargo_env() {
    if [[ $SHELL =~ \/(sh|bash|zsh|ash|pdksh) ]]; then
	if [[ -z "${CARGO_HOME}" ]]; then
	    source "$HOME/.cargo/env"
	else
	    source "${CARGO_HOME}/env"
	fi
    elif [[ $SHELL =~ \/fish ]]; then
	if [[ -z "${CARGO_HOME}" ]]; then
	    source "$HOME/.cargo/env.fish"
	else
	    source "${CARGO_HOME}/env.fish"
	fi
    elif [[ $SHELL =~ \/nushell ]]; then
	if [[ -z "${CARGO_HOME}" ]]; then
	    source "$HOME/.cargo/env.nu"
	else
	    source "${CARGO_HOME}/env.nu"
	fi
    else
	echo "[ERROR] Cannot source cargo environment automatically."
	echo "Open a new terminal and launch \"hello_toutui.sh\" again."
	exit $EXIT_NO_CARGO_PATH
    fi
}

install_packages() {
    local dep="$@"
    if (( ${#dep} == 0 )); then return; fi
    case $OS in
	linux)
	    DISTRO=${DISTRO:-$(get_distro)}
    	    case "$DISTRO" in
    	        arch*) sudo pacman -S ${dep[@]};;
    	        debian*) sudo apt install -y ${dep[@]};;
    	        fedora*) sudo dnf install -y ${dep[@]};;
    	        centos*) sudo yum install -y ${dep[@]};;
    	        opensuse*) sudo zypper install -y ${dep[@]};;
    	        *) install_from_source;;
    	    esac ;;
	macOS)
	    if command -v brew >/dev/null 2>&1; then
		brew install ${dep[@]}
	    else
		install_brew
	    fi;;
    esac
    echo "[INFO] Packages installed successfully."
}

post_install_msg() {
    if ! [[ -f "$CONFIG_DIR/.env" ]]; then
	echo "[INFO] No secret found in .env. Do this:"
    	echo "	$ mkdir -p ~/.config/toutui"
    	echo "	$ echo 'TOUTUI_SECRET_KEY=secret' > ~/.config/toutui/.env"
    fi
}

install_config() {
    mkdir -p "$CONFIG_DIR" 2>/dev/null || ( echo "[ERROR] Cannot create config directory \"${CONFIG_DIR}\""; exit $EXIT_CONFIG )

    # .env
    local env="${CONFIG_DIR}/.env"
    local prompt="Please provide a secret key to encrypt the token stored in the database ($env): "
    local key=
    until [[ -f "$env" && $(sed "s/TOUTUI_SECRET_KEY=//g" "$env") != "" ]]; do
	read -sp "$prompt: " key
        if ! [[ $key == "" ]]; then echo "TOUTUI_SECRET_KEY=$key" > "$env"; echo;fi
    done

    # config.
    if ! [[ -f config.example.toml ]]; then
	echo "[ERROR] \"config.example.toml\" not found."
	exit $EXIT_CONFIG
    else
	cp config.example.toml "${CONFIG_DIR}/config.toml" || (echo "[ERROR] Cannot copy \"config.toml\"."; exit $EXIT_CONFIG)
    fi
}

dep_already_installed() {
    local pkg_name=$1
    local cmd_check=${2:-$pkg_name}
    local installed="false"
    if [[ $OS == "linux" ]]; then
	case "$DISTRO" in
    	    arch*)     (pacman -Qq $pkg_name >/dev/null)2>/dev/null && installed="true";;
    	    debian*)   (dpkg -l | awk '{print $2}' | grep "^${pkg_name}$" >/dev/null)2>/dev/null && installed="true";;
	    fedora*)   (rpm -q "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
	    centos*)   (yum list installed "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
    	    opensuse*) (zypper se --installed-only "$pkg_name" &>/dev/null)2>/dev/null && installed="true";;
    	esac
    elif [[ $OS == "macOS" ]]; then
	if command -v brew >/dev/null 2>&1; then
	    if brew list "${pkg_name}" >/dev/null 2>&1; then
		installed="true"
	    fi
	else
	    install_brew
	fi
    fi
    if [[ $installed == "false" ]]; then
	if [[ $cmd_check != "no_check" && $(command -v $cmd_check >/dev/null 2>&1) ]]; then
	    installed="true"
	fi
    fi
    echo $installed
}

install_deps() {
    # Grab dependencies and optional dependencies
    # Optional deps start with "*" (e.g. *cvlc).
    local deps=()
    local optionals=()
    if [[ -f deps.txt ]]; then
	while read -r line; do
	    if [[ $line == "" || $line =~ ^\# ]]; then continue; fi
	    deps+=( "$line" )
	done < deps.txt
    else
	deps=("${HC_DEPS[@]}")
    fi

    # Ignore already installed deps
    # Keep track of optional deps
    local missing=()
    for dep in "${deps[@]}"; do
	if [[ $dep =~ ^\* ]]; then
	    # this is an optional dependency
	    deps=("${deps[@]/$dep}") # remove optional from deps
	    dep="${dep:1:${#dep}}" # trim
	    local optional="true"
	elif [[ $dep =~ ^% ]];then
	    # this is a forced dependency
	    deps=("${deps[@]/$dep}") # remove from deps
	    dep="${dep:1:${#dep}}" # trim
	    deps+=( "$dep" ) # add it back
	    local optional="false"
	else
	    local optional="false"
	fi
	# Check if package is for OS || distro
	# linux:XXX means for all distro
	# debian:XX means specific to debian/ubuntu
	if [[ "$dep" =~ ^($OS):([^:]*)(:(.*))? || "$dep" =~ ^($DISTRO):([^:]*)(:(.*))? ]]; then
	    target_sys=${BASH_REMATCH[1]}
	    dep=${BASH_REMATCH[2]}
	    cmd=${BASH_REMATCH[4]}
	    # if OS or DISTRO match, add to optional deps
	    if [[ $target_sys == $OS || $target_sys == $DISTRO ]]; then
	        # add only if not installed
		if [[ $optional == "true" ]]; then
		    if [[ $(dep_already_installed "$dep" "$cmd") == "false" ]]; then
			optionals+=( $dep )
		    fi
		else
		    if [[ $(dep_already_installed "$dep" "$cmd") == "false" ]]; then
			echo "[DEP] Missing dependency \"$dep\""
			missing+=( $dep )
		    fi
		fi
	    fi
	fi
    done
    install_packages "${missing[@]}" && echo "[INFO] Essential dependencies are installed."
    propose_optional_dependencies "${optionals[@]}"
}

install_toutui() {
    install_deps # install essential and/or optional deps
    install_config # create ~/.config/toutui/ etc.
    install_rust # cornerstone! toutui is written by a crab
    cargo build --release
    # copy Toutui binary to system path
    sudo cp ./target/release/Toutui "${INSTALL_DIR}/toutui" || exit $EXIT_BUILD_FAIL
    echo "[DONE] Install complete. Type \"toutui\" in your terminal to run it."
    echo "[ADVICE] Best experience with Kitty or Alacritty terminal."
    post_install_msg # only if .env not found
}

post_update_msg() {
    echo "[DONE] Update complete."
}

get_toutui_local_release() {
    if ! [[ -f Cargo.toml ]]; then
	echo "[ERROR] Cannot find \"Cargo.toml\"."
	exit $EXIT_NO_CARGO_TOML
    fi
    grep "version" Cargo.toml | head -1 | sed -E "s/^version\s*=\s*\"([^\"]*)\"\s*$/\1/"
}

get_toutui_github_release() {
    curl -s https://api.github.com/repos/AlbanDAVID/Toutui/releases/latest | grep tag_name | sed -E "s|.*\"v([^\"]*)\",|\1|"
}

display_changelog() {
    local changelog=$(curl -s https://api.github.com/repos/AlbanDAVID/Toutui/releases/latest | grep "\"body\"" | sed -E "s|^\s*\"body\":\s*\"([^\"]*)\"|\1|")
    echo -e "\x1b[2m### CHANGELOG ###\x1b[0m"
    echo -e "\x1b[2m$changelog\x1b[0m"
    echo -e "\x1b[2m#################\x1b[0m"
}

pull_latest_version() {
    local version=$1
    local answer=
    while :; do
	read -p "Would you like to pull the latest version? (Y/n) : " answer
	if [[ $answer =~ (n|N) ]]; then answer=no; break; fi
	if [[ $answer == "" || $answer =~ (y|Y) ]]; then answer=yes; break; fi
    done
    case $answer in
	no)
	    echo "[INFO] Ignoring latest version.";;
	yes)
	    echo "[INFO] Pulling latest version..."
	    git fetch && git pull
	    echo "[INFO] Installing latest version..."
	    cargo build --release
	    if [[ -f ./target/release/Toutui ]]; then
    	        sudo cp ./target/release/Toutui "${INSTALL_DIR}/toutui" || exit $EXIT_BUILD_FAIL
    	    fi
	    echo "[OK] Latest version installed (v$version)."
	    ;;
    esac
}

update_toutui() {
    install_deps # check for new deps
    local local_release=$(get_toutui_local_release)
    local github_release=$(get_toutui_github_release)
    if [[ $local_release == $github_release ]]; then
	echo "[INFO] Up to date (version $local_release)."
    else
	#echo "TODO: check if is behind or ahead?"
	display_changelog # display before pulling?
	pull_latest_version $github_release
    fi
    post_update_msg
}

load_exit_codes() {
    # Exit codes for convenience?
    EXIT_OK=0
    EXIT_FAIL=1
    EXIT_ROOT=2
    EXIT_UNKNOWN_OS=3
    EXIT_INCORRECT_ARG=4
    EXIT_NO_CARGO_TOML=5
    EXIT_NO_CARGO_PATH=6
    EXIT_CONFIG=7
    EXIT_BUILD_FAIL=8
    EXIT_INSTALL_DIR=9
}

do_not_run_as_root() {
    # Must not be run as root
    if [[ $EUID == 0 ]]; then
        echo "[ERROR] Do not run this script as root."
        exit $EXIT_ROOT
    fi
}

main "$@"

# TODO:
# - check for correct installation path (for now: /usr/bin/toutui)
# - test automatic dependencies install on more distributions
# - uninstall toutui
