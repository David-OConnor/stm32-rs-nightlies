#[doc = "Register `FMAC_CR` reader"]
pub struct R(crate::R<FMAC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMAC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMAC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMAC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMAC_CR` writer"]
pub struct W(crate::W<FMAC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMAC_CR_SPEC>;
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
impl From<crate::W<FMAC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMAC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIEN` reader - Enable read interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
pub type RIEN_R = crate::BitReader<bool>;
#[doc = "Field `RIEN` writer - Enable read interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
pub type RIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `WIEN` reader - Enable write interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
pub type WIEN_R = crate::BitReader<bool>;
#[doc = "Field `WIEN` writer - Enable write interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
pub type WIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `OVFLIEN` reader - Enable overflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type OVFLIEN_R = crate::BitReader<bool>;
#[doc = "Field `OVFLIEN` writer - Enable overflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type OVFLIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `UNFLIEN` reader - Enable underflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type UNFLIEN_R = crate::BitReader<bool>;
#[doc = "Field `UNFLIEN` writer - Enable underflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type UNFLIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `SATIEN` reader - Enable saturation error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type SATIEN_R = crate::BitReader<bool>;
#[doc = "Field `SATIEN` writer - Enable saturation error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
pub type SATIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `DMAREN` reader - Enable DMA read channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
pub type DMAREN_R = crate::BitReader<bool>;
#[doc = "Field `DMAREN` writer - Enable DMA read channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `DMAWEN` reader - Enable DMA write channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
pub type DMAWEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAWEN` writer - Enable DMA write channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `CLIPEN` reader - Enable clipping"]
pub type CLIPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLIPEN` writer - Enable clipping"]
pub type CLIPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Reset FMAC unit This resets the write and read pointers, the internal control logic, the FMAC_SR register and the FMAC_PARAM register, including the START bit if active. Other register settings are not affected. This bit is reset by hardware."]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Reset FMAC unit This resets the write and read pointers, the internal control logic, the FMAC_SR register and the FMAC_PARAM register, including the START bit if active. Other register settings are not affected. This bit is reset by hardware."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMAC_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable read interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable write interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable overflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn ovflien(&self) -> OVFLIEN_R {
        OVFLIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable underflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn unflien(&self) -> UNFLIEN_R {
        UNFLIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable saturation error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn satien(&self) -> SATIEN_R {
        SATIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA read channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable DMA write channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable clipping"]
    #[inline(always)]
    pub fn clipen(&self) -> CLIPEN_R {
        CLIPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset FMAC unit This resets the write and read pointers, the internal control logic, the FMAC_SR register and the FMAC_PARAM register, including the START bit if active. Other register settings are not affected. This bit is reset by hardware."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable read interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<0> {
        RIEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable write interrupt This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn wien(&mut self) -> WIEN_W<1> {
        WIEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable overflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn ovflien(&mut self) -> OVFLIEN_W<2> {
        OVFLIEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable underflow error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn unflien(&mut self) -> UNFLIEN_W<3> {
        UNFLIEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable saturation error interrupts This bit is set and cleared by software. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn satien(&mut self) -> SATIEN_W<4> {
        SATIEN_W::new(self)
    }
    #[doc = "Bit 8 - Enable DMA read channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<8> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 9 - Enable DMA write channel requests This bit can only be modified when START= 0 in the FMAC_PARAM register. A read returns the current state of the bit."]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<9> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 15 - Enable clipping"]
    #[inline(always)]
    #[must_use]
    pub fn clipen(&mut self) -> CLIPEN_W<15> {
        CLIPEN_W::new(self)
    }
    #[doc = "Bit 16 - Reset FMAC unit This resets the write and read pointers, the internal control logic, the FMAC_SR register and the FMAC_PARAM register, including the START bit if active. Other register settings are not affected. This bit is reset by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<16> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac_cr](index.html) module"]
pub struct FMAC_CR_SPEC;
impl crate::RegisterSpec for FMAC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmac_cr::R](R) reader structure"]
impl crate::Readable for FMAC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmac_cr::W](W) writer structure"]
impl crate::Writable for FMAC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMAC_CR to value 0"]
impl crate::Resettable for FMAC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
