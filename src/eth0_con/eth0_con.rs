#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ETH0_CON {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RXD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD0R {
    #[doc = "Data input RXD0A is selected"]
    VALUE1,
    #[doc = "Data input RXD0B is selected"]
    VALUE2,
    #[doc = "Data input RXD0C is selected"]
    VALUE3,
    #[doc = "Data input RXD0D is selected"]
    VALUE4,
}
impl RXD0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD0R::VALUE1 => 0,
            RXD0R::VALUE2 => 1,
            RXD0R::VALUE3 => 2,
            RXD0R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD0R {
        match value {
            0 => RXD0R::VALUE1,
            1 => RXD0R::VALUE2,
            2 => RXD0R::VALUE3,
            3 => RXD0R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD0R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD0R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD0R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD0R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD1R {
    #[doc = "Data input RXD1A is selected"]
    VALUE1,
    #[doc = "Data input RXD1B is selected"]
    VALUE2,
    #[doc = "Data input RXD1C is selected"]
    VALUE3,
    #[doc = "Data input RXD1D is selected"]
    VALUE4,
}
impl RXD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD1R::VALUE1 => 0,
            RXD1R::VALUE2 => 1,
            RXD1R::VALUE3 => 2,
            RXD1R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD1R {
        match value {
            0 => RXD1R::VALUE1,
            1 => RXD1R::VALUE2,
            2 => RXD1R::VALUE3,
            3 => RXD1R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD1R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD1R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD1R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD1R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD2R {
    #[doc = "Data input RXD2A is selected"]
    VALUE1,
    #[doc = "Data input RXD2B is selected"]
    VALUE2,
    #[doc = "Data input RXD2C is selected"]
    VALUE3,
    #[doc = "Data input RXD2D is selected"]
    VALUE4,
}
impl RXD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD2R::VALUE1 => 0,
            RXD2R::VALUE2 => 1,
            RXD2R::VALUE3 => 2,
            RXD2R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD2R {
        match value {
            0 => RXD2R::VALUE1,
            1 => RXD2R::VALUE2,
            2 => RXD2R::VALUE3,
            3 => RXD2R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD2R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD2R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD2R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD2R::VALUE4
    }
}
#[doc = "Possible values of the field `RXD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXD3R {
    #[doc = "Data input RXD3A is selected"]
    VALUE1,
    #[doc = "Data input RXD3B is selected"]
    VALUE2,
    #[doc = "Data input RXD3C is selected"]
    VALUE3,
    #[doc = "Data input RXD3D is selected"]
    VALUE4,
}
impl RXD3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXD3R::VALUE1 => 0,
            RXD3R::VALUE2 => 1,
            RXD3R::VALUE3 => 2,
            RXD3R::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXD3R {
        match value {
            0 => RXD3R::VALUE1,
            1 => RXD3R::VALUE2,
            2 => RXD3R::VALUE3,
            3 => RXD3R::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXD3R::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXD3R::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXD3R::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXD3R::VALUE4
    }
}
#[doc = "Possible values of the field `CLK_RMII`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_RMIIR {
    #[doc = "Data input RMIIA is selected"]
    VALUE1,
    #[doc = "Data input RMIIB is selected"]
    VALUE2,
    #[doc = "Data input RMIIC is selected"]
    VALUE3,
    #[doc = "Data input RMIID is selected"]
    VALUE4,
}
impl CLK_RMIIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_RMIIR::VALUE1 => 0,
            CLK_RMIIR::VALUE2 => 1,
            CLK_RMIIR::VALUE3 => 2,
            CLK_RMIIR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_RMIIR {
        match value {
            0 => CLK_RMIIR::VALUE1,
            1 => CLK_RMIIR::VALUE2,
            2 => CLK_RMIIR::VALUE3,
            3 => CLK_RMIIR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLK_RMIIR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLK_RMIIR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLK_RMIIR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLK_RMIIR::VALUE4
    }
}
#[doc = "Possible values of the field `CRS_DV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRS_DVR {
    #[doc = "Data input CRS_DVA is selected"]
    VALUE1,
    #[doc = "Data input CRS_DVB is selected"]
    VALUE2,
    #[doc = "Data input CRS_DVC is selected"]
    VALUE3,
    #[doc = "Data input CRS_DVD is selected"]
    VALUE4,
}
impl CRS_DVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRS_DVR::VALUE1 => 0,
            CRS_DVR::VALUE2 => 1,
            CRS_DVR::VALUE3 => 2,
            CRS_DVR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRS_DVR {
        match value {
            0 => CRS_DVR::VALUE1,
            1 => CRS_DVR::VALUE2,
            2 => CRS_DVR::VALUE3,
            3 => CRS_DVR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CRS_DVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CRS_DVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CRS_DVR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CRS_DVR::VALUE4
    }
}
#[doc = "Possible values of the field `CRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSR {
    #[doc = "Data input CRSA"]
    VALUE1,
    #[doc = "Data input CRSB"]
    VALUE2,
    #[doc = "Data input CRSC"]
    VALUE3,
    #[doc = "Data input CRSD"]
    VALUE4,
}
impl CRSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRSR::VALUE1 => 0,
            CRSR::VALUE2 => 1,
            CRSR::VALUE3 => 2,
            CRSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRSR {
        match value {
            0 => CRSR::VALUE1,
            1 => CRSR::VALUE2,
            2 => CRSR::VALUE3,
            3 => CRSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CRSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CRSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CRSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CRSR::VALUE4
    }
}
#[doc = "Possible values of the field `RXER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERR {
    #[doc = "Data input RXERA is selected"]
    VALUE1,
    #[doc = "Data input RXERB is selected"]
    VALUE2,
    #[doc = "Data input RXERC is selected"]
    VALUE3,
    #[doc = "Data input RXERD is selected"]
    VALUE4,
}
impl RXERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXERR::VALUE1 => 0,
            RXERR::VALUE2 => 1,
            RXERR::VALUE3 => 2,
            RXERR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXERR {
        match value {
            0 => RXERR::VALUE1,
            1 => RXERR::VALUE2,
            2 => RXERR::VALUE3,
            3 => RXERR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXERR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXERR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == RXERR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == RXERR::VALUE4
    }
}
#[doc = "Possible values of the field `COL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLR {
    #[doc = "Data input COLA is selected"]
    VALUE1,
    #[doc = "Data input COLB is selected"]
    VALUE2,
    #[doc = "Data input COLC is selected"]
    VALUE3,
    #[doc = "Data input COLD is selected"]
    VALUE4,
}
impl COLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            COLR::VALUE1 => 0,
            COLR::VALUE2 => 1,
            COLR::VALUE3 => 2,
            COLR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> COLR {
        match value {
            0 => COLR::VALUE1,
            1 => COLR::VALUE2,
            2 => COLR::VALUE3,
            3 => COLR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == COLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == COLR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == COLR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == COLR::VALUE4
    }
}
#[doc = "Possible values of the field `CLK_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_TXR {
    #[doc = "Data input CLK_TXA is selected"]
    VALUE1,
    #[doc = "Data input CLK_TXB is selected"]
    VALUE2,
    #[doc = "Data input CLK_TXC is selected"]
    VALUE3,
    #[doc = "Data input CLK_TXD is selected"]
    VALUE4,
}
impl CLK_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLK_TXR::VALUE1 => 0,
            CLK_TXR::VALUE2 => 1,
            CLK_TXR::VALUE3 => 2,
            CLK_TXR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLK_TXR {
        match value {
            0 => CLK_TXR::VALUE1,
            1 => CLK_TXR::VALUE2,
            2 => CLK_TXR::VALUE3,
            3 => CLK_TXR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == CLK_TXR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == CLK_TXR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == CLK_TXR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == CLK_TXR::VALUE4
    }
}
#[doc = "Possible values of the field `MDIO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIOR {
    #[doc = "Data input MDIA is selected"]
    VALUE1,
    #[doc = "Data input MDIB is selected"]
    VALUE2,
    #[doc = "Data input MDIC is selected"]
    VALUE3,
    #[doc = "Data input MDID is selected"]
    VALUE4,
}
impl MDIOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDIOR::VALUE1 => 0,
            MDIOR::VALUE2 => 1,
            MDIOR::VALUE3 => 2,
            MDIOR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDIOR {
        match value {
            0 => MDIOR::VALUE1,
            1 => MDIOR::VALUE2,
            2 => MDIOR::VALUE3,
            3 => MDIOR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MDIOR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MDIOR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MDIOR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MDIOR::VALUE4
    }
}
#[doc = "Possible values of the field `INFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFSELR {
    #[doc = "MII"]
    VALUE1,
    #[doc = "RMII"]
    VALUE2,
}
impl INFSELR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INFSELR::VALUE1 => false,
            INFSELR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INFSELR {
        match value {
            false => INFSELR::VALUE1,
            true => INFSELR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == INFSELR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == INFSELR::VALUE2
    }
}
#[doc = "Values that can be written to the field `RXD0`"]
pub enum RXD0W {
    #[doc = "Data input RXD0A is selected"]
    VALUE1,
    #[doc = "Data input RXD0B is selected"]
    VALUE2,
    #[doc = "Data input RXD0C is selected"]
    VALUE3,
    #[doc = "Data input RXD0D is selected"]
    VALUE4,
}
impl RXD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD0W::VALUE1 => 0,
            RXD0W::VALUE2 => 1,
            RXD0W::VALUE3 => 2,
            RXD0W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD0W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD0A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD0W::VALUE1)
    }
    #[doc = "Data input RXD0B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD0W::VALUE2)
    }
    #[doc = "Data input RXD0C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD0W::VALUE3)
    }
    #[doc = "Data input RXD0D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD0W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD1`"]
