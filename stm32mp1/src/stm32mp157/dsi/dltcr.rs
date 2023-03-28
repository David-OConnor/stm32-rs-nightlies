#[doc = "Register `DLTCR` reader"]
pub struct R(crate::R<DLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLTCR` writer"]
pub struct W(crate::W<DLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLTCR_SPEC>;
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
impl From<crate::W<DLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP2HS_TIME` reader - LP2HS_TIME"]
pub type LP2HS_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LP2HS_TIME` writer - LP2HS_TIME"]
pub type LP2HS_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLTCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `HS2LP_TIME` reader - HS2LP_TIME"]
pub type HS2LP_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HS2LP_TIME` writer - HS2LP_TIME"]
pub type HS2LP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLTCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> LP2HS_TIME_R {
        LP2HS_TIME_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> HS2LP_TIME_R {
        HS2LP_TIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hs_time(&mut self) -> LP2HS_TIME_W<0> {
        LP2HS_TIME_W::new(self)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    #[must_use]
    pub fn hs2lp_time(&mut self) -> HS2LP_TIME_W<16> {
        HS2LP_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host data lane timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dltcr](index.html) module"]
pub struct DLTCR_SPEC;
impl crate::RegisterSpec for DLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dltcr::R](R) reader structure"]
impl crate::Readable for DLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dltcr::W](W) writer structure"]
impl crate::Writable for DLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLTCR to value 0"]
impl crate::Resettable for DLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
