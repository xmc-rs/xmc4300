#[doc = "Register `FMMU_TYPE` reader"]
pub type R = crate::R<FmmuTypeSpec>;
#[doc = "Read Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAcc {
    #[doc = "0: Ignore mapping for read accesses"]
    Value1 = 0,
    #[doc = "1: Use mapping for read accesses"]
    Value2 = 1,
}
impl From<RAcc> for bool {
    #[inline(always)]
    fn from(variant: RAcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R_ACC` reader - Read Access"]
pub type RAccR = crate::BitReader<RAcc>;
impl RAccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RAcc {
        match self.bits {
            false => RAcc::Value1,
            true => RAcc::Value2,
        }
    }
    #[doc = "Ignore mapping for read accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RAcc::Value1
    }
    #[doc = "Use mapping for read accesses"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RAcc::Value2
    }
}
#[doc = "Write Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAcc {
    #[doc = "0: Ignore mapping for write accesses"]
    Value1 = 0,
    #[doc = "1: Use mapping for write accesses"]
    Value2 = 1,
}
impl From<WAcc> for bool {
    #[inline(always)]
    fn from(variant: WAcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `W_ACC` reader - Write Access"]
pub type WAccR = crate::BitReader<WAcc>;
impl WAccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAcc {
        match self.bits {
            false => WAcc::Value1,
            true => WAcc::Value2,
        }
    }
    #[doc = "Ignore mapping for write accesses"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAcc::Value1
    }
    #[doc = "Use mapping for write accesses"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAcc::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Read Access"]
    #[inline(always)]
    pub fn r_acc(&self) -> RAccR {
        RAccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Access"]
    #[inline(always)]
    pub fn w_acc(&self) -> WAccR {
        WAccR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "T0pe FMMU y\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmmu_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmmuTypeSpec;
impl crate::RegisterSpec for FmmuTypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fmmu_type::R`](R) reader structure"]
impl crate::Readable for FmmuTypeSpec {}
#[doc = "`reset()` method sets FMMU_TYPE to value 0"]
impl crate::Resettable for FmmuTypeSpec {
    const RESET_VALUE: u8 = 0;
}
