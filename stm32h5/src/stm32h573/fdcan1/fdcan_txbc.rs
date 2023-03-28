#[doc = "Register `FDCAN_TXBC` reader"]
pub struct R(crate::R<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBC` writer"]
pub struct W(crate::W<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBC_SPEC>;
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
impl From<crate::W<FDCAN_TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFQM` reader - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TFQM_R = crate::BitReader<bool>;
#[doc = "Field `TFQM` writer - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
pub type TFQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TXBC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Tx FIFO/queue mode This is a protected write (P) bit, which means that write access by the bits is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<24> {
        TFQM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbc](index.html) module"]
pub struct FDCAN_TXBC_SPEC;
impl crate::RegisterSpec for FDCAN_TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbc::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbc::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBC to value 0"]
impl crate::Resettable for FDCAN_TXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
