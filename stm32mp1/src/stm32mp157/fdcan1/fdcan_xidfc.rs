#[doc = "Register `FDCAN_XIDFC` reader"]
pub struct R(crate::R<FDCAN_XIDFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_XIDFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_XIDFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_XIDFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_XIDFC` writer"]
pub struct W(crate::W<FDCAN_XIDFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_XIDFC_SPEC>;
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
impl From<crate::W<FDCAN_XIDFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_XIDFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLESA` reader - FLESA"]
pub type FLESA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLESA` writer - FLESA"]
pub type FLESA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_XIDFC_SPEC, u16, u16, 14, O>;
#[doc = "Field `LSE` reader - LSE"]
pub type LSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSE` writer - LSE"]
pub type LSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_XIDFC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<2> {
        FLESA_W::new(self)
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<16> {
        LSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Settings for 29-bit extended message ID filtering. The FDCAN extended ID filter configuration register controls the filter path for standard messages as described in Figure709: Extended message ID filter path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_xidfc](index.html) module"]
pub struct FDCAN_XIDFC_SPEC;
impl crate::RegisterSpec for FDCAN_XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_xidfc::R](R) reader structure"]
impl crate::Readable for FDCAN_XIDFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_xidfc::W](W) writer structure"]
impl crate::Writable for FDCAN_XIDFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_XIDFC to value 0"]
impl crate::Resettable for FDCAN_XIDFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}