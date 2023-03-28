#[doc = "Register `GICD_IGROUPR4` reader"]
pub struct R(crate::R<GICD_IGROUPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IGROUPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IGROUPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IGROUPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IGROUPR4` writer"]
pub struct W(crate::W<GICD_IGROUPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IGROUPR4_SPEC>;
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
impl From<crate::W<GICD_IGROUPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IGROUPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IGROUPR4` reader - IGROUPR4"]
pub type IGROUPR4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IGROUPR4` writer - IGROUPR4"]
pub type IGROUPR4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IGROUPR4_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IGROUPR4"]
    #[inline(always)]
    pub fn igroupr4(&self) -> IGROUPR4_R {
        IGROUPR4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR4"]
    #[inline(always)]
    #[must_use]
    pub fn igroupr4(&mut self) -> IGROUPR4_W<0> {
        IGROUPR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID = x*32 to ID = x*32+31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_igroupr4](index.html) module"]
pub struct GICD_IGROUPR4_SPEC;
impl crate::RegisterSpec for GICD_IGROUPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_igroupr4::R](R) reader structure"]
impl crate::Readable for GICD_IGROUPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_igroupr4::W](W) writer structure"]
impl crate::Writable for GICD_IGROUPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IGROUPR4 to value 0"]
impl crate::Resettable for GICD_IGROUPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
