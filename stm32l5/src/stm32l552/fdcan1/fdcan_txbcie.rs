#[doc = "Register `FDCAN_TXBCIE` reader"]
pub struct R(crate::R<FDCAN_TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TXBCIE` writer"]
pub struct W(crate::W<FDCAN_TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBCIE_SPEC>;
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
impl From<crate::W<FDCAN_TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF` reader - Cancellation Finished Interrupt Enable"]
pub type CF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CF` writer - Cancellation Finished Interrupt Enable"]
pub type CF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBCIE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<0> {
        CF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_txbcie](index.html) module"]
pub struct FDCAN_TXBCIE_SPEC;
impl crate::RegisterSpec for FDCAN_TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_txbcie::R](R) reader structure"]
impl crate::Readable for FDCAN_TXBCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_txbcie::W](W) writer structure"]
impl crate::Writable for FDCAN_TXBCIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TXBCIE to value 0"]
impl crate::Resettable for FDCAN_TXBCIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
