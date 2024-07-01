#[doc = "Register `NFCR` reader"]
pub type R = crate::R<NFCR_SPEC>;
#[doc = "Register `NFCR` writer"]
pub type W = crate::W<NFCR_SPEC>;
#[doc = "Field `CFC` reader - CAN Frame Counter"]
pub type CFC_R = crate::FieldReader<u16>;
#[doc = "Field `CFC` writer - CAN Frame Counter"]
pub type CFC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CFSEL` reader - CAN Frame Count Selection"]
pub type CFSEL_R = crate::FieldReader;
#[doc = "Field `CFSEL` writer - CAN Frame Count Selection"]
pub type CFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "CAN Frame Counter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFMOD_A {
    #[doc = "0: Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    VALUE1 = 0,
    #[doc = "1: Time Stamp Mode: The frame counter is used to count bit times."]
    VALUE2 = 1,
    #[doc = "2: Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    VALUE3 = 2,
    #[doc = "3: Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    VALUE4 = 3,
}
impl From<CFMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CFMOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CFMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for CFMOD_A {}
#[doc = "Field `CFMOD` reader - CAN Frame Counter Mode"]
pub type CFMOD_R = crate::FieldReader<CFMOD_A>;
impl CFMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFMOD_A {
        match self.bits {
            0 => CFMOD_A::VALUE1,
            1 => CFMOD_A::VALUE2,
            2 => CFMOD_A::VALUE3,
            3 => CFMOD_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFMOD_A::VALUE1
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFMOD_A::VALUE2
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CFMOD_A::VALUE3
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CFMOD_A::VALUE4
    }
}
#[doc = "Field `CFMOD` writer - CAN Frame Counter Mode"]
pub type CFMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CFMOD_A, crate::Safe>;
impl<'a, REG> CFMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frame Count Mode: The frame counter is incremented upon the reception and transmission of frames."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFMOD_A::VALUE1)
    }
    #[doc = "Time Stamp Mode: The frame counter is used to count bit times."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFMOD_A::VALUE2)
    }
    #[doc = "Bit Timing Mode: The frame counter is used for analysis of the bit timing."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CFMOD_A::VALUE3)
    }
    #[doc = "Error Count Mode: The frame counter is used for counting when an error frame is received or an error is detected by the node."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CFMOD_A::VALUE4)
    }
}
#[doc = "CAN Frame Count Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCIE_A {
    #[doc = "0: CAN frame counter overflow interrupt is disabled."]
    VALUE1 = 0,
    #[doc = "1: CAN frame counter overflow interrupt is enabled."]
    VALUE2 = 1,
}
impl From<CFCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CFCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCIE` reader - CAN Frame Count Interrupt Enable"]
pub type CFCIE_R = crate::BitReader<CFCIE_A>;
impl CFCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFCIE_A {
        match self.bits {
            false => CFCIE_A::VALUE1,
            true => CFCIE_A::VALUE2,
        }
    }
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCIE_A::VALUE1
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCIE_A::VALUE2
    }
}
#[doc = "Field `CFCIE` writer - CAN Frame Count Interrupt Enable"]
pub type CFCIE_W<'a, REG> = crate::BitWriter<'a, REG, CFCIE_A>;
impl<'a, REG> CFCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN frame counter overflow interrupt is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFCIE_A::VALUE1)
    }
    #[doc = "CAN frame counter overflow interrupt is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFCIE_A::VALUE2)
    }
}
#[doc = "CAN Frame Counter Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFCOV_A {
    #[doc = "0: No overflow has occurred since last flag reset."]
    VALUE1 = 0,
    #[doc = "1: An overflow has occurred since last flag reset."]
    VALUE2 = 1,
}
impl From<CFCOV_A> for bool {
    #[inline(always)]
    fn from(variant: CFCOV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFCOV` reader - CAN Frame Counter Overflow Flag"]
pub type CFCOV_R = crate::BitReader<CFCOV_A>;
impl CFCOV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFCOV_A {
        match self.bits {
            false => CFCOV_A::VALUE1,
            true => CFCOV_A::VALUE2,
        }
    }
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFCOV_A::VALUE1
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFCOV_A::VALUE2
    }
}
#[doc = "Field `CFCOV` writer - CAN Frame Counter Overflow Flag"]
pub type CFCOV_W<'a, REG> = crate::BitWriter<'a, REG, CFCOV_A>;
impl<'a, REG> CFCOV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CFCOV_A::VALUE1)
    }
    #[doc = "An overflow has occurred since last flag reset."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CFCOV_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    pub fn cfc(&self) -> CFC_R {
        CFC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    pub fn cfsel(&self) -> CFSEL_R {
        CFSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    pub fn cfmod(&self) -> CFMOD_R {
        CFMOD_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    pub fn cfcie(&self) -> CFCIE_R {
        CFCIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    pub fn cfcov(&self) -> CFCOV_R {
        CFCOV_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CAN Frame Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cfc(&mut self) -> CFC_W<NFCR_SPEC> {
        CFC_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - CAN Frame Count Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cfsel(&mut self) -> CFSEL_W<NFCR_SPEC> {
        CFSEL_W::new(self, 16)
    }
    #[doc = "Bits 19:20 - CAN Frame Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cfmod(&mut self) -> CFMOD_W<NFCR_SPEC> {
        CFMOD_W::new(self, 19)
    }
    #[doc = "Bit 22 - CAN Frame Count Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfcie(&mut self) -> CFCIE_W<NFCR_SPEC> {
        CFCIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - CAN Frame Counter Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfcov(&mut self) -> CFCOV_W<NFCR_SPEC> {
        CFCOV_W::new(self, 23)
    }
}
#[doc = "Node Frame Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NFCR_SPEC;
impl crate::RegisterSpec for NFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nfcr::R`](R) reader structure"]
impl crate::Readable for NFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nfcr::W`](W) writer structure"]
impl crate::Writable for NFCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NFCR to value 0"]
impl crate::Resettable for NFCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
