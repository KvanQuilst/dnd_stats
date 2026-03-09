OOCD := openocd
OCDFLAGS=-f interface/stlink-v2.cfg -f target/lpc11xx.cfg

BIN_PATH := ./target/thumbv6m-none-eabi/debug
BIN := dnd-stats

PROGRAM := $(BIN_PATH)/$(BIN)

LPC_CHECKSUM := ../lpc1114-rt/lpc_checksum.py

default:

program: fix
	@${OOCD} ${OCDFLAGS} -c "program ${PROGRAM} verify reset exit"

debug:
	@${OOCD} ${OCDFLAGS} -c "gdb_port 3333; gdb_memory_map disable"

fix:
	@${LPC_CHECKSUM} ${PROGRAM}
