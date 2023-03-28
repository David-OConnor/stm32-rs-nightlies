#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUF` reader - Wakeup flag"]
pub type WUF_R = crate::BitReader<WUFR_A>;
#[doc = "Wakeup flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFR_A {
    #[doc = "0: No wakeup event occurred"]
    NoWakeupEvent = 0,
    #[doc = "1: A wakeup event was received from the WKUP pin or from the RTC alarm (Alarm A or Alarm B), RTC Tamper event, RTC TimeStamp event or RTC Wakeup)"]
    WakeupEvent = 1,
}
impl From<WUFR_A> for bool {
    #[inline(always)]
    fn from(variant: WUFR_A) -> Self {
        variant as u8 != 0
    }
}
impl WUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFR_A {
        match self.bits {
            false => WUFR_A::NoWakeupEvent,
            true => WUFR_A::WakeupEvent,
        }
    }
    #[doc = "Checks if the value of the field is `NoWakeupEvent`"]
    #[inline(always)]
    pub fn is_no_wakeup_event(&self) -> bool {
        *self == WUFR_A::NoWakeupEvent
    }
    #[doc = "Checks if the value of the field is `WakeupEvent`"]
    #[inline(always)]
    pub fn is_wakeup_event(&self) -> bool {
        *self == WUFR_A::WakeupEvent
    }
}
#[doc = "Field `SBF` reader - Standby flag"]
pub type SBF_R = crate::BitReader<SBFR_A>;
#[doc = "Standby flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBFR_A {
    #[doc = "0: Device has not been in Standby mode"]
    NoStandbyEvent = 0,
    #[doc = "1: Device has been in Standby mode"]
    StandbyEvent = 1,
}
impl From<SBFR_A> for bool {
    #[inline(always)]
    fn from(variant: SBFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBFR_A {
        match self.bits {
            false => SBFR_A::NoStandbyEvent,
            true => SBFR_A::StandbyEvent,
        }
    }
    #[doc = "Checks if the value of the field is `NoStandbyEvent`"]
    #[inline(always)]
    pub fn is_no_standby_event(&self) -> bool {
        *self == SBFR_A::NoStandbyEvent
    }
    #[doc = "Checks if the value of the field is `StandbyEvent`"]
    #[inline(always)]
    pub fn is_standby_event(&self) -> bool {
        *self == SBFR_A::StandbyEvent
    }
}
#[doc = "Field `PVDO` reader - PVD output"]
pub type PVDO_R = crate::BitReader<PVDOR_A>;
#[doc = "PVD output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDOR_A {
    #[doc = "0: VDD is higher than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    AboveThreshold = 0,
    #[doc = "1: VDD is lower than the PVD threshold selected with the PLS\\[2:0\\]
bits"]
    BelowThreshold = 1,
}
impl From<PVDOR_A> for bool {
    #[inline(always)]
    fn from(variant: PVDOR_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDOR_A {
        match self.bits {
            false => PVDOR_A::AboveThreshold,
            true => PVDOR_A::BelowThreshold,
        }
    }
    #[doc = "Checks if the value of the field is `AboveThreshold`"]
    #[inline(always)]
    pub fn is_above_threshold(&self) -> bool {
        *self == PVDOR_A::AboveThreshold
    }
    #[doc = "Checks if the value of the field is `BelowThreshold`"]
    #[inline(always)]
    pub fn is_below_threshold(&self) -> bool {
        *self == PVDOR_A::BelowThreshold
    }
}
#[doc = "Field `VREFINTRDYF` reader - Internal voltage reference ready flag"]
pub type VREFINTRDYF_R = crate::BitReader<VREFINTRDYFR_A>;
#[doc = "Internal voltage reference ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINTRDYFR_A {
    #[doc = "0: VREFINT is OFF"]
    NotReady = 0,
    #[doc = "1: VREFINT is ready"]
    Ready = 1,
}
impl From<VREFINTRDYFR_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINTRDYFR_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFINTRDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFINTRDYFR_A {
        match self.bits {
            false => VREFINTRDYFR_A::NotReady,
            true => VREFINTRDYFR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINTRDYFR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINTRDYFR_A::Ready
    }
}
#[doc = "Field `VOSF` reader - Voltage Scaling select flag"]
pub type VOSF_R = crate::BitReader<VOSFR_A>;
#[doc = "Voltage Scaling select flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSFR_A {
    #[doc = "0: Regulator is ready in the selected voltage range"]
    Ready = 0,
    #[doc = "1: Regulator voltage output is changing to the required VOS level"]
    NotReady = 1,
}
impl From<VOSFR_A> for bool {
    #[inline(always)]
    fn from(variant: VOSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl VOSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOSFR_A {
        match self.bits {
            false => VOSFR_A::Ready,
            true => VOSFR_A::NotReady,
        }
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSFR_A::Ready
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSFR_A::NotReady
    }
}
#[doc = "Field `REGLPF` reader - Regulator LP flag"]
pub type REGLPF_R = crate::BitReader<REGLPFR_A>;
#[doc = "Regulator LP flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGLPFR_A {
    #[doc = "0: Regulator is ready in Main mode"]
    Ready = 0,
    #[doc = "1: Regulator voltage is in low-power mode"]
    NotReady = 1,
}
impl From<REGLPFR_A> for bool {
    #[inline(always)]
    fn from(variant: REGLPFR_A) -> Self {
        variant as u8 != 0
    }
}
impl REGLPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGLPFR_A {
        match self.bits {
            false => REGLPFR_A::Ready,
            true => REGLPFR_A::NotReady,
        }
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == REGLPFR_A::Ready
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == REGLPFR_A::NotReady
    }
}
#[doc = "Field `EWUP1` reader - Enable WKUP pin 1"]
pub type EWUP1_R = crate::BitReader<EWUP1_A>;
#[doc = "Enable WKUP pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1_A {
    #[doc = "0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::Disabled,
            true => EWUP1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1_A::Enabled
    }
}
#[doc = "Field `EWUP1` writer - Enable WKUP pin 1"]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, EWUP1_A, O>;
impl<'a, const O: u8> EWUP1_W<'a, O> {
    #[doc = "WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Disabled)
    }
    #[doc = "WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Enabled)
    }
}
#[doc = "Field `EWUP2` reader - Enable WKUP pin 2"]
pub type EWUP2_R = crate::BitReader<EWUP2_A>;
#[doc = "Enable WKUP pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2_A {
    #[doc = "0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::Disabled,
            true => EWUP2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2_A::Enabled
    }
}
#[doc = "Field `EWUP2` writer - Enable WKUP pin 2"]
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, EWUP2_A, O>;
impl<'a, const O: u8> EWUP2_W<'a, O> {
    #[doc = "WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Disabled)
    }
    #[doc = "WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Enabled)
    }
}
#[doc = "Field `EWUP3` reader - Enable WKUP pin 3"]
pub type EWUP3_R = crate::BitReader<EWUP3_A>;
#[doc = "Enable WKUP pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3_A {
    #[doc = "0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::Disabled,
            true => EWUP3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3_A::Enabled
    }
}
#[doc = "Field `EWUP3` writer - Enable WKUP pin 3"]
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, EWUP3_A, O>;
impl<'a, const O: u8> EWUP3_W<'a, O> {
    #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Disabled)
    }
    #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup flag"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Standby flag"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD output"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal voltage reference ready flag"]
    #[inline(always)]
    pub fn vrefintrdyf(&self) -> VREFINTRDYF_R {
        VREFINTRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Voltage Scaling select flag"]
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Regulator LP flag"]
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable WKUP pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<8> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 9 - Enable WKUP pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<9> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 10 - Enable WKUP pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<10> {
        EWUP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
