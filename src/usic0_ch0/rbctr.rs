#[doc = "Register `RBCTR` reader"]
pub type R = crate::R<RbctrSpec>;
#[doc = "Register `RBCTR` writer"]
pub type W = crate::W<RbctrSpec>;
#[doc = "Field `DPTR` writer - Data Pointer"]
pub type DptrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LIMIT` reader - Limit For Interrupt Generation"]
pub type LimitR = crate::FieldReader;
#[doc = "Field `LIMIT` writer - Limit For Interrupt Generation"]
pub type LimitW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Standard Receive Buffer Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srbtm {
    #[doc = "0: Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    Value1 = 0,
    #[doc = "1: Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    Value2 = 1,
}
impl From<Srbtm> for bool {
    #[inline(always)]
    fn from(variant: Srbtm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBTM` reader - Standard Receive Buffer Trigger Mode"]
pub type SrbtmR = crate::BitReader<Srbtm>;
impl SrbtmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbtm {
        match self.bits {
            false => Srbtm::Value1,
            true => Srbtm::Value2,
        }
    }
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbtm::Value1
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbtm::Value2
    }
}
#[doc = "Field `SRBTM` writer - Standard Receive Buffer Trigger Mode"]
pub type SrbtmW<'a, REG> = crate::BitWriter<'a, REG, Srbtm>;
impl<'a, REG> SrbtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger mode 0: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=RBCTR.LIMIT."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srbtm::Value1)
    }
    #[doc = "Trigger mode 1: While TRBSR.SRBT=1, a standard receive buffer event will be generated whenever there is a new data received or data read out (depending on RBCTR.LOF setting). SRBT is cleared when TRBSR.RBFLVL=0."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srbtm::Value2)
    }
}
#[doc = "Standard Receive Buffer Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srbten {
    #[doc = "0: The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    Value1 = 0,
    #[doc = "1: The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    Value2 = 1,
}
impl From<Srbten> for bool {
    #[inline(always)]
    fn from(variant: Srbten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBTEN` reader - Standard Receive Buffer Trigger Enable"]
pub type SrbtenR = crate::BitReader<Srbten>;
impl SrbtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbten {
        match self.bits {
            false => Srbten::Value1,
            true => Srbten::Value2,
        }
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbten::Value1
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbten::Value2
    }
}
#[doc = "Field `SRBTEN` writer - Standard Receive Buffer Trigger Enable"]
pub type SrbtenW<'a, REG> = crate::BitWriter<'a, REG, Srbten>;
impl<'a, REG> SrbtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srbten::Value1)
    }
    #[doc = "The standard receive buffer event trigger through bit TRBSR.SRBT is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srbten::Value2)
    }
}
#[doc = "Standard Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srbinp {
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
impl From<Srbinp> for u8 {
    #[inline(always)]
    fn from(variant: Srbinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srbinp {
    type Ux = u8;
}
impl crate::IsEnum for Srbinp {}
#[doc = "Field `SRBINP` reader - Standard Receive Buffer Interrupt Node Pointer"]
pub type SrbinpR = crate::FieldReader<Srbinp>;
impl SrbinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srbinp> {
        match self.bits {
            0 => Some(Srbinp::Value1),
            1 => Some(Srbinp::Value2),
            2 => Some(Srbinp::Value3),
            3 => Some(Srbinp::Value4),
            4 => Some(Srbinp::Value5),
            5 => Some(Srbinp::Value6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbinp::Value1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbinp::Value2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Srbinp::Value3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Srbinp::Value4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Srbinp::Value5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Srbinp::Value6
    }
}
#[doc = "Field `SRBINP` writer - Standard Receive Buffer Interrupt Node Pointer"]
pub type SrbinpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Srbinp>;
impl<'a, REG> SrbinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Srbinp::Value6)
    }
}
#[doc = "Alternative Receive Buffer Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Arbinp {
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
impl From<Arbinp> for u8 {
    #[inline(always)]
    fn from(variant: Arbinp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Arbinp {
    type Ux = u8;
}
impl crate::IsEnum for Arbinp {}
#[doc = "Field `ARBINP` reader - Alternative Receive Buffer Interrupt Node Pointer"]
pub type ArbinpR = crate::FieldReader<Arbinp>;
impl ArbinpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Arbinp> {
        match self.bits {
            0 => Some(Arbinp::Value1),
            1 => Some(Arbinp::Value2),
            2 => Some(Arbinp::Value3),
            3 => Some(Arbinp::Value4),
            4 => Some(Arbinp::Value5),
            5 => Some(Arbinp::Value6),
            _ => None,
        }
    }
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbinp::Value1
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbinp::Value2
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Arbinp::Value3
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Arbinp::Value4
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Arbinp::Value5
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Arbinp::Value6
    }
}
#[doc = "Field `ARBINP` writer - Alternative Receive Buffer Interrupt Node Pointer"]
pub type ArbinpW<'a, REG> = crate::FieldWriter<'a, REG, 3, Arbinp>;
impl<'a, REG> ArbinpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output SR0 becomes activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value1)
    }
    #[doc = "Output SR1 becomes activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value2)
    }
    #[doc = "Output SR2 becomes activated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value3)
    }
    #[doc = "Output SR3 becomes activated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value4)
    }
    #[doc = "Output SR4 becomes activated."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value5)
    }
    #[doc = "Output SR5 becomes activated."]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Arbinp::Value6)
    }
}
#[doc = "Receiver Control Information Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcim {
    #[doc = "0: RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    Value1 = 0,
    #[doc = "1: RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    Value2 = 1,
    #[doc = "2: RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    Value3 = 2,
    #[doc = "3: RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    Value4 = 3,
}
impl From<Rcim> for u8 {
    #[inline(always)]
    fn from(variant: Rcim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcim {
    type Ux = u8;
}
impl crate::IsEnum for Rcim {}
#[doc = "Field `RCIM` reader - Receiver Control Information Mode"]
pub type RcimR = crate::FieldReader<Rcim>;
impl RcimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcim {
        match self.bits {
            0 => Rcim::Value1,
            1 => Rcim::Value2,
            2 => Rcim::Value3,
            3 => Rcim::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rcim::Value1
    }
    #[doc = "RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rcim::Value2
    }
    #[doc = "RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Rcim::Value3
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Rcim::Value4
    }
}
#[doc = "Field `RCIM` writer - Receiver Control Information Mode"]
pub type RcimW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rcim, crate::Safe>;
impl<'a, REG> RcimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcim::Value1)
    }
    #[doc = "RCI\\[4\\]
= SOF, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcim::Value2)
    }
    #[doc = "RCI\\[4\\]
= 0, RCI\\[3:0\\]
= WLEN"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Rcim::Value3)
    }
    #[doc = "RCI\\[4\\]
= PERR, RCI\\[3\\]
= PAR, RCI\\[2:1\\]
= 00B, RCI\\[0\\]
= SOF"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Rcim::Value4)
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
impl crate::IsEnum for Size {}
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
#[doc = "Receiver Notification Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnm {
    #[doc = "0: Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    Value1 = 0,
    #[doc = "1: RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    Value2 = 1,
}
impl From<Rnm> for bool {
    #[inline(always)]
    fn from(variant: Rnm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNM` reader - Receiver Notification Mode"]
pub type RnmR = crate::BitReader<Rnm>;
impl RnmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnm {
        match self.bits {
            false => Rnm::Value1,
            true => Rnm::Value2,
        }
    }
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rnm::Value1
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rnm::Value2
    }
}
#[doc = "Field `RNM` writer - Receiver Notification Mode"]
pub type RnmW<'a, REG> = crate::BitWriter<'a, REG, Rnm>;
impl<'a, REG> RnmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filling level mode: A standard receive buffer event occurs when the filling level equals the limit value and changes, either due to a read access from OUTR (LOF = 0) or due to a new received data word (LOF = 1)."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rnm::Value1)
    }
    #[doc = "RCI mode: A standard receive buffer event occurs when register OUTR is updated with a new value if the corresponding value in OUTR.RCI\\[4\\]
