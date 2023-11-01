#[doc = "Register `INPR` reader"]
pub type R = crate::R<INPR_SPEC>;
#[doc = "Register `INPR` writer"]
pub type W = crate::W<INPR_SPEC>;
#[doc = "Field `TSINP` reader - Transmit Shift Interrupt Node Pointer"]
pub type TSINP_R = crate::FieldReader<TSINP_A>;
#[doc = "Transmit Shift Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6 = 5,
}
impl From<TSINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TSINP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSINP_A {
    type Ux = u8;
}
impl TSINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSINP_A> {
        match self.bits {
            0 => Some(TSINP_A::VALUE1),
            1 => Some(TSINP_A::VALUE2),
            2 => Some(TSINP_A::VALUE3),
            3 => Some(TSINP_A::VALUE4),
            4 => Some(TSINP_A::VALUE5),
            5 => Some(TSINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSINP_A::VALUE1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSINP_A::VALUE2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSINP_A::VALUE3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSINP_A::VALUE4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TSINP_A::VALUE5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == TSINP_A::VALUE6
    }
}
#[doc = "Field `TSINP` writer - Transmit Shift Interrupt Node Pointer"]
pub type TSINP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, TSINP_A>;
impl<'a, REG, const O: u8> TSINP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(TSINP_A::VALUE6)
    }
}
#[doc = "Field `TBINP` reader - Transmit Buffer Interrupt Node Pointer"]
pub type TBINP_R = crate::FieldReader;
#[doc = "Field `TBINP` writer - Transmit Buffer Interrupt Node Pointer"]
pub type TBINP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `RINP` reader - Receive Interrupt Node Pointer"]
pub type RINP_R = crate::FieldReader;
#[doc = "Field `RINP` writer - Receive Interrupt Node Pointer"]
pub type RINP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `AINP` reader - Alternative Receive Interrupt Node Pointer"]
pub type AINP_R = crate::FieldReader;
#[doc = "Field `AINP` writer - Alternative Receive Interrupt Node Pointer"]
pub type AINP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PINP` reader - Transmit Shift Interrupt Node Pointer"]
pub type PINP_R = crate::FieldReader;
#[doc = "Field `PINP` writer - Transmit Shift Interrupt Node Pointer"]
pub type PINP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tsinp(&self) -> TSINP_R {
        TSINP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tbinp(&self) -> TBINP_R {
        TBINP_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rinp(&self) -> RINP_R {
        RINP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn ainp(&self) -> AINP_R {
        AINP_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn pinp(&self) -> PINP_R {
        PINP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tsinp(&mut self) -> TSINP_W<INPR_SPEC, 0> {
        TSINP_W::new(self)
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tbinp(&mut self) -> TBINP_W<INPR_SPEC, 4> {
        TBINP_W::new(self)
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rinp(&mut self) -> RINP_W<INPR_SPEC, 8> {
        RINP_W::new(self)
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn ainp(&mut self) -> AINP_W<INPR_SPEC, 12> {
        AINP_W::new(self)
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn pinp(&mut self) -> PINP_W<INPR_SPEC, 16> {
        PINP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INPR_SPEC;
impl crate::RegisterSpec for INPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inpr::R`](R) reader structure"]
impl crate::Readable for INPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inpr::W`](W) writer structure"]
impl crate::Writable for INPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPR to value 0"]
impl crate::Resettable for INPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
