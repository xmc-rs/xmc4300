#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Baud Rate Logic Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: No clock supplied"]
    Value1 = 0,
    #[doc = "1: fPERIPH"]
    Value2 = 1,
    #[doc = "2: fOHP"]
    Value3 = 2,
    #[doc = "4: hard wired to 0"]
    Value4 = 4,
    #[doc = "8: hard wired to 0"]
    Value5 = 8,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Baud Rate Logic Clock Select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clksel> {
        match self.bits {
            0 => Some(Clksel::Value1),
            1 => Some(Clksel::Value2),
            2 => Some(Clksel::Value3),
            4 => Some(Clksel::Value4),
            8 => Some(Clksel::Value5),
            _ => None,
        }
    }
    #[doc = "No clock supplied"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Clksel::Value1
    }
    #[doc = "fPERIPH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Clksel::Value2
    }
    #[doc = "fOHP"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Clksel::Value3
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Clksel::Value4
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Clksel::Value5
    }
}
#[doc = "Field `CLKSEL` writer - Baud Rate Logic Clock Select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock supplied"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value1)
    }
    #[doc = "fPERIPH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value2)
    }
    #[doc = "fOHP"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value3)
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value4)
    }
    #[doc = "hard wired to 0"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Value5)
    }
}
#[doc = "Field `MPSEL` reader - Message Pending Selector"]
pub type MpselR = crate::FieldReader;
#[doc = "Field `MPSEL` writer - Message Pending Selector"]
pub type MpselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    pub fn mpsel(&self) -> MpselR {
        MpselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Logic Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<McrSpec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bits 12:15 - Message Pending Selector"]
    #[inline(always)]
    #[must_use]
    pub fn mpsel(&mut self) -> MpselW<McrSpec> {
        MpselW::new(self, 12)
    }
}
#[doc = "Module Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