pub enum RXD1W {
    #[doc = "Data input RXD1A is selected"]
    VALUE1,
    #[doc = "Data input RXD1B is selected"]
    VALUE2,
    #[doc = "Data input RXD1C is selected"]
    VALUE3,
    #[doc = "Data input RXD1D is selected"]
    VALUE4,
}
impl RXD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD1W::VALUE1 => 0,
            RXD1W::VALUE2 => 1,
            RXD1W::VALUE3 => 2,
            RXD1W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD1W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD1A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD1W::VALUE1)
    }
    #[doc = "Data input RXD1B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD1W::VALUE2)
    }
    #[doc = "Data input RXD1C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD1W::VALUE3)
    }
    #[doc = "Data input RXD1D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD1W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD2`"]
pub enum RXD2W {
    #[doc = "Data input RXD2A is selected"]
    VALUE1,
    #[doc = "Data input RXD2B is selected"]
    VALUE2,
    #[doc = "Data input RXD2C is selected"]
    VALUE3,
    #[doc = "Data input RXD2D is selected"]
    VALUE4,
}
impl RXD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD2W::VALUE1 => 0,
            RXD2W::VALUE2 => 1,
            RXD2W::VALUE3 => 2,
            RXD2W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD2W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD2A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD2W::VALUE1)
    }
    #[doc = "Data input RXD2B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD2W::VALUE2)
    }
    #[doc = "Data input RXD2C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD2W::VALUE3)
    }
    #[doc = "Data input RXD2D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD2W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXD3`"]