= 0. If OUTR.RCI\\[4\\]
= 1, an alternative receive buffer event occurs instead of the standard receive buffer event."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rnm::Value2)
    }
}
#[doc = "Buffer Event on Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lof {
    #[doc = "0: A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    Value1 = 0,
    #[doc = "1: A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
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
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lof::Value1
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
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
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets lower due to a read access from OUTR."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lof::Value1)
    }
    #[doc = "A standard receive buffer event occurs when the filling level equals the limit value and gets bigger due to the reception of a new data word."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lof::Value2)
    }
}
#[doc = "Alternative Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbien {
    #[doc = "0: The alternative receive buffer interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The alternative receive buffer interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Arbien> for bool {
    #[inline(always)]
    fn from(variant: Arbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBIEN` reader - Alternative Receive Buffer Interrupt Enable"]
pub type ArbienR = crate::BitReader<Arbien>;
impl ArbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbien {
        match self.bits {
            false => Arbien::Value1,
            true => Arbien::Value2,
        }
    }
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Arbien::Value1
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Arbien::Value2
    }
}
#[doc = "Field `ARBIEN` writer - Alternative Receive Buffer Interrupt Enable"]
pub type ArbienW<'a, REG> = crate::BitWriter<'a, REG, Arbien>;
impl<'a, REG> ArbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The alternative receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbien::Value1)
    }
    #[doc = "The alternative receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Arbien::Value2)
    }
}
#[doc = "Standard Receive Buffer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srbien {
    #[doc = "0: The standard receive buffer interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The standard receive buffer interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Srbien> for bool {
    #[inline(always)]
    fn from(variant: Srbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRBIEN` reader - Standard Receive Buffer Interrupt Enable"]
pub type SrbienR = crate::BitReader<Srbien>;
impl SrbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbien {
        match self.bits {
            false => Srbien::Value1,
            true => Srbien::Value2,
        }
    }
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Srbien::Value1
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Srbien::Value2
    }
}
#[doc = "Field `SRBIEN` writer - Standard Receive Buffer Interrupt Enable"]
pub type SrbienW<'a, REG> = crate::BitWriter<'a, REG, Srbien>;
impl<'a, REG> SrbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The standard receive buffer interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Srbien::Value1)
    }
    #[doc = "The standard receive buffer interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Srbien::Value2)
    }
}
#[doc = "Receive Buffer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rberien {
    #[doc = "0: The receive buffer error interrupt generation is disabled."]
    Value1 = 0,
    #[doc = "1: The receive buffer error interrupt generation is enabled."]
    Value2 = 1,
}
impl From<Rberien> for bool {
    #[inline(always)]
    fn from(variant: Rberien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBERIEN` reader - Receive Buffer Error Interrupt Enable"]
pub type RberienR = crate::BitReader<Rberien>;
impl RberienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rberien {
        match self.bits {
            false => Rberien::Value1,
            true => Rberien::Value2,
        }
    }
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rberien::Value1
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rberien::Value2
    }
}
#[doc = "Field `RBERIEN` writer - Receive Buffer Error Interrupt Enable"]
pub type RberienW<'a, REG> = crate::BitWriter<'a, REG, Rberien>;
impl<'a, REG> RberienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receive buffer error interrupt generation is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rberien::Value1)
    }
    #[doc = "The receive buffer error interrupt generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rberien::Value2)
    }
}
impl R {
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    pub fn limit(&self) -> LimitR {
        LimitR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    pub fn srbtm(&self) -> SrbtmR {
        SrbtmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    pub fn srbten(&self) -> SrbtenR {
        SrbtenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn srbinp(&self) -> SrbinpR {
        SrbinpR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    pub fn arbinp(&self) -> ArbinpR {
        ArbinpR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    pub fn rcim(&self) -> RcimR {
        RcimR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    pub fn rnm(&self) -> RnmR {
        RnmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    pub fn lof(&self) -> LofR {
        LofR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn arbien(&self) -> ArbienR {
        ArbienR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn srbien(&self) -> SrbienR {
        SrbienR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    pub fn rberien(&self) -> RberienR {
        RberienR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dptr(&mut self) -> DptrW<RbctrSpec> {
        DptrW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Limit For Interrupt Generation"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LimitW<RbctrSpec> {
        LimitW::new(self, 8)
    }
    #[doc = "Bit 14 - Standard Receive Buffer Trigger Mode"]
    #[inline(always)]
    #[must_use]
    pub fn srbtm(&mut self) -> SrbtmW<RbctrSpec> {
        SrbtmW::new(self, 14)
    }
    #[doc = "Bit 15 - Standard Receive Buffer Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srbten(&mut self) -> SrbtenW<RbctrSpec> {
        SrbtenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Standard Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn srbinp(&mut self) -> SrbinpW<RbctrSpec> {
        SrbinpW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Alternative Receive Buffer Interrupt Node Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn arbinp(&mut self) -> ArbinpW<RbctrSpec> {
        ArbinpW::new(self, 19)
    }
    #[doc = "Bits 22:23 - Receiver Control Information Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rcim(&mut self) -> RcimW<RbctrSpec> {
        RcimW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Buffer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<RbctrSpec> {
        SizeW::new(self, 24)
    }
    #[doc = "Bit 27 - Receiver Notification Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rnm(&mut self) -> RnmW<RbctrSpec> {
        RnmW::new(self, 27)
    }
    #[doc = "Bit 28 - Buffer Event on Limit Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn lof(&mut self) -> LofW<RbctrSpec> {
        LofW::new(self, 28)
    }
    #[doc = "Bit 29 - Alternative Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arbien(&mut self) -> ArbienW<RbctrSpec> {
        ArbienW::new(self, 29)
    }
    #[doc = "Bit 30 - Standard Receive Buffer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn srbien(&mut self) -> SrbienW<RbctrSpec> {
        SrbienW::new(self, 30)
    }
    #[doc = "Bit 31 - Receive Buffer Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rberien(&mut self) -> RberienW<RbctrSpec> {
        RberienW::new(self, 31)
    }
}
#[doc = "Receiver Buffer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbctrSpec;
impl crate::RegisterSpec for RbctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbctr::R`](R) reader structure"]
impl crate::Readable for RbctrSpec {}
#[doc = "`write(|w| ..)` method takes [`rbctr::W`](W) writer structure"]
impl crate::Writable for RbctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBCTR to value 0"]
impl crate::Resettable for RbctrSpec {
    const RESET_VALUE: u32 = 0;
}
