#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Channel Configuration Register"]
    pub ccfg: CCFG,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - Kernel State Configuration Register"]
    pub kscfg: KSCFG,
    #[doc = "0x10 - Fractional Divider Register"]
    pub fdr: FDR,
    #[doc = "0x14 - Baud Rate Generator Register"]
    pub brg: BRG,
    #[doc = "0x18 - Interrupt Node Pointer Register"]
    pub inpr: INPR,
    #[doc = "0x1c - Input Control Register 0"]
    pub dx0cr: DX0CR,
    #[doc = "0x20 - Input Control Register 1"]
    pub dx1cr: DX1CR,
    #[doc = "0x24 - Input Control Register 2"]
    pub dx2cr: DX2CR,
    #[doc = "0x28 - Input Control Register 3"]
    pub dx3cr: DX3CR,
    #[doc = "0x2c - Input Control Register 4"]
    pub dx4cr: DX4CR,
    #[doc = "0x30 - Input Control Register 5"]
    pub dx5cr: DX5CR,
    #[doc = "0x34 - Shift Control Register"]
    pub sctr: SCTR,
    #[doc = "0x38 - Transmit Control/Status Register"]
    pub tcsr: TCSR,
    _reserved_13_pcr: [u8; 4usize],
    #[doc = "0x40 - Channel Control Register"]
    pub ccr: CCR,
    #[doc = "0x44 - Capture Mode Timer Register"]
    pub cmtr: CMTR,
    _reserved_16_psr: [u8; 4usize],
    #[doc = "0x4c - Protocol Status Clear Register"]
    pub pscr: PSCR,
    #[doc = "0x50 - Receiver Buffer Status Register"]
    pub rbufsr: RBUFSR,
    #[doc = "0x54 - Receiver Buffer Register"]
    pub rbuf: RBUF,
    #[doc = "0x58 - Receiver Buffer Register for Debugger"]
    pub rbufd: RBUFD,
    #[doc = "0x5c - Receiver Buffer Register 0"]
    pub rbuf0: RBUF0,
    #[doc = "0x60 - Receiver Buffer Register 1"]
    pub rbuf1: RBUF1,
    #[doc = "0x64 - Receiver Buffer 01 Status Register"]
    pub rbuf01sr: RBUF01SR,
    #[doc = "0x68 - Flag Modification Register"]
    pub fmr: FMR,
    _reserved25: [u8; 20usize],
    #[doc = "0x80 - Transmit Buffer"]
    pub tbuf: [TBUF; 32],
    #[doc = "0x100 - Bypass Data Register"]
    pub byp: BYP,
    #[doc = "0x104 - Bypass Control Register"]
    pub bypcr: BYPCR,
    #[doc = "0x108 - Transmitter Buffer Control Register"]
    pub tbctr: TBCTR,
    #[doc = "0x10c - Receiver Buffer Control Register"]
    pub rbctr: RBCTR,
    #[doc = "0x110 - Transmit/Receive Buffer Pointer Register"]
    pub trbptr: TRBPTR,
    #[doc = "0x114 - Transmit/Receive Buffer Status Register"]
    pub trbsr: TRBSR,
    #[doc = "0x118 - Transmit/Receive Buffer Status Clear Register"]
    pub trbscr: TRBSCR,
    #[doc = "0x11c - Receiver Buffer Output Register"]
    pub outr: OUTR,
    #[doc = "0x120 - Receiver Buffer Output Register L for Debugger"]
    pub outdr: OUTDR,
    _reserved35: [u8; 92usize],
    #[doc = "0x180 - Transmit FIFO Buffer"]
    pub in_: [IN; 32],
}
impl RegisterBlock {
    #[doc = "0x3c - Protocol Control Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub fn pcr_iismode(&self) -> &PCR_IISMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const PCR_IISMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub fn pcr_iismode_mut(&self) -> &mut PCR_IISMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PCR_IISMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub fn pcr_iicmode(&self) -> &PCR_IICMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const PCR_IICMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub fn pcr_iicmode_mut(&self) -> &mut PCR_IICMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PCR_IICMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub fn pcr_sscmode(&self) -> &PCR_SSCMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const PCR_SSCMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub fn pcr_sscmode_mut(&self) -> &mut PCR_SSCMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PCR_SSCMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub fn pcr_ascmode(&self) -> &PCR_ASCMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const PCR_ASCMODE) }
    }
    #[doc = "0x3c - Protocol Control Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub fn pcr_ascmode_mut(&self) -> &mut PCR_ASCMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PCR_ASCMODE) }
    }
    #[doc = "0x3c - Protocol Control Register"]
    #[inline(always)]
    pub fn pcr(&self) -> &PCR {
        unsafe { &*(((self as *const Self) as *const u8).add(60usize) as *const PCR) }
    }
    #[doc = "0x3c - Protocol Control Register"]
    #[inline(always)]
    pub fn pcr_mut(&self) -> &mut PCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(60usize) as *mut PCR) }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub fn psr_iismode(&self) -> &PSR_IISMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const PSR_IISMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIS Mode\\]"]
    #[inline(always)]
    pub fn psr_iismode_mut(&self) -> &mut PSR_IISMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PSR_IISMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub fn psr_iicmode(&self) -> &PSR_IICMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const PSR_IICMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[IIC Mode\\]"]
    #[inline(always)]
    pub fn psr_iicmode_mut(&self) -> &mut PSR_IICMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PSR_IICMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub fn psr_sscmode(&self) -> &PSR_SSCMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const PSR_SSCMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[SSC Mode\\]"]
    #[inline(always)]
    pub fn psr_sscmode_mut(&self) -> &mut PSR_SSCMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PSR_SSCMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub fn psr_ascmode(&self) -> &PSR_ASCMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const PSR_ASCMODE) }
    }
    #[doc = "0x48 - Protocol Status Register \\[ASC Mode\\]"]
    #[inline(always)]
    pub fn psr_ascmode_mut(&self) -> &mut PSR_ASCMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PSR_ASCMODE) }
    }
    #[doc = "0x48 - Protocol Status Register"]
    #[inline(always)]
    pub fn psr(&self) -> &PSR {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const PSR) }
    }
    #[doc = "0x48 - Protocol Status Register"]
    #[inline(always)]
    pub fn psr_mut(&self) -> &mut PSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut PSR) }
    }
}
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccfg](ccfg) module"]
pub type CCFG = crate::Reg<u32, _CCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG;
#[doc = "`read()` method returns [ccfg::R](ccfg::R) reader structure"]
impl crate::Readable for CCFG {}
#[doc = "Channel Configuration Register"]
pub mod ccfg;
#[doc = "Kernel State Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [kscfg](kscfg) module"]
pub type KSCFG = crate::Reg<u32, _KSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KSCFG;
#[doc = "`read()` method returns [kscfg::R](kscfg::R) reader structure"]
impl crate::Readable for KSCFG {}
#[doc = "`write(|w| ..)` method takes [kscfg::W](kscfg::W) writer structure"]
impl crate::Writable for KSCFG {}
#[doc = "Kernel State Configuration Register"]
pub mod kscfg;
#[doc = "Fractional Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "`write(|w| ..)` method takes [fdr::W](fdr::W) writer structure"]
impl crate::Writable for FDR {}
#[doc = "Fractional Divider Register"]
pub mod fdr;
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brg](brg) module"]
pub type BRG = crate::Reg<u32, _BRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG;
#[doc = "`read()` method returns [brg::R](brg::R) reader structure"]
impl crate::Readable for BRG {}
#[doc = "`write(|w| ..)` method takes [brg::W](brg::W) writer structure"]
impl crate::Writable for BRG {}
#[doc = "Baud Rate Generator Register"]
pub mod brg;
#[doc = "Interrupt Node Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inpr](inpr) module"]
pub type INPR = crate::Reg<u32, _INPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPR;
#[doc = "`read()` method returns [inpr::R](inpr::R) reader structure"]
impl crate::Readable for INPR {}
#[doc = "`write(|w| ..)` method takes [inpr::W](inpr::W) writer structure"]
impl crate::Writable for INPR {}
#[doc = "Interrupt Node Pointer Register"]
pub mod inpr;
#[doc = "Input Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx0cr](dx0cr) module"]
pub type DX0CR = crate::Reg<u32, _DX0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX0CR;
#[doc = "`read()` method returns [dx0cr::R](dx0cr::R) reader structure"]
impl crate::Readable for DX0CR {}
#[doc = "`write(|w| ..)` method takes [dx0cr::W](dx0cr::W) writer structure"]
impl crate::Writable for DX0CR {}
#[doc = "Input Control Register 0"]
pub mod dx0cr;
#[doc = "Input Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx1cr](dx1cr) module"]
pub type DX1CR = crate::Reg<u32, _DX1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX1CR;
#[doc = "`read()` method returns [dx1cr::R](dx1cr::R) reader structure"]
impl crate::Readable for DX1CR {}
#[doc = "`write(|w| ..)` method takes [dx1cr::W](dx1cr::W) writer structure"]
impl crate::Writable for DX1CR {}
#[doc = "Input Control Register 1"]
pub mod dx1cr;
#[doc = "Input Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx2cr](dx2cr) module"]
pub type DX2CR = crate::Reg<u32, _DX2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX2CR;
#[doc = "`read()` method returns [dx2cr::R](dx2cr::R) reader structure"]
impl crate::Readable for DX2CR {}
#[doc = "`write(|w| ..)` method takes [dx2cr::W](dx2cr::W) writer structure"]
impl crate::Writable for DX2CR {}
#[doc = "Input Control Register 2"]
pub mod dx2cr;
#[doc = "Input Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx3cr](dx3cr) module"]
pub type DX3CR = crate::Reg<u32, _DX3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX3CR;
#[doc = "`read()` method returns [dx3cr::R](dx3cr::R) reader structure"]
impl crate::Readable for DX3CR {}
#[doc = "`write(|w| ..)` method takes [dx3cr::W](dx3cr::W) writer structure"]
impl crate::Writable for DX3CR {}
#[doc = "Input Control Register 3"]
pub mod dx3cr;
#[doc = "Input Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx4cr](dx4cr) module"]
pub type DX4CR = crate::Reg<u32, _DX4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX4CR;
#[doc = "`read()` method returns [dx4cr::R](dx4cr::R) reader structure"]
impl crate::Readable for DX4CR {}
#[doc = "`write(|w| ..)` method takes [dx4cr::W](dx4cr::W) writer structure"]
impl crate::Writable for DX4CR {}
#[doc = "Input Control Register 4"]
pub mod dx4cr;
#[doc = "Input Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dx5cr](dx5cr) module"]
pub type DX5CR = crate::Reg<u32, _DX5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DX5CR;
#[doc = "`read()` method returns [dx5cr::R](dx5cr::R) reader structure"]
impl crate::Readable for DX5CR {}
#[doc = "`write(|w| ..)` method takes [dx5cr::W](dx5cr::W) writer structure"]
impl crate::Writable for DX5CR {}
#[doc = "Input Control Register 5"]
pub mod dx5cr;
#[doc = "Shift Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sctr](sctr) module"]
pub type SCTR = crate::Reg<u32, _SCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTR;
#[doc = "`read()` method returns [sctr::R](sctr::R) reader structure"]
impl crate::Readable for SCTR {}
#[doc = "`write(|w| ..)` method takes [sctr::W](sctr::W) writer structure"]
impl crate::Writable for SCTR {}
#[doc = "Shift Control Register"]
pub mod sctr;
#[doc = "Transmit Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcsr](tcsr) module"]
pub type TCSR = crate::Reg<u32, _TCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCSR;
#[doc = "`read()` method returns [tcsr::R](tcsr::R) reader structure"]
impl crate::Readable for TCSR {}
#[doc = "`write(|w| ..)` method takes [tcsr::W](tcsr::W) writer structure"]
impl crate::Writable for TCSR {}
#[doc = "Transmit Control/Status Register"]
pub mod tcsr;
#[doc = "Protocol Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "Protocol Control Register"]
pub mod pcr;
#[doc = "Protocol Control Register \\[ASC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr_ascmode](pcr_ascmode) module"]
pub type PCR_ASCMODE = crate::Reg<u32, _PCR_ASCMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR_ASCMODE;
#[doc = "`read()` method returns [pcr_ascmode::R](pcr_ascmode::R) reader structure"]
impl crate::Readable for PCR_ASCMODE {}
#[doc = "`write(|w| ..)` method takes [pcr_ascmode::W](pcr_ascmode::W) writer structure"]
impl crate::Writable for PCR_ASCMODE {}
#[doc = "Protocol Control Register \\[ASC Mode\\]"]
pub mod pcr_ascmode;
#[doc = "Protocol Control Register \\[SSC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr_sscmode](pcr_sscmode) module"]
pub type PCR_SSCMODE = crate::Reg<u32, _PCR_SSCMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR_SSCMODE;
#[doc = "`read()` method returns [pcr_sscmode::R](pcr_sscmode::R) reader structure"]
impl crate::Readable for PCR_SSCMODE {}
#[doc = "`write(|w| ..)` method takes [pcr_sscmode::W](pcr_sscmode::W) writer structure"]
impl crate::Writable for PCR_SSCMODE {}
#[doc = "Protocol Control Register \\[SSC Mode\\]"]
pub mod pcr_sscmode;
#[doc = "Protocol Control Register \\[IIC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr_iicmode](pcr_iicmode) module"]
pub type PCR_IICMODE = crate::Reg<u32, _PCR_IICMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR_IICMODE;
#[doc = "`read()` method returns [pcr_iicmode::R](pcr_iicmode::R) reader structure"]
impl crate::Readable for PCR_IICMODE {}
#[doc = "`write(|w| ..)` method takes [pcr_iicmode::W](pcr_iicmode::W) writer structure"]
impl crate::Writable for PCR_IICMODE {}
#[doc = "Protocol Control Register \\[IIC Mode\\]"]
pub mod pcr_iicmode;
#[doc = "Protocol Control Register \\[IIS Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcr_iismode](pcr_iismode) module"]
pub type PCR_IISMODE = crate::Reg<u32, _PCR_IISMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR_IISMODE;
#[doc = "`read()` method returns [pcr_iismode::R](pcr_iismode::R) reader structure"]
impl crate::Readable for PCR_IISMODE {}
#[doc = "`write(|w| ..)` method takes [pcr_iismode::W](pcr_iismode::W) writer structure"]
impl crate::Writable for PCR_IISMODE {}
#[doc = "Protocol Control Register \\[IIS Mode\\]"]
pub mod pcr_iismode;
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Channel Control Register"]
pub mod ccr;
#[doc = "Capture Mode Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmtr](cmtr) module"]
pub type CMTR = crate::Reg<u32, _CMTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMTR;
#[doc = "`read()` method returns [cmtr::R](cmtr::R) reader structure"]
impl crate::Readable for CMTR {}
#[doc = "`write(|w| ..)` method takes [cmtr::W](cmtr::W) writer structure"]
impl crate::Writable for CMTR {}
#[doc = "Capture Mode Timer Register"]
pub mod cmtr;
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "`write(|w| ..)` method takes [psr::W](psr::W) writer structure"]
impl crate::Writable for PSR {}
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "Protocol Status Register \\[ASC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr_ascmode](psr_ascmode) module"]
pub type PSR_ASCMODE = crate::Reg<u32, _PSR_ASCMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR_ASCMODE;
#[doc = "`read()` method returns [psr_ascmode::R](psr_ascmode::R) reader structure"]
impl crate::Readable for PSR_ASCMODE {}
#[doc = "`write(|w| ..)` method takes [psr_ascmode::W](psr_ascmode::W) writer structure"]
impl crate::Writable for PSR_ASCMODE {}
#[doc = "Protocol Status Register \\[ASC Mode\\]"]
pub mod psr_ascmode;
#[doc = "Protocol Status Register \\[SSC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr_sscmode](psr_sscmode) module"]
pub type PSR_SSCMODE = crate::Reg<u32, _PSR_SSCMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR_SSCMODE;
#[doc = "`read()` method returns [psr_sscmode::R](psr_sscmode::R) reader structure"]
impl crate::Readable for PSR_SSCMODE {}
#[doc = "`write(|w| ..)` method takes [psr_sscmode::W](psr_sscmode::W) writer structure"]
impl crate::Writable for PSR_SSCMODE {}
#[doc = "Protocol Status Register \\[SSC Mode\\]"]
pub mod psr_sscmode;
#[doc = "Protocol Status Register \\[IIC Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr_iicmode](psr_iicmode) module"]
pub type PSR_IICMODE = crate::Reg<u32, _PSR_IICMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR_IICMODE;
#[doc = "`read()` method returns [psr_iicmode::R](psr_iicmode::R) reader structure"]
impl crate::Readable for PSR_IICMODE {}
#[doc = "`write(|w| ..)` method takes [psr_iicmode::W](psr_iicmode::W) writer structure"]
impl crate::Writable for PSR_IICMODE {}
#[doc = "Protocol Status Register \\[IIC Mode\\]"]
pub mod psr_iicmode;
#[doc = "Protocol Status Register \\[IIS Mode\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [psr_iismode](psr_iismode) module"]
pub type PSR_IISMODE = crate::Reg<u32, _PSR_IISMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR_IISMODE;
#[doc = "`read()` method returns [psr_iismode::R](psr_iismode::R) reader structure"]
impl crate::Readable for PSR_IISMODE {}
#[doc = "`write(|w| ..)` method takes [psr_iismode::W](psr_iismode::W) writer structure"]
impl crate::Writable for PSR_IISMODE {}
#[doc = "Protocol Status Register \\[IIS Mode\\]"]
pub mod psr_iismode;
#[doc = "Protocol Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pscr](pscr) module"]
pub type PSCR = crate::Reg<u32, _PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSCR;
#[doc = "`write(|w| ..)` method takes [pscr::W](pscr::W) writer structure"]
impl crate::Writable for PSCR {}
#[doc = "Protocol Status Clear Register"]
pub mod pscr;
#[doc = "Receiver Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbufsr](rbufsr) module"]
pub type RBUFSR = crate::Reg<u32, _RBUFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUFSR;
#[doc = "`read()` method returns [rbufsr::R](rbufsr::R) reader structure"]
impl crate::Readable for RBUFSR {}
#[doc = "Receiver Buffer Status Register"]
pub mod rbufsr;
#[doc = "Receiver Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbuf](rbuf) module"]
pub type RBUF = crate::Reg<u32, _RBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUF;
#[doc = "`read()` method returns [rbuf::R](rbuf::R) reader structure"]
impl crate::Readable for RBUF {}
#[doc = "Receiver Buffer Register"]
pub mod rbuf;
#[doc = "Receiver Buffer Register for Debugger\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbufd](rbufd) module"]
pub type RBUFD = crate::Reg<u32, _RBUFD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUFD;
#[doc = "`read()` method returns [rbufd::R](rbufd::R) reader structure"]
impl crate::Readable for RBUFD {}
#[doc = "Receiver Buffer Register for Debugger"]
pub mod rbufd;
#[doc = "Receiver Buffer Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbuf0](rbuf0) module"]
pub type RBUF0 = crate::Reg<u32, _RBUF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUF0;
#[doc = "`read()` method returns [rbuf0::R](rbuf0::R) reader structure"]
impl crate::Readable for RBUF0 {}
#[doc = "Receiver Buffer Register 0"]
pub mod rbuf0;
#[doc = "Receiver Buffer Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbuf1](rbuf1) module"]
pub type RBUF1 = crate::Reg<u32, _RBUF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUF1;
#[doc = "`read()` method returns [rbuf1::R](rbuf1::R) reader structure"]
impl crate::Readable for RBUF1 {}
#[doc = "Receiver Buffer Register 1"]
pub mod rbuf1;
#[doc = "Receiver Buffer 01 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbuf01sr](rbuf01sr) module"]
pub type RBUF01SR = crate::Reg<u32, _RBUF01SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBUF01SR;
#[doc = "`read()` method returns [rbuf01sr::R](rbuf01sr::R) reader structure"]
impl crate::Readable for RBUF01SR {}
#[doc = "Receiver Buffer 01 Status Register"]
pub mod rbuf01sr;
#[doc = "Flag Modification Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fmr](fmr) module"]
pub type FMR = crate::Reg<u32, _FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMR;
#[doc = "`write(|w| ..)` method takes [fmr::W](fmr::W) writer structure"]
impl crate::Writable for FMR {}
#[doc = "Flag Modification Register"]
pub mod fmr;
#[doc = "Transmit Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbuf](tbuf) module"]
pub type TBUF = crate::Reg<u32, _TBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBUF;
#[doc = "`read()` method returns [tbuf::R](tbuf::R) reader structure"]
impl crate::Readable for TBUF {}
#[doc = "`write(|w| ..)` method takes [tbuf::W](tbuf::W) writer structure"]
impl crate::Writable for TBUF {}
#[doc = "Transmit Buffer"]
pub mod tbuf;
#[doc = "Bypass Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [byp](byp) module"]
pub type BYP = crate::Reg<u32, _BYP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BYP;
#[doc = "`read()` method returns [byp::R](byp::R) reader structure"]
impl crate::Readable for BYP {}
#[doc = "`write(|w| ..)` method takes [byp::W](byp::W) writer structure"]
impl crate::Writable for BYP {}
#[doc = "Bypass Data Register"]
pub mod byp;
#[doc = "Bypass Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bypcr](bypcr) module"]
pub type BYPCR = crate::Reg<u32, _BYPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BYPCR;
#[doc = "`read()` method returns [bypcr::R](bypcr::R) reader structure"]
impl crate::Readable for BYPCR {}
#[doc = "`write(|w| ..)` method takes [bypcr::W](bypcr::W) writer structure"]
impl crate::Writable for BYPCR {}
#[doc = "Bypass Control Register"]
pub mod bypcr;
#[doc = "Transmitter Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tbctr](tbctr) module"]
pub type TBCTR = crate::Reg<u32, _TBCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBCTR;
#[doc = "`read()` method returns [tbctr::R](tbctr::R) reader structure"]
impl crate::Readable for TBCTR {}
#[doc = "`write(|w| ..)` method takes [tbctr::W](tbctr::W) writer structure"]
impl crate::Writable for TBCTR {}
#[doc = "Transmitter Buffer Control Register"]
pub mod tbctr;
#[doc = "Receiver Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbctr](rbctr) module"]
pub type RBCTR = crate::Reg<u32, _RBCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBCTR;
#[doc = "`read()` method returns [rbctr::R](rbctr::R) reader structure"]
impl crate::Readable for RBCTR {}
#[doc = "`write(|w| ..)` method takes [rbctr::W](rbctr::W) writer structure"]
impl crate::Writable for RBCTR {}
#[doc = "Receiver Buffer Control Register"]
pub mod rbctr;
#[doc = "Transmit/Receive Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trbptr](trbptr) module"]
pub type TRBPTR = crate::Reg<u32, _TRBPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRBPTR;
#[doc = "`read()` method returns [trbptr::R](trbptr::R) reader structure"]
impl crate::Readable for TRBPTR {}
#[doc = "Transmit/Receive Buffer Pointer Register"]
pub mod trbptr;
#[doc = "Transmit/Receive Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trbsr](trbsr) module"]
pub type TRBSR = crate::Reg<u32, _TRBSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRBSR;
#[doc = "`read()` method returns [trbsr::R](trbsr::R) reader structure"]
impl crate::Readable for TRBSR {}
#[doc = "`write(|w| ..)` method takes [trbsr::W](trbsr::W) writer structure"]
impl crate::Writable for TRBSR {}
#[doc = "Transmit/Receive Buffer Status Register"]
pub mod trbsr;
#[doc = "Transmit/Receive Buffer Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [trbscr](trbscr) module"]
pub type TRBSCR = crate::Reg<u32, _TRBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRBSCR;
#[doc = "`write(|w| ..)` method takes [trbscr::W](trbscr::W) writer structure"]
impl crate::Writable for TRBSCR {}
#[doc = "Transmit/Receive Buffer Status Clear Register"]
pub mod trbscr;
#[doc = "Receiver Buffer Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outr](outr) module"]
pub type OUTR = crate::Reg<u32, _OUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTR;
#[doc = "`read()` method returns [outr::R](outr::R) reader structure"]
impl crate::Readable for OUTR {}
#[doc = "Receiver Buffer Output Register"]
pub mod outr;
#[doc = "Receiver Buffer Output Register L for Debugger\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outdr](outdr) module"]
pub type OUTDR = crate::Reg<u32, _OUTDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTDR;
#[doc = "`read()` method returns [outdr::R](outdr::R) reader structure"]
impl crate::Readable for OUTDR {}
#[doc = "Receiver Buffer Output Register L for Debugger"]
pub mod outdr;
#[doc = "Transmit FIFO Buffer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`write(|w| ..)` method takes [in_::W](in_::W) writer structure"]
impl crate::Writable for IN {}
#[doc = "Transmit FIFO Buffer"]
pub mod in_;
