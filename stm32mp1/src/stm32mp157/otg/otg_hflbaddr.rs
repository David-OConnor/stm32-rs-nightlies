#[doc = "Register `OTG_HFLBADDR` reader"]
pub struct R(crate::R<OTG_HFLBADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HFLBADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HFLBADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HFLBADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HFLBADDR` writer"]
pub struct W(crate::W<OTG_HFLBADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HFLBADDR_SPEC>;
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
impl From<crate::W<OTG_HFLBADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HFLBADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFLBADDR` reader - HFLBADDR"]
pub type HFLBADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HFLBADDR` writer - HFLBADDR"]
pub type HFLBADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HFLBADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HFLBADDR"]
    #[inline(always)]
    #[must_use]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<0> {
        HFLBADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hflbaddr](index.html) module"]
pub struct OTG_HFLBADDR_SPEC;
impl crate::RegisterSpec for OTG_HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hflbaddr::R](R) reader structure"]
impl crate::Readable for OTG_HFLBADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hflbaddr::W](W) writer structure"]
impl crate::Writable for OTG_HFLBADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HFLBADDR to value 0"]
impl crate::Resettable for OTG_HFLBADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
