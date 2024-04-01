#[doc = "Register `MARP` reader"]
pub type R = crate::R<MarpSpec>;
#[doc = "Register `MARP` writer"]
pub type W = crate::W<MarpSpec>;
#[doc = "PFLASH Margin Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Margin {
    #[doc = "0: Standard (default) margin."]
    Value1 = 0,
    #[doc = "1: Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    Value2 = 1,
    #[doc = "4: Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    Value3 = 4,
}
impl From<Margin> for u8 {
    #[inline(always)]
    fn from(variant: Margin) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Margin {
    type Ux = u8;
}
impl crate::IsEnum for Margin {}
#[doc = "Field `MARGIN` reader - PFLASH Margin Selection"]
pub type MarginR = crate::FieldReader<Margin>;
impl MarginR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Margin> {
        match self.bits {
            0 => Some(Margin::Value1),
            1 => Some(Margin::Value2),
            4 => Some(Margin::Value3),
            _ => None,
        }
    }
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Margin::Value1
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Margin::Value2
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Margin::Value3
    }
}
#[doc = "Field `MARGIN` writer - PFLASH Margin Selection"]
pub type MarginW<'a, REG> = crate::FieldWriter<'a, REG, 4, Margin>;
impl<'a, REG> MarginW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard (default) margin."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Margin::Value1)
    }
    #[doc = "Tight margin for 0 (low) level. Suboptimal 0-bits are read as 1s."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Margin::Value2)
    }
    #[doc = "Tight margin for 1 (high) level. Suboptimal 1-bits are read as 0s."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Margin::Value3)
    }
}
#[doc = "PFLASH Double-Bit Error Trap Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trapdis {
    #[doc = "0: If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    Value1 = 0,
    #[doc = "1: The double-bit error trap is disabled. Shall be used only during margin check"]
    Value2 = 1,
}
impl From<Trapdis> for bool {
    #[inline(always)]
    fn from(variant: Trapdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRAPDIS` reader - PFLASH Double-Bit Error Trap Disable"]
pub type TrapdisR = crate::BitReader<Trapdis>;
impl TrapdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trapdis {
        match self.bits {
            false => Trapdis::Value1,
            true => Trapdis::Value2,
        }
    }
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trapdis::Value1
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trapdis::Value2
    }
}
#[doc = "Field `TRAPDIS` writer - PFLASH Double-Bit Error Trap Disable"]
pub type TrapdisW<'a, REG> = crate::BitWriter<'a, REG, Trapdis>;
impl<'a, REG> TrapdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If a double-bit error occurs in PFLASH, a bus error trap is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Trapdis::Value1)
    }
    #[doc = "The double-bit error trap is disabled. Shall be used only during margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Trapdis::Value2)
    }
}
impl R {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    pub fn margin(&self) -> MarginR {
        MarginR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    pub fn trapdis(&self) -> TrapdisR {
        TrapdisR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PFLASH Margin Selection"]
    #[inline(always)]
    #[must_use]
    pub fn margin(&mut self) -> MarginW<MarpSpec> {
        MarginW::new(self, 0)
    }
    #[doc = "Bit 15 - PFLASH Double-Bit Error Trap Disable"]
    #[inline(always)]
    #[must_use]
    pub fn trapdis(&mut self) -> TrapdisW<MarpSpec> {
        TrapdisW::new(self, 15)
    }
}
#[doc = "Margin Control Register PFLASH\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`marp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`marp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MarpSpec;
impl crate::RegisterSpec for MarpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`marp::R`](R) reader structure"]
impl crate::Readable for MarpSpec {}
#[doc = "`write(|w| ..)` method takes [`marp::W`](W) writer structure"]
impl crate::Writable for MarpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MARP to value 0"]
impl crate::Resettable for MarpSpec {
    const RESET_VALUE: u32 = 0;
}
