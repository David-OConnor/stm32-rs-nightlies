#[doc = "Register `GICD_ISACTIVER6` reader"]
pub struct R(crate::R<GICD_ISACTIVER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISACTIVER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISACTIVER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISACTIVER6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISACTIVER6` writer"]
pub struct W(crate::W<GICD_ISACTIVER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISACTIVER6_SPEC>;
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
impl From<crate::W<GICD_ISACTIVER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISACTIVER6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISACTIVER6` reader - ISACTIVER6"]
pub type ISACTIVER6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISACTIVER6` writer - ISACTIVER6"]
pub type ISACTIVER6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ISACTIVER6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    pub fn isactiver6(&self) -> ISACTIVER6_R {
        ISACTIVER6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    #[must_use]
    pub fn isactiver6(&mut self) -> ISACTIVER6_W<0> {
        ISACTIVER6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver6](index.html) module"]
pub struct GICD_ISACTIVER6_SPEC;
impl crate::RegisterSpec for GICD_ISACTIVER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isactiver6::R](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver6::W](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ISACTIVER6 to value 0"]
impl crate::Resettable for GICD_ISACTIVER6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
