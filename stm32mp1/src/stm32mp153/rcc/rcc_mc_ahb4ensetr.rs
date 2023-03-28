#[doc = "Register `RCC_MC_AHB4ENSETR` reader"]
pub struct R(crate::R<RCC_MC_AHB4ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB4ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB4ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB4ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB4ENSETR` writer"]
pub struct W(crate::W<RCC_MC_AHB4ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB4ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB4ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB4ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - GPIOAEN"]
pub type GPIOAEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOAEN` writer - GPIOAEN"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOBEN` reader - GPIOBEN"]
pub type GPIOBEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBEN` writer - GPIOBEN"]
pub type GPIOBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOCEN` reader - GPIOCEN"]
pub type GPIOCEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCEN` writer - GPIOCEN"]
pub type GPIOCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIODEN` reader - GPIODEN"]
pub type GPIODEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODEN` writer - GPIODEN"]
pub type GPIODEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOEEN` reader - GPIOEEN"]
pub type GPIOEEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOEEN` writer - GPIOEEN"]
pub type GPIOEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOFEN` reader - GPIOFEN"]
pub type GPIOFEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFEN` writer - GPIOFEN"]
pub type GPIOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOGEN` reader - GPIOGEN"]
pub type GPIOGEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOGEN` writer - GPIOGEN"]
pub type GPIOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOHEN` reader - GPIOHEN"]
pub type GPIOHEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHEN` writer - GPIOHEN"]
pub type GPIOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOIEN` reader - GPIOIEN"]
pub type GPIOIEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOIEN` writer - GPIOIEN"]
pub type GPIOIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOJEN` reader - GPIOJEN"]
pub type GPIOJEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOJEN` writer - GPIOJEN"]
pub type GPIOJEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
#[doc = "Field `GPIOKEN` reader - GPIOKEN"]
pub type GPIOKEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOKEN` writer - GPIOKEN"]
pub type GPIOKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB4ENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<9> {
        GPIOJEN_W::new(self)
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioken(&mut self) -> GPIOKEN_W<10> {
        GPIOKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb4ensetr](index.html) module"]
pub struct RCC_MC_AHB4ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB4ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb4ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB4ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb4ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB4ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_MC_AHB4ENSETR to value 0"]
impl crate::Resettable for RCC_MC_AHB4ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
