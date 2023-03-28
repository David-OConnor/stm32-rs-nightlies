#[doc = "Register `GICD_ISENABLER5` reader"]
pub struct R(crate::R<GICD_ISENABLER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISENABLER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISENABLER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISENABLER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISENABLER5` writer"]
pub struct W(crate::W<GICD_ISENABLER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISENABLER5_SPEC>;
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
impl From<crate::W<GICD_ISENABLER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISENABLER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISENABLER5` reader - ISENABLER5"]
pub type ISENABLER5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISENABLER5` writer - ISENABLER5"]
pub type ISENABLER5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ISENABLER5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISENABLER5"]
    #[inline(always)]
    pub fn isenabler5(&self) -> ISENABLER5_R {
        ISENABLER5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER5"]
    #[inline(always)]
    #[must_use]
    pub fn isenabler5(&mut self) -> ISENABLER5_W<0> {
        ISENABLER5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler5](index.html) module"]
pub struct GICD_ISENABLER5_SPEC;
impl crate::RegisterSpec for GICD_ISENABLER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isenabler5::R](R) reader structure"]
impl crate::Readable for GICD_ISENABLER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler5::W](W) writer structure"]
impl crate::Writable for GICD_ISENABLER5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ISENABLER5 to value 0"]
impl crate::Resettable for GICD_ISENABLER5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
