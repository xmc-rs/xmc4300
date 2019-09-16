#[doc = "Reader of register RBCTR"]
pub type R = crate::R<u32, super::RBCTR>;
#[doc = "Writer for register RBCTR"]
pub type W = crate::W<u32, super::RBCTR>;
#[doc = "Register RBCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RBCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DPTR`"]
pub struct DPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `LIMIT`"]
pub type LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LIMIT`"]
pub struct LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Standard Receive Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTM_A {
    #[doc = "0: Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    VALUE1,
    #[doc = "1: Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    VALUE2,
}
impl From<SRBTM_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTM_A) -> Self {
        match variant {
            SRBTM_A::VALUE1 => false,
            SRBTM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRBTM`"]
pub type SRBTM_R = crate::R<bool, SRBTM_A>;
impl SRBTM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTM_A {
        match self.bits {
            false => SRBTM_A::VALUE1,
            true => SRBTM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBTM_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRBTM`"]
pub struct SRBTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBTM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Standard Receive Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBTEN_A {
    #[doc = "0: The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    VALUE1,
    #[doc = "1: The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    VALUE2,
}
impl From<SRBTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBTEN_A) -> Self {
        match variant {
            SRBTEN_A::VALUE1 => false,
            SRBTEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRBTEN`"]
pub type SRBTEN_R = crate::R<bool, SRBTEN_A>;
impl SRBTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBTEN_A {
        match self.bits {
            false => SRBTEN_A::VALUE1,
            true => SRBTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRBTEN`"]
pub struct SRBTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBTEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Standard Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6,
}
impl From<SRBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: SRBINP_A) -> Self {
        match variant {
            SRBINP_A::VALUE1 => 0,
            SRBINP_A::VALUE2 => 1,
            SRBINP_A::VALUE3 => 2,
            SRBINP_A::VALUE4 => 3,
            SRBINP_A::VALUE5 => 4,
            SRBINP_A::VALUE6 => 5,
        }
    }
}
#[doc = "Reader of field `SRBINP`"]
pub type SRBINP_R = crate::R<u8, SRBINP_A>;
impl SRBINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRBINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRBINP_A::VALUE1),
            1 => Val(SRBINP_A::VALUE2),
            2 => Val(SRBINP_A::VALUE3),
            3 => Val(SRBINP_A::VALUE4),
            4 => Val(SRBINP_A::VALUE5),
            5 => Val(SRBINP_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SRBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SRBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SRBINP_A::VALUE6
    }
}
#[doc = "Write proxy for field `SRBINP`"]
pub struct SRBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(SRBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Alternative Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBINP_A {
    #[doc = "0: Output SR0 becomes activated."]
    VALUE1,
    #[doc = "1: Output SR1 becomes activated."]
    VALUE2,
    #[doc = "2: Output SR2 becomes activated."]
    VALUE3,
    #[doc = "3: Output SR3 becomes activated."]
    VALUE4,
    #[doc = "4: Output SR4 becomes activated."]
    VALUE5,
    #[doc = "5: Output SR5 becomes activated."]
    VALUE6,
}
impl From<ARBINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBINP_A) -> Self {
        match variant {
            ARBINP_A::VALUE1 => 0,
            ARBINP_A::VALUE2 => 1,
            ARBINP_A::VALUE3 => 2,
            ARBINP_A::VALUE4 => 3,
            ARBINP_A::VALUE5 => 4,
            ARBINP_A::VALUE6 => 5,
        }
    }
}
#[doc = "Reader of field `ARBINP`"]
pub type ARBINP_R = crate::R<u8, ARBINP_A>;
impl ARBINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ARBINP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ARBINP_A::VALUE1),
            1 => Val(ARBINP_A::VALUE2),
            2 => Val(ARBINP_A::VALUE3),
            3 => Val(ARBINP_A::VALUE4),
            4 => Val(ARBINP_A::VALUE5),
            5 => Val(ARBINP_A::VALUE6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ARBINP_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ARBINP_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ARBINP_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ARBINP_A::VALUE6
    }
}
#[doc = "Write proxy for field `ARBINP`"]
pub struct ARBINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(ARBINP_A::VALUE6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Receiver Control Information Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCIM_A {
    #[doc = "0: RCI\\[4\\] = PERR, RCI\\[3:0\\] = WLEN"]
    VALUE1,
    #[doc = "1: RCI\\[4\\] = SOF, RCI\\[3:0\\] = WLEN"]
    VALUE2,
    #[doc = "2: RCI\\[4\\] = 0, RCI\\[3:0\\] = WLEN"]
    VALUE3,
    #[doc = "3: RCI\\[4\\] = PERR, RCI\\[3\\] = PAR, RCI\\[2:1\\] = 00B, RCI\\[0\\] = SOF"]
    VALUE4,
}
impl From<RCIM_A> for u8 {
    #[inline(always)]
    fn from(variant: RCIM_A) -> Self {
        match variant {
            RCIM_A::VALUE1 => 0,
            RCIM_A::VALUE2 => 1,
            RCIM_A::VALUE3 => 2,
            RCIM_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `RCIM`"]
pub type RCIM_R = crate::R<u8, RCIM_A>;
impl RCIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIM_A {
        match self.bits {
            0 => RCIM_A::VALUE1,
            1 => RCIM_A::VALUE2,
            2 => RCIM_A::VALUE3,
            3 => RCIM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RCIM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RCIM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RCIM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RCIM_A::VALUE4
    }
}
#[doc = "Write proxy for field `RCIM`"]
pub struct RCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCIM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3:0\\] = WLEN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE1)
    }
    #[doc = "RCI\\[4\\] = SOF, RCI\\[3:0\\] = WLEN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE2)
    }
    #[doc = "RCI\\[4\\] = 0, RCI\\[3:0\\] = WLEN"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE3)
    }
    #[doc = "RCI\\[4\\] = PERR, RCI\\[3\\] = PAR, RCI\\[2:1\\] = 00B, RCI\\[0\\] = SOF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(RCIM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    VALUE1,
    #[doc = "1: The FIFO buffer contains 2 entries."]
    VALUE2,
    #[doc = "2: The FIFO buffer contains 4 entries."]
    VALUE3,
    #[doc = "3: The FIFO buffer contains 8 entries."]
    VALUE4,
    #[doc = "4: The FIFO buffer contains 16 entries."]
    VALUE5,
    #[doc = "5: The FIFO buffer contains 32 entries."]
    VALUE6,
    #[doc = "6: The FIFO buffer contains 64 entries."]
    VALUE7,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::VALUE1 => 0,
            SIZE_A::VALUE2 => 1,
            SIZE_A::VALUE3 => 2,
            SIZE_A::VALUE4 => 3,
            SIZE_A::VALUE5 => 4,
            SIZE_A::VALUE6 => 5,
            SIZE_A::VALUE7 => 6,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::VALUE1),
            1 => Val(SIZE_A::VALUE2),
            2 => Val(SIZE_A::VALUE3),
            3 => Val(SIZE_A::VALUE4),
            4 => Val(SIZE_A::VALUE5),
            5 => Val(SIZE_A::VALUE6),
            6 => Val(SIZE_A::VALUE7),
            i => Res(i),
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
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Receiver Notification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNM_A {
    #[doc = "0: Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    VALUE1,
    #[doc = "1: RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\] = 0. If OUTR.RCI\\[4\\] = 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    VALUE2,
}
impl From<RNM_A> for bool {
    #[inline(always)]
    fn from(variant: RNM_A) -> Self {
        match variant {
            RNM_A::VALUE1 => false,
            RNM_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RNM`"]
pub type RNM_R = crate::R<bool, RNM_A>;
impl RNM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNM_A {
        match self.bits {
            false => RNM_A::VALUE1,
            true => RNM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RNM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RNM_A::VALUE2
    }
}
#[doc = "Write proxy for field `RNM`"]
pub struct RNM_W<'a> {
    w: &'a mut W,
}
impl<'a> RNM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RNM_A::VALUE1)
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\] = 0. If OUTR.RCI\\[4\\] = 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RNM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOF_A {
    #[doc = "0: A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    VALUE1,
    #[doc = "1: A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    VALUE2,
}
impl From<LOF_A> for bool {
    #[inline(always)]
    fn from(variant: LOF_A) -> Self {
        match variant {
            LOF_A::VALUE1 => false,
            LOF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `LOF`"]
pub type LOF_R = crate::R<bool, LOF_A>;
impl LOF_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LOF`"]
pub struct LOF_W<'a> {
    w: &'a mut W,
}
impl<'a> LOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LOF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Alternative Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBIEN_A {
    #[doc = "0: The alternative receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "1: The alternative receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl From<ARBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARBIEN_A) -> Self {
        match variant {
            ARBIEN_A::VALUE1 => false,
            ARBIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ARBIEN`"]
pub type ARBIEN_R = crate::R<bool, ARBIEN_A>;
impl ARBIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBIEN_A {
        match self.bits {
            false => ARBIEN_A::VALUE1,
            true => ARBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ARBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ARBIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ARBIEN`"]
pub struct ARBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE1)
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ARBIEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Standard Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRBIEN_A {
    #[doc = "0: The standard receive buffer interrupt generation is disabled."]
    VALUE1,
    #[doc = "1: The standard receive buffer interrupt generation is enabled."]
    VALUE2,
}
impl From<SRBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRBIEN_A) -> Self {
        match variant {
            SRBIEN_A::VALUE1 => false,
            SRBIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SRBIEN`"]
pub type SRBIEN_R = crate::R<bool, SRBIEN_A>;
impl SRBIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRBIEN_A {
        match self.bits {
            false => SRBIEN_A::VALUE1,
            true => SRBIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRBIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRBIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `SRBIEN`"]
pub struct SRBIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRBIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRBIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE1)
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SRBIEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Receive Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBERIEN_A {
    #[doc = "0: The receive buffer error interrupt generation is disabled."]
    VALUE1,
    #[doc = "1: The receive buffer error interrupt generation is enabled."]
    VALUE2,
}
impl From<RBERIEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBERIEN_A) -> Self {
        match variant {
            RBERIEN_A::VALUE1 => false,
            RBERIEN_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `RBERIEN`"]
pub type RBERIEN_R = crate::R<bool, RBERIEN_A>;
impl RBERIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBERIEN_A {
        match self.bits {
            false => RBERIEN_A::VALUE1,
            true => RBERIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RBERIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RBERIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `RBERIEN`"]
pub struct RBERIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBERIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBERIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE1)
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RBERIEN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&self) -> SRBTM_R {
        SRBTM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&self) -> SRBTEN_R {
        SRBTEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&self) -> SRBINP_R {
        SRBINP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&self) -> ARBINP_R {
        ARBINP_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&self) -> RCIM_R {
        RCIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&self) -> RNM_R {
        RNM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LOF_R {
        LOF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&self) -> ARBIEN_R {
        ARBIEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&self) -> SRBIEN_R {
        SRBIEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&self) -> RBERIEN_R {
        RBERIEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    pub fn dptr(&mut self) -> DPTR_W {
        DPTR_W { w: self }
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W {
        LIMIT_W { w: self }
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&mut self) -> SRBTM_W {
        SRBTM_W { w: self }
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&mut self) -> SRBTEN_W {
        SRBTEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&mut self) -> SRBINP_W {
        SRBINP_W { w: self }
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&mut self) -> ARBINP_W {
        ARBINP_W { w: self }
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&mut self) -> RCIM_W {
        RCIM_W { w: self }
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&mut self) -> RNM_W {
        RNM_W { w: self }
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&mut self) -> LOF_W {
        LOF_W { w: self }
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&mut self) -> ARBIEN_W {
        ARBIEN_W { w: self }
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&mut self) -> SRBIEN_W {
        SRBIEN_W { w: self }
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&mut self) -> RBERIEN_W {
        RBERIEN_W { w: self }
    }
}
