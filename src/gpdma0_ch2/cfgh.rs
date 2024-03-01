#[doc = "Register `CFGH` reader"]
pub type R = crate::R<CfghSpec>;
#[doc = "Register `CFGH` writer"]
pub type W = crate::W<CfghSpec>;
#[doc = "Flow Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcmode {
    #[doc = "0: Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    Value1 = 0,
    #[doc = "1: Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    Value2 = 1,
}
impl From<Fcmode> for bool {
    #[inline(always)]
    fn from(variant: Fcmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCMODE` reader - Flow Control Mode"]
pub type FcmodeR = crate::BitReader<Fcmode>;
impl FcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcmode {
        match self.bits {
            false => Fcmode::Value1,
            true => Fcmode::Value2,
        }
    }
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fcmode::Value1
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fcmode::Value2
    }
}
#[doc = "Field `FCMODE` writer - Flow Control Mode"]
pub type FcmodeW<'a, REG> = crate::BitWriter<'a, REG, Fcmode>;
impl<'a, REG> FcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Source transaction requests are serviced when they occur. Data pre-fetching is enabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmode::Value1)
    }
    #[doc = "Source transaction requests are not serviced until a destination transaction request occurs. In this mode, the amount of data transferred from the source is limited so that it is guaranteed to be transferred to the destination prior to block termination by the destination. Data pre-fetching is disabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fcmode::Value2)
    }
}
#[doc = "FIFO Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoMode {
    #[doc = "0: Space/data available for single AHB transfer of the specified transfer width."]
    Value1 = 0,
    #[doc = "1: Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    Value2 = 1,
}
impl From<FifoMode> for bool {
    #[inline(always)]
    fn from(variant: FifoMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_MODE` reader - FIFO Mode Select"]
pub type FifoModeR = crate::BitReader<FifoMode>;
impl FifoModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifoMode {
        match self.bits {
            false => FifoMode::Value1,
            true => FifoMode::Value2,
        }
    }
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FifoMode::Value1
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FifoMode::Value2
    }
}
#[doc = "Field `FIFO_MODE` writer - FIFO Mode Select"]
pub type FifoModeW<'a, REG> = crate::BitWriter<'a, REG, FifoMode>;
impl<'a, REG> FifoModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Space/data available for single AHB transfer of the specified transfer width."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FifoMode::Value1)
    }
    #[doc = "Data available is greater than or equal to half the FIFO depth for destination transfers and space available is greater than half the fifo depth for source transfers. The exceptions are at the end of a burst transaction request or at the end of a block transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FifoMode::Value2)
    }
}
#[doc = "Field `PROTCTL` reader - Protection Control"]
pub type ProtctlR = crate::FieldReader;
#[doc = "Field `PROTCTL` writer - Protection Control"]
pub type ProtctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SRC_PER` reader - Source Peripheral"]
pub type SrcPerR = crate::FieldReader;
#[doc = "Field `SRC_PER` writer - Source Peripheral"]
pub type SrcPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEST_PER` reader - Destination Peripheral"]
pub type DestPerR = crate::FieldReader;
#[doc = "Field `DEST_PER` writer - Destination Peripheral"]
pub type DestPerW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    pub fn fcmode(&self) -> FcmodeR {
        FcmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    pub fn protctl(&self) -> ProtctlR {
        ProtctlR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    pub fn src_per(&self) -> SrcPerR {
        SrcPerR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    pub fn dest_per(&self) -> DestPerR {
        DestPerR::new(((self.bits >> 11) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fcmode(&mut self) -> FcmodeW<CfghSpec> {
        FcmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_mode(&mut self) -> FifoModeW<CfghSpec> {
        FifoModeW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Protection Control"]
    #[inline(always)]
    #[must_use]
    pub fn protctl(&mut self) -> ProtctlW<CfghSpec> {
        ProtctlW::new(self, 2)
    }
    #[doc = "Bits 7:10 - Source Peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn src_per(&mut self) -> SrcPerW<CfghSpec> {
        SrcPerW::new(self, 7)
    }
    #[doc = "Bits 11:14 - Destination Peripheral"]
    #[inline(always)]
    #[must_use]
    pub fn dest_per(&mut self) -> DestPerW<CfghSpec> {
        DestPerW::new(self, 11)
    }
}
#[doc = "Configuration Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfghSpec;
impl crate::RegisterSpec for CfghSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgh::R`](R) reader structure"]
impl crate::Readable for CfghSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgh::W`](W) writer structure"]
impl crate::Writable for CfghSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGH to value 0x04"]
impl crate::Resettable for CfghSpec {
    const RESET_VALUE: u32 = 0x04;
}
