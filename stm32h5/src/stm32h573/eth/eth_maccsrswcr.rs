#[doc = "Register `ETH_MACCSRSWCR` reader"]
pub struct R(crate::R<ETH_MACCSRSWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACCSRSWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACCSRSWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACCSRSWCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACCSRSWCR` writer"]
pub struct W(crate::W<ETH_MACCSRSWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACCSRSWCR_SPEC>;
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
impl From<crate::W<ETH_MACCSRSWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACCSRSWCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCWE` reader - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read)."]
pub type RCWE_R = crate::BitReader<bool>;
#[doc = "Field `RCWE` writer - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read)."]
pub type RCWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCSRSWCR_SPEC, bool, O>;
#[doc = "Field `SEEN` reader - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space."]
pub type SEEN_R = crate::BitReader<bool>;
#[doc = "Field `SEEN` writer - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space."]
pub type SEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACCSRSWCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read)."]
    #[inline(always)]
    pub fn rcwe(&self) -> RCWE_R {
        RCWE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space."]
    #[inline(always)]
    pub fn seen(&self) -> SEEN_R {
        SEEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register Clear on Write 1 Enable When this bit is set, the access mode to some register fields changes to rc_w1 (clear on write) meaning that the application needs to set that respective bit to 1 to clear it. When this bit is reset, the access mode to these register fields remains rc_r (clear on read)."]
    #[inline(always)]
    #[must_use]
    pub fn rcwe(&mut self) -> RCWE_W<0> {
        RCWE_W::new(self)
    }
    #[doc = "Bit 8 - Slave Error Response Enable When this bit is set, the MAC responds with a Slave Error for accesses to reserved registers in CSR space. When this bit is reset, the MAC responds with an Okay response to any register accessed from CSR space."]
    #[inline(always)]
    #[must_use]
    pub fn seen(&mut self) -> SEEN_W<8> {
        SEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSR software control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_maccsrswcr](index.html) module"]
pub struct ETH_MACCSRSWCR_SPEC;
impl crate::RegisterSpec for ETH_MACCSRSWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_maccsrswcr::R](R) reader structure"]
impl crate::Readable for ETH_MACCSRSWCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_maccsrswcr::W](W) writer structure"]
impl crate::Writable for ETH_MACCSRSWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACCSRSWCR to value 0"]
impl crate::Resettable for ETH_MACCSRSWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
