#[doc = "Register `INPR` reader"]
pub type R = crate::R<InprSpec>;
#[doc = "Register `INPR` writer"]
pub type W = crate::W<InprSpec>;
#[doc = "Transmit Shift Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsinp {
    #[doc = "0: Output SR0 becomes activated."]
    Value1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    Value2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    Value3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    Value4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    Value5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    Value6 = 5,
}
impl From<Tsinp> for u8 {
    #[inline(always)]
    fn from(variant: Tsinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsinp {
    type Ux = u8;
}
#[doc = "Field `TSINP` reader - Transmit Shift Interrupt Node Pointer"]
pub type TsinpR = crate::FieldReader<Tsinp>;
impl TsinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsinp> {
        match self.bits {
            0 => Some(Tsinp::Value1),
            1 => Some(Tsinp::Value2),
            2 => Some(Tsinp::Value3),
            3 => Some(Tsinp::Value4),
            4 => Some(Tsinp::Value5),
            5 => Some(Tsinp::Value6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsinp::Value1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsinp::Value2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tsinp::Value3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tsinp::Value4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Tsinp::Value5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Tsinp::Value6
    }
}
#[doc = "Field `TSINP` writer - Transmit Shift Interrupt Node Pointer"]
pub type TsinpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsinp>;
impl<'a, REG> TsinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Tsinp::Value6)
    }
}
#[doc = "Field `TBINP` reader - Transmit Buffer Interrupt Node Pointer"]
pub type TbinpR = crate::FieldReader;
#[doc = "Field `TBINP` writer - Transmit Buffer Interrupt Node Pointer"]
pub type TbinpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RINP` reader - Receive Interrupt Node Pointer"]
pub type RinpR = crate::FieldReader;
#[doc = "Field `RINP` writer - Receive Interrupt Node Pointer"]
pub type RinpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AINP` reader - Alternative Receive Interrupt Node Pointer"]
pub type AinpR = crate::FieldReader;
#[doc = "Field `AINP` writer - Alternative Receive Interrupt Node Pointer"]
pub type AinpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PINP` reader - Transmit Shift Interrupt Node Pointer"]
pub type PinpR = crate::FieldReader;
#[doc = "Field `PINP` writer - Transmit Shift Interrupt Node Pointer"]
pub type PinpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tsinp(&self) -> TsinpR {
        TsinpR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn tbinp(&self) -> TbinpR {
        TbinpR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn rinp(&self) -> RinpR {
        RinpR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    pub fn ainp(&self) -> AinpR {
        AinpR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    pub fn pinp(&self) -> PinpR {
        PinpR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tsinp(&mut self) -> TsinpW<InprSpec> {
        TsinpW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn tbinp(&mut self) -> TbinpW<InprSpec> {
        TbinpW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn rinp(&mut self) -> RinpW<InprSpec> {
        RinpW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Alternative Receive Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn ainp(&mut self) -> AinpW<InprSpec> {
        AinpW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Transmit Shift Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn pinp(&mut self) -> PinpW<InprSpec> {
        PinpW::new(self, 16)
    }
}
#[doc = "Interrupt Node Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InprSpec;
impl crate::RegisterSpec for InprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inpr::R`](R) reader structure"]
impl crate::Readable for InprSpec {}
#[doc = "`write(|w| ..)` method takes [`inpr::W`](W) writer structure"]
impl crate::Writable for InprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INPR to value 0"]
impl crate::Resettable for InprSpec {
    const RESET_VALUE: u32 = 0;
}
