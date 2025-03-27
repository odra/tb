SHELL := /bin/bash
TB_BIN_DIR := ${HOME}/.local/bin
TB_BASE_DIR := ${HOME}/.config/tb

install:
	mkdir -p ${TB_BIN_DIR}
	mkdir -p ${TB_BASE_DIR}
	install ./src/* ${TB_BIN_DIR}/
	echo 'export TB_BASE_DIR=${TB_BASE_DIR}' > ${HOME}/.bashrc.d/tb.bashrc
	echo 'export TB_BIN_DIR=${TB_BIN_DIR}' >> ${HOME}/.bashrc.d/tb.bashrc
	echo 'export PATH=$$TB_BIN_DIR:$$PATH' >> ${HOME}/.bashrc.d/tb.bashrc
