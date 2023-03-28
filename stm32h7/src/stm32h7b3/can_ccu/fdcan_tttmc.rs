#[doc = "Register `FDCAN_TTTMC` reader"]
pub struct R(crate::R<FDCAN_TTTMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTTMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTTMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTTMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTTMC` writer"]
pub struct W(crate::W<FDCAN_TTTMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTTMC_SPEC>;
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
impl From<crate::W<FDCAN_TTTMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTTMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMSA` reader - Trigger Memory Start Address"]
pub type TMSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TMSA` writer - Trigger Memory Start Address"]
pub type TMSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTTMC_SPEC, u16, u16, 14, O>;
#[doc = "Field `TME` reader - Trigger Memory Elements"]
pub type TME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TME` writer - Trigger Memory Elements"]
pub type TME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTTMC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    pub fn tmsa(&self) -> TMSA_R {
        TMSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Trigger Memory Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn tmsa(&mut self) -> TMSA_W<2> {
        TMSA_W::new(self)
    }
    #[doc = "Bits 16:22 - Trigger Memory Elements"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<16> {
        TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Trigger Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tttmc](index.html) module"]
pub struct FDCAN_TTTMC_SPEC;
impl crate::RegisterSpec for FDCAN_TTTMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tttmc::R](R) reader structure"]
impl crate::Readable for FDCAN_TTTMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_tttmc::W](W) writer structure"]
impl crate::Writable for FDCAN_TTTMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTMC to value 0"]
impl crate::Resettable for FDCAN_TTTMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
