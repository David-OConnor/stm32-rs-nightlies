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
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader<ADENR_A>;
#[doc = "ADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR_A {
    #[doc = "0: ADC disabled"]
    Disabled = 0,
    #[doc = "1: ADC enabled"]
    Enabled = 1,
}
impl From<ADENR_A> for bool {
    #[inline(always)]
    fn from(variant: ADENR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADENR_A {
        match self.bits {
            false => ADENR_A::Disabled,
            true => ADENR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR_A::Enabled
    }
}
#[doc = "ADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW_AW {
    #[doc = "1: Enable the ADC"]
    Enabled = 1,
}
impl From<ADENW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` writer - ADEN"]
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADENW_AW, O>;
impl<'a, const O: u8> ADEN_W<'a, O> {
    #[doc = "Enable the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADENW_AW::Enabled)
    }
}
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader<ADDISR_A>;
#[doc = "ADDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR_A {
    #[doc = "0: No disable command active"]
    NotDisabling = 0,
    #[doc = "1: ADC disabling"]
    Disabling = 1,
}
impl From<ADDISR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDISR_A {
        match self.bits {
            false => ADDISR_A::NotDisabling,
            true => ADDISR_A::Disabling,
        }
    }
    #[doc = "Checks if the value of the field is `NotDisabling`"]
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR_A::NotDisabling
    }
    #[doc = "Checks if the value of the field is `Disabling`"]
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR_A::Disabling
    }
}
#[doc = "ADDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW_AW {
    #[doc = "1: Disable the ADC"]
    Disable = 1,
}
impl From<ADDISW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type ADDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADDISW_AW, O>;
impl<'a, const O: u8> ADDIS_W<'a, O> {
    #[doc = "Disable the ADC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDISW_AW::Disable)
    }
}
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader<ADSTARTR_A>;
#[doc = "ADSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR_A {
    #[doc = "0: No conversion ongoing"]
    NotActive = 0,
    #[doc = "1: ADC operating and may be converting"]
    Active = 1,
}
impl From<ADSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTARTR_A {
        match self.bits {
            false => ADSTARTR_A::NotActive,
            true => ADSTARTR_A::Active,
        }
    }
    #[doc = "Checks if the value of the field is `NotActive`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR_A::NotActive
    }
    #[doc = "Checks if the value of the field is `Active`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR_A::Active
    }
}
#[doc = "ADSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW_AW {
    #[doc = "1: Start the ADC conversion (may be delayed for hardware triggers)"]
    StartConversion = 1,
}
impl From<ADSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type ADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADSTARTW_AW, O>;
impl<'a, const O: u8> ADSTART_W<'a, O> {
    #[doc = "Start the ADC conversion (may be delayed for hardware triggers)"]
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut W {
        self.variant(ADSTARTW_AW::StartConversion)
    }
}
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader<ADSTPR_A>;
#[doc = "ADSTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR_A {
    #[doc = "0: No stop command active"]
    NotStopping = 0,
    #[doc = "1: ADC stopping conversion"]
    Stopping = 1,
}
impl From<ADSTPR_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTPR_A {
        match self.bits {
            false => ADSTPR_A::NotStopping,
            true => ADSTPR_A::Stopping,
        }
    }
    #[doc = "Checks if the value of the field is `NotStopping`"]
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR_A::NotStopping
    }
    #[doc = "Checks if the value of the field is `Stopping`"]
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR_A::Stopping
    }
}
#[doc = "ADSTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW_AW {
    #[doc = "1: Stop the active conversion"]
    StopConversion = 1,
}
impl From<ADSTPW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADSTPW_AW, O>;
impl<'a, const O: u8> ADSTP_W<'a, O> {
    #[doc = "Stop the active conversion"]
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut W {
        self.variant(ADSTPW_AW::StopConversion)
    }
}
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::BitReader<ADVREGEN_A>;
#[doc = "ADVREGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADVREGEN_A {
    #[doc = "0: ADC voltage regulator disabled"]
    Disabled = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    Enabled = 1,
}
impl From<ADVREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADVREGEN_A {
        match self.bits {
            false => ADVREGEN_A::Disabled,
            true => ADVREGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN_A::Enabled
    }
}
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADVREGEN_A, O>;
impl<'a, const O: u8> ADVREGEN_W<'a, O> {
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::Disabled)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::Enabled)
    }
}
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader<ADCALR_A>;
#[doc = "ADCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR_A {
    #[doc = "0: ADC calibration either not yet performed or completed"]
    NotCalibrating = 0,
    #[doc = "1: ADC calibration in progress"]
    Calibrating = 1,
}
impl From<ADCALR_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALR_A {
        match self.bits {
            false => ADCALR_A::NotCalibrating,
            true => ADCALR_A::Calibrating,
        }
    }
    #[doc = "Checks if the value of the field is `NotCalibrating`"]
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR_A::NotCalibrating
    }
    #[doc = "Checks if the value of the field is `Calibrating`"]
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR_A::Calibrating
    }
}
#[doc = "ADCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW_AW {
    #[doc = "1: Start the ADC calibration sequence"]
    StartCalibration = 1,
}
impl From<ADCALW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type ADCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADCALW_AW, O>;
impl<'a, const O: u8> ADCAL_W<'a, O> {
    #[doc = "Start the ADC calibration sequence"]
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut W {
        self.variant(ADCALW_AW::StartCalibration)
    }
}
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<1> {
        ADDIS_W::new(self)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<2> {
        ADSTART_W::new(self)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<4> {
        ADSTP_W::new(self)
    }
    #[doc = "Bit 28 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<28> {
        ADVREGEN_W::new(self)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<31> {
        ADCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
