#[doc = "Register `GCOMP` reader"]
pub struct R(crate::R<GCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCOMP` writer"]
pub struct W(crate::W<GCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCOMP_SPEC>;
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
impl From<crate::W<GCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCOMPCOEFF` reader - Gain compensation coefficient"]
pub type GCOMPCOEFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCOMPCOEFF` writer - Gain compensation coefficient"]
pub type GCOMPCOEFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCOMP_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W<0> {
        GCOMPCOEFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcomp](index.html) module"]
pub struct GCOMP_SPEC;
impl crate::RegisterSpec for GCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcomp::R](R) reader structure"]
impl crate::Readable for GCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcomp::W](W) writer structure"]
impl crate::Writable for GCOMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCOMP to value 0"]
impl crate::Resettable for GCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
