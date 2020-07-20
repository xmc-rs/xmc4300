#[doc = "Reader of register ESC_DL_CONTROL"]
pub type R = crate::R<u32, super::ESC_DL_CONTROL>;
#[doc = "Forwarding rule\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FR_A {
    #[doc = "0: EtherCAT frames are processed, Non-EtherCAT frames are forwarded without processing"]
    VALUE1 = 0,
    #[doc = "1: EtherCAT frames are processed, Non- EtherCAT frames are destroyed"]
    VALUE2 = 1,
}
impl From<FR_A> for bool {
    #[inline(always)]
    fn from(variant: FR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FR`"]
pub type FR_R = crate::R<bool, FR_A>;
impl FR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FR_A {
        match self.bits {
            false => FR_A::VALUE1,
            true => FR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FR_A::VALUE2
    }
}
#[doc = "Temporary use of settings in LP1-LP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMP_A {
    #[doc = "0: permanent use"]
    VALUE1 = 0,
    #[doc = "1: use for about 1 second, then revert to previous settings"]
    VALUE2 = 1,
}
impl From<TEMP_A> for bool {
    #[inline(always)]
    fn from(variant: TEMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<bool, TEMP_A>;
impl TEMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMP_A {
        match self.bits {
            false => TEMP_A::VALUE1,
            true => TEMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TEMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TEMP_A::VALUE2
    }
}
#[doc = "Loop Port 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LP0_A {
    #[doc = "0: Auto"]
    VALUE1 = 0,
    #[doc = "1: Auto Close"]
    VALUE2 = 1,
    #[doc = "2: Open"]
    VALUE3 = 2,
    #[doc = "3: Closed"]
    VALUE4 = 3,
}
impl From<LP0_A> for u8 {
    #[inline(always)]
    fn from(variant: LP0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LP0`"]
pub type LP0_R = crate::R<u8, LP0_A>;
impl LP0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP0_A {
        match self.bits {
            0 => LP0_A::VALUE1,
            1 => LP0_A::VALUE2,
            2 => LP0_A::VALUE3,
            3 => LP0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LP0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LP0_A::VALUE4
    }
}
#[doc = "Loop Port 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LP1_A {
    #[doc = "0: Auto"]
    VALUE1 = 0,
    #[doc = "1: Auto Close"]
    VALUE2 = 1,
    #[doc = "2: Open"]
    VALUE3 = 2,
    #[doc = "3: Closed"]
    VALUE4 = 3,
}
impl From<LP1_A> for u8 {
    #[inline(always)]
    fn from(variant: LP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LP1`"]
pub type LP1_R = crate::R<u8, LP1_A>;
impl LP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP1_A {
        match self.bits {
            0 => LP1_A::VALUE1,
            1 => LP1_A::VALUE2,
            2 => LP1_A::VALUE3,
            3 => LP1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LP1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LP1_A::VALUE4
    }
}
#[doc = "Loop Port 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LP2_A {
    #[doc = "0: Auto"]
    VALUE1 = 0,
    #[doc = "1: Auto Close"]
    VALUE2 = 1,
    #[doc = "2: Open"]
    VALUE3 = 2,
    #[doc = "3: Closed"]
    VALUE4 = 3,
}
impl From<LP2_A> for u8 {
    #[inline(always)]
    fn from(variant: LP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LP2`"]
pub type LP2_R = crate::R<u8, LP2_A>;
impl LP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP2_A {
        match self.bits {
            0 => LP2_A::VALUE1,
            1 => LP2_A::VALUE2,
            2 => LP2_A::VALUE3,
            3 => LP2_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP2_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP2_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LP2_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LP2_A::VALUE4
    }
}
#[doc = "Loop Port 3\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LP3_A {
    #[doc = "0: Auto"]
    VALUE1 = 0,
    #[doc = "1: Auto Close"]
    VALUE2 = 1,
    #[doc = "2: Open"]
    VALUE3 = 2,
    #[doc = "3: Closed"]
    VALUE4 = 3,
}
impl From<LP3_A> for u8 {
    #[inline(always)]
    fn from(variant: LP3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LP3`"]
pub type LP3_R = crate::R<u8, LP3_A>;
impl LP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP3_A {
        match self.bits {
            0 => LP3_A::VALUE1,
            1 => LP3_A::VALUE2,
            2 => LP3_A::VALUE3,
            3 => LP3_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LP3_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LP3_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == LP3_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == LP3_A::VALUE4
    }
}
#[doc = "RX FIFO Size\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RX_FIFO_SIZE_A {
    #[doc = "0: -40 ns (-80 ns)"]
    VALUE1 = 0,
    #[doc = "1: -40 ns (-80 ns)"]
    VALUE2 = 1,
    #[doc = "2: -40 ns"]
    VALUE3 = 2,
    #[doc = "3: -40 ns"]
    VALUE4 = 3,
    #[doc = "4: no change"]
    VALUE5 = 4,
    #[doc = "5: no change"]
    VALUE6 = 5,
    #[doc = "6: no change"]
    VALUE7 = 6,
    #[doc = "7: default"]
    VALUE8 = 7,
}
impl From<RX_FIFO_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FIFO_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX_FIFO_SIZE`"]
pub type RX_FIFO_SIZE_R = crate::R<u8, RX_FIFO_SIZE_A>;
impl RX_FIFO_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_SIZE_A {
        match self.bits {
            0 => RX_FIFO_SIZE_A::VALUE1,
            1 => RX_FIFO_SIZE_A::VALUE2,
            2 => RX_FIFO_SIZE_A::VALUE3,
            3 => RX_FIFO_SIZE_A::VALUE4,
            4 => RX_FIFO_SIZE_A::VALUE5,
            5 => RX_FIFO_SIZE_A::VALUE6,
            6 => RX_FIFO_SIZE_A::VALUE7,
            7 => RX_FIFO_SIZE_A::VALUE8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == RX_FIFO_SIZE_A::VALUE8
    }
}
#[doc = "EBUS Low Jitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LJ_A {
    #[doc = "0: Normal jitter"]
    VALUE1 = 0,
    #[doc = "1: Reduced jitter"]
    VALUE2 = 1,
}
impl From<LJ_A> for bool {
    #[inline(always)]
    fn from(variant: LJ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LJ`"]
pub type LJ_R = crate::R<bool, LJ_A>;
impl LJ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LJ_A {
        match self.bits {
            false => LJ_A::VALUE1,
            true => LJ_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LJ_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LJ_A::VALUE2
    }
}
#[doc = "EBUS remote link down signaling time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLD_ST_A {
    #[doc = "0: Default (~660 ms)"]
    VALUE1 = 0,
    #[doc = "1: Reduced (~80 us)"]
    VALUE2 = 1,
}
impl From<RLD_ST_A> for bool {
    #[inline(always)]
    fn from(variant: RLD_ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RLD_ST`"]
pub type RLD_ST_R = crate::R<bool, RLD_ST_A>;
impl RLD_ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLD_ST_A {
        match self.bits {
            false => RLD_ST_A::VALUE1,
            true => RLD_ST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RLD_ST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RLD_ST_A::VALUE2
    }
}
#[doc = "Station alias\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_ALIAS_A {
    #[doc = "0: Ignore Station Alias"]
    VALUE1 = 0,
    #[doc = "1: Alias can be used for all configured address command types (FPRD,FPWR,...)"]
    VALUE2 = 1,
}
impl From<S_ALIAS_A> for bool {
    #[inline(always)]
    fn from(variant: S_ALIAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S_ALIAS`"]
pub type S_ALIAS_R = crate::R<bool, S_ALIAS_A>;
impl S_ALIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_ALIAS_A {
        match self.bits {
            false => S_ALIAS_A::VALUE1,
            true => S_ALIAS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S_ALIAS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S_ALIAS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Forwarding rule"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Temporary use of settings in LP1-LP3"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Loop Port 0"]
    #[inline(always)]
    pub fn lp0(&self) -> LP0_R {
        LP0_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Loop Port 1"]
    #[inline(always)]
    pub fn lp1(&self) -> LP1_R {
        LP1_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Loop Port 2"]
    #[inline(always)]
    pub fn lp2(&self) -> LP2_R {
        LP2_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Loop Port 3"]
    #[inline(always)]
    pub fn lp3(&self) -> LP3_R {
        LP3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - RX FIFO Size"]
    #[inline(always)]
    pub fn rx_fifo_size(&self) -> RX_FIFO_SIZE_R {
        RX_FIFO_SIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - EBUS Low Jitter"]
    #[inline(always)]
    pub fn lj(&self) -> LJ_R {
        LJ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EBUS remote link down signaling time"]
    #[inline(always)]
    pub fn rld_st(&self) -> RLD_ST_R {
        RLD_ST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Station alias"]
    #[inline(always)]
    pub fn s_alias(&self) -> S_ALIAS_R {
        S_ALIAS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
