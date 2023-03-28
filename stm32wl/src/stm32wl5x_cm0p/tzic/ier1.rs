#[doc = "Register `IER1` reader"]
pub struct R(crate::R<IER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER1` writer"]
pub struct W(crate::W<IER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER1_SPEC>;
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
impl From<crate::W<IER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZICIE` reader - TZICIE"]
pub type TZICIE_R = crate::BitReader<bool>;
#[doc = "Field `TZICIE` writer - TZICIE"]
pub type TZICIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `TZSCIE` reader - TZSCIE"]
pub type TZSCIE_R = crate::BitReader<bool>;
#[doc = "Field `TZSCIE` writer - TZSCIE"]
pub type TZSCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `AESIE` reader - AESIE"]
pub type AESIE_R = crate::BitReader<bool>;
#[doc = "Field `AESIE` writer - AESIE"]
pub type AESIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `RNGIE` reader - RNGIE"]
pub type RNGIE_R = crate::BitReader<bool>;
#[doc = "Field `RNGIE` writer - RNGIE"]
pub type RNGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SUBGHZSPIIE` reader - SUBGHZSPIIE"]
pub type SUBGHZSPIIE_R = crate::BitReader<bool>;
#[doc = "Field `SUBGHZSPIIE` writer - SUBGHZSPIIE"]
pub type SUBGHZSPIIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `PWRIE` reader - PWRIE"]
pub type PWRIE_R = crate::BitReader<bool>;
#[doc = "Field `PWRIE` writer - PWRIE"]
pub type PWRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `FLASHIFIE` reader - FLASHIFIE"]
pub type FLASHIFIE_R = crate::BitReader<bool>;
#[doc = "Field `FLASHIFIE` writer - FLASHIFIE"]
pub type FLASHIFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `DMA1IE` reader - DMA1IE"]
pub type DMA1IE_R = crate::BitReader<bool>;
#[doc = "Field `DMA1IE` writer - DMA1IE"]
pub type DMA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `DMA2IE` reader - DMA2IE"]
pub type DMA2IE_R = crate::BitReader<bool>;
#[doc = "Field `DMA2IE` writer - DMA2IE"]
pub type DMA2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `DMAMUX1IE` reader - DMAMUX1IE"]
pub type DMAMUX1IE_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1IE` writer - DMAMUX1IE"]
pub type DMAMUX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `FLASHIE` reader - FLASHIE"]
pub type FLASHIE_R = crate::BitReader<bool>;
#[doc = "Field `FLASHIE` writer - FLASHIE"]
pub type FLASHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SRAM1IE` reader - SRAM1IE"]
pub type SRAM1IE_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1IE` writer - SRAM1IE"]
pub type SRAM1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `SRAM2IE` reader - SRAM2IE"]
pub type SRAM2IE_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2IE` writer - SRAM2IE"]
pub type SRAM2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
#[doc = "Field `PKAIE` reader - PKAIE"]
pub type PKAIE_R = crate::BitReader<bool>;
#[doc = "Field `PKAIE` writer - PKAIE"]
pub type PKAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&self) -> SUBGHZSPIIE_R {
        SUBGHZSPIIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&self) -> FLASHIFIE_R {
        FLASHIFIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzicie(&mut self) -> TZICIE_W<0> {
        TZICIE_W::new(self)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tzscie(&mut self) -> TZSCIE_W<1> {
        TZSCIE_W::new(self)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    #[must_use]
    pub fn aesie(&mut self) -> AESIE_W<2> {
        AESIE_W::new(self)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    #[must_use]
    pub fn rngie(&mut self) -> RNGIE_W<3> {
        RNGIE_W::new(self)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspiie(&mut self) -> SUBGHZSPIIE_W<4> {
        SUBGHZSPIIE_W::new(self)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    #[must_use]
    pub fn pwrie(&mut self) -> PWRIE_W<5> {
        PWRIE_W::new(self)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    #[must_use]
    pub fn flashifie(&mut self) -> FLASHIFIE_W<6> {
        FLASHIFIE_W::new(self)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma1ie(&mut self) -> DMA1IE_W<7> {
        DMA1IE_W::new(self)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    #[must_use]
    pub fn dma2ie(&mut self) -> DMA2IE_W<8> {
        DMA2IE_W::new(self)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W<9> {
        DMAMUX1IE_W::new(self)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    #[must_use]
    pub fn flashie(&mut self) -> FLASHIE_W<10> {
        FLASHIE_W::new(self)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    #[must_use]
    pub fn sram1ie(&mut self) -> SRAM1IE_W<11> {
        SRAM1IE_W::new(self)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    #[must_use]
    pub fn sram2ie(&mut self) -> SRAM2IE_W<12> {
        SRAM2IE_W::new(self)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    #[must_use]
    pub fn pkaie(&mut self) -> PKAIE_W<13> {
        PKAIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](index.html) module"]
pub struct IER1_SPEC;
impl crate::RegisterSpec for IER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier1::R](R) reader structure"]
impl crate::Readable for IER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier1::W](W) writer structure"]
impl crate::Writable for IER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER1 to value 0xffff_ffff"]
impl crate::Resettable for IER1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
