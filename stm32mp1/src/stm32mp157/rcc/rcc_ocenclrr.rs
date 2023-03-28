#[doc = "Register `RCC_OCENCLRR` reader"]
pub struct R(crate::R<RCC_OCENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_OCENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_OCENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_OCENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_OCENCLRR` writer"]
pub struct W(crate::W<RCC_OCENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_OCENCLRR_SPEC>;
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
impl From<crate::W<RCC_OCENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_OCENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSION` reader - HSION"]
pub type HSION_R = crate::BitReader<bool>;
#[doc = "Field `HSION` writer - HSION"]
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `HSIKERON` reader - HSIKERON"]
pub type HSIKERON_R = crate::BitReader<bool>;
#[doc = "Field `HSIKERON` writer - HSIKERON"]
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `CSION` reader - CSION"]
pub type CSION_R = crate::BitReader<bool>;
#[doc = "Field `CSION` writer - CSION"]
pub type CSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `CSIKERON` reader - CSIKERON"]
pub type CSIKERON_R = crate::BitReader<bool>;
#[doc = "Field `CSIKERON` writer - CSIKERON"]
pub type CSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub type DIGBYP_R = crate::BitReader<bool>;
#[doc = "Field `DIGBYP` writer - DIGBYP"]
pub type DIGBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `HSEON` reader - HSEON"]
pub type HSEON_R = crate::BitReader<bool>;
#[doc = "Field `HSEON` writer - HSEON"]
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `HSEKERON` reader - HSEKERON"]
pub type HSEKERON_R = crate::BitReader<bool>;
#[doc = "Field `HSEKERON` writer - HSEKERON"]
pub type HSEKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
#[doc = "Field `HSEBYP` reader - HSEBYP"]
pub type HSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `HSEBYP` writer - HSEBYP"]
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_OCENCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&self) -> HSEKERON_R {
        HSEKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<0> {
        HSION_W::new(self)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<1> {
        HSIKERON_W::new(self)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<4> {
        CSION_W::new(self)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<5> {
        CSIKERON_W::new(self)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    #[must_use]
    pub fn digbyp(&mut self) -> DIGBYP_W<7> {
        DIGBYP_W::new(self)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<8> {
        HSEON_W::new(self)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    #[must_use]
    pub fn hsekeron(&mut self) -> HSEKERON_W<9> {
        HSEKERON_W::new(self)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<10> {
        HSEBYP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ocenclrr](index.html) module"]
pub struct RCC_OCENCLRR_SPEC;
impl crate::RegisterSpec for RCC_OCENCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ocenclrr::R](R) reader structure"]
impl crate::Readable for RCC_OCENCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ocenclrr::W](W) writer structure"]
impl crate::Writable for RCC_OCENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_OCENCLRR to value 0x01"]
impl crate::Resettable for RCC_OCENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
