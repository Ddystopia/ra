INCLUDE memory.x
INCLUDE device.x

PROVIDE(__ebss = __bss_end__);
PROVIDE(_stack_start = __stack);
PROVIDE(DefaultHandler = Default_Handler);
PROVIDE(SysTick = Default_Handler);

INCLUDE fsp_base.ld
