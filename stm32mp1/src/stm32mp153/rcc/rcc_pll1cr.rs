#[doc = "Register `RCC_PLL1CR` reader"]
pub struct R(crate::R<RCC_PLL1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_PLL1CR` writer"]
pub struct W(crate::W<RCC_PLL1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL1CR_SPEC>;
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
impl From<crate::W<RCC_PLL1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLON` reader - PLLON"]
pub type PLLON_R = crate::BitReader<bool>;
#[doc = "Field `PLLON` writer - PLLON"]
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CR_SPEC, bool, O>;
#[doc = "Field `PLL1RDY` reader - PLL1RDY"]
pub type PLL1RDY_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_CTRL` reader - SSCG_CTRL"]
pub type SSCG_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_CTRL` writer - SSCG_CTRL"]
pub type SSCG_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CR_SPEC, bool, O>;
#[doc = "Field `DIVPEN` reader - DIVPEN"]
pub type DIVPEN_R = crate::BitReader<bool>;
#[doc = "Field `DIVPEN` writer - DIVPEN"]
pub type DIVPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CR_SPEC, bool, O>;
#[doc = "Field `DIVQEN` reader - DIVQEN"]
pub type DIVQEN_R = crate::BitReader<bool>;
#[doc = "Field `DIVQEN` writer - DIVQEN"]
pub type DIVQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CR_SPEC, bool, O>;
#[doc = "Field `DIVREN` reader - DIVREN"]
pub type DIVREN_R = crate::BitReader<bool>;
#[doc = "Field `DIVREN` writer - DIVREN"]
pub type DIVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1RDY"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> PLL1RDY_R {
        PLL1RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<0> {
        PLLON_W::new(self)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W<2> {
        SSCG_CTRL_W::new(self)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    #[must_use]
    pub fn divpen(&mut self) -> DIVPEN_W<4> {
        DIVPEN_W::new(self)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    #[must_use]
    pub fn divqen(&mut self) -> DIVQEN_W<5> {
        DIVQEN_W::new(self)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    #[must_use]
    pub fn divren(&mut self) -> DIVREN_W<6> {
        DIVREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_pll1cr](index.html) module"]
pub struct RCC_PLL1CR_SPEC;
impl crate::RegisterSpec for RCC_PLL1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_pll1cr::R](R) reader structure"]
impl crate::Readable for RCC_PLL1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_pll1cr::W](W) writer structure"]
impl crate::Writable for RCC_PLL1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_PLL1CR to value 0"]
impl crate::Resettable for RCC_PLL1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
