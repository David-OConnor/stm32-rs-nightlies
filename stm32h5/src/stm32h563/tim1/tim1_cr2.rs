#[doc = "Register `TIM1_CR2` reader"]
pub struct R(crate::R<TIM1_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_CR2` writer"]
pub struct W(crate::W<TIM1_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CR2_SPEC>;
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
impl From<crate::W<TIM1_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader<bool>;
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader<bool>;
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<bool>;
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `MMS` reader - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS` writer - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `TI1S` reader - tim_ti1 selection"]
pub type TI1S_R = crate::BitReader<bool>;
#[doc = "Field `TI1S` writer - tim_ti1 selection"]
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1` reader - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_R = crate::BitReader<bool>;
#[doc = "Field `OIS1` writer - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS1N` reader - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_R = crate::BitReader<bool>;
#[doc = "Field `OIS1N` writer - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS2` reader - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub type OIS2_R = crate::BitReader<bool>;
#[doc = "Field `OIS2` writer - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub type OIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS2N` reader - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub type OIS2N_R = crate::BitReader<bool>;
#[doc = "Field `OIS2N` writer - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub type OIS2N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS3` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub type OIS3_R = crate::BitReader<bool>;
#[doc = "Field `OIS3` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub type OIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS3N` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub type OIS3N_R = crate::BitReader<bool>;
#[doc = "Field `OIS3N` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub type OIS3N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS4` reader - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub type OIS4_R = crate::BitReader<bool>;
#[doc = "Field `OIS4` writer - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub type OIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS4N` reader - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub type OIS4N_R = crate::BitReader<bool>;
#[doc = "Field `OIS4N` writer - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub type OIS4N_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS5` reader - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub type OIS5_R = crate::BitReader<bool>;
#[doc = "Field `OIS5` writer - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub type OIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `OIS6` reader - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub type OIS6_R = crate::BitReader<bool>;
#[doc = "Field `OIS6` writer - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub type OIS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM1_CR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `MMS_1` reader - MMS\\[3\\]"]
pub type MMS_1_R = crate::BitReader<bool>;
#[doc = "Field `MMS_1` writer - MMS\\[3\\]"]
pub type MMS_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois4n(&self) -> OIS4N_R {
        OIS4N_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    pub fn mms_1(&self) -> MMS_1_R {
        MMS_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<10> {
        OIS2_W::new(self)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<11> {
        OIS2N_W::new(self)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<12> {
        OIS3_W::new(self)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<13> {
        OIS3N_W::new(self)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<14> {
        OIS4_W::new(self)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4n(&mut self) -> OIS4N_W<15> {
        OIS4N_W::new(self)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<16> {
        OIS5_W::new(self)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<18> {
        OIS6_W::new(self)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<20> {
        MMS2_W::new(self)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mms_1(&mut self) -> MMS_1_W<25> {
        MMS_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_cr2](index.html) module"]
pub struct TIM1_CR2_SPEC;
impl crate::RegisterSpec for TIM1_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_cr2::R](R) reader structure"]
impl crate::Readable for TIM1_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_cr2::W](W) writer structure"]
impl crate::Writable for TIM1_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM1_CR2 to value 0"]
impl crate::Resettable for TIM1_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
