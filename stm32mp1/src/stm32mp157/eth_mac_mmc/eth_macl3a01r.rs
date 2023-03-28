#[doc = "Register `ETH_MACL3A01R` reader"]
pub struct R(crate::R<ETH_MACL3A01R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL3A01R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL3A01R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL3A01R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACL3A01R` writer"]
pub struct W(crate::W<ETH_MACL3A01R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL3A01R_SPEC>;
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
impl From<crate::W<ETH_MACL3A01R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL3A01R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A01` reader - L3A01"]
pub type L3A01_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L3A01` writer - L3A01"]
pub type L3A01_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACL3A01R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - L3A01"]
    #[inline(always)]
    pub fn l3a01(&self) -> L3A01_R {
        L3A01_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A01"]
    #[inline(always)]
    #[must_use]
    pub fn l3a01(&mut self) -> L3A01_W<0> {
        L3A01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For IPv4 packets, the Layer 3 Address 0 Register 0 register contains the 32-bit IP Source Address field. For IPv6 packets, it contains Bits\\[31:0\\]
of the 128-bit IP Source Address or Destination Address field.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macl3a01r](index.html) module"]
pub struct ETH_MACL3A01R_SPEC;
impl crate::RegisterSpec for ETH_MACL3A01R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macl3a01r::R](R) reader structure"]
impl crate::Readable for ETH_MACL3A01R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macl3a01r::W](W) writer structure"]
impl crate::Writable for ETH_MACL3A01R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACL3A01R to value 0"]
impl crate::Resettable for ETH_MACL3A01R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
