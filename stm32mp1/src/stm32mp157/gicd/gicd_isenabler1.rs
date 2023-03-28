#[doc = "Register `GICD_ISENABLER1` reader"]
pub struct R(crate::R<GICD_ISENABLER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISENABLER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISENABLER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISENABLER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISENABLER1` writer"]
pub struct W(crate::W<GICD_ISENABLER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISENABLER1_SPEC>;
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
impl From<crate::W<GICD_ISENABLER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISENABLER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISENABLER1` reader - ISENABLER1"]
pub type ISENABLER1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ISENABLER1` writer - ISENABLER1"]
pub type ISENABLER1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ISENABLER1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ISENABLER1"]
    #[inline(always)]
    pub fn isenabler1(&self) -> ISENABLER1_R {
        ISENABLER1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER1"]
    #[inline(always)]
    #[must_use]
    pub fn isenabler1(&mut self) -> ISENABLER1_W<0> {
        ISENABLER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler1](index.html) module"]
pub struct GICD_ISENABLER1_SPEC;
impl crate::RegisterSpec for GICD_ISENABLER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isenabler1::R](R) reader structure"]
impl crate::Readable for GICD_ISENABLER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler1::W](W) writer structure"]
impl crate::Writable for GICD_ISENABLER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ISENABLER1 to value 0"]
impl crate::Resettable for GICD_ISENABLER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
