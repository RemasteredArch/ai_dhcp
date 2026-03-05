/*
 * From <https://github.com/embassy-rs/embassy/blob/5c1ca25/examples/rp/memory.x>. This is probably
 * not reasonably copyrightable, but the original file appears to be copyright Dario Nieuwenhuis and
 * James Munns, licensed under the
 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the
 * [MIT License](https://opensource.org/license/MIT).
 */
MEMORY
{
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}
