#[doc = "Register `ESC_DL_CONTROL` reader"]
pub type R = crate::R<EscDlControlSpec>;
#[doc = "Forwarding rule\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fr {
    #[doc = "0: EtherCAT frames are processed, Non-EtherCAT frames are forwarded without processing"]
    Value1 = 0,
    #[doc = "1: EtherCAT frames are processed, Non- EtherCAT frames are destroyed"]
    Value2 = 1,
}
impl From<Fr> for bool {
    #[inline(always)]
    fn from(variant: Fr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FR` reader - Forwarding rule"]
pub type FrR = crate::BitReader<Fr>;
impl FrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fr {
        match self.bits {
            false => Fr::Value1,
            true => Fr::Value2,
        }
    }
    #[doc = "EtherCAT frames are processed, Non-EtherCAT frames are forwarded without processing"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fr::Value1
    }
    #[doc = "EtherCAT frames are processed, Non- EtherCAT frames are destroyed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fr::Value2
    }
}
#[doc = "Temporary use of settings in LP1-LP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temp {
    #[doc = "0: permanent use"]
    Value1 = 0,
    #[doc = "1: use for about 1 second, then revert to previous settings"]
    Value2 = 1,
}
impl From<Temp> for bool {
    #[inline(always)]
    fn from(variant: Temp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMP` reader - Temporary use of settings in LP1-LP3"]
pub type TempR = crate::BitReader<Temp>;
impl TempR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Temp {
        match self.bits {
            false => Temp::Value1,
            true => Temp::Value2,
        }
    }
    #[doc = "permanent use"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Temp::Value1
    }
    #[doc = "use for about 1 second, then revert to previous settings"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Temp::Value2
    }
}
#[doc = "Loop Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lp0 {
    #[doc = "0: Auto"]
    Value1 = 0,
    #[doc = "1: Auto Close"]
    Value2 = 1,
    #[doc = "2: Open"]
    Value3 = 2,
    #[doc = "3: Closed"]
    Value4 = 3,
}
impl From<Lp0> for u8 {
    #[inline(always)]
    fn from(variant: Lp0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lp0 {
    type Ux = u8;
}
#[doc = "Field `LP0` reader - Loop Port 0"]
pub type Lp0R = crate::FieldReader<Lp0>;
impl Lp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp0 {
        match self.bits {
            0 => Lp0::Value1,
            1 => Lp0::Value2,
            2 => Lp0::Value3,
            3 => Lp0::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp0::Value1
    }
    #[doc = "Auto Close"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp0::Value2
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lp0::Value3
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lp0::Value4
    }
}
#[doc = "Loop Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lp1 {
    #[doc = "0: Auto"]
    Value1 = 0,
    #[doc = "1: Auto Close"]
    Value2 = 1,
    #[doc = "2: Open"]
    Value3 = 2,
    #[doc = "3: Closed"]
    Value4 = 3,
}
impl From<Lp1> for u8 {
    #[inline(always)]
    fn from(variant: Lp1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lp1 {
    type Ux = u8;
}
#[doc = "Field `LP1` reader - Loop Port 1"]
pub type Lp1R = crate::FieldReader<Lp1>;
impl Lp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp1 {
        match self.bits {
            0 => Lp1::Value1,
            1 => Lp1::Value2,
            2 => Lp1::Value3,
            3 => Lp1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp1::Value1
    }
    #[doc = "Auto Close"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp1::Value2
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lp1::Value3
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lp1::Value4
    }
}
#[doc = "Loop Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lp2 {
    #[doc = "0: Auto"]
    Value1 = 0,
    #[doc = "1: Auto Close"]
    Value2 = 1,
    #[doc = "2: Open"]
    Value3 = 2,
    #[doc = "3: Closed"]
    Value4 = 3,
}
impl From<Lp2> for u8 {
    #[inline(always)]
    fn from(variant: Lp2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lp2 {
    type Ux = u8;
}
#[doc = "Field `LP2` reader - Loop Port 2"]
pub type Lp2R = crate::FieldReader<Lp2>;
impl Lp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp2 {
        match self.bits {
            0 => Lp2::Value1,
            1 => Lp2::Value2,
            2 => Lp2::Value3,
            3 => Lp2::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp2::Value1
    }
    #[doc = "Auto Close"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp2::Value2
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lp2::Value3
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lp2::Value4
    }
}
#[doc = "Loop Port 3\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lp3 {
    #[doc = "0: Auto"]
    Value1 = 0,
    #[doc = "1: Auto Close"]
    Value2 = 1,
    #[doc = "2: Open"]
    Value3 = 2,
    #[doc = "3: Closed"]
    Value4 = 3,
}
impl From<Lp3> for u8 {
    #[inline(always)]
    fn from(variant: Lp3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lp3 {
    type Ux = u8;
}
#[doc = "Field `LP3` reader - Loop Port 3"]
pub type Lp3R = crate::FieldReader<Lp3>;
impl Lp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp3 {
        match self.bits {
            0 => Lp3::Value1,
            1 => Lp3::Value2,
            2 => Lp3::Value3,
            3 => Lp3::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lp3::Value1
    }
    #[doc = "Auto Close"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lp3::Value2
    }
    #[doc = "Open"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Lp3::Value3
    }
    #[doc = "Closed"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Lp3::Value4
    }
}
#[doc = "RX FIFO Size\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxFifoSize {
    #[doc = "0: -40 ns (-80 ns)"]
    Value1 = 0,
    #[doc = "1: -40 ns (-80 ns)"]
    Value2 = 1,
    #[doc = "2: -40 ns"]
    Value3 = 2,
    #[doc = "3: -40 ns"]
    Value4 = 3,
    #[doc = "4: no change"]
    Value5 = 4,
    #[doc = "5: no change"]
    Value6 = 5,
    #[doc = "6: no change"]
    Value7 = 6,
    #[doc = "7: default"]
    Value8 = 7,
}
impl From<RxFifoSize> for u8 {
    #[inline(always)]
    fn from(variant: RxFifoSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxFifoSize {
    type Ux = u8;
}
#[doc = "Field `RX_FIFO_SIZE` reader - RX FIFO Size"]
pub type RxFifoSizeR = crate::FieldReader<RxFifoSize>;
impl RxFifoSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFifoSize {
        match self.bits {
            0 => RxFifoSize::Value1,
            1 => RxFifoSize::Value2,
            2 => RxFifoSize::Value3,
            3 => RxFifoSize::Value4,
            4 => RxFifoSize::Value5,
            5 => RxFifoSize::Value6,
            6 => RxFifoSize::Value7,
            7 => RxFifoSize::Value8,
            _ => unreachable!(),
        }
    }
    #[doc = "-40 ns (-80 ns)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RxFifoSize::Value1
    }
    #[doc = "-40 ns (-80 ns)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RxFifoSize::Value2
    }
    #[doc = "-40 ns"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RxFifoSize::Value3
    }
    #[doc = "-40 ns"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RxFifoSize::Value4
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == RxFifoSize::Value5
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == RxFifoSize::Value6
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == RxFifoSize::Value7
    }
    #[doc = "default"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == RxFifoSize::Value8
    }
}
#[doc = "EBUS Low Jitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lj {
    #[doc = "0: Normal jitter"]
    Value1 = 0,
    #[doc = "1: Reduced jitter"]
    Value2 = 1,
}
impl From<Lj> for bool {
    #[inline(always)]
    fn from(variant: Lj) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LJ` reader - EBUS Low Jitter"]
pub type LjR = crate::BitReader<Lj>;
impl LjR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lj {
        match self.bits {
            false => Lj::Value1,
            true => Lj::Value2,
        }
    }
    #[doc = "Normal jitter"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lj::Value1
    }
    #[doc = "Reduced jitter"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lj::Value2
    }
}
#[doc = "EBUS remote link down signaling time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RldSt {
    #[doc = "0: Default (~660 ms)"]
    Value1 = 0,
    #[doc = "1: Reduced (~80 us)"]
    Value2 = 1,
}
impl From<RldSt> for bool {
    #[inline(always)]
    fn from(variant: RldSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RLD_ST` reader - EBUS remote link down signaling time"]
pub type RldStR = crate::BitReader<RldSt>;
impl RldStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RldSt {
        match self.bits {
            false => RldSt::Value1,
            true => RldSt::Value2,
        }
    }
    #[doc = "Default (~660 ms)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RldSt::Value1
    }
    #[doc = "Reduced (~80 us)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RldSt::Value2
    }
}
#[doc = "Station alias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAlias {
    #[doc = "0: Ignore Station Alias"]
    Value1 = 0,
    #[doc = "1: Alias can be used for all configured address command types (FPRD,FPWR,...)"]
    Value2 = 1,
}
impl From<SAlias> for bool {
    #[inline(always)]
    fn from(variant: SAlias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S_ALIAS` reader - Station alias"]
pub type SAliasR = crate::BitReader<SAlias>;
impl SAliasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SAlias {
        match self.bits {
            false => SAlias::Value1,
            true => SAlias::Value2,
        }
    }
    #[doc = "Ignore Station Alias"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SAlias::Value1
    }
    #[doc = "Alias can be used for all configured address command types (FPRD,FPWR,...)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SAlias::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Forwarding rule"]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Temporary use of settings in LP1-LP3"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Loop Port 0"]
    #[inline(always)]
    pub fn lp0(&self) -> Lp0R {
        Lp0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Loop Port 1"]
    #[inline(always)]
    pub fn lp1(&self) -> Lp1R {
        Lp1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Loop Port 2"]
    #[inline(always)]
    pub fn lp2(&self) -> Lp2R {
        Lp2R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Loop Port 3"]
    #[inline(always)]
    pub fn lp3(&self) -> Lp3R {
        Lp3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:18 - RX FIFO Size"]
    #[inline(always)]
    pub fn rx_fifo_size(&self) -> RxFifoSizeR {
        RxFifoSizeR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - EBUS Low Jitter"]
    #[inline(always)]
    pub fn lj(&self) -> LjR {
        LjR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - EBUS remote link down signaling time"]
    #[inline(always)]
    pub fn rld_st(&self) -> RldStR {
        RldStR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Station alias"]
    #[inline(always)]
    pub fn s_alias(&self) -> SAliasR {
        SAliasR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "ESC DL Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_dl_control::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EscDlControlSpec;
impl crate::RegisterSpec for EscDlControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_dl_control::R`](R) reader structure"]
impl crate::Readable for EscDlControlSpec {}
#[doc = "`reset()` method sets ESC_DL_CONTROL to value 0x0007_c001"]
impl crate::Resettable for EscDlControlSpec {
    const RESET_VALUE: u32 = 0x0007_c001;
}
