#[doc = "Register `ETH_DMAC0TxCR` reader"]
pub struct R(crate::R<ETH_DMAC0TX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0TX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0TX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0TX_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0TxCR` writer"]
pub struct W(crate::W<ETH_DMAC0TX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0TX_CR_SPEC>;
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
impl From<crate::W<ETH_DMAC0TX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0TX_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, bool, O>;
#[doc = "Field `TCW` reader - TCW"]
pub type TCW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCW` writer - TCW"]
pub type TCW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OSF` reader - OSF"]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - OSF"]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, bool, O>;
#[doc = "Field `TSE` reader - TSE"]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - TSE"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, bool, O>;
#[doc = "Field `TXPBL` reader - TXPBL"]
pub type TXPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXPBL` writer - TXPBL"]
pub type TXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `TQOS` reader - TQOS"]
pub type TQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TQOS` writer - TQOS"]
pub type TQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0TX_CR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    pub fn tqos(&self) -> TQOS_R {
        TQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    #[must_use]
    pub fn tcw(&mut self) -> TCW_W<1> {
        TCW_W::new(self)
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<4> {
        OSF_W::new(self)
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<12> {
        TSE_W::new(self)
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    #[must_use]
    pub fn txpbl(&mut self) -> TXPBL_W<16> {
        TXPBL_W::new(self)
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    #[must_use]
    pub fn tqos(&mut self) -> TQOS_W<24> {
        TQOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_cr](index.html) module"]
pub struct ETH_DMAC0TX_CR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0TX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0tx_cr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0TX_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_cr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0TX_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0TxCR to value 0"]
impl crate::Resettable for ETH_DMAC0TX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
