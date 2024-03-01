#[doc = "Register `TBCTR` reader"]
pub type R = crate::R<TbctrSpec>;
#[doc = "Register `TBCTR` writer"]
pub type W = crate::W<TbctrSpec>;
#[doc = "Field `DPTR` writer - Data Pointer"]
pub type DptrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub type LimitR = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Standard Transmit Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbtm {
    #[doc = "0: Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    Value1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    Value2 = 1,
}
impl From<Stbtm> for bool {
    #[inline(always)]
    fn from(variant: Stbtm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBTM` reader - Standard Transmit Buffer Trigger Mode"]
pub type StbtmR = crate::BitReader<Stbtm>;
impl StbtmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbtm {
        match self.bits {
            false => Stbtm::Value1,
            true => Stbtm::Value2,
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbtm::Value1
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbtm::Value2
    }
}
#[doc = "Field `STBTM` writer - Standard Transmit Buffer Trigger Mode"]
pub type StbtmW<'a, REG> = crate::BitWriter<'a, REG, Stbtm>;
impl<'a, REG> StbtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger mode 0: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stbtm::Value1)
    }
    #[doc = "Trigger mode 1: While TRBSR.STBT=1, a standard buffer event will be generated whenever there is a data transfer to TBUF or data write to INx (depending on TBCTR.LOF setting). STBT is cleared when TRBSR.TBFLVL=TBCTR.SIZE."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stbtm::Value2)
    }
}
#[doc = "Standard Transmit Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbten {
    #[doc = "0: The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    Value1 = 0,
    #[doc = "1: The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    Value2 = 1,
}
impl From<Stbten> for bool {
    #[inline(always)]
    fn from(variant: Stbten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBTEN` reader - Standard Transmit Buffer Trigger Enable"]
pub type StbtenR = crate::BitReader<Stbten>;
impl StbtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbten {
        match self.bits {
            false => Stbten::Value1,
            true => Stbten::Value2,
        }
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbten::Value1
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbten::Value2
    }
}
#[doc = "Field `STBTEN` writer - Standard Transmit Buffer Trigger Enable"]
pub type StbtenW<'a, REG> = crate::BitWriter<'a, REG, Stbten>;
impl<'a, REG> StbtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stbten::Value1)
    }
    #[doc = "The standard transmit buffer event trigger through bit TRBSR.STBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stbten::Value2)
    }
}
#[doc = "Standard Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stbinp {
    #[doc = "0: Output SR0 becomes activated."]
    Value1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    Value2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    Value3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    Value4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    Value5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    Value6 = 5,
}
impl From<Stbinp> for u8 {
    #[inline(always)]
    fn from(variant: Stbinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stbinp {
    type Ux = u8;
}
#[doc = "Field `STBINP` reader - Standard Transmit Buffer Interrupt Node Pointer"]
pub type StbinpR = crate::FieldReader<Stbinp>;
impl StbinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stbinp> {
        match self.bits {
            0 => Some(Stbinp::Value1),
            1 => Some(Stbinp::Value2),
            2 => Some(Stbinp::Value3),
            3 => Some(Stbinp::Value4),
            4 => Some(Stbinp::Value5),
            5 => Some(Stbinp::Value6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbinp::Value1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbinp::Value2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Stbinp::Value3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Stbinp::Value4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Stbinp::Value5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Stbinp::Value6
    }
}
#[doc = "Field `STBINP` writer - Standard Transmit Buffer Interrupt Node Pointer"]
pub type StbinpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Stbinp>;
impl<'a, REG> StbinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Stbinp::Value6)
    }
}
#[doc = "Alternative Transmit Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atbinp {
    #[doc = "0: Output SR0 becomes activated."]
    Value1 = 0,
    #[doc = "1: Output SR1 becomes activated."]
    Value2 = 1,
    #[doc = "2: Output SR2 becomes activated."]
    Value3 = 2,
    #[doc = "3: Output SR3 becomes activated."]
    Value4 = 3,
    #[doc = "4: Output SR4 becomes activated."]
    Value5 = 4,
    #[doc = "5: Output SR5 becomes activated."]
    Value6 = 5,
}
impl From<Atbinp> for u8 {
    #[inline(always)]
    fn from(variant: Atbinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atbinp {
    type Ux = u8;
}
#[doc = "Field `ATBINP` reader - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type AtbinpR = crate::FieldReader<Atbinp>;
impl AtbinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atbinp> {
        match self.bits {
            0 => Some(Atbinp::Value1),
            1 => Some(Atbinp::Value2),
            2 => Some(Atbinp::Value3),
            3 => Some(Atbinp::Value4),
            4 => Some(Atbinp::Value5),
            5 => Some(Atbinp::Value6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Atbinp::Value1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Atbinp::Value2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Atbinp::Value3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Atbinp::Value4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Atbinp::Value5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Atbinp::Value6
    }
}
#[doc = "Field `ATBINP` writer - Alternative Transmit Buffer Interrupt Node Pointer"]
pub type AtbinpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Atbinp>;
impl<'a, REG> AtbinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Atbinp::Value6)
    }
}
#[doc = "Buffer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    Value1 = 0,
    #[doc = "1: The FIFO buffer contains 2 entries."]
    Value2 = 1,
    #[doc = "2: The FIFO buffer contains 4 entries."]
    Value3 = 2,
    #[doc = "3: The FIFO buffer contains 8 entries."]
    Value4 = 3,
    #[doc = "4: The FIFO buffer contains 16 entries."]
    Value5 = 4,
    #[doc = "5: The FIFO buffer contains 32 entries."]
    Value6 = 5,
    #[doc = "6: The FIFO buffer contains 64 entries."]
    Value7 = 6,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
#[doc = "Field `SIZE` reader - Buffer Size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::Value1),
            1 => Some(Size::Value2),
            2 => Some(Size::Value3),
            3 => Some(Size::Value4),
            4 => Some(Size::Value5),
            5 => Some(Size::Value6),
            6 => Some(Size::Value7),
            _ => None,
        }
    }
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Size::Value1
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Size::Value2
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Size::Value3
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Size::Value4
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Size::Value5
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Size::Value6
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Size::Value7
    }
}
#[doc = "Field `SIZE` writer - Buffer Size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The FIFO mechanism is disabled. The buffer does not accept any request for data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value1)
    }
    #[doc = "The FIFO buffer contains 2 entries."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value2)
    }
    #[doc = "The FIFO buffer contains 4 entries."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value3)
    }
    #[doc = "The FIFO buffer contains 8 entries."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value4)
    }
    #[doc = "The FIFO buffer contains 16 entries."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value5)
    }
    #[doc = "The FIFO buffer contains 32 entries."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value6)
    }
    #[doc = "The FIFO buffer contains 64 entries."]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Value7)
    }
}
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lof {
    #[doc = "0: A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    Value1 = 0,
    #[doc = "1: A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    Value2 = 1,
}
impl From<Lof> for bool {
    #[inline(always)]
    fn from(variant: Lof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOF` reader - Buffer Event on Limit Overflow"]
pub type LofR = crate::BitReader<Lof>;
impl LofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lof {
        match self.bits {
            false => Lof::Value1,
            true => Lof::Value2,
        }
    }
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lof::Value1
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lof::Value2
    }
}
#[doc = "Field `LOF` writer - Buffer Event on Limit Overflow"]
pub type LofW<'a, REG> = crate::BitWriter<'a, REG, Lof>;
impl<'a, REG> LofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A standard transmit buffer event occurs when the filling level equals the limit value and gets lower due to transmission of a data word."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lof::Value1)
    }
    #[doc = "A standard transmit buffer interrupt event occurs when the filling level equals the limit value and gets bigger due to a write access to a data input location INx."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lof::Value2)
    }
}
#[doc = "Standard Transmit Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stbien {
    #[doc = "0: The standard transmit buffer interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The standard transmit buffer interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Stbien> for bool {
    #[inline(always)]
    fn from(variant: Stbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBIEN` reader - Standard Transmit Buffer Interrupt Enable"]
pub type StbienR = crate::BitReader<Stbien>;
impl StbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stbien {
        match self.bits {
            false => Stbien::Value1,
            true => Stbien::Value2,
        }
    }
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stbien::Value1
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stbien::Value2
    }
}
#[doc = "Field `STBIEN` writer - Standard Transmit Buffer Interrupt Enable"]
pub type StbienW<'a, REG> = crate::BitWriter<'a, REG, Stbien>;
impl<'a, REG> StbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard transmit buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stbien::Value1)
    }
    #[doc = "The standard transmit buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stbien::Value2)
    }
}
#[doc = "Transmit Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tberien {
    #[doc = "0: The transmit buffer error interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The transmit buffer error interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Tberien> for bool {
    #[inline(always)]
    fn from(variant: Tberien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBERIEN` reader - Transmit Buffer Error Interrupt Enable"]
pub type TberienR = crate::BitReader<Tberien>;
impl TberienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tberien {
        match self.bits {
            false => Tberien::Value1,
            true => Tberien::Value2,
        }
    }
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tberien::Value1
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tberien::Value2
    }
}
#[doc = "Field `TBERIEN` writer - Transmit Buffer Error Interrupt Enable"]
pub type TberienW<'a, REG> = crate::BitWriter<'a, REG, Tberien>;
impl<'a, REG> TberienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tberien::Value1)
    }
    #[doc = "The transmit buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tberien::Value2)
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    pub fn stbtm(&self) -> StbtmR {
        StbtmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    pub fn stbten(&self) -> StbtenR {
        StbtenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn stbinp(&self) -> StbinpR {
        StbinpR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn atbinp(&self) -> AtbinpR {
        AtbinpR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LofR {
        LofR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn stbien(&self) -> StbienR {
        StbienR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn tberien(&self) -> TberienR {
        TberienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dptr(&mut self) -> DptrW<TbctrSpec> {
        DptrW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LimitW<TbctrSpec> {
        LimitW::new(self, 8)
    }
    #[doc = "Bit 14 - Standard Transmit Buffer Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn stbtm(&mut self) -> StbtmW<TbctrSpec> {
        StbtmW::new(self, 14)
    }
    #[doc = "Bit 15 - Standard Transmit Buffer Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbten(&mut self) -> StbtenW<TbctrSpec> {
        StbtenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Standard Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn stbinp(&mut self) -> StbinpW<TbctrSpec> {
        StbinpW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Alternative Transmit Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn atbinp(&mut self) -> AtbinpW<TbctrSpec> {
        AtbinpW::new(self, 19)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<TbctrSpec> {
        SizeW::new(self, 24)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn lof(&mut self) -> LofW<TbctrSpec> {
        LofW::new(self, 28)
    }
    #[doc = "Bit 30 - Standard Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stbien(&mut self) -> StbienW<TbctrSpec> {
        StbienW::new(self, 30)
    }
    #[doc = "Bit 31 - Transmit Buffer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tberien(&mut self) -> TberienW<TbctrSpec> {
        TberienW::new(self, 31)
    }
}
#[doc = "Transmitter Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctrSpec;
impl crate::RegisterSpec for TbctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctr::R`](R) reader structure"]
impl crate::Readable for TbctrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctr::W`](W) writer structure"]
impl crate::Writable for TbctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TbctrSpec {
    const RESET_VALUE: u32 = 0;
}
