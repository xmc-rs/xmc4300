#[doc = "Register `TCST` reader"]
pub type R = crate::R<TcstSpec>;
#[doc = "Timer Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trb {
    #[doc = "0: Timer is stopped"]
    Value1 = 0,
    #[doc = "1: Timer is running"]
    Value2 = 1,
}
impl From<Trb> for bool {
    #[inline(always)]
    fn from(variant: Trb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRB` reader - Timer Run Bit"]
pub type TrbR = crate::BitReader<Trb>;
impl TrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trb {
        match self.bits {
            false => Trb::Value1,
            true => Trb::Value2,
        }
    }
    #[doc = "Timer is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trb::Value1
    }
    #[doc = "Timer is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trb::Value2
    }
}
#[doc = "Timer Counting Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdir {
    #[doc = "0: Timer is counting up"]
    Value1 = 0,
    #[doc = "1: Timer is counting down"]
    Value2 = 1,
}
impl From<Cdir> for bool {
    #[inline(always)]
    fn from(variant: Cdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDIR` reader - Timer Counting Direction"]
pub type CdirR = crate::BitReader<Cdir>;
impl CdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdir {
        match self.bits {
            false => Cdir::Value1,
            true => Cdir::Value2,
        }
    }
    #[doc = "Timer is counting up"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cdir::Value1
    }
    #[doc = "Timer is counting down"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cdir::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Timer Run Bit"]
    #[inline(always)]
    pub fn trb(&self) -> TrbR {
        TrbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Counting Direction"]
    #[inline(always)]
    pub fn cdir(&self) -> CdirR {
        CdirR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Slice Timer Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcstSpec;
impl crate::RegisterSpec for TcstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcst::R`](R) reader structure"]
impl crate::Readable for TcstSpec {}
#[doc = "`reset()` method sets TCST to value 0"]
impl crate::Resettable for TcstSpec {
    const RESET_VALUE: u32 = 0;
}
