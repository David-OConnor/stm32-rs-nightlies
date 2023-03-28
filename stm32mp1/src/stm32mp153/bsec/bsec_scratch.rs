#[doc = "Register `BSEC_SCRATCH` reader"]
pub struct R(crate::R<BSEC_SCRATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_SCRATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_SCRATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_SCRATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_SCRATCH` writer"]
pub struct W(crate::W<BSEC_SCRATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_SCRATCH_SPEC>;
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
impl From<crate::W<BSEC_SCRATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_SCRATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BSEC_SCRATCH_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC scratch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_scratch](index.html) module"]
pub struct BSEC_SCRATCH_SPEC;
impl crate::RegisterSpec for BSEC_SCRATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_scratch::R](R) reader structure"]
impl crate::Readable for BSEC_SCRATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_scratch::W](W) writer structure"]
impl crate::Writable for BSEC_SCRATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BSEC_SCRATCH to value 0"]
impl crate::Resettable for BSEC_SCRATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