pub enum RXD3W {
    #[doc = "Data input RXD3A is selected"]
    VALUE1,
    #[doc = "Data input RXD3B is selected"]
    VALUE2,
    #[doc = "Data input RXD3C is selected"]
    VALUE3,
    #[doc = "Data input RXD3D is selected"]
    VALUE4,
}
impl RXD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXD3W::VALUE1 => 0,
            RXD3W::VALUE2 => 1,
            RXD3W::VALUE3 => 2,
            RXD3W::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXD3W<'a> {
    w: &'a mut W,
}
impl<'a> _RXD3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXD3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXD3A is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXD3W::VALUE1)
    }
    #[doc = "Data input RXD3B is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXD3W::VALUE2)
    }
    #[doc = "Data input RXD3C is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXD3W::VALUE3)
    }
    #[doc = "Data input RXD3D is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXD3W::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_RMII`"]
pub enum CLK_RMIIW {
    #[doc = "Data input RMIIA is selected"]
    VALUE1,
    #[doc = "Data input RMIIB is selected"]
    VALUE2,
    #[doc = "Data input RMIIC is selected"]
    VALUE3,
    #[doc = "Data input RMIID is selected"]
    VALUE4,
}
impl CLK_RMIIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_RMIIW::VALUE1 => 0,
            CLK_RMIIW::VALUE2 => 1,
            CLK_RMIIW::VALUE3 => 2,
            CLK_RMIIW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_RMIIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_RMIIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_RMIIW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RMIIA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLK_RMIIW::VALUE1)
    }
    #[doc = "Data input RMIIB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLK_RMIIW::VALUE2)
    }
    #[doc = "Data input RMIIC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLK_RMIIW::VALUE3)
    }
    #[doc = "Data input RMIID is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLK_RMIIW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRS_DV`"]
