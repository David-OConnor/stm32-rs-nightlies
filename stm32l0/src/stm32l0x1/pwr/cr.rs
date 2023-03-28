#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPSDSR` reader - Low-power deepsleep/Sleep/Low-power run"]
pub type LPSDSR_R = crate::BitReader<LPSDSR_A>;
#[doc = "Low-power deepsleep/Sleep/Low-power run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSDSR_A {
    #[doc = "0: Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    MainMode = 0,
    #[doc = "1: Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    LowPowerMode = 1,
}
impl From<LPSDSR_A> for bool {
    #[inline(always)]
    fn from(variant: LPSDSR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSDSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSDSR_A {
        match self.bits {
            false => LPSDSR_A::MainMode,
            true => LPSDSR_A::LowPowerMode,
        }
    }
    #[doc = "Checks if the value of the field is `MainMode`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPSDSR_A::MainMode
    }
    #[doc = "Checks if the value of the field is `LowPowerMode`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPSDSR_A::LowPowerMode
    }
}
#[doc = "Field `LPSDSR` writer - Low-power deepsleep/Sleep/Low-power run"]
pub type LPSDSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LPSDSR_A, O>;
impl<'a, const O: u8> LPSDSR_W<'a, O> {
    #[doc = "Voltage regulator on during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::MainMode)
    }
    #[doc = "Voltage regulator in low-power mode during Deepsleep/Sleep/Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPSDSR_A::LowPowerMode)
    }
}
#[doc = "Field `PDDS` reader - Power down deepsleep"]
pub type PDDS_R = crate::BitReader<PDDS_A>;
#[doc = "Power down deepsleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDDS_A {
    #[doc = "0: Enter Stop mode when the CPU enters deepsleep"]
    StopMode = 0,
    #[doc = "1: Enter Standby mode when the CPU enters deepsleep"]
    StandbyMode = 1,
}
impl From<PDDS_A> for bool {
    #[inline(always)]
    fn from(variant: PDDS_A) -> Self {
        variant as u8 != 0
    }
}
impl PDDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDDS_A {
        match self.bits {
            false => PDDS_A::StopMode,
            true => PDDS_A::StandbyMode,
        }
    }
    #[doc = "Checks if the value of the field is `StopMode`"]
    #[inline(always)]
    pub fn is_stop_mode(&self) -> bool {
        *self == PDDS_A::StopMode
    }
    #[doc = "Checks if the value of the field is `StandbyMode`"]
    #[inline(always)]
    pub fn is_standby_mode(&self) -> bool {
        *self == PDDS_A::StandbyMode
    }
}
#[doc = "Field `PDDS` writer - Power down deepsleep"]
pub type PDDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PDDS_A, O>;
impl<'a, const O: u8> PDDS_W<'a, O> {
    #[doc = "Enter Stop mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn stop_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StopMode)
    }
    #[doc = "Enter Standby mode when the CPU enters deepsleep"]
    #[inline(always)]
    pub fn standby_mode(self) -> &'a mut W {
        self.variant(PDDS_A::StandbyMode)
    }
}
#[doc = "Field `CWUF` reader - Clear wakeup flag"]
pub type CWUF_R = crate::BitReader<CWUFW_A>;
#[doc = "Clear wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUFW_A {
    #[doc = "1: Clear the WUF Wakeup flag after 2 system clock cycles"]
    Clear = 1,
}
impl From<CWUFW_A> for bool {
    #[inline(always)]
    fn from(variant: CWUFW_A) -> Self {
        variant as u8 != 0
    }
}
impl CWUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CWUFW_A> {
        match self.bits {
            true => Some(CWUFW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CWUFW_A::Clear
    }
}
#[doc = "Field `CWUF` writer - Clear wakeup flag"]
pub type CWUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CWUFW_A, O>;
impl<'a, const O: u8> CWUF_W<'a, O> {
    #[doc = "Clear the WUF Wakeup flag after 2 system clock cycles"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUFW_A::Clear)
    }
}
#[doc = "Field `CSBF` reader - Clear standby flag"]
pub type CSBF_R = crate::BitReader<CSBFW_A>;
#[doc = "Clear standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBFW_A {
    #[doc = "1: Clear the SBF Standby flag"]
    Clear = 1,
}
impl From<CSBFW_A> for bool {
    #[inline(always)]
    fn from(variant: CSBFW_A) -> Self {
        variant as u8 != 0
    }
}
impl CSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSBFW_A> {
        match self.bits {
            true => Some(CSBFW_A::Clear),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Clear`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CSBFW_A::Clear
    }
}
#[doc = "Field `CSBF` writer - Clear standby flag"]
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CSBFW_A, O>;
impl<'a, const O: u8> CSBF_W<'a, O> {
    #[doc = "Clear the SBF Standby flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSBFW_A::Clear)
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<PVDE_A>;
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE_A {
    #[doc = "0: PVD Disabled"]
    Disabled = 0,
    #[doc = "1: PVD Enabled"]
    Enabled = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::Disabled,
            true => PVDE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE_A::Enabled
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PVDE_A, O>;
impl<'a, const O: u8> PVDE_W<'a, O> {
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::Disabled)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::Enabled)
    }
}
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PLS_R = crate::FieldReader<u8, PLS_A>;
#[doc = "PVD level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: 1.9 V"]
    V19 = 0,
    #[doc = "1: 2.1 V"]
    V21 = 1,
    #[doc = "2: 2.3 V"]
    V23 = 2,
    #[doc = "3: 2.5 V"]
    V25 = 3,
    #[doc = "4: 2.7 V"]
    V27 = 4,
    #[doc = "5: 2.9 V"]
    V29 = 5,
    #[doc = "6: 3.1 V"]
    V31 = 6,
    #[doc = "7: External input analog voltage (Compare internally to VREFINT)"]
    External = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
impl PLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::V19,
            1 => PLS_A::V21,
            2 => PLS_A::V23,
            3 => PLS_A::V25,
            4 => PLS_A::V27,
            5 => PLS_A::V29,
            6 => PLS_A::V31,
            7 => PLS_A::External,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline(always)]
    pub fn is_v1_9(&self) -> bool {
        *self == PLS_A::V19
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v2_1(&self) -> bool {
        *self == PLS_A::V21
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == PLS_A::V23
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PLS_A::V25
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == PLS_A::V27
    }
    #[doc = "Checks if the value of the field is `V29`"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PLS_A::V29
    }
    #[doc = "Checks if the value of the field is `V31`"]
    #[inline(always)]
    pub fn is_v3_1(&self) -> bool {
        *self == PLS_A::V31
    }
    #[doc = "Checks if the value of the field is `External`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLS_A::External
    }
}
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, PLS_A, 3, O>;
impl<'a, const O: u8> PLS_W<'a, O> {
    #[doc = "1.9 V"]
    #[inline(always)]
    pub fn v1_9(self) -> &'a mut W {
        self.variant(PLS_A::V19)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn v2_1(self) -> &'a mut W {
        self.variant(PLS_A::V21)
    }
    #[doc = "2.3 V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut W {
        self.variant(PLS_A::V23)
    }
    #[doc = "2.5 V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(PLS_A::V25)
    }
    #[doc = "2.7 V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut W {
        self.variant(PLS_A::V27)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(PLS_A::V29)
    }
    #[doc = "3.1 V"]
    #[inline(always)]
    pub fn v3_1(self) -> &'a mut W {
        self.variant(PLS_A::V31)
    }
    #[doc = "External input analog voltage (Compare internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLS_A::External)
    }
}
#[doc = "Field `DBP` reader - Disable backup domain write protection"]
pub type DBP_R = crate::BitReader<DBP_A>;
#[doc = "Disable backup domain write protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP_A {
    #[doc = "0: Access to RTC, RTC Backup and RCC CSR registers disabled"]
    Disabled = 0,
    #[doc = "1: Access to RTC, RTC Backup and RCC CSR registers enabled"]
    Enabled = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::Disabled,
            true => DBP_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP_A::Enabled
    }
}
#[doc = "Field `DBP` writer - Disable backup domain write protection"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBP_A, O>;
impl<'a, const O: u8> DBP_W<'a, O> {
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::Disabled)
    }
    #[doc = "Access to RTC, RTC Backup and RCC CSR registers enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBP_A::Enabled)
    }
}
#[doc = "Field `ULP` reader - Ultra-low-power mode"]
pub type ULP_R = crate::BitReader<ULP_A>;
#[doc = "Ultra-low-power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULP_A {
    #[doc = "0: VREFINT is on in low-power mode"]
    Enabled = 0,
    #[doc = "1: VREFINT is off in low-power mode"]
    Disabled = 1,
}
impl From<ULP_A> for bool {
    #[inline(always)]
    fn from(variant: ULP_A) -> Self {
        variant as u8 != 0
    }
}
impl ULP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULP_A {
        match self.bits {
            false => ULP_A::Enabled,
            true => ULP_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ULP_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ULP_A::Disabled
    }
}
#[doc = "Field `ULP` writer - Ultra-low-power mode"]
pub type ULP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ULP_A, O>;
impl<'a, const O: u8> ULP_W<'a, O> {
    #[doc = "VREFINT is on in low-power mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ULP_A::Enabled)
    }
    #[doc = "VREFINT is off in low-power mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ULP_A::Disabled)
    }
}
#[doc = "Field `FWU` reader - Fast wakeup"]
pub type FWU_R = crate::BitReader<FWU_A>;
#[doc = "Fast wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWU_A {
    #[doc = "0: Low-power modes exit occurs only when VREFINT is ready"]
    Disabled = 0,
    #[doc = "1: VREFINT start up time is ignored when exiting low-power modes"]
    Enabled = 1,
}
impl From<FWU_A> for bool {
    #[inline(always)]
    fn from(variant: FWU_A) -> Self {
        variant as u8 != 0
    }
}
impl FWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWU_A {
        match self.bits {
            false => FWU_A::Disabled,
            true => FWU_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FWU_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FWU_A::Enabled
    }
}
#[doc = "Field `FWU` writer - Fast wakeup"]
pub type FWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FWU_A, O>;
impl<'a, const O: u8> FWU_W<'a, O> {
    #[doc = "Low-power modes exit occurs only when VREFINT is ready"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FWU_A::Disabled)
    }
    #[doc = "VREFINT start up time is ignored when exiting low-power modes"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FWU_A::Enabled)
    }
}
#[doc = "Field `VOS` reader - Voltage scaling range selection"]
pub type VOS_R = crate::FieldReader<u8, VOS_A>;
#[doc = "Voltage scaling range selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    #[doc = "1: 1.8 V (range 1)"]
    V18 = 1,
    #[doc = "2: 1.5 V (range 2)"]
    V15 = 2,
    #[doc = "3: 1.2 V (range 3)"]
    V12 = 3,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
impl VOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VOS_A> {
        match self.bits {
            1 => Some(VOS_A::V18),
            2 => Some(VOS_A::V15),
            3 => Some(VOS_A::V12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v1_8(&self) -> bool {
        *self == VOS_A::V18
    }
    #[doc = "Checks if the value of the field is `V15`"]
    #[inline(always)]
    pub fn is_v1_5(&self) -> bool {
        *self == VOS_A::V15
    }
    #[doc = "Checks if the value of the field is `V12`"]
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS_A::V12
    }
}
#[doc = "Field `VOS` writer - Voltage scaling range selection"]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, VOS_A, 2, O>;
impl<'a, const O: u8> VOS_W<'a, O> {
    #[doc = "1.8 V (range 1)"]
    #[inline(always)]
    pub fn v1_8(self) -> &'a mut W {
        self.variant(VOS_A::V18)
    }
    #[doc = "1.5 V (range 2)"]
    #[inline(always)]
    pub fn v1_5(self) -> &'a mut W {
        self.variant(VOS_A::V15)
    }
    #[doc = "1.2 V (range 3)"]
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(VOS_A::V12)
    }
}
#[doc = "Field `DS_EE_KOFF` reader - Deep sleep mode with Flash memory kept off"]
pub type DS_EE_KOFF_R = crate::BitReader<DS_EE_KOFF_A>;
#[doc = "Deep sleep mode with Flash memory kept off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DS_EE_KOFF_A {
    #[doc = "0: NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    NvmwakeUp = 0,
    #[doc = "1: NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    Nvmsleep = 1,
}
impl From<DS_EE_KOFF_A> for bool {
    #[inline(always)]
    fn from(variant: DS_EE_KOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl DS_EE_KOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DS_EE_KOFF_A {
        match self.bits {
            false => DS_EE_KOFF_A::NvmwakeUp,
            true => DS_EE_KOFF_A::Nvmsleep,
        }
    }
    #[doc = "Checks if the value of the field is `NvmwakeUp`"]
    #[inline(always)]
    pub fn is_nvmwake_up(&self) -> bool {
        *self == DS_EE_KOFF_A::NvmwakeUp
    }
    #[doc = "Checks if the value of the field is `Nvmsleep`"]
    #[inline(always)]
    pub fn is_nvmsleep(&self) -> bool {
        *self == DS_EE_KOFF_A::Nvmsleep
    }
}
#[doc = "Field `DS_EE_KOFF` writer - Deep sleep mode with Flash memory kept off"]
pub type DS_EE_KOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DS_EE_KOFF_A, O>;
impl<'a, const O: u8> DS_EE_KOFF_W<'a, O> {
    #[doc = "NVM woken up when exiting from Deepsleep mode even if the bit RUN_PD is set"]
    #[inline(always)]
    pub fn nvmwake_up(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::NvmwakeUp)
    }
    #[doc = "NVM not woken up when exiting from low-power mode (if the bit RUN_PD is set)"]
    #[inline(always)]
    pub fn nvmsleep(self) -> &'a mut W {
        self.variant(DS_EE_KOFF_A::Nvmsleep)
    }
}
#[doc = "Field `LPRUN` reader - Low power run mode"]
pub type LPRUN_R = crate::BitReader<LPRUN_A>;
#[doc = "Low power run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRUN_A {
    #[doc = "0: Voltage regulator in Main mode in Low-power run mode"]
    MainMode = 0,
    #[doc = "1: Voltage regulator in low-power mode in Low-power run mode"]
    LowPowerMode = 1,
}
impl From<LPRUN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRUN_A {
        match self.bits {
            false => LPRUN_A::MainMode,
            true => LPRUN_A::LowPowerMode,
        }
    }
    #[doc = "Checks if the value of the field is `MainMode`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPRUN_A::MainMode
    }
    #[doc = "Checks if the value of the field is `LowPowerMode`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPRUN_A::LowPowerMode
    }
}
#[doc = "Field `LPRUN` writer - Low power run mode"]
pub type LPRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LPRUN_A, O>;
impl<'a, const O: u8> LPRUN_W<'a, O> {
    #[doc = "Voltage regulator in Main mode in Low-power run mode"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::MainMode)
    }
    #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPRUN_A::LowPowerMode)
    }
}
#[doc = "Field `LPDS` reader - Regulator in Low-power deepsleep mode"]
pub type LPDS_R = crate::BitReader<LPDS_A>;
#[doc = "Regulator in Low-power deepsleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPDS_A {
    #[doc = "0: Voltage regulator in Main mode during Deepsleep mode (Stop mode)"]
    MainMode = 0,
    #[doc = "1: Voltage regulator switches to low-power mode when the CPU enters Deepsleep mode (Stop mode)"]
    LowPowerMode = 1,
}
impl From<LPDS_A> for bool {
    #[inline(always)]
    fn from(variant: LPDS_A) -> Self {
        variant as u8 != 0
    }
}
impl LPDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDS_A {
        match self.bits {
            false => LPDS_A::MainMode,
            true => LPDS_A::LowPowerMode,
        }
    }
    #[doc = "Checks if the value of the field is `MainMode`"]
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPDS_A::MainMode
    }
    #[doc = "Checks if the value of the field is `LowPowerMode`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPDS_A::LowPowerMode
    }
}
#[doc = "Field `LPDS` writer - Regulator in Low-power deepsleep mode"]
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LPDS_A, O>;
impl<'a, const O: u8> LPDS_W<'a, O> {
    #[doc = "Voltage regulator in Main mode during Deepsleep mode (Stop mode)"]
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPDS_A::MainMode)
    }
    #[doc = "Voltage regulator switches to low-power mode when the CPU enters Deepsleep mode (Stop mode)"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPDS_A::LowPowerMode)
    }
}
impl R {
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    pub fn lpsdsr(&self) -> LPSDSR_R {
        LPSDSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn pdds(&self) -> PDDS_R {
        PDDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    pub fn cwuf(&self) -> CWUF_R {
        CWUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    pub fn csbf(&self) -> CSBF_R {
        CSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    pub fn ulp(&self) -> ULP_R {
        ULP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    pub fn fwu(&self) -> FWU_R {
        FWU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    pub fn ds_ee_koff(&self) -> DS_EE_KOFF_R {
        DS_EE_KOFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    pub fn lprun(&self) -> LPRUN_R {
        LPRUN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Regulator in Low-power deepsleep mode"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power deepsleep/Sleep/Low-power run"]
    #[inline(always)]
    #[must_use]
    pub fn lpsdsr(&mut self) -> LPSDSR_W<0> {
        LPSDSR_W::new(self)
    }
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn pdds(&mut self) -> PDDS_W<1> {
        PDDS_W::new(self)
    }
    #[doc = "Bit 2 - Clear wakeup flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwuf(&mut self) -> CWUF_W<2> {
        CWUF_W::new(self)
    }
    #[doc = "Bit 3 - Clear standby flag"]
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<3> {
        CSBF_W::new(self)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 5:7 - PVD level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - Disable backup domain write protection"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bit 9 - Ultra-low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn ulp(&mut self) -> ULP_W<9> {
        ULP_W::new(self)
    }
    #[doc = "Bit 10 - Fast wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fwu(&mut self) -> FWU_W<10> {
        FWU_W::new(self)
    }
    #[doc = "Bits 11:12 - Voltage scaling range selection"]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<11> {
        VOS_W::new(self)
    }
    #[doc = "Bit 13 - Deep sleep mode with Flash memory kept off"]
    #[inline(always)]
    #[must_use]
    pub fn ds_ee_koff(&mut self) -> DS_EE_KOFF_W<13> {
        DS_EE_KOFF_W::new(self)
    }
    #[doc = "Bit 14 - Low power run mode"]
    #[inline(always)]
    #[must_use]
    pub fn lprun(&mut self) -> LPRUN_W<14> {
        LPRUN_W::new(self)
    }
    #[doc = "Bit 16 - Regulator in Low-power deepsleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<16> {
        LPDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0x1000"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
