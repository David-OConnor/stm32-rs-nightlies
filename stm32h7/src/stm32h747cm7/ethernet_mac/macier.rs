#[doc = "Register `MACIER` reader"]
pub struct R(crate::R<MACIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACIER` writer"]
pub struct W(crate::W<MACIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACIER_SPEC>;
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
impl From<crate::W<MACIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHYIE` reader - PHY Interrupt Enable"]
pub type PHYIE_R = crate::BitReader<bool>;
#[doc = "Field `PHYIE` writer - PHY Interrupt Enable"]
pub type PHYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
#[doc = "Field `PMTIE` reader - PMT Interrupt Enable"]
pub type PMTIE_R = crate::BitReader<bool>;
#[doc = "Field `PMTIE` writer - PMT Interrupt Enable"]
pub type PMTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
#[doc = "Field `LPIIE` reader - LPI Interrupt Enable"]
pub type LPIIE_R = crate::BitReader<bool>;
#[doc = "Field `LPIIE` writer - LPI Interrupt Enable"]
pub type LPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - Timestamp Interrupt Enable"]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - Timestamp Interrupt Enable"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
#[doc = "Field `TXSTSIE` reader - Transmit Status Interrupt Enable"]
pub type TXSTSIE_R = crate::BitReader<bool>;
#[doc = "Field `TXSTSIE` writer - Transmit Status Interrupt Enable"]
pub type TXSTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
#[doc = "Field `RXSTSIE` reader - Receive Status Interrupt Enable"]
pub type RXSTSIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSTSIE` writer - Receive Status Interrupt Enable"]
pub type RXSTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn phyie(&mut self) -> PHYIE_W<3> {
        PHYIE_W::new(self)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pmtie(&mut self) -> PMTIE_W<4> {
        PMTIE_W::new(self)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpiie(&mut self) -> LPIIE_W<5> {
        LPIIE_W::new(self)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<12> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txstsie(&mut self) -> TXSTSIE_W<13> {
        TXSTSIE_W::new(self)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxstsie(&mut self) -> RXSTSIE_W<14> {
        RXSTSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macier](index.html) module"]
pub struct MACIER_SPEC;
impl crate::RegisterSpec for MACIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macier::R](R) reader structure"]
impl crate::Readable for MACIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macier::W](W) writer structure"]
impl crate::Writable for MACIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACIER to value 0"]
impl crate::Resettable for MACIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
