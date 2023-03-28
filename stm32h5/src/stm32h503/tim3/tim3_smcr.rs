#[doc = "Register `TIM3_SMCR` reader"]
pub struct R(crate::R<TIM3_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_SMCR` writer"]
pub struct W(crate::W<TIM3_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_SMCR_SPEC>;
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
impl From<crate::W<TIM3_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
pub type OCCS_R = crate::BitReader<bool>;
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
pub type OCCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `TS1` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS1` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_SMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
pub type ETPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_SMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_R = crate::BitReader<bool>;
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_R = crate::BitReader<bool>;
#[doc = "Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `TS2` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS2` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_SMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMSPE` reader - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
pub type SMSPE_R = crate::BitReader<bool>;
#[doc = "Field `SMSPE` writer - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
pub type SMSPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
#[doc = "Field `SMSPS` reader - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
pub type SMSPS_R = crate::BitReader<bool>;
#[doc = "Field `SMSPS` writer - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
pub type SMSPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM3_SMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
    #[inline(always)]
    pub fn smspe(&self) -> SMSPE_R {
        SMSPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
    #[inline(always)]
    pub fn smsps(&self) -> SMSPS_R {
        SMSPS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> SMS1_W<0> {
        SMS1_W::new(self)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source Note: If the OCREF clear selection feature is not supported, this bit is reserved and forced by hardware to ‘0’. ."]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<3> {
        OCCS_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<4> {
        TS1_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample tim_etrp signal and the length of the digital filter applied to tim_etrp. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal tim_etrp frequency must be at most 1/4 of tim_ker_ck frequency. A prescaler can be enabled to reduce tim_etrp frequency. It is useful when inputting fast external clocks on tim_etr_in."]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with tim_trgi connected to tim_etrf (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, tim_trgi must not be connected to tim_etrf in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is tim_etrf."]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether tim_etr_in or tim_etr_in is used for trigger operations"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (tim_trgi) is linked to the polarity selected on the external input (see Input Control register and Control Register description. Note: The gated mode must not be used if tim_ti1f_ed is selected as the trigger input (TS=00100). Indeed, tim_ti1f_ed outputs 1 pulse for each transition on tim_ti1f, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the tim_trgo signal must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> SMS2_W<16> {
        SMS2_W::new(self)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for product specific implementation details. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<20> {
        TS2_W::new(self)
    }
    #[doc = "Bit 24 - SMS preload enable This bit selects whether the SMS\\[3:0\\]
bitfield is preloaded"]
    #[inline(always)]
    #[must_use]
    pub fn smspe(&mut self) -> SMSPE_W<24> {
        SMSPE_W::new(self)
    }
    #[doc = "Bit 25 - SMS preload source This bit selects whether the events that triggers the SMS\\[3:0\\]
bitfield transfer from preload to active"]
    #[inline(always)]
    #[must_use]
    pub fn smsps(&mut self) -> SMSPS_W<25> {
        SMSPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_smcr](index.html) module"]
pub struct TIM3_SMCR_SPEC;
impl crate::RegisterSpec for TIM3_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim3_smcr::R](R) reader structure"]
impl crate::Readable for TIM3_SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_smcr::W](W) writer structure"]
impl crate::Writable for TIM3_SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM3_SMCR to value 0"]
impl crate::Resettable for TIM3_SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
