#[doc = "Register `LPTIM_CCR2` reader"]
pub struct R(crate::R<LPTIM_CCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CCR2` writer"]
pub struct W(crate::W<LPTIM_CCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CCR2_SPEC>;
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
impl From<crate::W<LPTIM_CCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR2` reader - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM_CCR2 register is read-only and cannot be programmed."]
pub type CCR2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR2` writer - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM_CCR2 register is read-only and cannot be programmed."]
pub type CCR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_CCR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM_CCR2 register is read-only and cannot be programmed."]
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/compare 2 value If channel CC2 is configured as output: CCR2 is the value to be loaded in the capture/compare 2 register. Depending on the PRELOAD option, the CCR2 register is immediately updated if the PRELOAD bit is reset and updated at next LPTIM update event if PREOAD bit is reset. The capture/compare register 2 contains the value to be compared to the counter LPTIM_CNT and signaled on OC2 output. If channel CC2 is configured as input: CCR2 becomes read-only, it contains the counter value transferred by the last input capture 2 event. The LPTIM_CCR2 register is read-only and cannot be programmed."]
    #[inline(always)]
    #[must_use]
    pub fn ccr2(&mut self) -> CCR2_W<0> {
        CCR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_ccr2](index.html) module"]
pub struct LPTIM_CCR2_SPEC;
impl crate::RegisterSpec for LPTIM_CCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_ccr2::R](R) reader structure"]
impl crate::Readable for LPTIM_CCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_ccr2::W](W) writer structure"]
impl crate::Writable for LPTIM_CCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTIM_CCR2 to value 0"]
impl crate::Resettable for LPTIM_CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
