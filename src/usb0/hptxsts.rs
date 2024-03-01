#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HptxstsSpec>;
#[doc = "Register `HPTXSTS` writer"]
pub type W = crate::W<HptxstsSpec>;
#[doc = "Periodic Transmit Data FIFO Space Available\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PtxFspcAvail {
    #[doc = "0: Periodic TxFIFO is full"]
    Value1 = 0,
    #[doc = "1: 1 word available"]
    Value2 = 1,
    #[doc = "2: 2 words available"]
    Value3 = 2,
}
impl From<PtxFspcAvail> for u16 {
    #[inline(always)]
    fn from(variant: PtxFspcAvail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PtxFspcAvail {
    type Ux = u16;
}
#[doc = "Field `PTxFSpcAvail` reader - Periodic Transmit Data FIFO Space Available"]
pub type PtxFspcAvailR = crate::FieldReader<PtxFspcAvail>;
impl PtxFspcAvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PtxFspcAvail> {
        match self.bits {
            0 => Some(PtxFspcAvail::Value1),
            1 => Some(PtxFspcAvail::Value2),
            2 => Some(PtxFspcAvail::Value3),
            _ => None,
        }
    }
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PtxFspcAvail::Value1
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PtxFspcAvail::Value2
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PtxFspcAvail::Value3
    }
}
#[doc = "Field `PTxFSpcAvail` writer - Periodic Transmit Data FIFO Space Available"]
pub type PtxFspcAvailW<'a, REG> = crate::FieldWriter<'a, REG, 16, PtxFspcAvail>;
impl<'a, REG> PtxFspcAvailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PtxFspcAvail::Value1)
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PtxFspcAvail::Value2)
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PtxFspcAvail::Value3)
    }
}
#[doc = "Periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PtxQspcAvail {
    #[doc = "0: Periodic Transmit Request Queue is full"]
    Value1 = 0,
    #[doc = "1: 1 location available"]
    Value2 = 1,
    #[doc = "2: 2 locations available"]
    Value3 = 2,
}
impl From<PtxQspcAvail> for u8 {
    #[inline(always)]
    fn from(variant: PtxQspcAvail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PtxQspcAvail {
    type Ux = u8;
}
#[doc = "Field `PTxQSpcAvail` reader - Periodic Transmit Request Queue Space Available"]
pub type PtxQspcAvailR = crate::FieldReader<PtxQspcAvail>;
impl PtxQspcAvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PtxQspcAvail> {
        match self.bits {
            0 => Some(PtxQspcAvail::Value1),
            1 => Some(PtxQspcAvail::Value2),
            2 => Some(PtxQspcAvail::Value3),
            _ => None,
        }
    }
    #[doc = "Periodic Transmit Request Queue is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PtxQspcAvail::Value1
    }
    #[doc = "1 location available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PtxQspcAvail::Value2
    }
    #[doc = "2 locations available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PtxQspcAvail::Value3
    }
}
#[doc = "Field `PTxQTop` reader - Top of the Periodic Transmit Request Queue"]
pub type PtxQtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&self) -> PtxFspcAvailR {
        PtxFspcAvailR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptx_qspc_avail(&self) -> PtxQspcAvailR {
        PtxQspcAvailR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptx_qtop(&self) -> PtxQtopR {
        PtxQtopR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_fspc_avail(&mut self) -> PtxFspcAvailW<HptxstsSpec> {
        PtxFspcAvailW::new(self, 0)
    }
}
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxstsSpec;
impl crate::RegisterSpec for HptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HptxstsSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxsts::W`](W) writer structure"]
impl crate::Writable for HptxstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0100;
}
