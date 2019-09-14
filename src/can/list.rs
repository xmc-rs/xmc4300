#[doc = "Reader of register LIST[%s]"]
pub type R = crate::R<u32, super::LIST>;
#[doc = "Reader of field `BEGIN`"]
pub type BEGIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<u8, u8>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "List Empty Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMPTY_A {
    #[doc = "0: At least one message object is allocated to list i."]
    VALUE1,
    #[doc = "1: No message object is allocated to the list i. List i is empty."]
    VALUE2,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        match variant {
            EMPTY_A::VALUE1 => false,
            EMPTY_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, EMPTY_A>;
impl EMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::VALUE1,
            true => EMPTY_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMPTY_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EMPTY_A::VALUE2
    }
}
impl R {
    #[doc = "Bits 0:7 - List Begin"]
    #[inline(always)]
    pub fn begin(&self) -> BEGIN_R {
        BEGIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - List End"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - List Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - List Empty Indication"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
