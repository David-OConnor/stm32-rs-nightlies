#[doc = "Register `ICR1` reader"]
pub struct R(crate::R<ICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR1` writer"]
pub struct W(crate::W<ICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR1_SPEC>;
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
impl From<crate::W<ICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZICCF` reader - TZICCF"]
pub type TZICCF_R = crate::BitReader<bool>;
#[doc = "Field `TZICCF` writer - TZICCF"]
pub type TZICCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `TZSCCF` reader - TZSCCF"]
pub type TZSCCF_R = crate::BitReader<bool>;
#[doc = "Field `TZSCCF` writer - TZSCCF"]
pub type TZSCCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `AESCF` reader - AESCF"]
pub type AESCF_R = crate::BitReader<bool>;
#[doc = "Field `AESCF` writer - AESCF"]
pub type AESCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `RNGCF` reader - RNGCF"]
pub type RNGCF_R = crate::BitReader<bool>;
#[doc = "Field `RNGCF` writer - RNGCF"]
pub type RNGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `SUBGHZSPICF` reader - SUBGHZSPICF"]
pub type SUBGHZSPICF_R = crate::BitReader<bool>;
#[doc = "Field `SUBGHZSPICF` writer - SUBGHZSPICF"]
pub type SUBGHZSPICF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `PWRCF` reader - PWRCF"]
pub type PWRCF_R = crate::BitReader<bool>;
#[doc = "Field `PWRCF` writer - PWRCF"]
pub type PWRCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `FLASHIFCF` reader - FLASHIFCF"]
pub type FLASHIFCF_R = crate::BitReader<bool>;
#[doc = "Field `FLASHIFCF` writer - FLASHIFCF"]
pub type FLASHIFCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `DMA1CF` reader - DMA1CF"]
pub type DMA1CF_R = crate::BitReader<bool>;
#[doc = "Field `DMA1CF` writer - DMA1CF"]
pub type DMA1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `DMA2CF` reader - DMA2CF"]
pub type DMA2CF_R = crate::BitReader<bool>;
#[doc = "Field `DMA2CF` writer - DMA2CF"]
pub type DMA2CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `DMAMUX1CF` reader - DMAMUX1CF"]
pub type DMAMUX1CF_R = crate::BitReader<bool>;
#[doc = "Field `DMAMUX1CF` writer - DMAMUX1CF"]
pub type DMAMUX1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `FLASHCF` reader - FLASHCF"]
pub type FLASHCF_R = crate::BitReader<bool>;
#[doc = "Field `FLASHCF` writer - FLASHCF"]
pub type FLASHCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `SRAM1CF` reader - SRAM1CF"]
pub type SRAM1CF_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1CF` writer - SRAM1CF"]
pub type SRAM1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `SRAM2CF` reader - SRAM2CF"]
pub type SRAM2CF_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2CF` writer - SRAM2CF"]
pub type SRAM2CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
#[doc = "Field `PKACF` reader - PKACF"]
pub type PKACF_R = crate::BitReader<bool>;
#[doc = "Field `PKACF` writer - PKACF"]
pub type PKACF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&self) -> TZICCF_R {
        TZICCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&self) -> TZSCCF_R {
        TZSCCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&self) -> AESCF_R {
        AESCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&self) -> RNGCF_R {
        RNGCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&self) -> SUBGHZSPICF_R {
        SUBGHZSPICF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&self) -> PWRCF_R {
        PWRCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&self) -> FLASHIFCF_R {
        FLASHIFCF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&self) -> DMA1CF_R {
        DMA1CF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&self) -> DMA2CF_R {
        DMA2CF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&self) -> DMAMUX1CF_R {
        DMAMUX1CF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&self) -> FLASHCF_R {
        FLASHCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&self) -> SRAM1CF_R {
        SRAM1CF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&self) -> SRAM2CF_R {
        SRAM2CF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&self) -> PKACF_R {
        PKACF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    #[must_use]
    pub fn tziccf(&mut self) -> TZICCF_W<0> {
        TZICCF_W::new(self)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    #[must_use]
    pub fn tzsccf(&mut self) -> TZSCCF_W<1> {
        TZSCCF_W::new(self)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    #[must_use]
    pub fn aescf(&mut self) -> AESCF_W<2> {
        AESCF_W::new(self)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    #[must_use]
    pub fn rngcf(&mut self) -> RNGCF_W<3> {
        RNGCF_W::new(self)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    #[must_use]
    pub fn subghzspicf(&mut self) -> SUBGHZSPICF_W<4> {
        SUBGHZSPICF_W::new(self)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    #[must_use]
    pub fn pwrcf(&mut self) -> PWRCF_W<5> {
        PWRCF_W::new(self)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    #[must_use]
    pub fn flashifcf(&mut self) -> FLASHIFCF_W<6> {
        FLASHIFCF_W::new(self)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    #[must_use]
    pub fn dma1cf(&mut self) -> DMA1CF_W<7> {
        DMA1CF_W::new(self)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    #[must_use]
    pub fn dma2cf(&mut self) -> DMA2CF_W<8> {
        DMA2CF_W::new(self)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux1cf(&mut self) -> DMAMUX1CF_W<9> {
        DMAMUX1CF_W::new(self)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    #[must_use]
    pub fn flashcf(&mut self) -> FLASHCF_W<10> {
        FLASHCF_W::new(self)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    #[must_use]
    pub fn sram1cf(&mut self) -> SRAM1CF_W<11> {
        SRAM1CF_W::new(self)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    #[must_use]
    pub fn sram2cf(&mut self) -> SRAM2CF_W<12> {
        SRAM2CF_W::new(self)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    #[must_use]
    pub fn pkacf(&mut self) -> PKACF_W<13> {
        PKACF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZIC interrupt status clear register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr1](index.html) module"]
pub struct ICR1_SPEC;
impl crate::RegisterSpec for ICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr1::R](R) reader structure"]
impl crate::Readable for ICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr1::W](W) writer structure"]
impl crate::Writable for ICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR1 to value 0"]
impl crate::Resettable for ICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
