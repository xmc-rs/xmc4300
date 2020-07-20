#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Host Channel Characteristics Register"]
    pub hcchar: HCCHAR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Host Channel Interrupt Register"]
    pub hcint: HCINT,
    #[doc = "0x0c - Host Channel Interrupt Mask Register"]
    pub hcintmsk: HCINTMSK,
    _reserved_3_hctsiz: [u8; 4usize],
    _reserved_4_hcdma: [u8; 4usize],
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - Host Channel DMA Buffer Address Register"]
    pub hcdmab: HCDMAB,
}
impl RegisterBlock {
    #[doc = "0x10 - Host Channel Transfer Size Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub fn hctsiz_scatgather(&self) -> &HCTSIZ_SCATGATHER {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const HCTSIZ_SCATGATHER) }
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub fn hctsiz_scatgather_mut(&self) -> &mut HCTSIZ_SCATGATHER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut HCTSIZ_SCATGATHER) }
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub fn hctsiz_buffermode(&self) -> &HCTSIZ_BUFFERMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const HCTSIZ_BUFFERMODE) }
    }
    #[doc = "0x10 - Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub fn hctsiz_buffermode_mut(&self) -> &mut HCTSIZ_BUFFERMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut HCTSIZ_BUFFERMODE) }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub fn hcdma_scatgather(&self) -> &HCDMA_SCATGATHER {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const HCDMA_SCATGATHER) }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[SCATGATHER\\]"]
    #[inline(always)]
    pub fn hcdma_scatgather_mut(&self) -> &mut HCDMA_SCATGATHER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut HCDMA_SCATGATHER) }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub fn hcdma_buffermode(&self) -> &HCDMA_BUFFERMODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const HCDMA_BUFFERMODE) }
    }
    #[doc = "0x14 - Host Channel DMA Address Register \\[BUFFERMODE\\]"]
    #[inline(always)]
    pub fn hcdma_buffermode_mut(&self) -> &mut HCDMA_BUFFERMODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut HCDMA_BUFFERMODE) }
    }
}
#[doc = "Host Channel Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar](hcchar) module"]
pub type HCCHAR = crate::Reg<u32, _HCCHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCCHAR;
#[doc = "`read()` method returns [hcchar::R](hcchar::R) reader structure"]
impl crate::Readable for HCCHAR {}
#[doc = "`write(|w| ..)` method takes [hcchar::W](hcchar::W) writer structure"]
impl crate::Writable for HCCHAR {}
#[doc = "Host Channel Characteristics Register"]
pub mod hcchar;
#[doc = "Host Channel Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint](hcint) module"]
pub type HCINT = crate::Reg<u32, _HCINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINT;
#[doc = "`read()` method returns [hcint::R](hcint::R) reader structure"]
impl crate::Readable for HCINT {}
#[doc = "`write(|w| ..)` method takes [hcint::W](hcint::W) writer structure"]
impl crate::Writable for HCINT {}
#[doc = "Host Channel Interrupt Register"]
pub mod hcint;
#[doc = "Host Channel Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk](hcintmsk) module"]
pub type HCINTMSK = crate::Reg<u32, _HCINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCINTMSK;
#[doc = "`read()` method returns [hcintmsk::R](hcintmsk::R) reader structure"]
impl crate::Readable for HCINTMSK {}
#[doc = "`write(|w| ..)` method takes [hcintmsk::W](hcintmsk::W) writer structure"]
impl crate::Writable for HCINTMSK {}
#[doc = "Host Channel Interrupt Mask Register"]
pub mod hcintmsk;
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz_buffermode](hctsiz_buffermode) module"]
pub type HCTSIZ_BUFFERMODE = crate::Reg<u32, _HCTSIZ_BUFFERMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ_BUFFERMODE;
#[doc = "`read()` method returns [hctsiz_buffermode::R](hctsiz_buffermode::R) reader structure"]
impl crate::Readable for HCTSIZ_BUFFERMODE {}
#[doc = "`write(|w| ..)` method takes [hctsiz_buffermode::W](hctsiz_buffermode::W) writer structure"]
impl crate::Writable for HCTSIZ_BUFFERMODE {}
#[doc = "Host Channel Transfer Size Register \\[BUFFERMODE\\]"]
pub mod hctsiz_buffermode;
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz_scatgather](hctsiz_scatgather) module"]
pub type HCTSIZ_SCATGATHER = crate::Reg<u32, _HCTSIZ_SCATGATHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTSIZ_SCATGATHER;
#[doc = "`read()` method returns [hctsiz_scatgather::R](hctsiz_scatgather::R) reader structure"]
impl crate::Readable for HCTSIZ_SCATGATHER {}
#[doc = "`write(|w| ..)` method takes [hctsiz_scatgather::W](hctsiz_scatgather::W) writer structure"]
impl crate::Writable for HCTSIZ_SCATGATHER {}
#[doc = "Host Channel Transfer Size Register \\[SCATGATHER\\]"]
pub mod hctsiz_scatgather;
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma_buffermode](hcdma_buffermode) module"]
pub type HCDMA_BUFFERMODE = crate::Reg<u32, _HCDMA_BUFFERMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA_BUFFERMODE;
#[doc = "`read()` method returns [hcdma_buffermode::R](hcdma_buffermode::R) reader structure"]
impl crate::Readable for HCDMA_BUFFERMODE {}
#[doc = "`write(|w| ..)` method takes [hcdma_buffermode::W](hcdma_buffermode::W) writer structure"]
impl crate::Writable for HCDMA_BUFFERMODE {}
#[doc = "Host Channel DMA Address Register \\[BUFFERMODE\\]"]
pub mod hcdma_buffermode;
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma_scatgather](hcdma_scatgather) module"]
pub type HCDMA_SCATGATHER = crate::Reg<u32, _HCDMA_SCATGATHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMA_SCATGATHER;
#[doc = "`read()` method returns [hcdma_scatgather::R](hcdma_scatgather::R) reader structure"]
impl crate::Readable for HCDMA_SCATGATHER {}
#[doc = "`write(|w| ..)` method takes [hcdma_scatgather::W](hcdma_scatgather::W) writer structure"]
impl crate::Writable for HCDMA_SCATGATHER {}
#[doc = "Host Channel DMA Address Register \\[SCATGATHER\\]"]
pub mod hcdma_scatgather;
#[doc = "Host Channel DMA Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdmab](hcdmab) module"]
pub type HCDMAB = crate::Reg<u32, _HCDMAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCDMAB;
#[doc = "`read()` method returns [hcdmab::R](hcdmab::R) reader structure"]
impl crate::Readable for HCDMAB {}
#[doc = "Host Channel DMA Buffer Address Register"]
pub mod hcdmab;
