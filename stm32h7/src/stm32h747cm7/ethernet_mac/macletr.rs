#[doc = "Register `MACLETR` reader"]
pub struct R(crate::R<MACLETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACLETR` writer"]
pub struct W(crate::W<MACLETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLETR_SPEC>;
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
impl From<crate::W<MACLETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPIET` reader - LPI Entry Timer"]
pub type LPIET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPIET` writer - LPI Entry Timer"]
pub type LPIET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACLETR_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new((self.bits >> 3) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<3> {
        LPIET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI entry timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macletr](index.html) module"]
pub struct MACLETR_SPEC;
impl crate::RegisterSpec for MACLETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macletr::R](R) reader structure"]
impl crate::Readable for MACLETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macletr::W](W) writer structure"]
impl crate::Writable for MACLETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACLETR to value 0"]
impl crate::Resettable for MACLETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
