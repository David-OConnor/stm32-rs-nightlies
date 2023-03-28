#[doc = "Register `CPAR4` reader"]
pub struct R(crate::R<CPAR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPAR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPAR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPAR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPAR4` writer"]
pub struct W(crate::W<CPAR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPAR4_SPEC>;
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
impl From<crate::W<CPAR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPAR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPAR4_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<0> {
        NDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpar4](index.html) module"]
pub struct CPAR4_SPEC;
impl crate::RegisterSpec for CPAR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpar4::R](R) reader structure"]
impl crate::Readable for CPAR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpar4::W](W) writer structure"]
impl crate::Writable for CPAR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPAR4 to value 0"]
impl crate::Resettable for CPAR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
