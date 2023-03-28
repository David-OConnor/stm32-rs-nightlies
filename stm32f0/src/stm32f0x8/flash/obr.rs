#[doc = "Register `OBR` reader"]
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OPTERR_R = crate::BitReader<OPTERR_A>;
#[doc = "Option byte error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTERR_A {
    #[doc = "1: The loaded option byte and its complement do not match"]
    OptionByteError = 1,
}
impl From<OPTERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTERR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTERR_A> {
        match self.bits {
            true => Some(OPTERR_A::OptionByteError),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OptionByteError`"]
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR_A::OptionByteError
    }
}
#[doc = "Field `RDPRT` reader - Read protection level status"]
pub type RDPRT_R = crate::FieldReader<u8, RDPRT_A>;
#[doc = "Read protection level status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPRT_A {
    #[doc = "0: Level 0"]
    Level0 = 0,
    #[doc = "1: Level 1"]
    Level1 = 1,
    #[doc = "3: Level 2"]
    Level2 = 3,
}
impl From<RDPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT_A) -> Self {
        variant as _
    }
}
impl RDPRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDPRT_A> {
        match self.bits {
            0 => Some(RDPRT_A::Level0),
            1 => Some(RDPRT_A::Level1),
            3 => Some(RDPRT_A::Level2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Level0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT_A::Level0
    }
    #[doc = "Checks if the value of the field is `Level1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT_A::Level1
    }
    #[doc = "Checks if the value of the field is `Level2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT_A::Level2
    }
}
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<WDG_SW_A>;
#[doc = "WDG_SW\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDG_SW_A {
    #[doc = "0: Hardware watchdog"]
    Hardware = 0,
    #[doc = "1: Software watchdog"]
    Software = 1,
}
impl From<WDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl WDG_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDG_SW_A {
        match self.bits {
            false => WDG_SW_A::Hardware,
            true => WDG_SW_A::Software,
        }
    }
    #[doc = "Checks if the value of the field is `Hardware`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW_A::Hardware
    }
    #[doc = "Checks if the value of the field is `Software`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW_A::Software
    }
}
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP_A>;
#[doc = "nRST_STOP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP_A {
    #[doc = "0: Reset generated when entering Stop mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<N_RST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STOP_A {
        match self.bits {
            false => N_RST_STOP_A::Reset,
            true => N_RST_STOP_A::NoReset,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP_A::Reset
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP_A::NoReset
    }
}
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY_A>;
#[doc = "nRST_STDBY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY_A {
    #[doc = "0: Reset generated when entering Standby mode"]
    Reset = 0,
    #[doc = "1: No reset generated"]
    NoReset = 1,
}
impl From<N_RST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STDBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STDBY_A {
        match self.bits {
            false => N_RST_STDBY_A::Reset,
            true => N_RST_STDBY_A::NoReset,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY_A::Reset
    }
    #[doc = "Checks if the value of the field is `NoReset`"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY_A::NoReset
    }
}
#[doc = "Field `nBOOT0` reader - nBOOT0"]
pub type N_BOOT0_R = crate::BitReader<N_BOOT0_A>;
#[doc = "nBOOT0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT0_A {
    #[doc = "0: When BOOT_SEL is cleared, select the device boot mode"]
    Disabled = 0,
    #[doc = "1: When BOOT_SEL is cleared, select the device boot mode"]
    Enabled = 1,
}
impl From<N_BOOT0_A> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT0_A) -> Self {
        variant as u8 != 0
    }
}
impl N_BOOT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_BOOT0_A {
        match self.bits {
            false => N_BOOT0_A::Disabled,
            true => N_BOOT0_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT0_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT0_A::Enabled
    }
}
#[doc = "Field `nBOOT1` reader - BOOT1"]
pub type N_BOOT1_R = crate::BitReader<N_BOOT1_A>;
#[doc = "BOOT1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1_A {
    #[doc = "0: Together with BOOT0, select the device boot mode"]
    Disabled = 0,
    #[doc = "1: Together with BOOT0, select the device boot mode"]
    Enabled = 1,
}
impl From<N_BOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1_A) -> Self {
        variant as u8 != 0
    }
}
impl N_BOOT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> N_BOOT1_A {
        match self.bits {
            false => N_BOOT1_A::Disabled,
            true => N_BOOT1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT1_A::Enabled
    }
}
#[doc = "Field `VDDA_MONITOR` reader - VDDA_MONITOR"]
pub type VDDA_MONITOR_R = crate::BitReader<VDDA_MONITOR_A>;
#[doc = "VDDA_MONITOR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA_MONITOR_A {
    #[doc = "0: VDDA power supply supervisor disabled"]
    Disabled = 0,
    #[doc = "1: VDDA power supply supervisor enabled"]
    Enabled = 1,
}
impl From<VDDA_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDA_MONITOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VDDA_MONITOR_A {
        match self.bits {
            false => VDDA_MONITOR_A::Disabled,
            true => VDDA_MONITOR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Enabled
    }
}
#[doc = "Field `RAM_PARITY_CHECK` reader - RAM_PARITY_CHECK"]
pub type RAM_PARITY_CHECK_R = crate::BitReader<RAM_PARITY_CHECK_A>;
#[doc = "RAM_PARITY_CHECK\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_PARITY_CHECK_A {
    #[doc = "0: RAM parity check enabled"]
    Enabled = 0,
    #[doc = "1: RAM parity check disabled"]
    Disabled = 1,
}
impl From<RAM_PARITY_CHECK_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_PARITY_CHECK_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_PARITY_CHECK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_PARITY_CHECK_A {
        match self.bits {
            false => RAM_PARITY_CHECK_A::Enabled,
            true => RAM_PARITY_CHECK_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAM_PARITY_CHECK_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAM_PARITY_CHECK_A::Disabled
    }
}
#[doc = "Field `BOOT_SEL` reader - BOOT_SEL"]
pub type BOOT_SEL_R = crate::BitReader<BOOT_SEL_A>;
#[doc = "BOOT_SEL\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_SEL_A {
    #[doc = "0: BOOT0 signal is defined by nBOOT0 option bit"]
    NBoot0 = 0,
    #[doc = "1: BOOT0 signal is defined by BOOT0 pin value (legacy mode)"]
    Boot0 = 1,
}
impl From<BOOT_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_SEL_A {
        match self.bits {
            false => BOOT_SEL_A::NBoot0,
            true => BOOT_SEL_A::Boot0,
        }
    }
    #[doc = "Checks if the value of the field is `NBoot0`"]
    #[inline(always)]
    pub fn is_n_boot0(&self) -> bool {
        *self == BOOT_SEL_A::NBoot0
    }
    #[doc = "Checks if the value of the field is `Boot0`"]
    #[inline(always)]
    pub fn is_boot0(&self) -> bool {
        *self == BOOT_SEL_A::Boot0
    }
}
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Read protection level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 8 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - nRST_STOP"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - nBOOT0"]
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOT1"]
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDA_MONITOR"]
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RAM_PARITY_CHECK"]
    #[inline(always)]
    pub fn ram_parity_check(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BOOT_SEL"]
    #[inline(always)]
    pub fn boot_sel(&self) -> BOOT_SEL_R {
        BOOT_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Option byte register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obr](index.html) module"]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [obr::R](R) reader structure"]
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OBR to value 0x03ff_fff2"]
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_fff2;
}
