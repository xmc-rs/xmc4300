#[doc = "Register `CON` reader"]
pub struct R(crate::R<CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CON` writer"]
pub struct W(crate::W<CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECATRSTEN` reader - Enable EtherCAT Reset Request"]
pub type ECATRSTEN_R = crate::BitReader<ECATRSTEN_A>;
#[doc = "Enable EtherCAT Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECATRSTEN_A {
    #[doc = "0: Reset request by EtherCAT Master disabled"]
    VALUE1 = 0,
    #[doc = "1: Reset request by EtherCAT Master enabled"]
    VALUE2 = 1,
}
impl From<ECATRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ECATRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ECATRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECATRSTEN_A {
        match self.bits {
            false => ECATRSTEN_A::VALUE1,
            true => ECATRSTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ECATRSTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ECATRSTEN_A::VALUE2
    }
}
#[doc = "Field `ECATRSTEN` writer - Enable EtherCAT Reset Request"]
pub type ECATRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CON_SPEC, ECATRSTEN_A, O>;
impl<'a, const O: u8> ECATRSTEN_W<'a, O> {
    #[doc = "Reset request by EtherCAT Master disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ECATRSTEN_A::VALUE1)
    }
    #[doc = "Reset request by EtherCAT Master enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ECATRSTEN_A::VALUE2)
    }
}
#[doc = "Field `LATCHIN0SEL` reader - LATCHIN0 Input Select"]
pub type LATCHIN0SEL_R = crate::FieldReader<u8, LATCHIN0SEL_A>;
#[doc = "LATCHIN0 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATCHIN0SEL_A {
    #[doc = "0: Data input LATCHIN0A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input LATCHIN0B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input LATCHIN0C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input LATCHIN0D is selected"]
    VALUE4 = 3,
}
impl From<LATCHIN0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LATCHIN0SEL_A) -> Self {
        variant as _
    }
}
impl LATCHIN0SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCHIN0SEL_A {
        match self.bits {
            0 => LATCHIN0SEL_A::VALUE1,
            1 => LATCHIN0SEL_A::VALUE2,
            2 => LATCHIN0SEL_A::VALUE3,
            3 => LATCHIN0SEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN0SEL_A::VALUE4
    }
}
#[doc = "Field `LATCHIN0SEL` writer - LATCHIN0 Input Select"]
pub type LATCHIN0SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CON_SPEC, u8, LATCHIN0SEL_A, 2, O>;
impl<'a, const O: u8> LATCHIN0SEL_W<'a, O> {
    #[doc = "Data input LATCHIN0A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE1)
    }
    #[doc = "Data input LATCHIN0B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE2)
    }
    #[doc = "Data input LATCHIN0C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE3)
    }
    #[doc = "Data input LATCHIN0D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN0SEL_A::VALUE4)
    }
}
#[doc = "Field `LATCHIN0` reader - EtherCAT LATCH_IN0 Input Signal"]
pub type LATCHIN0_R = crate::BitReader<bool>;
#[doc = "Field `LATCHIN1SEL` reader - LATCHIN1 Input Select"]
pub type LATCHIN1SEL_R = crate::FieldReader<u8, LATCHIN1SEL_A>;
#[doc = "LATCHIN1 Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATCHIN1SEL_A {
    #[doc = "0: Data input LATCHIN1A is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input LATCHIN1B is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input LATCHIN1C is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input LATCHIN1D is selected"]
    VALUE4 = 3,
}
impl From<LATCHIN1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LATCHIN1SEL_A) -> Self {
        variant as _
    }
}
impl LATCHIN1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LATCHIN1SEL_A {
        match self.bits {
            0 => LATCHIN1SEL_A::VALUE1,
            1 => LATCHIN1SEL_A::VALUE2,
            2 => LATCHIN1SEL_A::VALUE3,
            3 => LATCHIN1SEL_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LATCHIN1SEL_A::VALUE4
    }
}
#[doc = "Field `LATCHIN1SEL` writer - LATCHIN1 Input Select"]
pub type LATCHIN1SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CON_SPEC, u8, LATCHIN1SEL_A, 2, O>;
impl<'a, const O: u8> LATCHIN1SEL_W<'a, O> {
    #[doc = "Data input LATCHIN1A is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE1)
    }
    #[doc = "Data input LATCHIN1B is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE2)
    }
    #[doc = "Data input LATCHIN1C is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE3)
    }
    #[doc = "Data input LATCHIN1D is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(LATCHIN1SEL_A::VALUE4)
    }
}
#[doc = "Field `LATCHIN1` reader - EtherCAT LATCH_IN1 Input Signal"]
pub type LATCHIN1_R = crate::BitReader<bool>;
#[doc = "Field `PHYOFFSET` reader - Ethernet PHY Address Offset"]
pub type PHYOFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHYOFFSET` writer - Ethernet PHY Address Offset"]
pub type PHYOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CON_SPEC, u8, u8, 5, O>;
#[doc = "Field `MDIO` reader - MDIO Input Select"]
pub type MDIO_R = crate::FieldReader<u8, MDIO_A>;
#[doc = "MDIO Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDIO_A {
    #[doc = "0: Data input MDIA is selected"]
    VALUE1 = 0,
    #[doc = "1: Data input MDIB is selected"]
    VALUE2 = 1,
    #[doc = "2: Data input MDIC is selected"]
    VALUE3 = 2,
    #[doc = "3: Data input MDID is selected"]
    VALUE4 = 3,
}
impl From<MDIO_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIO_A) -> Self {
        variant as _
    }
}
impl MDIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIO_A {
        match self.bits {
            0 => MDIO_A::VALUE1,
            1 => MDIO_A::VALUE2,
            2 => MDIO_A::VALUE3,
            3 => MDIO_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MDIO_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MDIO_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == MDIO_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == MDIO_A::VALUE4
    }
}
#[doc = "Field `MDIO` writer - MDIO Input Select"]
pub type MDIO_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CON_SPEC, u8, MDIO_A, 2, O>;
impl<'a, const O: u8> MDIO_W<'a, O> {
    #[doc = "Data input MDIA is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(MDIO_A::VALUE4)
    }
}
impl R {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    pub fn ecatrsten(&self) -> ECATRSTEN_R {
        ECATRSTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    pub fn latchin0sel(&self) -> LATCHIN0SEL_R {
        LATCHIN0SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - EtherCAT LATCH_IN0 Input Signal"]
    #[inline(always)]
    pub fn latchin0(&self) -> LATCHIN0_R {
        LATCHIN0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    pub fn latchin1sel(&self) -> LATCHIN1SEL_R {
        LATCHIN1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - EtherCAT LATCH_IN1 Input Signal"]
    #[inline(always)]
    pub fn latchin1(&self) -> LATCHIN1_R {
        LATCHIN1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    pub fn phyoffset(&self) -> PHYOFFSET_R {
        PHYOFFSET_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable EtherCAT Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn ecatrsten(&mut self) -> ECATRSTEN_W<0> {
        ECATRSTEN_W::new(self)
    }
    #[doc = "Bits 8:9 - LATCHIN0 Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn latchin0sel(&mut self) -> LATCHIN0SEL_W<8> {
        LATCHIN0SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - LATCHIN1 Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn latchin1sel(&mut self) -> LATCHIN1SEL_W<12> {
        LATCHIN1SEL_W::new(self)
    }
    #[doc = "Bits 16:20 - Ethernet PHY Address Offset"]
    #[inline(always)]
    #[must_use]
    pub fn phyoffset(&mut self) -> PHYOFFSET_W<16> {
        PHYOFFSET_W::new(self)
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn mdio(&mut self) -> MDIO_W<22> {
        MDIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EtherCAT 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](index.html) module"]
pub struct CON_SPEC;
impl crate::RegisterSpec for CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [con::R](R) reader structure"]
impl crate::Readable for CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [con::W](W) writer structure"]
impl crate::Writable for CON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
