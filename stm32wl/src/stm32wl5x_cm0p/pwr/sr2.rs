#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `C2BOOTS` reader - PU2 boot/wakeup request source information"]
pub type C2BOOTS_R = crate::BitReader<bool>;
#[doc = "Field `RFBUSYS` reader - Radio BUSY signal status"]
pub type RFBUSYS_R = crate::BitReader<RFBUSYS_A>;
#[doc = "Radio BUSY signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYS_A {
    #[doc = "0: radio busy signal low (not busy)"]
    NotBusy = 0,
    #[doc = "1: radio busy signal high (busy)"]
    Busy = 1,
}
impl From<RFBUSYS_A> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYS_A) -> Self {
        variant as u8 != 0
    }
}
impl RFBUSYS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFBUSYS_A {
        match self.bits {
            false => RFBUSYS_A::NotBusy,
            true => RFBUSYS_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYS_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYS_A::Busy
    }
}
#[doc = "Field `RFBUSYMS` reader - Radio BUSY masked signal status"]
pub type RFBUSYMS_R = crate::BitReader<RFBUSYMS_A>;
#[doc = "Radio BUSY masked signal status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFBUSYMS_A {
    #[doc = "0: radio busy masked signal low (not busy)"]
    NotBusy = 0,
    #[doc = "1: radio busy masked signal high (busy)"]
    Busy = 1,
}
impl From<RFBUSYMS_A> for bool {
    #[inline(always)]
    fn from(variant: RFBUSYMS_A) -> Self {
        variant as u8 != 0
    }
}
impl RFBUSYMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFBUSYMS_A {
        match self.bits {
            false => RFBUSYMS_A::NotBusy,
            true => RFBUSYMS_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == RFBUSYMS_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == RFBUSYMS_A::Busy
    }
}
#[doc = "Field `SMPSRDY` reader - SMPS ready flag"]
pub type SMPSRDY_R = crate::BitReader<SMPSRDY_A>;
#[doc = "SMPS ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPSRDY_A {
    #[doc = "0: SMPS step-down converter not ready or off"]
    NotReady = 0,
    #[doc = "1: SMPS step-down converter ready"]
    Ready = 1,
}
impl From<SMPSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SMPSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPSRDY_A {
        match self.bits {
            false => SMPSRDY_A::NotReady,
            true => SMPSRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == SMPSRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SMPSRDY_A::Ready
    }
}
#[doc = "Field `LDORDY` reader - LDO ready flag"]
pub type LDORDY_R = crate::BitReader<LDORDY_A>;
#[doc = "LDO ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDORDY_A {
    #[doc = "0: LDO not ready or off"]
    NotReady = 0,
    #[doc = "1: LDO ready"]
    Ready = 1,
}
impl From<LDORDY_A> for bool {
    #[inline(always)]
    fn from(variant: LDORDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LDORDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDORDY_A {
        match self.bits {
            false => LDORDY_A::NotReady,
            true => LDORDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LDORDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LDORDY_A::Ready
    }
}
#[doc = "Field `RFEOLF` reader - Radio end of life flag"]
pub type RFEOLF_R = crate::BitReader<RFEOLF_A>;
#[doc = "Radio end of life flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEOLF_A {
    #[doc = "0: Supply voltage above radio end-of-life operating low level"]
    Above = 0,
    #[doc = "1: Supply voltage below radio end-of-life operating low level"]
    Below = 1,
}
impl From<RFEOLF_A> for bool {
    #[inline(always)]
    fn from(variant: RFEOLF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEOLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEOLF_A {
        match self.bits {
            false => RFEOLF_A::Above,
            true => RFEOLF_A::Below,
        }
    }
    #[doc = "Checks if the value of the field is `Above`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RFEOLF_A::Above
    }
    #[doc = "Checks if the value of the field is `Below`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RFEOLF_A::Below
    }
}
#[doc = "Field `REGMRS` reader - regulator2 low power flag"]
pub type REGMRS_R = crate::BitReader<REGMRS_A>;
#[doc = "regulator2 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGMRS_A {
    #[doc = "0: Main regulator supplied directly from VDD"]
    VDd = 0,
    #[doc = "1: Main regulator supplied through LDO or SMPS"]
    LdoSmps = 1,
}
impl From<REGMRS_A> for bool {
    #[inline(always)]
    fn from(variant: REGMRS_A) -> Self {
        variant as u8 != 0
    }
}
impl REGMRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGMRS_A {
        match self.bits {
            false => REGMRS_A::VDd,
            true => REGMRS_A::LdoSmps,
        }
    }
    #[doc = "Checks if the value of the field is `VDd`"]
    #[inline(always)]
    pub fn is_v_dd(&self) -> bool {
        *self == REGMRS_A::VDd
    }
    #[doc = "Checks if the value of the field is `LdoSmps`"]
    #[inline(always)]
    pub fn is_ldo_smps(&self) -> bool {
        *self == REGMRS_A::LdoSmps
    }
}
#[doc = "Field `FLASHRDY` reader - Flash ready"]
pub type FLASHRDY_R = crate::BitReader<FLASHRDY_A>;
#[doc = "Flash ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHRDY_A {
    #[doc = "0: Flash memory not ready to be accessed"]
    NotReady = 0,
    #[doc = "1: Flash memory ready to be accessed"]
    Ready = 1,
}
impl From<FLASHRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHRDY_A {
        match self.bits {
            false => FLASHRDY_A::NotReady,
            true => FLASHRDY_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == FLASHRDY_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == FLASHRDY_A::Ready
    }
}
#[doc = "Field `REGLPS` reader - regulator1 started"]
pub type REGLPS_R = crate::BitReader<REGLPS_A>;
#[doc = "regulator1 started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPS_A {
    #[doc = "0: LPR not ready"]
    NotReady = 0,
    #[doc = "1: LPR ready"]
    Ready = 1,
}
impl From<REGLPS_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPS_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPS_A {
        match self.bits {
            false => REGLPS_A::NotReady,
            true => REGLPS_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPS_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPS_A::Ready
    }
}
#[doc = "Field `REGLPF` reader - regulator1 low power flag"]
pub type REGLPF_R = crate::BitReader<REGLPF_A>;
#[doc = "regulator1 low power flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPF_A {
    #[doc = "0: Main regulator (MR) ready and used"]
    Main = 0,
    #[doc = "1: Low-power regulator (LPR) used"]
    LowPower = 1,
}
impl From<REGLPF_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPF_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPF_A {
        match self.bits {
            false => REGLPF_A::Main,
            true => REGLPF_A::LowPower,
        }
    }
    #[doc = "Checks if the value of the field is `Main`"]
    #[inline(always)]
    pub fn is_main(&self) -> bool {
        *self == REGLPF_A::Main
    }
    #[doc = "Checks if the value of the field is `LowPower`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == REGLPF_A::LowPower
    }
}
#[doc = "Field `VOSF` reader - Voltage scaling flag"]
pub type VOSF_R = crate::BitReader<VOSF_A>;
#[doc = "Voltage scaling flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSF_A {
    #[doc = "0: Regulator ready in the selected voltage range"]
    Ready = 0,
    #[doc = "1: Regulator output voltage changed to the required voltage level"]
    Change = 1,
}
impl From<VOSF_A> for bool {
    #[inline(always)]
    fn from(variant: VOSF_A) -> Self {
        variant as u8 != 0
    }
}
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOSF_A {
        match self.bits {
            false => VOSF_A::Ready,
            true => VOSF_A::Change,
        }
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSF_A::Ready
    }
    #[doc = "Checks if the value of the field is `Change`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == VOSF_A::Change
    }
}
#[doc = "Field `PVDO` reader - Power voltage detector output"]
pub type PVDO_R = crate::BitReader<PVDO_A>;
#[doc = "Power voltage detector output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDO_A {
    #[doc = "0: VDD or voltage level on PVD_IN above the selected PVD threshold"]
    Above = 0,
    #[doc = "1: VDD or voltage level on PVD_IN below the selected PVD threshold"]
    Below = 1,
}
impl From<PVDO_A> for bool {
    #[inline(always)]
    fn from(variant: PVDO_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDO_A {
        match self.bits {
            false => PVDO_A::Above,
            true => PVDO_A::Below,
        }
    }
    #[doc = "Checks if the value of the field is `Above`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVDO_A::Above
    }
    #[doc = "Checks if the value of the field is `Below`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVDO_A::Below
    }
}
#[doc = "Field `PVMO3` reader - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
pub type PVMO3_R = crate::BitReader<PVMO3_A>;
#[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMO3_A {
    #[doc = "0: VDDA voltage above PVM3 threshold (around 1.62 V)"]
    Above = 0,
    #[doc = "1: VDDA voltage below PVM3 threshold (around 1.62 V)"]
    Below = 1,
}
impl From<PVMO3_A> for bool {
    #[inline(always)]
    fn from(variant: PVMO3_A) -> Self {
        variant as u8 != 0
    }
}
impl PVMO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVMO3_A {
        match self.bits {
            false => PVMO3_A::Above,
            true => PVMO3_A::Below,
        }
    }
    #[doc = "Checks if the value of the field is `Above`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == PVMO3_A::Above
    }
    #[doc = "Checks if the value of the field is `Below`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == PVMO3_A::Below
    }
}
impl R {
    #[doc = "Bit 0 - PU2 boot/wakeup request source information"]
    #[inline(always)]
    pub fn c2boots(&self) -> C2BOOTS_R {
        C2BOOTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Radio BUSY signal status"]
    #[inline(always)]
    pub fn rfbusys(&self) -> RFBUSYS_R {
        RFBUSYS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Radio BUSY masked signal status"]
    #[inline(always)]
    pub fn rfbusyms(&self) -> RFBUSYMS_R {
        RFBUSYMS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMPS ready flag"]
    #[inline(always)]
    pub fn smpsrdy(&self) -> SMPSRDY_R {
        SMPSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LDO ready flag"]
    #[inline(always)]
    pub fn ldordy(&self) -> LDORDY_R {
        LDORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Radio end of life flag"]
    #[inline(always)]
    pub fn rfeolf(&self) -> RFEOLF_R {
        RFEOLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - regulator2 low power flag"]
    #[inline(always)]
    pub fn regmrs(&self) -> REGMRS_R {
        REGMRS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash ready"]
    #[inline(always)]
    pub fn flashrdy(&self) -> FLASHRDY_R {
        FLASHRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - regulator1 started"]
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - regulator1 low power flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Voltage scaling flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power voltage detector output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
    #[inline(always)]
    pub fn pvmo3(&self) -> PVMO3_R {
        PVMO3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Power status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
