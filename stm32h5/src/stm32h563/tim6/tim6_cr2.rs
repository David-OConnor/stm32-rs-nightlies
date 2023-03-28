#[doc = "Register `TIM6_CR2` reader"]
pub struct R(crate::R<TIM6_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CR2` writer"]
pub struct W(crate::W<TIM6_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CR2_SPEC>;
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
impl From<crate::W<TIM6_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMS` reader - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMS` writer - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM6_CR2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: Note: The clock of the slave timer or he peripheral receiving the tim_trgo must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_cr2](index.html) module"]
pub struct TIM6_CR2_SPEC;
impl crate::RegisterSpec for TIM6_CR2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim6_cr2::R](R) reader structure"]
impl crate::Readable for TIM6_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_cr2::W](W) writer structure"]
impl crate::Writable for TIM6_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM6_CR2 to value 0"]
impl crate::Resettable for TIM6_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
