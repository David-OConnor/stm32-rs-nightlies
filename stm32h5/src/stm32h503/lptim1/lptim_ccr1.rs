#[doc = "Register `LPTIM_CCR1` reader"]
pub struct R(crate::R<LPTIM_CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CCR1` writer"]
pub struct W(crate::W<LPTIM_CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CCR1_SPEC>;
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
impl From<crate::W<LPTIM_CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1` reader - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed."]
pub type CCR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR1` writer - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed."]
pub type CCR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 1 value If channel CC1 is configured as output: CCR1 is the value to be loaded in the capture/compare 1 register. Depending on the PRELOAD option, the CCR1 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 1 contains the value to be compared to the counter LPTIM_CNT and signaled on OC1 output. If channel CC1 is configured as input: CCR1 becomes read-only, it contains the counter value transferred by the last input capture 1 event. The LPTIM_CCR1 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr1(&mut self) -> CCR1_W<0> {
        CCR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ccr1](index.html) module"]
pub struct LPTIM_CCR1_SPEC;
impl crate::RegisterSpec for LPTIM_CCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_ccr1::R](R) reader structure"]
impl crate::Readable for LPTIM_CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_ccr1::W](W) writer structure"]
impl crate::Writable for LPTIM_CCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CCR1 to value 0"]
impl crate::Resettable for LPTIM_CCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}