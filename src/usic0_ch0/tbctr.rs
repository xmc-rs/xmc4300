#[doc = "Register `TBCTR` reader"]
pub struct R(crate::R<TBCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBCTR` writer"]
pub struct W(crate::W<TBCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBCTR_SPEC>;
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
impl From<crate::W<TBCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPTR` writer - Data Pointer"]
pub type DPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub type LIMIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBCTR_SPEC, u8, u8, 6, O>;
#[doc = "Field `STBTM` reader - Standard Transmit Buffer Trigger Mode"]
pub type STBTM_R = crate::BitReader<STBTM_A>;
#[doc = "Standard Transmit Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBTM_A {
    #[doc = "0: Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    VALUE1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    VALUE2 = 1,
}
impl From<STBTM_A> for bool {
    #[inline(always)]
    fn from(variant: STBTM_A) -> Self {
        variant as u8 != 0
    }
}
impl STBTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBTM_A {
        match self.bits {
            false => STBTM_A::VALUE1,
            true => STBTM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBTM_A::VALUE2
    }
}
#[doc = "Field `STBTM` writer - Standard Transmit Buffer Trigger Mode"]
pub type STBTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBCTR_SPEC, STBTM_A, O>;
impl<'a, const O: u8> STBTM_W<'a, O> {
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTM_A::VALUE2)
    }
}
#[doc = "Field `STBTEN` reader - Standard Transmit Buffer Trigger Enable"]
pub type STBTEN_R = crate::BitReader<STBTEN_A>;
#[doc = "Standard Transmit Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBTEN_A {
    #[doc = "0: The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    VALUE2 = 1,
}
impl From<STBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: STBTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STBTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBTEN_A {
        match self.bits {
            false => STBTEN_A::VALUE1,
            true => STBTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBTEN_A::VALUE2
    }
}
#[doc = "Field `STBTEN` writer - Standard Transmit Buffer Trigger Enable"]
pub type STBTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBCTR_SPEC, STBTEN_A, O>;
impl<'a, const O: u8> STBTEN_W<'a, O> {
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBTEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBTEN_A::VALUE2)
    }
}
#[doc = "Field `STBINP` reader - Standard Transmit Buffer Interrupt Node Pointer"]
pub type STBINP_R = crate::FieldReader<u8, STBINP_A>;
#[doc = "Standard Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STBINP_A {
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
impl From<STBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: STBINP_A) -> Self {
        variant as _
    }
}
impl STBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STBINP_A> {
        match self.bits {
            0 => Some(STBINP_A::VALUE1),
            1 => Some(STBINP_A::VALUE2),
            2 => Some(STBINP_A::VALUE3),
            3 => Some(STBINP_A::VALUE4),
            4 => Some(STBINP_A::VALUE5),
            5 => Some(STBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == STBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == STBINP_A::VALUE6
    }
}
#[doc = "Field `STBINP` writer - Standard Transmit Buffer Interrupt Node Pointer"]
pub type STBINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBCTR_SPEC, u8, STBINP_A, 3, O>;
impl<'a, const O: u8> STBINP_W<'a, O> {
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(STBINP_A::VALUE6)
    }
}
#[doc = "Field `ATBINP` reader - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type ATBINP_R = crate::FieldReader<u8, ATBINP_A>;
#[doc = "Alternative Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ATBINP_A {
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
impl From<ATBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ATBINP_A) -> Self {
        variant as _
    }
}
impl ATBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ATBINP_A> {
        match self.bits {
            0 => Some(ATBINP_A::VALUE1),
            1 => Some(ATBINP_A::VALUE2),
            2 => Some(ATBINP_A::VALUE3),
            3 => Some(ATBINP_A::VALUE4),
            4 => Some(ATBINP_A::VALUE5),
            5 => Some(ATBINP_A::VALUE6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ATBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ATBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ATBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ATBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ATBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ATBINP_A::VALUE6
    }
}
#[doc = "Field `ATBINP` writer - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type ATBINP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBCTR_SPEC, u8, ATBINP_A, 3, O>;
impl<'a, const O: u8> ATBINP_W<'a, O> {
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ATBINP_A::VALUE6)
    }
}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub type SIZE_R = crate::FieldReader<u8, SIZE_A>;
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1 = 0,
    #[doc = "1: The FIFO buffer contains 2 entries."]
    VALUE2 = 1,
    #[doc = "2: The FIFO buffer contains 4 entries."]
    VALUE3 = 2,
    #[doc = "3: The FIFO buffer contains 8 entries."]
    VALUE4 = 3,
    #[doc = "4: The FIFO buffer contains 16 entries."]
    VALUE5 = 4,
    #[doc = "5: The FIFO buffer contains 32 entries."]
    VALUE6 = 5,
    #[doc = "6: The FIFO buffer contains 64 entries."]
    VALUE7 = 6,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIZE_A> {
        match self.bits {
            0 => Some(SIZE_A::VALUE1),
            1 => Some(SIZE_A::VALUE2),
            2 => Some(SIZE_A::VALUE3),
            3 => Some(SIZE_A::VALUE4),
            4 => Some(SIZE_A::VALUE5),
            5 => Some(SIZE_A::VALUE6),
            6 => Some(SIZE_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIZE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIZE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SIZE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SIZE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SIZE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SIZE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SIZE_A::VALUE7
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBCTR_SPEC, u8, SIZE_A, 3, O>;
impl<'a, const O: u8> SIZE_W<'a, O> {
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(SIZE_A::VALUE7)
    }
}
#[doc = "Field `LOF` reader - Buffer Event on Limit Overflow"]
pub type LOF_R = crate::BitReader<LOF_A>;
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOF_A {
    #[doc = "0: A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    VALUE1 = 0,
    #[doc = "1: A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    VALUE2 = 1,
}
impl From<LOF_A> for bool {
    #[inline(always)]
    fn from(variant: LOF_A) -> Self {
        variant as u8 != 0
    }
}
impl LOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOF_A {
        match self.bits {
            false => LOF_A::VALUE1,
            true => LOF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOF_A::VALUE2
    }
}
#[doc = "Field `LOF` writer - Buffer Event on Limit Overflow"]
pub type LOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBCTR_SPEC, LOF_A, O>;
impl<'a, const O: u8> LOF_W<'a, O> {
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOF_A::VALUE2)
    }
}
#[doc = "Field `STBIEN` reader - Standard Transmit Buffer Interrupt Enable"]
pub type STBIEN_R = crate::BitReader<STBIEN_A>;
#[doc = "Standard Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STBIEN_A {
    #[doc = "0: The standard transmit buffer interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The standard transmit buffer interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<STBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: STBIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STBIEN_A {
        match self.bits {
            false => STBIEN_A::VALUE1,
            true => STBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBIEN_A::VALUE2
    }
}
#[doc = "Field `STBIEN` writer - Standard Transmit Buffer Interrupt Enable"]
pub type STBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBCTR_SPEC, STBIEN_A, O>;
impl<'a, const O: u8> STBIEN_W<'a, O> {
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STBIEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STBIEN_A::VALUE2)
    }
}
#[doc = "Field `TBERIEN` reader - Transmit Buffer Error Interrupt Enable"]
pub type TBERIEN_R = crate::BitReader<TBERIEN_A>;
#[doc = "Transmit Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBERIEN_A {
    #[doc = "0: The transmit buffer error interrupt generation is disabled."]
    VALUE1 = 0,
    #[doc = "1: The transmit buffer error interrupt generation is enabled."]
    VALUE2 = 1,
}
impl From<TBERIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TBERIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TBERIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBERIEN_A {
        match self.bits {
            false => TBERIEN_A::VALUE1,
            true => TBERIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBERIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBERIEN_A::VALUE2
    }
}
#[doc = "Field `TBERIEN` writer - Transmit Buffer Error Interrupt Enable"]
pub type TBERIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBCTR_SPEC, TBERIEN_A, O>;
impl<'a, const O: u8> TBERIEN_W<'a, O> {
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TBERIEN_A::VALUE1)
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TBERIEN_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    pub fn stbtm(&self) -> STBTM_R {
        STBTM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    pub fn stbten(&self) -> STBTEN_R {
        STBTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn stbinp(&self) -> STBINP_R {
        STBINP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn atbinp(&self) -> ATBINP_R {
        ATBINP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LOF_R {
        LOF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn stbien(&self) -> STBIEN_R {
        STBIEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn tberien(&self) -> TBERIEN_R {
        TBERIEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dptr(&mut self) -> DPTR_W<0> {
        DPTR_W::new(self)
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<8> {
        LIMIT_W::new(self)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbtm(&mut self) -> STBTM_W<14> {
        STBTM_W::new(self)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbten(&mut self) -> STBTEN_W<15> {
        STBTEN_W::new(self)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn stbinp(&mut self) -> STBINP_W<16> {
        STBINP_W::new(self)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn atbinp(&mut self) -> ATBINP_W<19> {
        ATBINP_W::new(self)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<24> {
        SIZE_W::new(self)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn lof(&mut self) -> LOF_W<28> {
        LOF_W::new(self)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbien(&mut self) -> STBIEN_W<30> {
        STBIEN_W::new(self)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tberien(&mut self) -> TBERIEN_W<31> {
        TBERIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbctr](index.html) module"]
pub struct TBCTR_SPEC;
impl crate::RegisterSpec for TBCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbctr::R](R) reader structure"]
impl crate::Readable for TBCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbctr::W](W) writer structure"]
impl crate::Writable for TBCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TBCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
