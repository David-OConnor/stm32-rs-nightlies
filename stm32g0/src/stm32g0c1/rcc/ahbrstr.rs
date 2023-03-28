#[doc = "Register `AHBRSTR` reader"]
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRSTR` writer"]
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `DMA2RST` reader - DMA1 reset"]
pub type DMA2RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA2RST` writer - DMA1 reset"]
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `FLASHRST` reader - FLITF reset"]
pub type FLASHRST_R = crate::BitReader<bool>;
#[doc = "Field `FLASHRST` writer - FLITF reset"]
pub type FLASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader<bool>;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `AESRST` reader - AES hardware accelerator reset"]
pub type AESRST_R = crate::BitReader<bool>;
#[doc = "Field `AESRST` writer - AES hardware accelerator reset"]
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
#[doc = "Field `RNGRST` reader - Random number generator reset"]
pub type RNGRST_R = crate::BitReader<bool>;
#[doc = "Field `RNGRST` writer - Random number generator reset"]
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBRSTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    #[must_use]
    pub fn flashrst(&mut self) -> FLASHRST_W<8> {
        FLASHRST_W::new(self)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 16 - AES hardware accelerator reset"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<16> {
        AESRST_W::new(self)
    }
    #[doc = "Bit 18 - Random number generator reset"]
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](index.html) module"]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrstr::R](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
