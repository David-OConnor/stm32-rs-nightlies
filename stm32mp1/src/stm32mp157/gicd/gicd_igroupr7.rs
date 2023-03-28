#[doc = "Register `GICD_IGROUPR7` reader"]
pub struct R(crate::R<GICD_IGROUPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IGROUPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IGROUPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IGROUPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IGROUPR7` writer"]
pub struct W(crate::W<GICD_IGROUPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IGROUPR7_SPEC>;
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
impl From<crate::W<GICD_IGROUPR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IGROUPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IGROUPR7` reader - IGROUPR7"]
pub type IGROUPR7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IGROUPR7` writer - IGROUPR7"]
pub type IGROUPR7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IGROUPR7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IGROUPR7"]
    #[inline(always)]
    pub fn igroupr7(&self) -> IGROUPR7_R {
        IGROUPR7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR7"]
    #[inline(always)]
    #[must_use]
    pub fn igroupr7(&mut self) -> IGROUPR7_W<0> {
        IGROUPR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr7](index.html) module"]
pub struct GICD_IGROUPR7_SPEC;
impl crate::RegisterSpec for GICD_IGROUPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_igroupr7::R](R) reader structure"]
impl crate::Readable for GICD_IGROUPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr7::W](W) writer structure"]
impl crate::Writable for GICD_IGROUPR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR7 to value 0"]
impl crate::Resettable for GICD_IGROUPR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
