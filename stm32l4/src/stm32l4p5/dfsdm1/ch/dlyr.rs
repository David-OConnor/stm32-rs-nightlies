#[doc = "Register `DLYR` reader"]
pub struct R(crate::R<DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYR` writer"]
pub struct W(crate::W<DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYR_SPEC>;
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
impl From<crate::W<DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PLSSKP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PLSSKP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DLYR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<0> {
        PLSSKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel y delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyr](index.html) module"]
pub struct DLYR_SPEC;
impl crate::RegisterSpec for DLYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyr::R](R) reader structure"]
impl crate::Readable for DLYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyr::W](W) writer structure"]
impl crate::Writable for DLYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLYR to value 0"]
impl crate::Resettable for DLYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
