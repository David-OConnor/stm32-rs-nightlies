#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLSRC` writer - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLM` reader - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLM` writer - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
pub type PLLM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLN` reader - Main PLLSYS multiplication factor N"]
pub type PLLN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLN` writer - Main PLLSYS multiplication factor N"]
pub type PLLN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 7, O>;
#[doc = "Field `PLLPEN` reader - Main PLLSYSP output enable"]
pub type PLLPEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLPEN` writer - Main PLLSYSP output enable"]
pub type PLLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLP` reader - Main PLL division factor P for PPLSYSSAICLK"]
pub type PLLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLP` writer - Main PLL division factor P for PPLSYSSAICLK"]
pub type PLLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PLLQEN` reader - Main PLLSYSQ output enable"]
pub type PLLQEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLQEN` writer - Main PLLSYSQ output enable"]
pub type PLLQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ` reader - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
pub type PLLQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLQ` writer - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
pub type PLLQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLREN` reader - Main PLLSYSR PLLCLK output enable"]
pub type PLLREN_R = crate::BitReader<bool>;
#[doc = "Field `PLLREN` writer - Main PLLSYSR PLLCLK output enable"]
pub type PLLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLR` reader - Main PLLSYS division factor R for SYSCLK (system clock)"]
pub type PLLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLR` writer - Main PLLSYS division factor R for SYSCLK (system clock)"]
pub type PLLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Main PLLSYS multiplication factor N"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Main PLLSYSP output enable"]
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK"]
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Main PLLSYSQ output enable"]
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Main PLLSYSR PLLCLK output enable"]
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)"]
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<0> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 4:6 - Division factor M for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<4> {
        PLLM_W::new(self)
    }
    #[doc = "Bits 8:14 - Main PLLSYS multiplication factor N"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<8> {
        PLLN_W::new(self)
    }
    #[doc = "Bit 16 - Main PLLSYSP output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllpen(&mut self) -> PLLPEN_W<16> {
        PLLPEN_W::new(self)
    }
    #[doc = "Bits 17:21 - Main PLL division factor P for PPLSYSSAICLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllp(&mut self) -> PLLP_W<17> {
        PLLP_W::new(self)
    }
    #[doc = "Bit 24 - Main PLLSYSQ output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllqen(&mut self) -> PLLQEN_W<24> {
        PLLQEN_W::new(self)
    }
    #[doc = "Bits 25:27 - Main PLLSYS division factor Q for PLLSYSUSBCLK"]
    #[inline(always)]
    #[must_use]
    pub fn pllq(&mut self) -> PLLQ_W<25> {
        PLLQ_W::new(self)
    }
    #[doc = "Bit 28 - Main PLLSYSR PLLCLK output enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllren(&mut self) -> PLLREN_W<28> {
        PLLREN_W::new(self)
    }
    #[doc = "Bits 29:31 - Main PLLSYS division factor R for SYSCLK (system clock)"]
    #[inline(always)]
    #[must_use]
    pub fn pllr(&mut self) -> PLLR_W<29> {
        PLLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLSYS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2204_0100"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2204_0100;
}