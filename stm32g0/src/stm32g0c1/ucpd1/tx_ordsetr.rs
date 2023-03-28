#[doc = "Register `TX_ORDSETR` reader"]
pub struct R(crate::R<TX_ORDSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ORDSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ORDSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ORDSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_ORDSETR` writer"]
pub struct W(crate::W<TX_ORDSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_ORDSETR_SPEC>;
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
impl From<crate::W<TX_ORDSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_ORDSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXORDSET` reader - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
pub type TXORDSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXORDSET` writer - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
pub type TXORDSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_ORDSETR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last."]
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<0> {
        TXORDSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD Tx ordered set type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ordsetr](index.html) module"]
pub struct TX_ORDSETR_SPEC;
impl crate::RegisterSpec for TX_ORDSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ordsetr::R](R) reader structure"]
impl crate::Readable for TX_ORDSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ordsetr::W](W) writer structure"]
impl crate::Writable for TX_ORDSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_ORDSETR to value 0"]
impl crate::Resettable for TX_ORDSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
