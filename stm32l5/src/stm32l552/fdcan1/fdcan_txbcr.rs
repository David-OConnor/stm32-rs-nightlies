#[doc = "Register `FDCAN_TXBCR` reader"]
pub struct R(crate::R<FDCAN_TXBCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBCR` writer"]
pub struct W(crate::W<FDCAN_TXBCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBCR_SPEC>;
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
impl From<crate::W<FDCAN_TXBCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CR` reader - Cancellation Request"]
pub type CR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CR` writer - Cancellation Request"]
pub type CR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Cancellation Request"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation Request"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcr](index.html) module"]
pub struct FDCAN_TXBCR_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcr::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcr::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCR to value 0"]
impl crate::Resettable for FDCAN_TXBCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
