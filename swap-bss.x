/* Force .bss to be emitted as real bytes in the image. See build.rs for why.
 *
 * lld decides an output section is NOBITS when every input section it collects
 * is NOBITS -- a LONG(0) byte command is not enough to flip it. So the anchor
 * below (a PROGBITS input section defined in src/swapfix.rs) has to come first,
 * which makes the merged .bss PROGBITS and gets the zeros written to disk.
 */
SECTIONS
{
  .bss :
  {
    KEEP(*(.swapfix_anchor))
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    *(.gnu.linkonce.b.*)
    *(COMMON)
  }
} INSERT AFTER .data
