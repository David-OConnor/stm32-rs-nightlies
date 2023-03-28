#[doc = "Register `L2CACR` reader"]
pub struct R(crate::R<L2CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2CACR` writer"]
pub struct W(crate::W<L2CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CACR_SPEC>;
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
impl From<crate::W<L2CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTA` reader - Constant Alpha"]
pub type CONSTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTA` writer - Constant Alpha"]
pub type CONSTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2CACR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    #[must_use]
    pub fn consta(&mut self) -> CONSTA_W<0> {
        CONSTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Constant Alpha Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cacr](index.html) module"]
pub struct L2CACR_SPEC;
impl crate::RegisterSpec for L2CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2cacr::R](R) reader structure"]
impl crate::Readable for L2CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2cacr::W](W) writer structure"]
impl crate::Writable for L2CACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2CACR to value 0"]
impl crate::Resettable for L2CACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
