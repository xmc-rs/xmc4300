#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - Arbitration Configuration Register"]
    pub arbcfg: ARBCFG,
    #[doc = "0x84 - Arbitration Priority Register"]
    pub arbpr: ARBPR,
    #[doc = "0x88 - Channel Assignment Register"]
    pub chass: CHASS,
    _reserved3: [u8; 20usize],
    #[doc = "0xa0 - Input Class Register"]
    pub iclass: [ICLASS; 2],
    _reserved4: [u8; 8usize],
    #[doc = "0xb0 - Alias Register"]
    pub alias: ALIAS,
    _reserved5: [u8; 4usize],
    #[doc = "0xb8 - Boundary Select Register"]
    pub bound: BOUND,
    _reserved6: [u8; 4usize],
    #[doc = "0xc0 - Synchronization Control Register"]
    pub synctr: SYNCTR,
    _reserved7: [u8; 4usize],
    #[doc = "0xc8 - Boundary Flag Register"]
    pub bfl: BFL,
    #[doc = "0xcc - Boundary Flag Software Register"]
    pub bfls: BFLS,
    #[doc = "0xd0 - Boundary Flag Control Register"]
    pub bflc: BFLC,
    #[doc = "0xd4 - Boundary Flag Node Pointer Register"]
    pub bflnp: BFLNP,
    _reserved11: [u8; 40usize],
    #[doc = "0x100 - Queue 0 Source Control Register"]
    pub qctrl0: QCTRL0,
    #[doc = "0x104 - Queue 0 Mode Register"]
    pub qmr0: QMR0,
    #[doc = "0x108 - Queue 0 Status Register"]
    pub qsr0: QSR0,
    #[doc = "0x10c - Queue 0 Register 0"]
    pub q0r0: Q0R0,
    _reserved_15_qbur0: [u8; 4usize],
    _reserved16: [u8; 12usize],
    #[doc = "0x120 - Autoscan Source Control Register"]
    pub asctrl: ASCTRL,
    #[doc = "0x124 - Autoscan Source Mode Register"]
    pub asmr: ASMR,
    #[doc = "0x128 - Autoscan Source Channel Select Register"]
    pub assel: ASSEL,
    #[doc = "0x12c - Autoscan Source Pending Register"]
    pub aspnd: ASPND,
    _reserved20: [u8; 80usize],
    #[doc = "0x180 - Channel Event Flag Register"]
    pub ceflag: CEFLAG,
    #[doc = "0x184 - Result Event Flag Register"]
    pub reflag: REFLAG,
    #[doc = "0x188 - Source Event Flag Register"]
    pub seflag: SEFLAG,
    _reserved23: [u8; 4usize],
    #[doc = "0x190 - Channel Event Flag Clear Register"]
    pub cefclr: CEFCLR,
    #[doc = "0x194 - Result Event Flag Clear Register"]
    pub refclr: REFCLR,
    #[doc = "0x198 - Source Event Flag Clear Register"]
    pub sefclr: SEFCLR,
    _reserved26: [u8; 4usize],
    #[doc = "0x1a0 - Channel Event Node Pointer Register 0"]
    pub cevnp0: CEVNP0,
    _reserved27: [u8; 12usize],
    #[doc = "0x1b0 - Result Event Node Pointer Register 0"]
    pub revnp0: REVNP0,
    #[doc = "0x1b4 - Result Event Node Pointer Register 1"]
    pub revnp1: REVNP1,
    _reserved29: [u8; 8usize],
    #[doc = "0x1c0 - Source Event Node Pointer Register"]
    pub sevnp: SEVNP,
    _reserved30: [u8; 4usize],
    #[doc = "0x1c8 - Service Request Software Activation Trigger"]
    pub sract: SRACT,
    _reserved31: [u8; 36usize],
    #[doc = "0x1f0 - E0ternal Multiplexer Control Register"]
    pub emuxctr: EMUXCTR,
    _reserved32: [u8; 4usize],
    #[doc = "0x1f8 - Valid Flag Register"]
    pub vfr: VFR,
    _reserved33: [u8; 4usize],
    #[doc = "0x200 - Channel Ctrl. Reg."]
    pub chctr: [CHCTR; 8],
    _reserved34: [u8; 96usize],
    #[doc = "0x280 - Result Control Register"]
    pub rcr: [RCR; 16],
    _reserved35: [u8; 64usize],
    #[doc = "0x300 - Result Register"]
    pub res: [RES; 16],
    _reserved36: [u8; 64usize],
    #[doc = "0x380 - Result Register, Debug"]
    pub resd: [RESD; 16],
}
impl RegisterBlock {
    #[doc = "0x110 - Queue 0 Backup Register"]
    #[inline(always)]
    pub fn qbur0(&self) -> &QBUR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const QBUR0) }
    }
    #[doc = "0x110 - Queue 0 Backup Register"]
    #[inline(always)]
    pub fn qbur0_mut(&self) -> &mut QBUR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut QBUR0) }
    }
    #[doc = "0x110 - Queue 0 Input Register"]
    #[inline(always)]
    pub fn qinr0(&self) -> &QINR0 {
        unsafe { &*(((self as *const Self) as *const u8).add(272usize) as *const QINR0) }
    }
    #[doc = "0x110 - Queue 0 Input Register"]
    #[inline(always)]
    pub fn qinr0_mut(&self) -> &mut QINR0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(272usize) as *mut QINR0) }
    }
}
#[doc = "Arbitration Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [arbcfg](arbcfg) module"]
pub type ARBCFG = crate::Reg<u32, _ARBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARBCFG;
#[doc = "`read()` method returns [arbcfg::R](arbcfg::R) reader structure"]
impl crate::Readable for ARBCFG {}
#[doc = "`write(|w| ..)` method takes [arbcfg::W](arbcfg::W) writer structure"]
impl crate::Writable for ARBCFG {}
#[doc = "Arbitration Configuration Register"]
pub mod arbcfg;
#[doc = "Arbitration Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [arbpr](arbpr) module"]
pub type ARBPR = crate::Reg<u32, _ARBPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARBPR;
#[doc = "`read()` method returns [arbpr::R](arbpr::R) reader structure"]
impl crate::Readable for ARBPR {}
#[doc = "`write(|w| ..)` method takes [arbpr::W](arbpr::W) writer structure"]
impl crate::Writable for ARBPR {}
#[doc = "Arbitration Priority Register"]
pub mod arbpr;
#[doc = "Channel Assignment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chass](chass) module"]
pub type CHASS = crate::Reg<u32, _CHASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHASS;
#[doc = "`read()` method returns [chass::R](chass::R) reader structure"]
impl crate::Readable for CHASS {}
#[doc = "`write(|w| ..)` method takes [chass::W](chass::W) writer structure"]
impl crate::Writable for CHASS {}
#[doc = "Channel Assignment Register"]
pub mod chass;
#[doc = "Input Class Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iclass](iclass) module"]
pub type ICLASS = crate::Reg<u32, _ICLASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICLASS;
#[doc = "`read()` method returns [iclass::R](iclass::R) reader structure"]
impl crate::Readable for ICLASS {}
#[doc = "`write(|w| ..)` method takes [iclass::W](iclass::W) writer structure"]
impl crate::Writable for ICLASS {}
#[doc = "Input Class Register"]
pub mod iclass;
#[doc = "Alias Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alias](alias) module"]
pub type ALIAS = crate::Reg<u32, _ALIAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALIAS;
#[doc = "`read()` method returns [alias::R](alias::R) reader structure"]
impl crate::Readable for ALIAS {}
#[doc = "`write(|w| ..)` method takes [alias::W](alias::W) writer structure"]
impl crate::Writable for ALIAS {}
#[doc = "Alias Register"]
pub mod alias;
#[doc = "Boundary Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bound](bound) module"]
pub type BOUND = crate::Reg<u32, _BOUND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOUND;
#[doc = "`read()` method returns [bound::R](bound::R) reader structure"]
impl crate::Readable for BOUND {}
#[doc = "`write(|w| ..)` method takes [bound::W](bound::W) writer structure"]
impl crate::Writable for BOUND {}
#[doc = "Boundary Select Register"]
pub mod bound;
#[doc = "Synchronization Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [synctr](synctr) module"]
pub type SYNCTR = crate::Reg<u32, _SYNCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCTR;
#[doc = "`read()` method returns [synctr::R](synctr::R) reader structure"]
impl crate::Readable for SYNCTR {}
#[doc = "`write(|w| ..)` method takes [synctr::W](synctr::W) writer structure"]
impl crate::Writable for SYNCTR {}
#[doc = "Synchronization Control Register"]
pub mod synctr;
#[doc = "Boundary Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bfl](bfl) module"]
pub type BFL = crate::Reg<u32, _BFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFL;
#[doc = "`read()` method returns [bfl::R](bfl::R) reader structure"]
impl crate::Readable for BFL {}
#[doc = "`write(|w| ..)` method takes [bfl::W](bfl::W) writer structure"]
impl crate::Writable for BFL {}
#[doc = "Boundary Flag Register"]
pub mod bfl;
#[doc = "Boundary Flag Software Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bfls](bfls) module"]
pub type BFLS = crate::Reg<u32, _BFLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFLS;
#[doc = "`write(|w| ..)` method takes [bfls::W](bfls::W) writer structure"]
impl crate::Writable for BFLS {}
#[doc = "Boundary Flag Software Register"]
pub mod bfls;
#[doc = "Boundary Flag Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bflc](bflc) module"]
pub type BFLC = crate::Reg<u32, _BFLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFLC;
#[doc = "`read()` method returns [bflc::R](bflc::R) reader structure"]
impl crate::Readable for BFLC {}
#[doc = "`write(|w| ..)` method takes [bflc::W](bflc::W) writer structure"]
impl crate::Writable for BFLC {}
#[doc = "Boundary Flag Control Register"]
pub mod bflc;
#[doc = "Boundary Flag Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bflnp](bflnp) module"]
pub type BFLNP = crate::Reg<u32, _BFLNP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFLNP;
#[doc = "`read()` method returns [bflnp::R](bflnp::R) reader structure"]
impl crate::Readable for BFLNP {}
#[doc = "`write(|w| ..)` method takes [bflnp::W](bflnp::W) writer structure"]
impl crate::Writable for BFLNP {}
#[doc = "Boundary Flag Node Pointer Register"]
pub mod bflnp;
#[doc = "Queue 0 Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qctrl0](qctrl0) module"]
pub type QCTRL0 = crate::Reg<u32, _QCTRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QCTRL0;
#[doc = "`read()` method returns [qctrl0::R](qctrl0::R) reader structure"]
impl crate::Readable for QCTRL0 {}
#[doc = "`write(|w| ..)` method takes [qctrl0::W](qctrl0::W) writer structure"]
impl crate::Writable for QCTRL0 {}
#[doc = "Queue 0 Source Control Register"]
pub mod qctrl0;
#[doc = "Queue 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qmr0](qmr0) module"]
pub type QMR0 = crate::Reg<u32, _QMR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QMR0;
#[doc = "`read()` method returns [qmr0::R](qmr0::R) reader structure"]
impl crate::Readable for QMR0 {}
#[doc = "`write(|w| ..)` method takes [qmr0::W](qmr0::W) writer structure"]
impl crate::Writable for QMR0 {}
#[doc = "Queue 0 Mode Register"]
pub mod qmr0;
#[doc = "Queue 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qsr0](qsr0) module"]
pub type QSR0 = crate::Reg<u32, _QSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSR0;
#[doc = "`read()` method returns [qsr0::R](qsr0::R) reader structure"]
impl crate::Readable for QSR0 {}
#[doc = "Queue 0 Status Register"]
pub mod qsr0;
#[doc = "Queue 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [q0r0](q0r0) module"]
pub type Q0R0 = crate::Reg<u32, _Q0R0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _Q0R0;
#[doc = "`read()` method returns [q0r0::R](q0r0::R) reader structure"]
impl crate::Readable for Q0R0 {}
#[doc = "Queue 0 Register 0"]
pub mod q0r0;
#[doc = "Queue 0 Input Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qinr0](qinr0) module"]
pub type QINR0 = crate::Reg<u32, _QINR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QINR0;
#[doc = "`write(|w| ..)` method takes [qinr0::W](qinr0::W) writer structure"]
impl crate::Writable for QINR0 {}
#[doc = "Queue 0 Input Register"]
pub mod qinr0;
#[doc = "Queue 0 Backup Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qbur0](qbur0) module"]
pub type QBUR0 = crate::Reg<u32, _QBUR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QBUR0;
#[doc = "`read()` method returns [qbur0::R](qbur0::R) reader structure"]
impl crate::Readable for QBUR0 {}
#[doc = "Queue 0 Backup Register"]
pub mod qbur0;
#[doc = "Autoscan Source Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [asctrl](asctrl) module"]
pub type ASCTRL = crate::Reg<u32, _ASCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASCTRL;
#[doc = "`read()` method returns [asctrl::R](asctrl::R) reader structure"]
impl crate::Readable for ASCTRL {}
#[doc = "`write(|w| ..)` method takes [asctrl::W](asctrl::W) writer structure"]
impl crate::Writable for ASCTRL {}
#[doc = "Autoscan Source Control Register"]
pub mod asctrl;
#[doc = "Autoscan Source Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [asmr](asmr) module"]
pub type ASMR = crate::Reg<u32, _ASMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASMR;
#[doc = "`read()` method returns [asmr::R](asmr::R) reader structure"]
impl crate::Readable for ASMR {}
#[doc = "`write(|w| ..)` method takes [asmr::W](asmr::W) writer structure"]
impl crate::Writable for ASMR {}
#[doc = "Autoscan Source Mode Register"]
pub mod asmr;
#[doc = "Autoscan Source Channel Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [assel](assel) module"]
pub type ASSEL = crate::Reg<u32, _ASSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASSEL;
#[doc = "`read()` method returns [assel::R](assel::R) reader structure"]
impl crate::Readable for ASSEL {}
#[doc = "`write(|w| ..)` method takes [assel::W](assel::W) writer structure"]
impl crate::Writable for ASSEL {}
#[doc = "Autoscan Source Channel Select Register"]
pub mod assel;
#[doc = "Autoscan Source Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aspnd](aspnd) module"]
pub type ASPND = crate::Reg<u32, _ASPND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASPND;
#[doc = "`read()` method returns [aspnd::R](aspnd::R) reader structure"]
impl crate::Readable for ASPND {}
#[doc = "`write(|w| ..)` method takes [aspnd::W](aspnd::W) writer structure"]
impl crate::Writable for ASPND {}
#[doc = "Autoscan Source Pending Register"]
pub mod aspnd;
#[doc = "Channel Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ceflag](ceflag) module"]
pub type CEFLAG = crate::Reg<u32, _CEFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEFLAG;
#[doc = "`read()` method returns [ceflag::R](ceflag::R) reader structure"]
impl crate::Readable for CEFLAG {}
#[doc = "`write(|w| ..)` method takes [ceflag::W](ceflag::W) writer structure"]
impl crate::Writable for CEFLAG {}
#[doc = "Channel Event Flag Register"]
pub mod ceflag;
#[doc = "Result Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reflag](reflag) module"]
pub type REFLAG = crate::Reg<u32, _REFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFLAG;
#[doc = "`read()` method returns [reflag::R](reflag::R) reader structure"]
impl crate::Readable for REFLAG {}
#[doc = "`write(|w| ..)` method takes [reflag::W](reflag::W) writer structure"]
impl crate::Writable for REFLAG {}
#[doc = "Result Event Flag Register"]
pub mod reflag;
#[doc = "Source Event Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seflag](seflag) module"]
pub type SEFLAG = crate::Reg<u32, _SEFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEFLAG;
#[doc = "`read()` method returns [seflag::R](seflag::R) reader structure"]
impl crate::Readable for SEFLAG {}
#[doc = "`write(|w| ..)` method takes [seflag::W](seflag::W) writer structure"]
impl crate::Writable for SEFLAG {}
#[doc = "Source Event Flag Register"]
pub mod seflag;
#[doc = "Channel Event Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cefclr](cefclr) module"]
pub type CEFCLR = crate::Reg<u32, _CEFCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEFCLR;
#[doc = "`write(|w| ..)` method takes [cefclr::W](cefclr::W) writer structure"]
impl crate::Writable for CEFCLR {}
#[doc = "Channel Event Flag Clear Register"]
pub mod cefclr;
#[doc = "Result Event Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [refclr](refclr) module"]
pub type REFCLR = crate::Reg<u32, _REFCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCLR;
#[doc = "`write(|w| ..)` method takes [refclr::W](refclr::W) writer structure"]
impl crate::Writable for REFCLR {}
#[doc = "Result Event Flag Clear Register"]
pub mod refclr;
#[doc = "Source Event Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sefclr](sefclr) module"]
pub type SEFCLR = crate::Reg<u32, _SEFCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEFCLR;
#[doc = "`write(|w| ..)` method takes [sefclr::W](sefclr::W) writer structure"]
impl crate::Writable for SEFCLR {}
#[doc = "Source Event Flag Clear Register"]
pub mod sefclr;
#[doc = "Channel Event Node Pointer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cevnp0](cevnp0) module"]
pub type CEVNP0 = crate::Reg<u32, _CEVNP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEVNP0;
#[doc = "`read()` method returns [cevnp0::R](cevnp0::R) reader structure"]
impl crate::Readable for CEVNP0 {}
#[doc = "`write(|w| ..)` method takes [cevnp0::W](cevnp0::W) writer structure"]
impl crate::Writable for CEVNP0 {}
#[doc = "Channel Event Node Pointer Register 0"]
pub mod cevnp0;
#[doc = "Result Event Node Pointer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [revnp0](revnp0) module"]
pub type REVNP0 = crate::Reg<u32, _REVNP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVNP0;
#[doc = "`read()` method returns [revnp0::R](revnp0::R) reader structure"]
impl crate::Readable for REVNP0 {}
#[doc = "`write(|w| ..)` method takes [revnp0::W](revnp0::W) writer structure"]
impl crate::Writable for REVNP0 {}
#[doc = "Result Event Node Pointer Register 0"]
pub mod revnp0;
#[doc = "Result Event Node Pointer Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [revnp1](revnp1) module"]
pub type REVNP1 = crate::Reg<u32, _REVNP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVNP1;
#[doc = "`read()` method returns [revnp1::R](revnp1::R) reader structure"]
impl crate::Readable for REVNP1 {}
#[doc = "`write(|w| ..)` method takes [revnp1::W](revnp1::W) writer structure"]
impl crate::Writable for REVNP1 {}
#[doc = "Result Event Node Pointer Register 1"]
pub mod revnp1;
#[doc = "Source Event Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sevnp](sevnp) module"]
pub type SEVNP = crate::Reg<u32, _SEVNP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEVNP;
#[doc = "`read()` method returns [sevnp::R](sevnp::R) reader structure"]
impl crate::Readable for SEVNP {}
#[doc = "`write(|w| ..)` method takes [sevnp::W](sevnp::W) writer structure"]
impl crate::Writable for SEVNP {}
#[doc = "Source Event Node Pointer Register"]
pub mod sevnp;
#[doc = "Service Request Software Activation Trigger\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sract](sract) module"]
pub type SRACT = crate::Reg<u32, _SRACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRACT;
#[doc = "`write(|w| ..)` method takes [sract::W](sract::W) writer structure"]
impl crate::Writable for SRACT {}
#[doc = "Service Request Software Activation Trigger"]
pub mod sract;
#[doc = "E0ternal Multiplexer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [emuxctr](emuxctr) module"]
pub type EMUXCTR = crate::Reg<u32, _EMUXCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUXCTR;
#[doc = "`read()` method returns [emuxctr::R](emuxctr::R) reader structure"]
impl crate::Readable for EMUXCTR {}
#[doc = "`write(|w| ..)` method takes [emuxctr::W](emuxctr::W) writer structure"]
impl crate::Writable for EMUXCTR {}
#[doc = "E0ternal Multiplexer Control Register"]
pub mod emuxctr;
#[doc = "Valid Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vfr](vfr) module"]
pub type VFR = crate::Reg<u32, _VFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VFR;
#[doc = "`read()` method returns [vfr::R](vfr::R) reader structure"]
impl crate::Readable for VFR {}
#[doc = "`write(|w| ..)` method takes [vfr::W](vfr::W) writer structure"]
impl crate::Writable for VFR {}
#[doc = "Valid Flag Register"]
pub mod vfr;
#[doc = "Channel Ctrl. Reg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chctr](chctr) module"]
pub type CHCTR = crate::Reg<u32, _CHCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTR;
#[doc = "`read()` method returns [chctr::R](chctr::R) reader structure"]
impl crate::Readable for CHCTR {}
#[doc = "`write(|w| ..)` method takes [chctr::W](chctr::W) writer structure"]
impl crate::Writable for CHCTR {}
#[doc = "Channel Ctrl. Reg."]
pub mod chctr;
#[doc = "Result Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "Result Control Register"]
pub mod rcr;
#[doc = "Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [res](res) module"]
pub type RES = crate::Reg<u32, _RES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RES;
#[doc = "`read()` method returns [res::R](res::R) reader structure"]
impl crate::Readable for RES {}
#[doc = "`write(|w| ..)` method takes [res::W](res::W) writer structure"]
impl crate::Writable for RES {}
#[doc = "Result Register"]
pub mod res;
#[doc = "Result Register, Debug\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resd](resd) module"]
pub type RESD = crate::Reg<u32, _RESD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESD;
#[doc = "`read()` method returns [resd::R](resd::R) reader structure"]
impl crate::Readable for RESD {}
#[doc = "Result Register, Debug"]
pub mod resd;
