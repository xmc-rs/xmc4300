#[doc = "Register `NFCR` reader"]
pub type R = crate::R<NfcrSpec>;
#[doc = "Register `NFCR` writer"]
pub type W = crate::W<NfcrSpec>;
#[doc = "Field `CFC` reader - CAN Frame Counter"]
pub type CfcR = crate::FieldReader<u16>;
#[doc = "Field `CFC` writer - CAN Frame Counter"]
pub type CfcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CFSEL` reader - CAN Frame Count Selection"]
pub type CfselR = crate::FieldReader;
#[doc = "Field `CFSEL` writer - CAN Frame Count Selection"]
pub type CfselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "CAN Frame Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfmod {
    #[doc = "0: Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    Value1 = 0,
    #[doc = "1: Time Stamp Mode: The frame counter is used to count bit times."]
    Value2 = 1,
    #[doc = "2: Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    Value3 = 2,
    #[doc = "3: Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    Value4 = 3,
}
impl From<Cfmod> for u8 {
    #[inline(always)]
    fn from(variant: Cfmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfmod {
    type Ux = u8;
}
impl crate::IsEnum for Cfmod {}
#[doc = "Field `CFMOD` reader - CAN Frame Counter Mode"]
pub type CfmodR = crate::FieldReader<Cfmod>;
impl CfmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfmod {
        match self.bits {
            0 => Cfmod::Value1,
            1 => Cfmod::Value2,
            2 => Cfmod::Value3,
            3 => Cfmod::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfmod::Value1
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfmod::Value2
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Cfmod::Value3
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Cfmod::Value4
    }
}
#[doc = "Field `CFMOD` writer - CAN Frame Counter Mode"]
pub type CfmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cfmod, crate::Safe>;
impl<'a, REG> CfmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmod::Value1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmod::Value2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmod::Value3)
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfmod::Value4)
    }
}
#[doc = "CAN Frame Count Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfcie {
    #[doc = "0: CAN frame counter overflow interrupt is disabled."]
    Value1 = 0,
    #[doc = "1: CAN frame counter overflow interrupt is enabled."]
    Value2 = 1,
}
impl From<Cfcie> for bool {
    #[inline(always)]
    fn from(variant: Cfcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCIE` reader - CAN Frame Count Interrupt Enable"]
pub type CfcieR = crate::BitReader<Cfcie>;
impl CfcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfcie {
        match self.bits {
            false => Cfcie::Value1,
            true => Cfcie::Value2,
        }
    }
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfcie::Value1
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfcie::Value2
    }
}
#[doc = "Field `CFCIE` writer - CAN Frame Count Interrupt Enable"]
pub type CfcieW<'a, REG> = crate::BitWriter<'a, REG, Cfcie>;
impl<'a, REG> CfcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcie::Value1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcie::Value2)
    }
}
#[doc = "CAN Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfcov {
    #[doc = "0: No overflow has occurred since last flag reset."]
    Value1 = 0,
    #[doc = "1: An overflow has occurred since last flag reset."]
    Value2 = 1,
}
impl From<Cfcov> for bool {
    #[inline(always)]
    fn from(variant: Cfcov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCOV` reader - CAN Frame Counter Overflow Flag"]
pub type CfcovR = crate::BitReader<Cfcov>;
impl CfcovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfcov {
        match self.bits {
            false => Cfcov::Value1,
            true => Cfcov::Value2,
        }
    }
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfcov::Value1
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfcov::Value2
    }
}
#[doc = "Field `CFCOV` writer - CAN Frame Counter Overflow Flag"]
pub type CfcovW<'a, REG> = crate::BitWriter<'a, REG, Cfcov>;
impl<'a, REG> CfcovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcov::Value1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfcov::Value2)
    }
}
impl R {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CfcR {
        CfcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CfselR {
        CfselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CfmodR {
        CfmodR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CfcieR {
        CfcieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CfcovR {
        CfcovR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cfc(&mut self) -> CfcW<NfcrSpec> {
        CfcW::new(self, 0)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cfsel(&mut self) -> CfselW<NfcrSpec> {
        CfselW::new(self, 16)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfmod(&mut self) -> CfmodW<NfcrSpec> {
        CfmodW::new(self, 19)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfcie(&mut self) -> CfcieW<NfcrSpec> {
        CfcieW::new(self, 22)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfcov(&mut self) -> CfcovW<NfcrSpec> {
        CfcovW::new(self, 23)
    }
}
#[doc = "Node Frame Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NfcrSpec;
impl crate::RegisterSpec for NfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcr::R`](R) reader structure"]
impl crate::Readable for NfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`nfcr::W`](W) writer structure"]
impl crate::Writable for NfcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NfcrSpec {
    const RESET_VALUE: u32 = 0;
}
