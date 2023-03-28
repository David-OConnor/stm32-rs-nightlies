#[doc = "Register `MMC_RX_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_RX_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_RX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_RX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCRCERPIM` reader - RXCRCERPIM"]
pub type RXCRCERPIM_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCERPIM` writer - RXCRCERPIM"]
pub type RXCRCERPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `RXALGNERPIM` reader - RXALGNERPIM"]
pub type RXALGNERPIM_R = crate::BitReader<bool>;
#[doc = "Field `RXALGNERPIM` writer - RXALGNERPIM"]
pub type RXALGNERPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `RXUCGPIM` reader - RXUCGPIM"]
pub type RXUCGPIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUCGPIM` writer - RXUCGPIM"]
pub type RXUCGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `RXLPIUSCIM` reader - RXLPIUSCIM"]
pub type RXLPIUSCIM_R = crate::BitReader<bool>;
#[doc = "Field `RXLPIUSCIM` writer - RXLPIUSCIM"]
pub type RXLPIUSCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `RXLPITRCIM` reader - RXLPITRCIM"]
pub type RXLPITRCIM_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIM"]
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W<5> {
        RXCRCERPIM_W::new(self)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W<6> {
        RXALGNERPIM_W::new(self)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W<17> {
        RXUCGPIM_W::new(self)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    #[must_use]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W<26> {
        RXLPIUSCIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MMC Receive Interrupt Mask register maintains the masks for the interrupts generated when receive statistic counters reach half of their maximum value or the maximum values.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_rx_interrupt_mask](index.html) module"]
pub struct MMC_RX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_rx_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_rx_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_RX_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
