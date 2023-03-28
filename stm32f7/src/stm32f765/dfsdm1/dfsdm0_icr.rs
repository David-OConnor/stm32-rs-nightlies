#[doc = "Register `DFSDM0_ICR` reader"]
pub struct R(crate::R<DFSDM0_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM0_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM0_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM0_ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM0_ICR` writer"]
pub struct W(crate::W<DFSDM0_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM0_ICR_SPEC>;
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
impl From<crate::W<DFSDM0_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM0_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader<bool>;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM0_ICR_SPEC, bool, O>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader<bool>;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM0_ICR_SPEC, bool, O>;
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag"]
pub type CLRCKABF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag"]
pub type CLRCKABF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM0_ICR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag"]
pub type CLRSCDF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag"]
pub type CLRSCDF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM0_ICR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<2> {
        CLRJOVRF_W::new(self)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<3> {
        CLRROVRF_W::new(self)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<16> {
        CLRCKABF_W::new(self)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<24> {
        CLRSCDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm0_icr](index.html) module"]
pub struct DFSDM0_ICR_SPEC;
impl crate::RegisterSpec for DFSDM0_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm0_icr::R](R) reader structure"]
impl crate::Readable for DFSDM0_ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm0_icr::W](W) writer structure"]
impl crate::Writable for DFSDM0_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM0_ICR to value 0"]
impl crate::Resettable for DFSDM0_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}