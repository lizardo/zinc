/* Memory layout for nRF51822 (QFAA variant).
 *
 * Source: nRF51822-PS version v1.3
 *
 * FIXME: for better performance, we might want to place Radio, Crypto and CPU
 * memory in separate 4K RAM sections as recommended by nRF51822-PS v3.1,
 * section 3.2.2. */

_data_load = LOADADDR(.data);

ENTRY(main)

MEMORY
{
    rom(RX)   : ORIGIN = 0x00000000, LENGTH = 256K
    ram(WAIL) : ORIGIN = 0x20000000, LENGTH = 16K
}

__STACK_BASE  = ORIGIN(ram) + LENGTH(ram);

REGION_ALIAS("vectors", rom);

INCLUDE layout_common.ld
