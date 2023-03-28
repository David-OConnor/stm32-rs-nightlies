#[doc = "Register `GICD_ICENABLER3` reader"]
pub struct R(crate::R<GICD_ICENABLER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICENABLER3` writer"]
pub struct W(crate::W<GICD_ICENABLER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER3_SPEC>;
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
impl From<crate::W<GICD_ICENABLER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICENABLER3` reader - ICENABLER3"]
pub type ICENABLER3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ICENABLER3` writer - ICENABLER3"]
pub type ICENABLER3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICENABLER3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ICENABLER3"]
    #[inline(always)]
    pub fn icenabler3(&self) -> ICENABLER3_R {
        ICENABLER3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER3"]
    #[inline(always)]
    #[must_use]
    pub fn icenabler3(&mut self) -> ICENABLER3_W<0> {
        ICENABLER3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler3](index.html) module"]
pub struct GICD_ICENABLER3_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icenabler3::R](R) reader structure"]
impl crate::Readable for GICD_ICENABLER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler3::W](W) writer structure"]
impl crate::Writable for GICD_ICENABLER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ICENABLER3 to value 0"]
impl crate::Resettable for GICD_ICENABLER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
