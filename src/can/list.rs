#[doc = "Register `LIST[%s]` reader"]
pub type R = crate::R<LIST_SPEC>;
#[doc = "Field `BEGIN` reader - List Begin"]
pub type BEGIN_R = crate::FieldReader;
#[doc = "Field `END` reader - List End"]
pub type END_R = crate::FieldReader;
#[doc = "Field `SIZE` reader - List Size"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `EMPTY` reader - List Empty Indication"]
pub type EMPTY_R = crate::BitReader<EMPTY_A>;
#[doc = "List Empty Indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EMPTY_A {
    #[doc = "0: At least one message object is allocated to list i."]
    VALUE1 = 0,
    #[doc = "1: No message object is allocated to the list i. List i is empty."]
    VALUE2 = 1,
}
impl From<EMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: EMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl EMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EMPTY_A {
        match self.bits {
            false => EMPTY_A::VALUE1,
            true => EMPTY_A::VALUE2,
        }
    }
    #[doc = "At least one message object is allocated to list i."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EMPTY_A::VALUE1
    }
    #[doc = "No message object is allocated to the list i. List i is empty."]
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
        EMPTY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "List Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`list::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIST_SPEC;
impl crate::RegisterSpec for LIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`list::R`](R) reader structure"]
impl crate::Readable for LIST_SPEC {}
#[doc = "`reset()` method sets LIST[%s]
to value 0"]
impl crate::Resettable for LIST_SPEC {
    const RESET_VALUE: u32 = 0;
}
