#[doc = "Register `WTCR` reader"]
pub struct R(crate::R<WTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTCR` writer"]
pub struct W(crate::W<WTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTCR_SPEC>;
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
impl From<crate::W<WTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCYC` reader - DCYC"]
pub type DCYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCYC` writer - DCYC"]
pub type DCYC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DCYC"]
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DCYC"]
    #[inline(always)]
    #[must_use]
    pub fn dcyc(&mut self) -> DCYC_W<0> {
        DCYC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "write timing configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcr](index.html) module"]
pub struct WTCR_SPEC;
impl crate::RegisterSpec for WTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtcr::R](R) reader structure"]
impl crate::Readable for WTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtcr::W](W) writer structure"]
impl crate::Writable for WTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WTCR to value 0"]
impl crate::Resettable for WTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
