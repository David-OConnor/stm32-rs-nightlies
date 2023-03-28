#[doc = "Register `IOPSMENR` reader"]
pub struct R(crate::R<IOPSMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOPSMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOPSMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOPSMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOPSMENR` writer"]
pub struct W(crate::W<IOPSMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOPSMENR_SPEC>;
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
impl From<crate::W<IOPSMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOPSMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode Set and cleared by software."]
pub type GPIOASMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode Set and cleared by software."]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode Set and cleared by software."]
pub type GPIOBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode Set and cleared by software."]
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode Set and cleared by software."]
pub type GPIOCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode Set and cleared by software."]
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode Set and cleared by software."]
pub type GPIODSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode Set and cleared by software."]
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
#[doc = "Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode Set and cleared by software."]
pub type GPIOFSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode Set and cleared by software."]
pub type GPIOFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOPSMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    pub fn gpiofsmen(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn gpiofsmen(&mut self) -> GPIOFSMEN_W<5> {
        GPIOFSMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC I/O port in Sleep mode clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iopsmenr](index.html) module"]
pub struct IOPSMENR_SPEC;
impl crate::RegisterSpec for IOPSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iopsmenr::R](R) reader structure"]
impl crate::Readable for IOPSMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iopsmenr::W](W) writer structure"]
impl crate::Writable for IOPSMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOPSMENR to value 0x3f"]
impl crate::Resettable for IOPSMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
