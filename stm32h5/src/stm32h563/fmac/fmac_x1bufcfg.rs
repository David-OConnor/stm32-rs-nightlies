#[doc = "Register `FMAC_X1BUFCFG` reader"]
pub struct R(crate::R<FMAC_X1BUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMAC_X1BUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMAC_X1BUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMAC_X1BUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMAC_X1BUFCFG` writer"]
pub struct W(crate::W<FMAC_X1BUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMAC_X1BUFCFG_SPEC>;
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
impl From<crate::W<FMAC_X1BUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMAC_X1BUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X1_BASE` reader - Base address of X1 buffer"]
pub type X1_BASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X1_BASE` writer - Base address of X1 buffer"]
pub type X1_BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_X1BUFCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `X1_BUF_SIZE` reader - Allocated size of X1 buffer in 16-bit words The minimum buffer size is the number of feed-forward taps in the filter (+ the watermark threshold - 1)."]
pub type X1_BUF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `X1_BUF_SIZE` writer - Allocated size of X1 buffer in 16-bit words The minimum buffer size is the number of feed-forward taps in the filter (+ the watermark threshold - 1)."]
pub type X1_BUF_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMAC_X1BUFCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `FULL_WM` reader - Watermark for buffer full flag Defines the threshold for setting the X1 buffer full flag when operating in circular mode. The flag is set if the number of free spaces in the buffer is less than 2FULL_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred into the buffer under one interrupt. Threshold should be set to 1 if DMA write requests are enabled (DMAWEN = 1 in FMAC_CR register)."]
pub type FULL_WM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FULL_WM` writer - Watermark for buffer full flag Defines the threshold for setting the X1 buffer full flag when operating in circular mode. The flag is set if the number of free spaces in the buffer is less than 2FULL_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred into the buffer under one interrupt. Threshold should be set to 1 if DMA write requests are enabled (DMAWEN = 1 in FMAC_CR register)."]
pub type FULL_WM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMAC_X1BUFCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Base address of X1 buffer"]
    #[inline(always)]
    pub fn x1_base(&self) -> X1_BASE_R {
        X1_BASE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Allocated size of X1 buffer in 16-bit words The minimum buffer size is the number of feed-forward taps in the filter (+ the watermark threshold - 1)."]
    #[inline(always)]
    pub fn x1_buf_size(&self) -> X1_BUF_SIZE_R {
        X1_BUF_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Watermark for buffer full flag Defines the threshold for setting the X1 buffer full flag when operating in circular mode. The flag is set if the number of free spaces in the buffer is less than 2FULL_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred into the buffer under one interrupt. Threshold should be set to 1 if DMA write requests are enabled (DMAWEN = 1 in FMAC_CR register)."]
    #[inline(always)]
    pub fn full_wm(&self) -> FULL_WM_R {
        FULL_WM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base address of X1 buffer"]
    #[inline(always)]
    #[must_use]
    pub fn x1_base(&mut self) -> X1_BASE_W<0> {
        X1_BASE_W::new(self)
    }
    #[doc = "Bits 8:15 - Allocated size of X1 buffer in 16-bit words The minimum buffer size is the number of feed-forward taps in the filter (+ the watermark threshold - 1)."]
    #[inline(always)]
    #[must_use]
    pub fn x1_buf_size(&mut self) -> X1_BUF_SIZE_W<8> {
        X1_BUF_SIZE_W::new(self)
    }
    #[doc = "Bits 24:25 - Watermark for buffer full flag Defines the threshold for setting the X1 buffer full flag when operating in circular mode. The flag is set if the number of free spaces in the buffer is less than 2FULL_WM. 2: Threshold = 4 3: Threshold = 8 Setting a threshold greater than 1 allows several data to be transferred into the buffer under one interrupt. Threshold should be set to 1 if DMA write requests are enabled (DMAWEN = 1 in FMAC_CR register)."]
    #[inline(always)]
    #[must_use]
    pub fn full_wm(&mut self) -> FULL_WM_W<24> {
        FULL_WM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC X1 buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac_x1bufcfg](index.html) module"]
pub struct FMAC_X1BUFCFG_SPEC;
impl crate::RegisterSpec for FMAC_X1BUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmac_x1bufcfg::R](R) reader structure"]
impl crate::Readable for FMAC_X1BUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmac_x1bufcfg::W](W) writer structure"]
impl crate::Writable for FMAC_X1BUFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMAC_X1BUFCFG to value 0"]
impl crate::Resettable for FMAC_X1BUFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
