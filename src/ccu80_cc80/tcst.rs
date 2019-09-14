#[doc = "Reader of register TCST"]
pub type R = crate::R<u32, super::TCST>;
#[doc = "Timer Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRB_A {
    #[doc = "0: Timer is stopped"]
    VALUE1,
    #[doc = "1: Timer is running"]
    VALUE2,
}
impl From<TRB_A> for bool {
    #[inline(always)]
    fn from(variant: TRB_A) -> Self {
        match variant {
            TRB_A::VALUE1 => false,
            TRB_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TRB`"]
pub type TRB_R = crate::R<bool, TRB_A>;
impl TRB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRB_A {
        match self.bits {
            false => TRB_A::VALUE1,
            true => TRB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRB_A::VALUE2
    }
}
#[doc = "Timer Counting Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDIR_A {
    #[doc = "0: Timer is counting up"]
    VALUE1,
    #[doc = "1: Timer is counting down"]
    VALUE2,
}
impl From<CDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDIR_A) -> Self {
        match variant {
            CDIR_A::VALUE1 => false,
            CDIR_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `CDIR`"]
pub type CDIR_R = crate::R<bool, CDIR_A>;
impl CDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIR_A {
        match self.bits {
            false => CDIR_A::VALUE1,
            true => CDIR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CDIR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CDIR_A::VALUE2
    }
}
#[doc = "Dead Time Counter 1 Run bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR1_A {
    #[doc = "0: Dead Time counter is idle"]
    VALUE1,
    #[doc = "1: Dead Time counter is running"]
    VALUE2,
}
impl From<DTR1_A> for bool {
    #[inline(always)]
    fn from(variant: DTR1_A) -> Self {
        match variant {
            DTR1_A::VALUE1 => false,
            DTR1_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DTR1`"]
pub type DTR1_R = crate::R<bool, DTR1_A>;
impl DTR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR1_A {
        match self.bits {
            false => DTR1_A::VALUE1,
            true => DTR1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTR1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTR1_A::VALUE2
    }
}
#[doc = "Dead Time Counter 2 Run bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTR2_A {
    #[doc = "0: Dead Time counter is idle"]
    VALUE1,
    #[doc = "1: Dead Time counter is running"]
    VALUE2,
}
impl From<DTR2_A> for bool {
    #[inline(always)]
    fn from(variant: DTR2_A) -> Self {
        match variant {
            DTR2_A::VALUE1 => false,
            DTR2_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `DTR2`"]
pub type DTR2_R = crate::R<bool, DTR2_A>;
impl DTR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTR2_A {
        match self.bits {
            false => DTR2_A::VALUE1,
            true => DTR2_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTR2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTR2_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Timer Run Bit"]
    #[inline(always)]
    pub fn trb(&self) -> TRB_R {
        TRB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer Counting Direction"]
    #[inline(always)]
    pub fn cdir(&self) -> CDIR_R {
        CDIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dead Time Counter 1 Run bit"]
    #[inline(always)]
    pub fn dtr1(&self) -> DTR1_R {
        DTR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dead Time Counter 2 Run bit"]
    #[inline(always)]
    pub fn dtr2(&self) -> DTR2_R {
        DTR2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
