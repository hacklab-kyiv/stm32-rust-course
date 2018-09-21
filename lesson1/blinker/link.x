MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x40000   /* 256k */
  RAM (rwx)  : ORIGIN = 0x10000000, LENGTH = 0x08000   /*  32k */
}

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM)); /* initial SP value */
    KEEP(*(.reset_handler));

    /* Omitted: The rest of the vector table */

    *(.text.*);
  } > FLASH

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
  }
}