pub enum CRS_DVW {
    #[doc = "Data input CRS_DVA is selected"]
    VALUE1,
    #[doc = "Data input CRS_DVB is selected"]
    VALUE2,
    #[doc = "Data input CRS_DVC is selected"]
    VALUE3,
    #[doc = "Data input CRS_DVD is selected"]
    VALUE4,
}
impl CRS_DVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRS_DVW::VALUE1 => 0,
            CRS_DVW::VALUE2 => 1,
            CRS_DVW::VALUE3 => 2,
            CRS_DVW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRS_DVW<'a> {
    w: &'a mut W,
}
impl<'a> _CRS_DVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRS_DVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input CRS_DVA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRS_DVW::VALUE1)
    }
    #[doc = "Data input CRS_DVB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRS_DVW::VALUE2)
    }
    #[doc = "Data input CRS_DVC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRS_DVW::VALUE3)
    }
    #[doc = "Data input CRS_DVD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRS_DVW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRS`"]
pub enum CRSW {
    #[doc = "Data input CRSA"]
    VALUE1,
    #[doc = "Data input CRSB"]
    VALUE2,
    #[doc = "Data input CRSC"]
    VALUE3,
    #[doc = "Data input CRSD"]
    VALUE4,
}
impl CRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRSW::VALUE1 => 0,
            CRSW::VALUE2 => 1,
            CRSW::VALUE3 => 2,
            CRSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRSW<'a> {
    w: &'a mut W,
}
impl<'a> _CRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input CRSA"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRSW::VALUE1)
    }
    #[doc = "Data input CRSB"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRSW::VALUE2)
    }
    #[doc = "Data input CRSC"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRSW::VALUE3)
    }
    #[doc = "Data input CRSD"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXER`"]
pub enum RXERW {
    #[doc = "Data input RXERA is selected"]
    VALUE1,
    #[doc = "Data input RXERB is selected"]
    VALUE2,
    #[doc = "Data input RXERC is selected"]
    VALUE3,
    #[doc = "Data input RXERD is selected"]
    VALUE4,
}
impl RXERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXERW::VALUE1 => 0,
            RXERW::VALUE2 => 1,
            RXERW::VALUE3 => 2,
            RXERW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXERW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input RXERA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXERW::VALUE1)
    }
    #[doc = "Data input RXERB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXERW::VALUE2)
    }
    #[doc = "Data input RXERC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(RXERW::VALUE3)
    }
    #[doc = "Data input RXERD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(RXERW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COL`"]
pub enum COLW {
    #[doc = "Data input COLA is selected"]
    VALUE1,
    #[doc = "Data input COLB is selected"]
    VALUE2,
    #[doc = "Data input COLC is selected"]
    VALUE3,
    #[doc = "Data input COLD is selected"]
    VALUE4,
}
impl COLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            COLW::VALUE1 => 0,
            COLW::VALUE2 => 1,
            COLW::VALUE3 => 2,
            COLW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COLW<'a> {
    w: &'a mut W,
}
impl<'a> _COLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input COLA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(COLW::VALUE1)
    }
    #[doc = "Data input COLB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(COLW::VALUE2)
    }
    #[doc = "Data input COLC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(COLW::VALUE3)
    }
    #[doc = "Data input COLD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(COLW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK_TX`"]
pub enum CLK_TXW {
    #[doc = "Data input CLK_TXA is selected"]
    VALUE1,
    #[doc = "Data input CLK_TXB is selected"]
    VALUE2,
    #[doc = "Data input CLK_TXC is selected"]
    VALUE3,
    #[doc = "Data input CLK_TXD is selected"]
    VALUE4,
}
impl CLK_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLK_TXW::VALUE1 => 0,
            CLK_TXW::VALUE2 => 1,
            CLK_TXW::VALUE3 => 2,
            CLK_TXW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLK_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLK_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input CLK_TXA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLK_TXW::VALUE1)
    }
    #[doc = "Data input CLK_TXB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLK_TXW::VALUE2)
    }
    #[doc = "Data input CLK_TXC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLK_TXW::VALUE3)
    }
    #[doc = "Data input CLK_TXD is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLK_TXW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDIO`"]
