#[doc = "Register `FDR` reader"]
pub type R = crate::R<FDR_SPEC>;
#[doc = "Register `FDR` writer"]
pub type W = crate::W<FDR_SPEC>;
#[doc = "Field `STEP` reader - Step Value"]
pub type STEP_R = crate::FieldReader<u16>;
#[doc = "Field `STEP` writer - Step Value"]
pub type STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Divider Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DM_A {
    #[doc = "0: The divider is switched off, fFD = 0."]
    VALUE1 = 0,
    #[doc = "1: Normal divider mode selected."]
    VALUE2 = 1,
    #[doc = "2: Fractional divider mode selected."]
    VALUE3 = 2,
    #[doc = "3: The divider is switched off, fFD = 0."]
    VALUE4 = 3,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DM_A {
    type Ux = u8;
}
impl crate::IsEnum for DM_A {}
#[doc = "Field `DM` reader - Divider Mode"]
pub type DM_R = crate::FieldReader<DM_A>;
impl DM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DM_A {
        match self.bits {
            0 => DM_A::VALUE1,
            1 => DM_A::VALUE2,
            2 => DM_A::VALUE3,
            3 => DM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DM_A::VALUE1
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DM_A::VALUE2
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == DM_A::VALUE3
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == DM_A::VALUE4
    }
}
#[doc = "Field `DM` writer - Divider Mode"]
pub type DM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DM_A, crate::Safe>;
impl<'a, REG> DM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::VALUE1)
    }
    #[doc = "Normal divider mode selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::VALUE2)
    }
    #[doc = "Fractional divider mode selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::VALUE3)
    }
    #[doc = "The divider is switched off, fFD = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(DM_A::VALUE4)
    }
}
#[doc = "Field `RESULT` reader - Result Value"]
pub type RESULT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Step Value"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<FDR_SPEC> {
        STEP_W::new(self, 0)
    }
    #[doc = "Bits 14:15 - Divider Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DM_W<FDR_SPEC> {
        DM_W::new(self, 14)
    }
}
#[doc = "Fractional Divider Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr::R`](R) reader structure"]
impl crate::Readable for FDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fdr::W`](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDR to value 0"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
