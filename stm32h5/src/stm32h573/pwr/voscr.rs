#[doc = "Register `VOSCR` reader"]
pub struct R(crate::R<VOSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOSCR` writer"]
pub struct W(crate::W<VOSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOSCR_SPEC>;
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
impl From<crate::W<VOSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOS` reader - voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOS` writer - voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VOSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<4> {
        VOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR voltage scaling control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [voscr](index.html) module"]
pub struct VOSCR_SPEC;
impl crate::RegisterSpec for VOSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [voscr::R](R) reader structure"]
impl crate::Readable for VOSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [voscr::W](W) writer structure"]
impl crate::Writable for VOSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VOSCR to value 0"]
impl crate::Resettable for VOSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