pub enum MDIOW {
    #[doc = "Data input MDIA is selected"]
    VALUE1,
    #[doc = "Data input MDIB is selected"]
    VALUE2,
    #[doc = "Data input MDIC is selected"]
    VALUE3,
    #[doc = "Data input MDID is selected"]
    VALUE4,
}
impl MDIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDIOW::VALUE1 => 0,
            MDIOW::VALUE2 => 1,
            MDIOW::VALUE3 => 2,
            MDIOW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIOW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Data input MDIA is selected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MDIOW::VALUE1)
    }
    #[doc = "Data input MDIB is selected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MDIOW::VALUE2)
    }
    #[doc = "Data input MDIC is selected"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MDIOW::VALUE3)
    }
    #[doc = "Data input MDID is selected"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MDIOW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INFSEL`"]
pub enum INFSELW {
    #[doc = "MII"]
    VALUE1,
    #[doc = "RMII"]
    VALUE2,
}
impl INFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INFSELW::VALUE1 => false,
            INFSELW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INFSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MII"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(INFSELW::VALUE1)
    }
    #[doc = "RMII"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(INFSELW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline]
    pub fn rxd0(&self) -> RXD0R {
        RXD0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline]
    pub fn rxd1(&self) -> RXD1R {
        RXD1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline]
    pub fn rxd2(&self) -> RXD2R {
        RXD2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline]
    pub fn rxd3(&self) -> RXD3R {
        RXD3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline]
    pub fn clk_rmii(&self) -> CLK_RMIIR {
        CLK_RMIIR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline]
    pub fn crs_dv(&self) -> CRS_DVR {
        CRS_DVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline]
    pub fn crs(&self) -> CRSR {
        CRSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline]
    pub fn rxer(&self) -> RXERR {
        RXERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline]
    pub fn col(&self) -> COLR {
        COLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline]
    pub fn clk_tx(&self) -> CLK_TXR {
        CLK_TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline]
    pub fn mdio(&self) -> MDIOR {
        MDIOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline]
    pub fn infsel(&self) -> INFSELR {
        INFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - MAC Receive Input 0"]
    #[inline]
    pub fn rxd0(&mut self) -> _RXD0W {
        _RXD0W { w: self }
    }
    #[doc = "Bits 2:3 - MAC Receive Input 1"]
    #[inline]
    pub fn rxd1(&mut self) -> _RXD1W {
        _RXD1W { w: self }
    }
    #[doc = "Bits 4:5 - MAC Receive Input 2"]
    #[inline]
    pub fn rxd2(&mut self) -> _RXD2W {
        _RXD2W { w: self }
    }
    #[doc = "Bits 6:7 - MAC Receive Input 3"]
    #[inline]
    pub fn rxd3(&mut self) -> _RXD3W {
        _RXD3W { w: self }
    }
    #[doc = "Bits 8:9 - RMII clock input"]
    #[inline]
    pub fn clk_rmii(&mut self) -> _CLK_RMIIW {
        _CLK_RMIIW { w: self }
    }
    #[doc = "Bits 10:11 - CRS_DV input"]
    #[inline]
    pub fn crs_dv(&mut self) -> _CRS_DVW {
        _CRS_DVW { w: self }
    }
    #[doc = "Bits 12:13 - CRS input"]
    #[inline]
    pub fn crs(&mut self) -> _CRSW {
        _CRSW { w: self }
    }
    #[doc = "Bits 14:15 - RXER Input"]
    #[inline]
    pub fn rxer(&mut self) -> _RXERW {
        _RXERW { w: self }
    }
    #[doc = "Bits 16:17 - COL input"]
    #[inline]
    pub fn col(&mut self) -> _COLW {
        _COLW { w: self }
    }
    #[doc = "Bits 18:19 - CLK_TX input"]
    #[inline]
    pub fn clk_tx(&mut self) -> _CLK_TXW {
        _CLK_TXW { w: self }
    }
    #[doc = "Bits 22:23 - MDIO Input Select"]
    #[inline]
    pub fn mdio(&mut self) -> _MDIOW {
        _MDIOW { w: self }
    }
    #[doc = "Bit 26 - Ethernet MAC Interface Selection"]
    #[inline]
    pub fn infsel(&mut self) -> _INFSELW {
        _INFSELW { w: self }
    }
}
