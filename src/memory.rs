
/*
TODO: Here we should handle rom-bank switching. When we specify an address between [0x4000, 0x8000)
      we need to return the data in the appropriate ROM bank. How do we know which one that is?
      I'm not sure yet. :)
      We should load the entire file to memory (somewhere) and keep the ROM bank ID saved, then
      here we can read the file buffer at the appropriate offset which we calculate from the ID.
*/
struct Memory {
    mem: Vec<u8>,
    size: usize
}
