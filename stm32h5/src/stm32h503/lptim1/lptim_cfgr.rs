#[doc = "Register `LPTIM_CFGR` reader"]
pub struct R(crate::R<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CFGR` writer"]
pub struct W(crate::W<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR_SPEC>;
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
impl From<crate::W<LPTIM_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `CKPOL` reader - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKPOL` writer - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
pub type CKPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type CKFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
pub type TRGFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
pub type TRIGEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature"]
pub type TIMOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_R = crate::BitReader<bool>;
#[doc = "Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape"]
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `WAVPOL` reader - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
pub type WAVPOL_R = crate::BitReader<bool>;
#[doc = "Field `WAVPOL` writer - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
pub type WAVPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
pub type PRELOAD_R = crate::BitReader<bool>;
#[doc = "Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
pub type PRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_R = crate::BitReader<bool>;
#[doc = "Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
pub type COUNTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
#[doc = "Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_R = crate::BitReader<bool>;
#[doc = "Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPTIM_CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM uses:"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<0> {
        CKSEL_W::new(self)
    }
    #[doc = "Bits 1:2 - Clock Polarity When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes."]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<1> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<3> {
        CKFLT_W::new(self)
    }
    #[doc = "Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<6> {
        TRGFLT_W::new(self)
    }
    #[doc = "Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<9> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that serves as a trigger event for the LPTIM among the below 8 available sources: See for details."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<13> {
        TRIGSEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<17> {
        TRIGEN_W::new(self)
    }
    #[doc = "Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<19> {
        TIMOUT_W::new(self)
    }
    #[doc = "Bit 20 - Waveform shape The WAVE bit controls the output shape"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<20> {
        WAVE_W::new(self)
    }
    #[doc = "Bit 21 - Waveform shape polarity The WAVPOL bit controls the output polarity Note: If the LPTIM implements at least one capture/compare channel, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<21> {
        WAVPOL_W::new(self)
    }
    #[doc = "Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR, LPTIM_RCR and the LPTIM_CCRx registers update modality"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<22> {
        PRELOAD_W::new(self)
    }
    #[doc = "Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<23> {
        COUNTMODE_W::new(self)
    }
    #[doc = "Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<24> {
        ENC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr](index.html) module"]
pub struct LPTIM_CFGR_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cfgr::R](R) reader structure"]
impl crate::Readable for LPTIM_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr::W](W) writer structure"]
impl crate::Writable for LPTIM_CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CFGR to value 0"]
impl crate::Resettable for LPTIM_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
