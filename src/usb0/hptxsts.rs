#[doc = "Register `HPTXSTS` reader"]
pub type R = crate::R<HPTXSTS_SPEC>;
#[doc = "Register `HPTXSTS` writer"]
pub type W = crate::W<HPTXSTS_SPEC>;
#[doc = "Periodic Transmit Data FIFO Space Available\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PTX_FSPC_AVAIL_A {
    #[doc = "0: Periodic TxFIFO is full"]
    VALUE1 = 0,
    #[doc = "1: 1 word available"]
    VALUE2 = 1,
    #[doc = "2: 2 words available"]
    VALUE3 = 2,
}
impl From<PTX_FSPC_AVAIL_A> for u16 {
    #[inline(always)]
    fn from(variant: PTX_FSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTX_FSPC_AVAIL_A {
    type Ux = u16;
}
impl crate::IsEnum for PTX_FSPC_AVAIL_A {}
#[doc = "Field `PTxFSpcAvail` reader - Periodic Transmit Data FIFO Space Available"]
pub type PTX_FSPC_AVAIL_R = crate::FieldReader<PTX_FSPC_AVAIL_A>;
impl PTX_FSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTX_FSPC_AVAIL_A> {
        match self.bits {
            0 => Some(PTX_FSPC_AVAIL_A::VALUE1),
            1 => Some(PTX_FSPC_AVAIL_A::VALUE2),
            2 => Some(PTX_FSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE1
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE2
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTX_FSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `PTxFSpcAvail` writer - Periodic Transmit Data FIFO Space Available"]
pub type PTX_FSPC_AVAIL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PTX_FSPC_AVAIL_A>;
impl<'a, REG> PTX_FSPC_AVAIL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Periodic TxFIFO is full"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PTX_FSPC_AVAIL_A::VALUE1)
    }
    #[doc = "1 word available"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PTX_FSPC_AVAIL_A::VALUE2)
    }
    #[doc = "2 words available"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(PTX_FSPC_AVAIL_A::VALUE3)
    }
}
#[doc = "Periodic Transmit Request Queue Space Available\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTX_QSPC_AVAIL_A {
    #[doc = "0: Periodic Transmit Request Queue is full"]
    VALUE1 = 0,
    #[doc = "1: 1 location available"]
    VALUE2 = 1,
    #[doc = "2: 2 locations available"]
    VALUE3 = 2,
}
impl From<PTX_QSPC_AVAIL_A> for u8 {
    #[inline(always)]
    fn from(variant: PTX_QSPC_AVAIL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PTX_QSPC_AVAIL_A {
    type Ux = u8;
}
impl crate::IsEnum for PTX_QSPC_AVAIL_A {}
#[doc = "Field `PTxQSpcAvail` reader - Periodic Transmit Request Queue Space Available"]
pub type PTX_QSPC_AVAIL_R = crate::FieldReader<PTX_QSPC_AVAIL_A>;
impl PTX_QSPC_AVAIL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PTX_QSPC_AVAIL_A> {
        match self.bits {
            0 => Some(PTX_QSPC_AVAIL_A::VALUE1),
            1 => Some(PTX_QSPC_AVAIL_A::VALUE2),
            2 => Some(PTX_QSPC_AVAIL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Periodic Transmit Request Queue is full"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE1
    }
    #[doc = "1 location available"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE2
    }
    #[doc = "2 locations available"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PTX_QSPC_AVAIL_A::VALUE3
    }
}
#[doc = "Field `PTxQTop` reader - Top of the Periodic Transmit Request Queue"]
pub type PTX_QTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&self) -> PTX_FSPC_AVAIL_R {
        PTX_FSPC_AVAIL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic Transmit Request Queue Space Available"]
    #[inline(always)]
    pub fn ptx_qspc_avail(&self) -> PTX_QSPC_AVAIL_R {
        PTX_QSPC_AVAIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the Periodic Transmit Request Queue"]
    #[inline(always)]
    pub fn ptx_qtop(&self) -> PTX_QTOP_R {
        PTX_QTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Periodic Transmit Data FIFO Space Available"]
    #[inline(always)]
    pub fn ptx_fspc_avail(&mut self) -> PTX_FSPC_AVAIL_W<HPTXSTS_SPEC> {
        PTX_FSPC_AVAIL_W::new(self, 0)
    }
}
#[doc = "Host Periodic Transmit FIFO/ Queue Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxsts::R`](R) reader structure"]
impl crate::Readable for HPTXSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxsts::W`](W) writer structure"]
impl crate::Writable for HPTXSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXSTS to value 0x0008_0100"]
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0008_0100;
}
