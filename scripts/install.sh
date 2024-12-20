#!/bin/bash
#
# Eigen CLI binary installation script, borrowed from Goose [https://github.com/metaplex-foundation/goose/blob/main/scripts/install.sh].
# ------------------------------------
#
#
# The purpose of this script is to automate the download and installation
# of the Eigen binary.
#
# The script does a (simple) platform detection, downloads the binary
# for the detected platform and copies it to a folder in the PATH variable.
#
# Currently the supported platforms are macOS, Linux, or another Unix-like OS
# running variations of the sh Unix shells.
#

RED() { echo $'\e[1;31m'$1$'\e[0m'; }
GRN() { echo $'\e[1;32m'$1$'\e[0m'; }
CYN() { echo $'\e[1;36m'$1$'\e[0m'; }

abort_on_error() {
    if [ ! $1 -eq 0 ]; then
        RED "Aborting: operation failed"
        exit 1
    fi
}

CYN  "Eigen CLI binary installation script"
echo "---------------------------------------"
echo ""

OS_FLAVOUR="$(uname -s)"
PROCESSOR="$(uname -m)"

# we need to check whether we are running on an ARM
# architecture or not

case "$PROCESSOR" in
    arm* | aarch* | ppc* )
        if [ "$OS_FLAVOUR" != Darwin ]; then
            echo "Binary for $PROCESSOR architecture is not currently supported. Please follow the instructions at:"
            echo "  => $(CYN https://github.com/tensor-foundation/eigen)"
            echo ""
            echo "to build Eigen from the source code."
            exit 1
        fi
        ;;

    *)
        # good to go
        ;;
esac

RELEASE_URL="https://github.com/tensor-foundation/eigen/releases"
RELEASE="latest"
BIN="eigen"
VERSION="linux-x86_64"

if [ "$OS_FLAVOUR" = Darwin ]; then
    case "$PROCESSOR" in
        arm* )
            VERSION="macos-arm64"
            ;;
        *)
            VERSION="macos-x86_64"
            ;;
    esac
fi

DIST="$VERSION"

printf "Do you want the latest release(y/n): "
read answer
if [ "$answer" != "${answer#[Yy]}" ] ;then
        echo "";
else
        printf "You can find all the releases here $RELEASE_URL"
        echo ""
        printf "Enter relase version (e.g, v0.8.7, v1.2.1): "
        read release
        RELEASE="$release"
fi

# creates a temporary directory to save the distribution file
SOURCE="$(mktemp -d)"

echo "$(CYN "1.") 🖥  $(CYN "Downloading distribution")"
echo ""

# downloads the distribution file
if [ $RELEASE != "latest" ] ;then
    URL="$RELEASE_URL/download/$RELEASE/$BIN-$DIST"
else
    URL="$RELEASE_URL/$RELEASE/download/$BIN-$DIST"
fi
echo "Remote URL: $URL"
echo ""
curl -f -L $URL --output "$SOURCE/$DIST"
abort_on_error $?

SIZE=$(wc -c "$SOURCE/$DIST" | grep -oE "[0-9]+" | head -n 1)

if [ $SIZE -eq 0 ]; then
    RED "Aborting: could not download Eigen distribution"
    exit 1
fi

# makes sure the binary will be executable
chmod u+x "$SOURCE/$DIST"
abort_on_error $?

echo ""
echo "$(CYN "2.") 📤 $(CYN "Moving binary into place")"
echo ""

if [ ! "$(command -v $BIN)" = "" ]; then
    # binary already found on system, ask if we should
    # replace it
    EXISTING="$(which $BIN)"

    echo "Eigen binary was found at:"
    echo "  => $(CYN $EXISTING)"
    echo ""
    echo -n "$(CYN "Replace it? [Y/n]") (default 'n'): "
    read REPLACE

    if [ -z "REPLACE" ]; then
        REPLACE="n"
    fi

    if [ "$REPLACE" != "${REPLACE#[Yy]}" ]; then
        echo ""
        echo "'$BIN' binary will be moved to '$(dirname "$EXISTING")'."

        mv "$SOURCE/$DIST" "$EXISTING"
        abort_on_error $?
    else
        # nothing else to do, replacement was cancelled
        RED "Aborting: replacement cancelled"
        exit 1
    fi
else
    # determines a suitable directory for the binary - preference:
    # 1) ~/.cargo/bin if exists
    # 2) ~/bin otherwise
    TARGET="$HOME/.cargo/bin"

    if [ ! -d "$TARGET" ]; then
        TARGET="$HOME/bin"

        if [ ! -d "$TARGET" ]; then
            mkdir $TARGET
        fi
    fi

    echo "'$BIN' binary will be moved to '$TARGET'."

    mv "$SOURCE/$DIST" "$TARGET/$BIN"
    abort_on_error $?

    if [ "$(command -v $BIN)" = "" ]; then
        ENV_FILE="$HOME/.$(basename $SHELL)rc"

        if [ -f "$ENV_FILE" ]; then
            echo "  => adding '$TARGET' to 'PATH' variable in '$ENV_FILE'"
            echo "export PATH=\"$HOME/bin:\$PATH\"" >> "$ENV_FILE"
        else
            echo "  => adding '$TARGET' to 'PATH' variable to execute 'eigen' from any directory."
            echo "     - file '$(CYN $ENV_FILE)' was not found"
            echo ""
            echo -n "$(CYN "Would you like to create '$ENV_FILE'? [Y/n]") (default 'n'): "
            read CREATE

            if [ -z "REPLACE" ]; then
                CREATE="n"
            fi

            if [ "$CREATE" != "${CREATE#[Yy]}" ]; then
                echo "  => adding '$TARGET' to 'PATH' variable in '$ENV_FILE'"
                echo "export PATH=\"$HOME/bin:\$PATH\"" >> "$ENV_FILE"
            else
                echo ""
                echo "     $(RED "[File creation cancelled]")"
                echo ""
                echo "     - to manually add '$TARGET' to 'PATH' you will need to:"
                echo ""
                echo "       1. create a file named '$(basename $ENV_FILE)' in your directory '$(dirname $ENV_FILE)'"
                echo "       2. add the following line to the file:"
                echo ""
                echo "           export PATH=\"$HOME/bin:\$PATH\""
            fi
        fi
    fi
fi

echo ""
# sanity check
if [ "$(command -v $BIN)" = "" ]; then
    # installation was completed, but Eigen is not in the PATH
    echo "✅ $(GRN "Installation complete:") restart your shell to update 'PATH' variable or type '$TARGET/$BIN' to start using it."
else
    # success
    echo "✅ $(GRN "Installation successful:") type '$BIN' to start using it."
fi
