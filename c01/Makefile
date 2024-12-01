BUILD_DIR = build 
CMAKE_FLAGS = -DCMAKE_BUILD_TYPE=Debug -DPICO_BOARD=pico
MAKE_FLAGS = -j4
OPENOCD_FLAGS = -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" -c "program build/blink.elf"
ENV_VARS = export PICO_SDK_PATH=/home/marc/Desktop/pico-sdk

all: rebuild

rebuild: clean configure compile 

clean:
	rm -rf $(BUILD_DIR)

configure:
	cinputs
	mkdir -p $(BUILD_DIR)
	cd $(BUILD_DIR) && $(ENV_VARS) && cmake $(CMAKE_FLAGS) .. 

compile:
	cd $(BUILD_DIR) && make $(MAKE_FLAGS)

debug:
	sudo openocd $(OPENOCD_FLAGS)

connect:
	cd $(BUILD_DIR) && gdb-multiarch -x ../commands.gdb

