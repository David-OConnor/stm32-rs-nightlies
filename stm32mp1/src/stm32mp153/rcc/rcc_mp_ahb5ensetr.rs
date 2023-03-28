#[doc = "Register `RCC_MP_AHB5ENSETR` reader"]
pub struct R(crate::R<RCC_MP_AHB5ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB5ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB5ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB5ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB5ENSETR` writer"]
pub struct W(crate::W<RCC_MP_AHB5ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB5ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB5ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB5ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOZEN` reader - GPIOZEN"]
pub type GPIOZEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOZEN` writer - GPIOZEN"]
pub type GPIOZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
#[doc = "Field `CRYP1EN` reader - CRYP1EN"]
pub type CRYP1EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYP1EN` writer - CRYP1EN"]
pub type CRYP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
#[doc = "Field `HASH1EN` reader - HASH1EN"]
pub type HASH1EN_R = crate::BitReader<bool>;
#[doc = "Field `HASH1EN` writer - HASH1EN"]
pub type HASH1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
#[doc = "Field `RNG1EN` reader - RNG1EN"]
pub type RNG1EN_R = crate::BitReader<bool>;
#[doc = "Field `RNG1EN` writer - RNG1EN"]
pub type RNG1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
#[doc = "Field `BKPSRAMEN` reader - BKPSRAMEN"]
pub type BKPSRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `BKPSRAMEN` writer - BKPSRAMEN"]
pub type BKPSRAMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
#[doc = "Field `AXIMCEN` reader - AXIMCEN"]
pub type AXIMCEN_R = crate::BitReader<bool>;
#[doc = "Field `AXIMCEN` writer - AXIMCEN"]
pub type AXIMCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_AHB5ENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    pub fn aximcen(&self) -> AXIMCEN_R {
        AXIMCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiozen(&mut self) -> GPIOZEN_W<0> {
        GPIOZEN_W::new(self)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    #[must_use]
    pub fn cryp1en(&mut self) -> CRYP1EN_W<4> {
        CRYP1EN_W::new(self)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    #[must_use]
    pub fn hash1en(&mut self) -> HASH1EN_W<5> {
        HASH1EN_W::new(self)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    #[must_use]
    pub fn rng1en(&mut self) -> RNG1EN_W<6> {
        RNG1EN_W::new(self)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    #[must_use]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<8> {
        BKPSRAMEN_W::new(self)
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    #[must_use]
    pub fn aximcen(&mut self) -> AXIMCEN_W<16> {
        AXIMCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb5ensetr](index.html) module"]
pub struct RCC_MP_AHB5ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB5ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb5ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB5ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb5ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB5ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB5ENSETR to value 0x0001_0000"]
impl crate::Resettable for RCC_MP_AHB5ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
