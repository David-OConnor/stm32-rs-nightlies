#[doc = "Register `RCC_AHB5RSTCLRR` reader"]
pub struct R(crate::R<RCC_AHB5RSTCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHB5RSTCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHB5RSTCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHB5RSTCLRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHB5RSTCLRR` writer"]
pub struct W(crate::W<RCC_AHB5RSTCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHB5RSTCLRR_SPEC>;
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
impl From<crate::W<RCC_AHB5RSTCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHB5RSTCLRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOZRST` reader - GPIOZRST"]
pub type GPIOZRST_R = crate::BitReader<bool>;
#[doc = "Field `GPIOZRST` writer - GPIOZRST"]
pub type GPIOZRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB5RSTCLRR_SPEC, bool, O>;
#[doc = "Field `CRYP1RST` reader - CRYP1RST"]
pub type CRYP1RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYP1RST` writer - CRYP1RST"]
pub type CRYP1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB5RSTCLRR_SPEC, bool, O>;
#[doc = "Field `HASH1RST` reader - HASH1RST"]
pub type HASH1RST_R = crate::BitReader<bool>;
#[doc = "Field `HASH1RST` writer - HASH1RST"]
pub type HASH1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB5RSTCLRR_SPEC, bool, O>;
#[doc = "Field `RNG1RST` reader - RNG1RST"]
pub type RNG1RST_R = crate::BitReader<bool>;
#[doc = "Field `RNG1RST` writer - RNG1RST"]
pub type RNG1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB5RSTCLRR_SPEC, bool, O>;
#[doc = "Field `AXIMCRST` reader - AXIMCRST"]
pub type AXIMCRST_R = crate::BitReader<bool>;
#[doc = "Field `AXIMCRST` writer - AXIMCRST"]
pub type AXIMCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_AHB5RSTCLRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W<0> {
        GPIOZRST_W::new(self)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    #[must_use]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W<4> {
        CRYP1RST_W::new(self)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    #[must_use]
    pub fn hash1rst(&mut self) -> HASH1RST_W<5> {
        HASH1RST_W::new(self)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    #[must_use]
    pub fn rng1rst(&mut self) -> RNG1RST_W<6> {
        RNG1RST_W::new(self)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    #[must_use]
    pub fn aximcrst(&mut self) -> AXIMCRST_W<16> {
        AXIMCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahb5rstclrr](index.html) module"]
pub struct RCC_AHB5RSTCLRR_SPEC;
impl crate::RegisterSpec for RCC_AHB5RSTCLRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahb5rstclrr::R](R) reader structure"]
impl crate::Readable for RCC_AHB5RSTCLRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahb5rstclrr::W](W) writer structure"]
impl crate::Writable for RCC_AHB5RSTCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_AHB5RSTCLRR to value 0"]
impl crate::Resettable for RCC_AHB5RSTCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
