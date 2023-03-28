#[doc = "Register `ETH_DMAC1IER` reader"]
pub struct R(crate::R<ETH_DMAC1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC1IER` writer"]
pub struct W(crate::W<ETH_DMAC1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC1IER_SPEC>;
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
impl From<crate::W<ETH_DMAC1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `TXSE` reader - Transmit Stopped Enable"]
pub type TXSE_R = crate::BitReader<bool>;
#[doc = "Field `TXSE` writer - Transmit Stopped Enable"]
pub type TXSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `TBUE` reader - Transmit Buffer Unavailable Enable"]
pub type TBUE_R = crate::BitReader<bool>;
#[doc = "Field `TBUE` writer - Transmit Buffer Unavailable Enable"]
pub type TBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `RBUE` reader - Receive Buffer Unavailable Enable"]
pub type RBUE_R = crate::BitReader<bool>;
#[doc = "Field `RBUE` writer - Receive Buffer Unavailable Enable"]
pub type RBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RSE_R = crate::BitReader<bool>;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `RWTE` reader - Receive Watchdog Timeout Enable"]
pub type RWTE_R = crate::BitReader<bool>;
#[doc = "Field `RWTE` writer - Receive Watchdog Timeout Enable"]
pub type RWTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `ETIE` reader - Early Transmit Interrupt Enable"]
pub type ETIE_R = crate::BitReader<bool>;
#[doc = "Field `ETIE` writer - Early Transmit Interrupt Enable"]
pub type ETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `ERIE` reader - Early Receive Interrupt Enable"]
pub type ERIE_R = crate::BitReader<bool>;
#[doc = "Field `ERIE` writer - Early Receive Interrupt Enable"]
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `FBEE` reader - Fatal Bus Error Enable"]
pub type FBEE_R = crate::BitReader<bool>;
#[doc = "Field `FBEE` writer - Fatal Bus Error Enable"]
pub type FBEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `CDEE` reader - Context Descriptor Error Enable"]
pub type CDEE_R = crate::BitReader<bool>;
#[doc = "Field `CDEE` writer - Context Descriptor Error Enable"]
pub type CDEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AIE_R = crate::BitReader<bool>;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NIE_R = crate::BitReader<bool>;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC1IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txse(&mut self) -> TXSE_W<1> {
        TXSE_W::new(self)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbue(&mut self) -> TBUE_W<2> {
        TBUE_W::new(self)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RBUE_W<7> {
        RBUE_W::new(self)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<8> {
        RSE_W::new(self)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RWTE_W<9> {
        RWTE_W::new(self)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<10> {
        ETIE_W::new(self)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<11> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FBEE_W<12> {
        FBEE_W::new(self)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cdee(&mut self) -> CDEE_W<13> {
        CDEE_W::new(self)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<14> {
        AIE_W::new(self)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<15> {
        NIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac1ier](index.html) module"]
pub struct ETH_DMAC1IER_SPEC;
impl crate::RegisterSpec for ETH_DMAC1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac1ier::R](R) reader structure"]
impl crate::Readable for ETH_DMAC1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac1ier::W](W) writer structure"]
impl crate::Writable for ETH_DMAC1IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC1IER to value 0x8000"]
impl crate::Resettable for ETH_DMAC1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
