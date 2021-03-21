#!/bin/sh

# The present file is a command line utility for installing, updating and remove lsfp.
# It is provided with the same license as the project, a MIT license.

print_help()
{
  echo This is a help message
}

install()
{
  if [ ! -z "`which curl`" ]; then
    echo "Proceeding to download binary with curl"
  elif [ ! -z "`which wget`" ]; then
    echo "Proceeding to download binary with wget"
  else
    echo "Neither curl nor wget were found in the device, please install at least one of them"
  fi
  # Allow to choose lite or complete, give possibility of creating alias for ls
}

update()
{
  if [ -z "${LSFP_VERSION}" ]; then
    echo "It seems like lsfp has never been installed here, please run 'lsfp-installer install'"
    exit 1
  fi
  version=${LSFP_VERSION}
  echo "To update from ${version} to latest"
}

features()
{
  # features=$LSFP_FEATURES(=10 or 11 or 01 or 00)
  echo "Get ready to change the features of your lsfp installation"
}

remove()
{
  echo "Unsetting environment variables and removing binary from `basename $0`"
}

if [ -z "$1" ]; then
  print_help
else
  case "$1" in
    "install")
      install
      ;;
    "update")
      update
      ;;
    "features")
      features
      ;;
    "remove")
      remove
    ;;
    *)
      echo "The command you typed does not exist" && echo
      if [ "$1" = "instal" -o "$1" = "installl" -o "$1" = "istall" -o "$1" = "imstall" -o "$1" = "i" ]; then
        echo "Did you mean 'install'?"
      fi
  esac
fi
