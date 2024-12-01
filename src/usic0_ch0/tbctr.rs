#[doc = "Register `TBCTR` reader"]
pub type R = crate::R<TBCTR_SPEC>;
#[doc = "Register `TBCTR` writer"]
pub type W = crate::W<TBCTR_SPEC>;
#[doc = "Field `DPTR` writer - Data Pointer"]
pub type DPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub type LIMIT_R = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub type LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
#[doc = "Field `STBTM` reader - Standard Transmit Buffer Trigger Mode"]
pub type STBTM_R = crate::BitReader<STBTM_A>;
impl STBTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBTM_A {
        match self.bits {
            false => STBTM_A::VALUE1,
            true => STBTM_A::VALUE2,
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBTM_A::VALUE1
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBTM_A::VALUE2
    }
}
#[doc = "Field `STBTM` writer - Standard Transmit Buffer Trigger Mode"]
pub type STBTM_W<'a, REG> = crate::BitWriter<'a, REG, STBTM_A>;
impl<'a, REG> STBTM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STBTM_A::VALUE1)
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STBTM_A::VALUE2)
    }
}
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
#[doc = "Field `STBTEN` reader - Standard Transmit Buffer Trigger Enable"]
pub type STBTEN_R = crate::BitReader<STBTEN_A>;
impl STBTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBTEN_A {
        match self.bits {
            false => STBTEN_A::VALUE1,
            true => STBTEN_A::VALUE2,
        }
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBTEN_A::VALUE1
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBTEN_A::VALUE2
    }
}
#[doc = "Field `STBTEN` writer - Standard Transmit Buffer Trigger Enable"]
pub type STBTEN_W<'a, REG> = crate::BitWriter<'a, REG, STBTEN_A>;
impl<'a, REG> STBTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STBTEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STBTEN_A::VALUE2)
    }
}
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
impl crate::FieldSpec for STBINP_A {
    type Ux = u8;
}
impl crate::IsEnum for STBINP_A {}
#[doc = "Field `STBINP` reader - Standard Transmit Buffer Interrupt Node Pointer"]
pub type STBINP_R = crate::FieldReader<STBINP_A>;
impl STBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STBINP_A> {
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
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBINP_A::VALUE1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBINP_A::VALUE2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STBINP_A::VALUE3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STBINP_A::VALUE4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == STBINP_A::VALUE5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == STBINP_A::VALUE6
    }
}
#[doc = "Field `STBINP` writer - Standard Transmit Buffer Interrupt Node Pointer"]
pub type STBINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, STBINP_A>;
impl<'a, REG> STBINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(STBINP_A::VALUE6)
    }
}
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
impl crate::FieldSpec for ATBINP_A {
    type Ux = u8;
}
impl crate::IsEnum for ATBINP_A {}
#[doc = "Field `ATBINP` reader - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type ATBINP_R = crate::FieldReader<ATBINP_A>;
impl ATBINP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ATBINP_A> {
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
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ATBINP_A::VALUE1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ATBINP_A::VALUE2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ATBINP_A::VALUE3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ATBINP_A::VALUE4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == ATBINP_A::VALUE5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == ATBINP_A::VALUE6
    }
}
#[doc = "Field `ATBINP` writer - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type ATBINP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ATBINP_A>;
impl<'a, REG> ATBINP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(ATBINP_A::VALUE6)
    }
}
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
impl crate::FieldSpec for SIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for SIZE_A {}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub type SIZE_R = crate::FieldReader<SIZE_A>;
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIZE_A> {
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
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SIZE_A::VALUE1
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SIZE_A::VALUE2
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SIZE_A::VALUE3
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SIZE_A::VALUE4
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == SIZE_A::VALUE5
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == SIZE_A::VALUE6
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == SIZE_A::VALUE7
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SIZE_A>;
impl<'a, REG> SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE_A::VALUE7)
    }
}
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
#[doc = "Field `LOF` reader - Buffer Event on Limit Overflow"]
pub type LOF_R = crate::BitReader<LOF_A>;
impl LOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOF_A {
        match self.bits {
            false => LOF_A::VALUE1,
            true => LOF_A::VALUE2,
        }
    }
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LOF_A::VALUE1
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LOF_A::VALUE2
    }
}
#[doc = "Field `LOF` writer - Buffer Event on Limit Overflow"]
pub type LOF_W<'a, REG> = crate::BitWriter<'a, REG, LOF_A>;
impl<'a, REG> LOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LOF_A::VALUE1)
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LOF_A::VALUE2)
    }
}
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
#[doc = "Field `STBIEN` reader - Standard Transmit Buffer Interrupt Enable"]
pub type STBIEN_R = crate::BitReader<STBIEN_A>;
impl STBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STBIEN_A {
        match self.bits {
            false => STBIEN_A::VALUE1,
            true => STBIEN_A::VALUE2,
        }
    }
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STBIEN_A::VALUE1
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STBIEN_A::VALUE2
    }
}
#[doc = "Field `STBIEN` writer - Standard Transmit Buffer Interrupt Enable"]
pub type STBIEN_W<'a, REG> = crate::BitWriter<'a, REG, STBIEN_A>;
impl<'a, REG> STBIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STBIEN_A::VALUE1)
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STBIEN_A::VALUE2)
    }
}
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
#[doc = "Field `TBERIEN` reader - Transmit Buffer Error Interrupt Enable"]
pub type TBERIEN_R = crate::BitReader<TBERIEN_A>;
impl TBERIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TBERIEN_A {
        match self.bits {
            false => TBERIEN_A::VALUE1,
            true => TBERIEN_A::VALUE2,
        }
    }
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TBERIEN_A::VALUE1
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TBERIEN_A::VALUE2
    }
}
#[doc = "Field `TBERIEN` writer - Transmit Buffer Error Interrupt Enable"]
pub type TBERIEN_W<'a, REG> = crate::BitWriter<'a, REG, TBERIEN_A>;
impl<'a, REG> TBERIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TBERIEN_A::VALUE1)
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn dptr(&mut self) -> DPTR_W<TBCTR_SPEC> {
        DPTR_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W<TBCTR_SPEC> {
        LIMIT_W::new(self, 8)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    pub fn stbtm(&mut self) -> STBTM_W<TBCTR_SPEC> {
        STBTM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    pub fn stbten(&mut self) -> STBTEN_W<TBCTR_SPEC> {
        STBTEN_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn stbinp(&mut self) -> STBINP_W<TBCTR_SPEC> {
        STBINP_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn atbinp(&mut self) -> ATBINP_W<TBCTR_SPEC> {
        ATBINP_W::new(self, 19)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<TBCTR_SPEC> {
        SIZE_W::new(self, 24)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&mut self) -> LOF_W<TBCTR_SPEC> {
        LOF_W::new(self, 28)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn stbien(&mut self) -> STBIEN_W<TBCTR_SPEC> {
        STBIEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn tberien(&mut self) -> TBERIEN_W<TBCTR_SPEC> {
        TBERIEN_W::new(self, 31)
    }
}
#[doc = "Transmitter Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBCTR_SPEC;
impl crate::RegisterSpec for TBCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctr::R`](R) reader structure"]
impl crate::Readable for TBCTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbctr::W`](W) writer structure"]
impl crate::Writable for TBCTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TBCTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
