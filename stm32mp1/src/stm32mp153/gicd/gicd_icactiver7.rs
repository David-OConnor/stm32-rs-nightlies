#[doc = "Register `GICD_ICACTIVER7` reader"]
pub struct R(crate::R<GICD_ICACTIVER7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICACTIVER7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICACTIVER7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICACTIVER7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICACTIVER7` writer"]
pub struct W(crate::W<GICD_ICACTIVER7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICACTIVER7_SPEC>;
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
impl From<crate::W<GICD_ICACTIVER7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICACTIVER7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACTIVER7` reader - ICACTIVER7"]
pub type ICACTIVER7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICACTIVER7` writer - ICACTIVER7"]
pub type ICACTIVER7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICACTIVER7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    pub fn icactiver7(&self) -> ICACTIVER7_R {
        ICACTIVER7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    #[must_use]
    pub fn icactiver7(&mut self) -> ICACTIVER7_W<0> {
        ICACTIVER7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver7](index.html) module"]
pub struct GICD_ICACTIVER7_SPEC;
impl crate::RegisterSpec for GICD_ICACTIVER7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icactiver7::R](R) reader structure"]
impl crate::Readable for GICD_ICACTIVER7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver7::W](W) writer structure"]
impl crate::Writable for GICD_ICACTIVER7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICACTIVER7 to value 0"]
impl crate::Resettable for GICD_ICACTIVER7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
