extern crate gblite;

#[test]
fn register_set() {
    let rc = registers::RegisterCache::new();
    rc.set(Reg16::HL, 0x322);
    assert_eq!(rc.get(Reg16::HL), 0x322);
}
