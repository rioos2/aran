TOOLS_PATH=./tools
CONFIG_PATH=$(TOOLS_PATH)/config
CONFIG_TEMPLATE_PATH=$(CONFIG_PATH)/template
LICENSE_SO_PATH=$(TOOLS_PATH)/license


UNAME_S := $(shell uname -s)
OPENSSL := $(shell openssl version | grep 1.1)
run :=
bldr_run :=

define OPENSSL_ERROR
FATAL: you must have openssl 1.1x.
endef

define NOT_SET_RIOOS_HOME
FATAL: you must set RIOOS_HOME, 'mkdir ~/code/rioos/home; export RIOOS_HOME=~/code/rioos/home'.
	   Export the variable in ~/.bashrc file
endef

ifndef RIOOS_HOME
 $(error $(NOT_SET_RIOOS_HOME))
endif

ifeq ($(UNAME_S),Darwin)
	forego := support/mac/bin/forego
else
	forego := support/linux/bin/forego
endif

BIN = rioos
LIB = builder-db builder-apimachinery  builder-deployment builder-scaling common core builder-api-client http-client
API = builder-api
AUD = builder-api-audit
APS = builder-api-appstore

ALL = $(BIN) $(LIB) $(API) $(AUD) $(APS)
VERSION := $(shell cat VERSION)

.DEFAULT_GOAL := buildbin

setup:
	@echo "» $(RIOOS_HOME)"
ifeq ("$(wildcard $(RIOOS_HOME)/config/pullcache)","")
	mkdir -p $(RIOOS_HOME)/config/pullcache  > /dev/null
endif
	@echo "✔ mkdir $(RIOOS_HOME)/config/pullcache"
ifeq ("$(wildcard $(RIOOS_HOME)/config/template)","")
	mkdir -p $(RIOOS_HOME)/config/template > /dev/null
	cp -R -f $(CONFIG_TEMPLATE_PATH)/* $(RIOOS_HOME)/config/template  > /dev/null
endif
	@echo "✔ mkdir $(RIOOS_HOME)/config/template && cp -R"
ifeq ("$(wildcard $(RIOOS_HOME)/license)","")
	mkdir -p $(RIOOS_HOME)/license > /dev/null
	cp -R -f $(LICENSE_SO_PATH) $(RIOOS_HOME) > /dev/null
endif
	@echo "✔ mkdir $(RIOOS_HOME)/license && cp -R"
	@echo "« $(RIOOS_HOME)"

initialize: setup

build: initialize buildbin buildlib buildapi buildaud buildapp ## builds all the components
buildall: build
.PHONY: build buildall

buildbin: $(addprefix build-,$(BIN)) ## builds the binary components
.PHONY: builbin

buildlib: $(addprefix build-,$(LIB)) ## builds the library components
.PHONY: buildlib

buildapi: $(addprefix build-,$(API)) ## builds the API components
.PHONY: buildapi

buildaud: $(addprefix build-,$(AUD)) ## builds the audit components
.PHONY: buildaud

buildapp: $(addprefix build-,$(APS)) ## builds the marketplace components
.PHONY: buildapp

unit: unit-bin unit-lib unit-srv ## executes all the components' unit test suites
unit-all: unit
.PHONY: unit unit-all

unit-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit test suites
.PHONY: unit-bin


unit-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit test suites
.PHONY: unit-lib

unit-srv: $(addprefix unit-,$(SRV)) ## executes the service components' unit test suites
.PHONY: unit-srv

lint: lint-bin lint-lib lint-srv ## executs all components' lints
lint-all: lint
.PHONY: lint lint-all

lint-bin: $(addprefix lint-,$(BIN))
.PHONY: lint-bin

lint-lib: $(addprefix lint-,$(LIB))
.PHONY: lint-lib

lint-srv: $(addprefix lint-,$(SRV))
.PHONY: lint-srv

functional: functional-bin functional-lib functional-srv ## executes all the components' functional test suites
functional-all: functional
test: functional ## executes all components' test suites
.PHONY: functional functional-all test

functional-bin: $(addprefix unit-,$(BIN)) ## executes the binary components' unit functional suites
.PHONY: functional-bin

functional-lib: $(addprefix unit-,$(LIB)) ## executes the library components' unit functional suites
.PHONY: functional-lib

functional-srv: $(addprefix unit-,$(SRV)) ## executes the service components' unit functional suites
.PHONY: functional-srv

clean: clean-bin clean-lib clean-srv ## cleans all the components' clean test suites
clean-all: clean
.PHONY: clean clean-all

clean-bin: $(addprefix clean-,$(BIN)) ## cleans the binary components' project trees
.PHONY: clean-bin

clean-lib: $(addprefix clean-,$(LIB)) ## cleans the library components' project trees
.PHONY: clean-lib

clean-srv: $(addprefix clean-,$(SRV)) ## cleans the service components' project trees
.PHONY: clean-srv

fmt: fmt-bin fmt-lib fmt-srv ## formats all the components' codebases
fmt-all: fmt
.PHONY: fmt fmt-all

fmt-bin: $(addprefix fmt-,$(BIN)) ## formats the binary components' codebases
.PHONY: clean-bin

fmt-lib: $(addprefix fmt-,$(LIB)) ## formats the library components' codebases
.PHONY: clean-lib

fmt-srv: $(addprefix fmt-,$(SRV)) ## formats the service components' codebases
.PHONY: clean-srv

help:
	@perl -nle'print $& if m{^[a-zA-Z_-]+:.*?## .*$$}' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
.PHONY: help

bldr-run: build-srv ## launches a development shell running the API
	$(bldr_run) sh -c '$(forego) start -f support/Procfile -e support/bldr.env'
.PHONY: bldr-run

image: ## no-op
.PHONY: image

distclean: clean ## fully cleans up project tree
.PHONY: distclean

tag-release:
	sh -c 'git tag $(VERSION)'

define BUILD
build-$1: initialize ## builds the $1 component
	$(run) sh -c 'cd components/$1 && cargo build $(FLAGS)'
.PHONY: build-$1

endef
$(foreach component,$(ALL),$(eval $(call BUILD,$(component))))

define UNIT
unit-$1: image ## executes the $1 component's unit test suite
	$(run) sh -c 'cd components/$1 && cargo test'
.PHONY: unit-$1

endef
$(foreach component,$(ALL),$(eval $(call UNIT,$(component))))

define FUNCTIONAL
functional-$1: image ## executes the $1 component's functional test suite
	$(run) sh -c 'cd components/$1 && cargo test --features functional'
.PHONY: functional-$1

endef
$(foreach component,$(ALL),$(eval $(call FUNCTIONAL,$(component))))

define CLEAN
clean-$1: image ## cleans the $1 component's project tree
	$(run) sh -c 'cd components/$1 && cargo clean'
.PHONY: clean-$1

endef
$(foreach component,$(ALL),$(eval $(call CLEAN,$(component))))

define FMT
fmt-$1: image ## formats the $1 component
	$(run) sh -c 'cd components/$1 && cargo fmt'
.PHONY: fmt-$1

endef
$(foreach component,$(ALL),$(eval $(call FMT,$(component))))
