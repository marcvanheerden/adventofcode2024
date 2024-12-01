OPENOCD_FLAGS = -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" -c "program target/thumbv6m-none-eabi/release/rp2040-project-template"

debug:
	cargo b -r 
	sudo openocd $(OPENOCD_FLAGS)

connect:
	gdb-multiarch -x commands.gdb

