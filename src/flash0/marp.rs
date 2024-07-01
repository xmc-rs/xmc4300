#[doc = "Register `MARP` reader"]
pub type R = crate::R<MARP_SPEC>;
#[doc = "Register `MARP` writer"]
pub type W = crate::W<MARP_SPEC>;
#[doc = "PFLASH Margin Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MARGIN_A {
    #[doc = "0: Standard (default) margin."]
    VALUE1 = 0,
    #[doc = "1: Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    VALUE2 = 1,
    #[doc = "4: Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    VALUE3 = 4,
}
impl From<MARGIN_A> for u8 {
    #[inline(always)]
    fn from(variant: MARGIN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MARGIN_A {
    type Ux = u8;
}
impl crate::IsEnum for MARGIN_A {}
#[doc = "Field `MARGIN` reader - PFLASH Margin Selection"]
pub type MARGIN_R = crate::FieldReader<MARGIN_A>;
impl MARGIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MARGIN_A> {
        match self.bits {
            0 => Some(MARGIN_A::VALUE1),
            1 => Some(MARGIN_A::VALUE2),
            4 => Some(MARGIN_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MARGIN_A::VALUE1
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MARGIN_A::VALUE2
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MARGIN_A::VALUE3
    }
}
#[doc = "Field `MARGIN` writer - PFLASH Margin Selection"]
pub type MARGIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MARGIN_A>;
impl<'a, REG> MARGIN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MARGIN_A::VALUE1)
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MARGIN_A::VALUE2)
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MARGIN_A::VALUE3)
    }
}
#[doc = "PFLASH Double-Bit Error Trap Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRAPDIS_A {
    #[doc = "0: If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    VALUE1 = 0,
    #[doc = "1: The double-bit error trap is disabled. Shall be used only during margin check"]
    VALUE2 = 1,
}
impl From<TRAPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TRAPDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRAPDIS` reader - PFLASH Double-Bit Error Trap Disable"]
pub type TRAPDIS_R = crate::BitReader<TRAPDIS_A>;
impl TRAPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRAPDIS_A {
        match self.bits {
            false => TRAPDIS_A::VALUE1,
            true => TRAPDIS_A::VALUE2,
        }
    }
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRAPDIS_A::VALUE1
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRAPDIS_A::VALUE2
    }
}
#[doc = "Field `TRAPDIS` writer - PFLASH Double-Bit Error Trap Disable"]
pub type TRAPDIS_W<'a, REG> = crate::BitWriter<'a, REG, TRAPDIS_A>;
impl<'a, REG> TRAPDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRAPDIS_A::VALUE1)
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRAPDIS_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&self) -> MARGIN_R {
        MARGIN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&self) -> TRAPDIS_R {
        TRAPDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    #[must_use]
    pub fn margin(&mut self) -> MARGIN_W<MARP_SPEC> {
        MARGIN_W::new(self, 0)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trapdis(&mut self) -> TRAPDIS_W<MARP_SPEC> {
        TRAPDIS_W::new(self, 15)
    }
}
#[doc = "Margin Control Register PFLASH\n\nYou can [`read`](crate::Reg::read) this register and get [`marp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`marp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MARP_SPEC;
impl crate::RegisterSpec for MARP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`marp::R`](R) reader structure"]
impl crate::Readable for MARP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`marp::W`](W) writer structure"]
impl crate::Writable for MARP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MARP to value 0"]
impl crate::Resettable for MARP_SPEC {
    const RESET_VALUE: u32 = 0;
}
