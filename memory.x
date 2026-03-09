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
    /*
     * The Raspberry Pi Pico's official C SDK defaults to using 2 KiB from two smaller (4 KiB)
     * memory banks for each core's stack (`SCRATCH_X` and `SCRATCH_Y`, respectively), with the
     * other 256 KiB (from four 64 KiB banks) going towards the heap used by `malloc` (`RAM`).
     *
     * Embassy is heapless, so the entire 264 KiB of all six memory banks is used for the primary
     * core's stack. Inspecting `embassy_rp::multicore::spawn_core1`, the stack of the secondary
     * core is not some special memory location (e.g., `SCRATCH_Y` in the official C SDK), but just
     * a byte array that's passed in as an argument.
     *
     * See:
     *
     * - <https://petewarden.com/2024/01/16/understanding-the-raspberry-pi-picos-memory-layout/>
     * - <https://github.com/raspberrypi/pico-sdk/blob/0c65e1d/src/rp2_common/pico_standard_link/memmap_default.ld>
     * - <https://github.com/embassy-rs/embassy/blob/e5651b8/embassy-rp/src/multicore.rs#L160-L288>
     */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}
