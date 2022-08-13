#!/bin/bash
set -eux

SCRIPT_DIR=$(cd $(dirname $0); pwd)

VERSION_YEAR=2020
VERSION_UPDATE=1
VERSION_REV=217
PREFIX=l_mkl_${VERSION_YEAR}.${VERSION_UPDATE}.${VERSION_REV}

curl -sfLO http://registrationcenter-download.intel.com/akdlm/irc_nas/tec/16533/${PREFIX}.tgz
tar xf ${PREFIX}.tgz
cd ${PREFIX}
./install.sh -s ${SCRIPT_DIR}/silent.cfg
