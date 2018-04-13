#!/bin/bash

# Copyright 2017 The RioCorp Inc Authors.

rioos::util::sortable_date() {
  date "+%Y%m%d-%H%M%S"
}

# Test whether openssl is installed.
# Sets:
#  OPENSSL_BIN: The path to the openssl binary to use
function rioos::util::test_openssl_installed {
    openssl version >& /dev/null
    if [ "$?" != "0" ]; then
      echo "Failed to run openssl. Please ensure openssl is installed"
      exit 1
    fi
    OPENSSL_BIN=$(command -v openssl)
}

# creates a client CA, args are sudo, dest-dir, ca-id, purpose
# purpose is dropped in after "key encipherment", you usually want
# '"client auth"'
# '"server auth"'
# '"client auth","server auth"'
function rioos::util::create_signing_certkey {
    local sudo=$1
    local dest_dir=$2
    local id=$3
    local purpose=$4
    # Create client ca
    ${sudo} /bin/bash -e <<EOF
    rm -f "${dest_dir}/${id}-ca.crt" "${dest_dir}/${id}-ca.key"
    ${OPENSSL_BIN} req -x509 -sha256 -new -nodes -days 365 -newkey rsa:2048 -keyout "${dest_dir}/${id}-ca.key" -out "${dest_dir}/${id}-ca.crt" -subj "/C=xx/ST=x/L=x/O=x/OU=x/CN=ca/emailAddress=x/"
    echo '{"signing":{"default":{"expiry":"43800h","usages":["signing","key encipherment",${purpose}]}}}' > "${dest_dir}/${id}-ca-config.json"
EOF
}

# signs a client certificate: args are sudo, dest-dir, CA, filename (roughly), username, groups...
function rioos::util::create_client_certkey {
    local sudo=$1
    local dest_dir=$2
    local ca=$3
    local id=$4
    local cn=${5:-$4}
    local groups=""
    local SEP=""
    shift 5
    while [ -n "${1:-}" ]; do
        groups+="${SEP}{\"O\":\"$1\"}"
        SEP=","
        shift 1
    done
    ${sudo} /bin/bash -e <<EOF
    cd ${dest_dir}
    echo '{"CN":"${cn}","names":[${groups}],"hosts":[""],"key":{"algo":"rsa","size":2048}}' | ${CFSSL_BIN} gencert -ca=${ca}.crt -ca-key=${ca}.key -config=${ca}-config.json - | ${CFSSLJSON_BIN} -bare client-${id}
    mv "client-${id}-key.pem" "client-${id}.key"
    mv "client-${id}.pem" "client-${id}.crt"
    rm -f "client-${id}.csr"
EOF
}

# creates a self-contained rioconfig: args are sudo, dest-dir, ca file, host, port, client id, token(optional)
function rioos::util::write_client_rioconfig {
    local sudo=$1
    local dest_dir=$2
    local ca_file=$3
    local api_host=$4
    local api_port=$5
    local client_id=$6
    local token=${7:-}
    cat <<EOF | ${sudo} tee "${dest_dir}"/${client_id}.rioconfig > /dev/null
apiVersion: v1
kind: Config
clusters:
  cluster:
    certificate-authority: ${ca_file}
    server: https://${api_host}:${api_port}/
    name: local-up-cluster
users:
  user:
    token: ${token}
    client-certificate: ${dest_dir}/client-${client_id}.crt
    client-key: ${dest_dir}/client-${client_id}.key
    name: local-up-cluster
contexts:
  context:
    cluster: local-up-cluster
    user: local-up-cluster
    name: local-up-cluster
current-context: local-up-cluster
EOF

    # flatten the rioconfig files to make them self contained
    username=$(whoami)
    ${sudo} /bin/bash -e <<EOF 
    chown ${username} "${dest_dir}/${client_id}.rioconfig"
EOF
}

# signs a serving certificate: args are sudo, dest-dir, ca, filename (roughly), subject, hosts...
function rioos::util::create_serving_certkey {
    local sudo=$1
    local dest_dir=$2
    local ca=$3
    local id=$4
    local cn=${5:-$4}
    local hosts=""
    local SEP=""
    shift 5
    while [ -n "${1:-}" ]; do
        hosts+="${SEP}\"$1\""
        SEP=","
        shift 1
    done
    ${sudo} /bin/bash -e <<EOF
    cd ${dest_dir}
    echo '{"CN":"${cn}","hosts":[${hosts}],"key":{"algo":"rsa","size":2048}}' | ${CFSSL_BIN} gencert -ca=${ca}.crt -ca-key=${ca}.key -config=${ca}-config.json - | ${CFSSLJSON_BIN} -bare serving-${id}
    mv "serving-${id}-key.pem" "serving-${id}.key"
    mv "serving-${id}.pem" "serving-${id}.crt"
    rm -f "serving-${id}.csr"
    openssl pkcs12 -export -inkey "serving-${id}.key"  -in "serving-${id}.crt" -name "serving-${id}_name" -out "serving-${id}.pfx" -password pass:RIO123

EOF
}

