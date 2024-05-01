#[doc = "Register `DTSSTAT` reader"]
pub type R = crate::R<DtsstatSpec>;
#[doc = "Field `RESULT` reader - Result of the DTS Measurement"]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Sensor Ready Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: The DTS is not ready"]
    Const0 = 0,
    #[doc = "1: The DTS is ready"]
    Const1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Sensor Ready Status"]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::Const0,
            true => Rdy::Const1,
        }
    }
    #[doc = "The DTS is not ready"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Rdy::Const0
    }
    #[doc = "The DTS is ready"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Rdy::Const1
    }
}
#[doc = "Sensor Busy Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: not busy"]
    Const0 = 0,
    #[doc = "1: busy"]
    Const1 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Sensor Busy Status"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Const0,
            true => Busy::Const1,
        }
    }
    #[doc = "not busy"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Busy::Const0
    }
    #[doc = "busy"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Busy::Const1
    }
}
impl R {
    #[doc = "Bits 0:9 - Result of the DTS Measurement"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 14 - Sensor Ready Status"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sensor Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Die Temperature Sensor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtsstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtsstatSpec;
impl crate::RegisterSpec for DtsstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtsstat::R`](R) reader structure"]
impl crate::Readable for DtsstatSpec {}
#[doc = "`reset()` method sets DTSSTAT to value 0"]
impl crate::Resettable for DtsstatSpec {
    const RESET_VALUE: u32 = 0;
}
