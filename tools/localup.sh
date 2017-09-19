#!/bin/bash

# Copyright 2017 The RioCorp Inc Authors.
#

RIOOS_ROOT=$(dirname "${BASH_SOURCE}")/..

# This command runs a local rioos cluster.

# owner of client certs, default to current user if not specified
USER=${USER:-$(whoami)}

HOSTNAME_OVERRIDE=${HOSTNAME_OVERRIDE:-"127.0.0.1"}

# WARNING: For DNS to work on most setups you should export API_HOST as the lxc ip address,
API_HOST=${API_HOST:-localhost}
API_HOST_IP=${API_HOST_IP:-"127.0.0.1"}
FIRST_SERVICE_CLUSTER_IP=${FIRST_SERVICE_CLUSTER_IP:-10.0.0.1}


# Stop right away if the build fails
set -e

source "${RIOOS_ROOT}/tools/lib/init.sh"

function usage {
            echo "Rio/OS PKI Infrastructure (certificate authority)."
            echo " ./tools/localup.sh -h  (this 'help' usage description)"
            echo " ./tools/localup.sh (build a PKI infrastructure for Rio/OS)"
}

### Allow user to supply the source directory.
while getopts "h" OPTION
do
    case $OPTION in
        h)
            usage
            exit
            ;;
        ?)
            usage
            exit
            ;;
    esac
done


# Shut down anyway if there's an error.
set +e

# This is the default dir and filename where the apiserver will generate a self-signed cert
# which should be able to be used as the CA to verify itself
CERT_DIR=${RIOOS_HOME:-"/var/lib/rioos"}/config
ROOT_CA_FILE=${CERT_DIR}/server-ca.crt
ROOT_CA_KEY=${CERT_DIR}/server-ca.key
SERVICE_ACCOUNT_KEY=${CERT_DIR}/serviceaccount.key
CLUSTER_SIGNING_CERT_FILE=${CLUSTER_SIGNING_CERT_FILE:-"${ROOT_CA_FILE}"}
CLUSTER_SIGNING_KEY_FILE=${CLUSTER_SIGNING_KEY_FILE:-"${ROOT_CA_KEY}"}


# Ensure CERT_DIR is created for auto-generated crt/key and kubeconfig
mkdir -p "${CERT_DIR}" &>/dev/null || sudo mkdir -p "${CERT_DIR}"
CONTROLPLANE_SUDO=$(test -w "${CERT_DIR}" || echo "sudo -E")

function warning {
  message=$1

  echo $(tput bold)$(tput setaf 1)
  echo "INFO: ${message}"
  echo $(tput sgr0)
}

function set_service_accounts {
    SERVICE_ACCOUNT_KEY=${SERVICE_ACCOUNT_KEY:-/tmp/rio-serviceaccount.key}
    # Generate ServiceAccount key if needed
    if [[ ! -f "${SERVICE_ACCOUNT_KEY}" ]]; then
      mkdir -p "$(dirname ${SERVICE_ACCOUNT_KEY})"
      openssl genrsa -out "${SERVICE_ACCOUNT_KEY}" 2048 2>/dev/null
    fi
}

function start_pkica {
    # This is the default dir and filename where the apiserver will generate a self-signed cert
    # which should be able to be used as the CA to verify itself

    rioos::util::create_signing_certkey "${CONTROLPLANE_SUDO}" "${CERT_DIR}" server '"server auth"'
    #rioos::util::create_signing_certkey "${CONTROLPLANE_SUDO}" "${CERT_DIR}" client '"client auth"'

    # serving cert for rioos-apiserver
    rioos::util::create_serving_certkey "${CONTROLPLANE_SUDO}" "${CERT_DIR}" "server-ca" rioos-apiserver rioos.default rioos.default.svc "localhost" ${API_HOST_IP} ${API_HOST} ${FIRST_SERVICE_CLUSTER_IP}

    # We don't use client-ca.
		# Create client certs signed with client-ca, given id, given CN and a number of groups
		# TO-DO: will remove after test the full identity
    #rioos::util::create_client_certkey "${CONTROLPLANE_SUDO}" "${CERT_DIR}" 'client-ca' controller system:kube-controller-manager
}

function print_success {
  echo "PKI Infrastructure setup SUCCESS."
  echo "Run 'systemctl start rioos-api-server' to start rioos cluster."
}

rioos::util::test_openssl_installed
rioos::util::ensure-cfssl


echo "Starting PKI now!"
set_service_accounts
start_pkica

print_success