# Opposite of rioos::util::ensure-temp-dir()
rioos::util::cleanup-temp-dir() {
  rm -rf "${RIOOS_TEMP}"
}

rioos::util::trap_add() {
  local trap_add_cmd
  trap_add_cmd=$1
  shift

  for trap_add_name in "$@"; do
    local existing_cmd
    local new_cmd

    # Grab the currently defined trap commands for this trap
    existing_cmd=`trap -p "${trap_add_name}" |  awk -F"'" '{print $2}'`

    if [[ -z "${existing_cmd}" ]]; then
      new_cmd="${trap_add_cmd}"
    else
      new_cmd="${existing_cmd};${trap_add_cmd}"
    fi

    # Assign the test
    trap "${new_cmd}" "${trap_add_name}"
  done
}


# Create a temp dir that'll be deleted at the end of this bash session.
#
# Vars set:
#   KUBE_TEMP
rioos::util::ensure-temp-dir() {
  if [[ -z ${RIOOS_TEMP-} ]]; then
    RIOOS_TEMP=$(mktemp -d 2>/dev/null || mktemp -d -t rioos.XXXXXX)
    rioos::util::trap_add rioos::util::cleanup-temp-dir EXIT
  fi
}


# Downloads cfssl/cfssljson into $1 directory if they do not already exist in PATH
#
# Assumed vars:
#   $1 (cfssl directory) (optional)
#
# Sets:
#  CFSSL_BIN: The path of the installed cfssl binary
#  CFSSLJSON_BIN: The path of the installed cfssljson binary
#
function rioos::util::ensure-cfssl {
  if command -v cfssl &>/dev/null && command -v cfssljson &>/dev/null; then
    CFSSL_BIN=$(command -v cfssl)
    CFSSLJSON_BIN=$(command -v cfssljson)
    return 0
  fi

  # Create a temp dir for cfssl if no directory was given
  local cfssldir=${1:-}
  if [[ -z "${cfssldir}" ]]; then
    rioos::util::ensure-temp-dir
    cfssldir="${RIOOS_TEMP}/cfssl"
  fi

  mkdir -p "${cfssldir}"
  pushd "${cfssldir}" > /dev/null

    echo "Unable to successfully run 'cfssl' from $PATH; downloading instead..."
    kernel=$(uname -s)
    case "${kernel}" in
      Linux)
        curl -s -L -o cfssl https://pkg.cfssl.org/R1.2/cfssl_linux-amd64
        curl -s -L -o cfssljson https://pkg.cfssl.org/R1.2/cfssljson_linux-amd64
        ;;
      Darwin)
        curl -s -L -o cfssl https://pkg.cfssl.org/R1.2/cfssl_darwin-amd64
        curl -s -L -o cfssljson https://pkg.cfssl.org/R1.2/cfssljson_darwin-amd64
        ;;
      *)
        echo "Unknown, unsupported platform: ${kernel}." >&2
        echo "Supported platforms: Linux, Darwin." >&2
        exit 2
    esac

    chmod +x cfssl || true
    chmod +x cfssljson || true

    CFSSL_BIN="${cfssldir}/cfssl"
    CFSSLJSON_BIN="${cfssldir}/cfssljson"
    if [[ ! -x ${CFSSL_BIN} || ! -x ${CFSSLJSON_BIN} ]]; then
      echo "Failed to download 'cfssl'. Please install cfssl and cfssljson and verify they are in \$PATH."
      echo "Hint: export PATH=\$PATH:\$GOPATH/bin; go get -u github.com/cloudflare/cfssl/cmd/..."
      exit 1
    fi
  popd > /dev/null
}

# Some useful colors.
if [[ -z "${color_start-}" ]]; then
  declare -r color_start="\033["
  declare -r color_red="${color_start}0;31m"
  declare -r color_yellow="${color_start}0;33m"
  declare -r color_green="${color_start}0;32m"
  declare -r color_norm="${color_start}0m"
fi

# ex: ts=2 sw=2 et filetype=sh
