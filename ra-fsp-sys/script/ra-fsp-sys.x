INCLUDE memory.x
INCLUDE device.x

PROVIDE(DefaultHandler = Default_Handler);

PROVIDE(NonMaskableInt = DefaultHandler);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

INCLUDE fsp_base.ld

PROVIDE(
  __ebss = DEFINED(__ram_zero$$Limit)
    ? __ram_zero$$Limit
    : DEFINED(__bss_end__) ? __bss_end__ : 0
);

PROVIDE(
  _stack_start = DEFINED(.bss.g_main_stack)
    ? ADDR(.bss.g_main_stack) + SIZEOF(.bss.g_main_stack)
    : DEFINED(__stack) ? __stack : 0
);

ASSERT(_stack_start != 0, "Cannot determine stack start address");
ASSERT(__ebss != 0, "Cannot determine the end of the BSS section");
